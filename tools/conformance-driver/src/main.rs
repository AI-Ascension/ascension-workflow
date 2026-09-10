// SPDX-License-Identifier: MIT

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const TOKEN: &str = "synthetic-conformance-token";

struct Arguments {
    binary: PathBuf,
    catalog: PathBuf,
    capabilities: PathBuf,
    missing_capabilities: PathBuf,
    negative: PathBuf,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => {
            println!("conformance: PASS");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("conformance: FAIL: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let arguments = parse_arguments(env::args().skip(1).collect())?;
    let definitions = json_files(&arguments.catalog)?;
    if definitions.len() != 13 {
        return Err(format!(
            "expected 13 catalog definitions, found {}",
            definitions.len()
        ));
    }
    let port = free_port()?;
    let address = format!("127.0.0.1:{port}");
    let store = temporary_store_path();
    let mut server = Command::new(&arguments.binary)
        .args([
            "serve",
            "--listen",
            &address,
            "--store",
            store.to_str().ok_or("temporary store path is not UTF-8")?,
            "--auth-profile",
            "synthetic",
        ])
        .env("STS2_WORKFLOW_TOKEN_SYNTHETIC", TOKEN)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("start service: {error}"))?;
    let result = exercise_process(&arguments, &address, &definitions, &mut server);
    let _ = server.kill();
    let _ = server.wait();
    let _ = fs::remove_file(store);
    result
}

fn exercise_process(
    arguments: &Arguments,
    address: &str,
    definitions: &[PathBuf],
    server: &mut Child,
) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if server
            .try_wait()
            .map_err(|error| format!("poll service: {error}"))?
            .is_some()
        {
            return Err("service exited before health was ready".to_owned());
        }
        if http_status(address, "/v1/health", None, None).is_ok() {
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }
    if http_status(address, "/v1/health", None, None).is_err() {
        return Err("service health did not become ready".to_owned());
    }
    for definition in definitions {
        let _ = run_cli(
            &arguments.binary,
            &[
                "validate",
                definition_to_str(definition)?,
                "--capabilities",
                path_to_str(&arguments.capabilities)?,
                "--listen",
                address,
                "--auth-profile",
                "synthetic",
            ],
        )?;
    }
    let negative = run_cli(
        &arguments.binary,
        &[
            "validate",
            path_to_str(&arguments.negative)?,
            "--capabilities",
            path_to_str(&arguments.missing_capabilities)?,
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    if !negative.contains("\"valid\":false") || !negative.contains("capability_unavailable") {
        return Err("negative capability case did not report a capability error".to_owned());
    }
    let map_definition = definitions
        .iter()
        .find(|path| path.file_name().and_then(|name| name.to_str()) == Some("map.strict.json"))
        .ok_or("catalog is missing map.strict.json")?;
    let run_response = run_cli(
        &arguments.binary,
        &[
            "run",
            definition_to_str(map_definition)?,
            "--instance",
            "instance.synthetic.full",
            "--profile",
            "synthetic",
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    let run_id = json_string(&run_response, "workflow_run_id")?;
    let _ = run_cli(
        &arguments.binary,
        &[
            "status",
            &run_id,
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    let _ = run_cli(
        &arguments.binary,
        &[
            "events",
            &run_id,
            "--after-sequence",
            "0",
            "--limit",
            "32",
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    for (command, revision) in [("pause", "1"), ("resume", "2"), ("step", "3")] {
        let _ = run_cli(
            &arguments.binary,
            &[
                command,
                &run_id,
                "--expected-revision",
                revision,
                "--format",
                "json",
                "--listen",
                address,
                "--auth-profile",
                "synthetic",
            ],
        )?;
    }
    let _ = run_cli(
        &arguments.binary,
        &[
            "replay",
            &run_id,
            "--offline",
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "synthetic",
        ],
    )?;
    Ok(())
}

fn run_cli(binary: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new(binary)
        .args(arguments)
        .env("STS2_WORKFLOW_TOKEN_SYNTHETIC", TOKEN)
        .output()
        .map_err(|error| format!("run CLI: {error}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(format!(
            "CLI failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn json_string(body: &str, key: &str) -> Result<String, String> {
    let marker = format!("\"{key}\":\"");
    let start = body
        .find(&marker)
        .map(|index| index + marker.len())
        .ok_or_else(|| format!("JSON response does not contain {key}"))?;
    let end = body[start..]
        .find('"')
        .map(|index| start + index)
        .ok_or_else(|| format!("JSON response has an unterminated {key}"))?;
    Ok(body[start..end].to_owned())
}

fn http_status(
    address: &str,
    path: &str,
    token: Option<&str>,
    body: Option<&[u8]>,
) -> Result<u16, String> {
    let address = address
        .parse::<SocketAddr>()
        .map_err(|error| error.to_string())?;
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_millis(200))
        .map_err(|error| error.to_string())?;
    let body = body.unwrap_or_default();
    let authorization = token.map_or(String::new(), |value| {
        format!("Authorization: Bearer {value}\r\n")
    });
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {address}\r\n{authorization}Connection: close\r\nContent-Length: {}\r\n\r\n",
        body.len()
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| error.to_string())?;
    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|error| error.to_string())?;
    let line = std::str::from_utf8(&response)
        .map_err(|error| error.to_string())?
        .lines()
        .next()
        .ok_or("empty HTTP response")?;
    line.split_ascii_whitespace()
        .nth(1)
        .ok_or("malformed HTTP response")?
        .parse::<u16>()
        .map_err(|error| error.to_string())
}

fn parse_arguments(arguments: Vec<String>) -> Result<Arguments, String> {
    let mut values = Vec::new();
    let mut index = 0;
    while index < arguments.len() {
        let name = &arguments[index];
        let value = arguments
            .get(index + 1)
            .ok_or_else(|| format!("missing value for {name}"))?;
        values.push((name.as_str(), PathBuf::from(value)));
        index += 2;
    }
    let get = |name: &str| {
        values
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value.clone())
            .ok_or_else(|| format!("missing required argument {name}"))
    };
    Ok(Arguments {
        binary: get("--binary")?,
        catalog: get("--catalog")?,
        capabilities: get("--capabilities")?,
        missing_capabilities: get("--missing-capabilities")?,
        negative: get("--negative")?,
    })
}

fn json_files(directory: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = fs::read_dir(directory)
        .map_err(|error| format!("read catalog: {error}"))?
        .map(|entry| {
            entry
                .map(|value| value.path())
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    files.retain(|path| {
        path.extension()
            .is_some_and(|extension| extension == "json")
    });
    files.sort();
    Ok(files)
}

fn free_port() -> Result<u16, String> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|error| error.to_string())?;
    Ok(listener
        .local_addr()
        .map_err(|error| error.to_string())?
        .port())
}

fn temporary_store_path() -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |value| value.as_millis());
    env::temp_dir().join(format!("ascension-conformance-{millis}.json"))
}

fn path_to_str(path: &Path) -> Result<&str, String> {
    path.to_str()
        .ok_or_else(|| format!("path is not UTF-8: {}", path.display()))
}

fn definition_to_str(path: &Path) -> Result<&str, String> {
    path_to_str(path)
}
