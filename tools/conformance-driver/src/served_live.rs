// SPDX-License-Identifier: MIT

use serde_json::{Value, json};
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const TOKEN: &str = "served-live-conformance-token";
const MAX_RESPONSE_BYTES: usize = 64 * 1024;

struct Arguments {
    runtime_binary: PathBuf,
    workflow_binary: PathBuf,
    definition: PathBuf,
    capability_contract: PathBuf,
    target_catalog: PathBuf,
    provider_capabilities: PathBuf,
}

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn create() -> Result<Self, String> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("read clock: {error}"))?
            .as_nanos();
        let path = env::temp_dir().join(format!(
            "ascension-served-live-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir(&path).map_err(|error| format!("create temporary directory: {error}"))?;
        Ok(Self(path))
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct Server(Child);

impl Server {
    fn stop(&mut self) -> String {
        let _ = self.0.kill();
        let _ = self.0.wait();
        let mut stderr = String::new();
        if let Some(mut reader) = self.0.stderr.take() {
            let _ = reader.read_to_string(&mut stderr);
        }
        let stderr = stderr.trim();
        if stderr.is_empty() {
            String::new()
        } else {
            format!("; service stderr: {stderr}")
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => {
            println!(
                "served-live conformance: production target catalog and capabilities match; live definition validates"
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("served-live conformance: FAIL: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let arguments = parse_arguments(env::args().skip(1).collect())?;
    let definition = read_json(&arguments.definition)?;
    let expected_capabilities = read_json(&arguments.capability_contract)?;
    let expected_target_catalog = read_json(&arguments.target_catalog)?;
    let provider_capabilities = read_json(&arguments.provider_capabilities)?;
    if definition.get("mode").and_then(Value::as_str) != Some("strict")
        || definition.get("game_profile").and_then(Value::as_str) != Some("sts2-live-v1")
        || definition
            .pointer("/annotations/synthetic")
            .and_then(Value::as_bool)
            != Some(false)
    {
        return Err(String::from(
            "served-live definition must be strict, non-synthetic, and use sts2-live-v1",
        ));
    }
    let temporary = TemporaryDirectory::create()?;
    let address = format!("127.0.0.1:{}", free_port()?);
    let mut server = start_server(
        &arguments.runtime_binary,
        &address,
        &temporary,
        &provider_capabilities,
    )?;
    let result = exercise(
        &arguments,
        &address,
        &temporary,
        &expected_capabilities,
        &expected_target_catalog,
        &mut server,
    );
    let diagnostics = server.stop();
    result.map_err(|error| format!("{error}{diagnostics}"))
}

fn exercise(
    arguments: &Arguments,
    address: &str,
    temporary: &TemporaryDirectory,
    expected_capabilities: &Value,
    expected_target_catalog: &Value,
    server: &mut Server,
) -> Result<(), String> {
    wait_for_health(address, server)?;

    let target_catalog = http_get(address, "/v1/workflow-targets", Some(TOKEN))?;
    if &target_catalog != expected_target_catalog {
        return Err(format!(
            "served target catalog drifted from {}",
            arguments.target_catalog.display()
        ));
    }

    let capability_response = http_get(address, "/v1/capabilities", Some(TOKEN))?;
    let reported_capabilities = capability_response
        .get("capabilities")
        .ok_or("served capabilities response omitted its capability manifest")?;
    if reported_capabilities != expected_capabilities {
        return Err(format!(
            "served capability manifest drifted from {}",
            arguments.capability_contract.display()
        ));
    }

    let actual_manifest_path = temporary.path("reported-capabilities.json");
    fs::write(
        &actual_manifest_path,
        serde_json::to_vec(reported_capabilities)
            .map_err(|error| format!("encode reported capabilities: {error}"))?,
    )
    .map_err(|error| format!("write reported capability manifest: {error}"))?;
    let validation = Command::new(&arguments.workflow_binary)
        .args([
            "validate",
            path_to_str(&arguments.definition)?,
            "--capabilities",
            path_to_str(&actual_manifest_path)?,
            "--format",
            "json",
            "--listen",
            address,
            "--auth-profile",
            "served",
        ])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("STS2_WORKFLOW_TOKEN_SERVED", TOKEN)
        .output()
        .map_err(|error| format!("run Harness live validation command: {error}"))?;
    if !validation.status.success() {
        return Err(format!(
            "Harness live validation failed with {}: {}",
            validation.status,
            String::from_utf8_lossy(&validation.stderr)
        ));
    }
    let validation: Value = serde_json::from_slice(&validation.stdout)
        .map_err(|error| format!("parse Harness validation response: {error}"))?;
    if validation.get("valid").and_then(Value::as_bool) != Some(true) {
        return Err(format!(
            "Harness rejected the served-live definition: {}",
            validation
        ));
    }
    Ok(())
}

fn start_server(
    binary: &Path,
    address: &str,
    temporary: &TemporaryDirectory,
    provider_capabilities: &Value,
) -> Result<Server, String> {
    let provider_configuration = json!({
        "schema_version": "ascension.workflow-provider-policy-config.v1",
        "store_path": temporary.path("provider-policy.sqlite"),
        "key_reference": "STS2_SERVED_PROVIDER_POLICY_KEY",
        "scope": {
            "project_id": "served-live-conformance",
            "run_id": "run.workflow.conformance",
            "episode_id": "episode.workflow.conformance",
            "agent_id": "agent.workflow.conformance"
        },
        "capabilities": provider_capabilities,
        "selected_profile": "codex-app-server-fixture-v1"
    });
    let context_configuration = json!({
        "schema_version": "ascension.workflow-context-owner-config.v1",
        "store_path": temporary.path("context-owner.sqlite"),
        "key_reference": "STS2_SERVED_CONTEXT_OWNER_KEY",
        "owner_id": "served-live-conformance",
        "owner_version": "v1",
        "context_ref": "context.live.v1",
        "limits": {
            "max_items": 64,
            "max_notes": 16,
            "max_context_bytes": 131072,
            "max_objective_bytes": 512,
            "max_control_events": 64
        }
    });
    Command::new(binary)
        .arg("serve-workflow")
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("STS2_WORKFLOW_LISTEN", address)
        .env("STS2_WORKFLOW_STORE", temporary.path("workflow.sqlite"))
        .env("STS2_WORKFLOW_AUTH_PROFILE", "served")
        .env("STS2_WORKFLOW_TOKEN_SERVED", TOKEN)
        .env(
            "STS2_WORKFLOW_PROVIDER_POLICY_CONFIG",
            serde_json::to_string(&provider_configuration)
                .map_err(|error| format!("encode provider-policy configuration: {error}"))?,
        )
        .env(
            "STS2_SERVED_PROVIDER_POLICY_KEY",
            "1111111111111111111111111111111111111111111111111111111111111111",
        )
        .env(
            "STS2_WORKFLOW_CONTEXT_OWNER_CONFIG",
            serde_json::to_string(&context_configuration)
                .map_err(|error| format!("encode context-owner configuration: {error}"))?,
        )
        .env(
            "STS2_SERVED_CONTEXT_OWNER_KEY",
            "2222222222222222222222222222222222222222222222222222222222222222",
        )
        .env("STS2_GATEWAY_ADDR", "127.0.0.1:1")
        .env("STS2_GATEWAY_TOKEN", "unused-conformance-token")
        .env("STS2_MCP_BINARY", "/bin/false")
        .env("STS2_RUNTIME_PROFILE", "runtime-v4-expert")
        .env("STS2_INSTANCE_ID", "instance.workflow.conformance")
        .env("STS2_CALLER_ID", "caller.workflow.conformance")
        .env("STS2_SESSION_ID", "session.workflow.conformance")
        .env("STS2_LEASE_ID", "lease.workflow.conformance")
        .env("STS2_LEASE_EPOCH", "1")
        .env("STS2_MCP_SESSION_ID", "mcp-session.workflow.conformance")
        .env("STS2_RUN_ID", "run.workflow.conformance")
        .env("STS2_EPISODE_ID", "episode.runtime.conformance")
        .env("STS2_TRAJECTORY_ID", "trajectory.workflow.conformance")
        .env("STS2_TRACE_ID", "trace.workflow.conformance")
        .env("STS2_ARTIFACT_ID", "artifact.workflow.conformance")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map(Server)
        .map_err(|error| format!("start Harness served-live workflow service: {error}"))
}

fn wait_for_health(address: &str, server: &mut Server) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if server
            .0
            .try_wait()
            .map_err(|error| format!("poll Harness workflow service: {error}"))?
            .is_some()
        {
            return Err(String::from(
                "Harness workflow service exited before health was ready",
            ));
        }
        if http_get(address, "/v1/health", None).is_ok() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(25));
    }
    Err(String::from(
        "Harness served-live service health deadline exceeded",
    ))
}

fn http_get(address: &str, path: &str, token: Option<&str>) -> Result<Value, String> {
    let address = address
        .parse::<SocketAddr>()
        .map_err(|error| format!("parse loopback address: {error}"))?;
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_millis(300))
        .map_err(|error| format!("connect to Harness workflow service: {error}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|error| format!("set HTTP read timeout: {error}"))?;
    let authorization = token.map_or(String::new(), |value| {
        format!("Authorization: Bearer {value}\r\n")
    });
    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {address}\r\n{authorization}Connection: close\r\nContent-Length: 0\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("write {path} request: {error}"))?;
    let mut response = Vec::new();
    stream
        .take((MAX_RESPONSE_BYTES + 1) as u64)
        .read_to_end(&mut response)
        .map_err(|error| format!("read {path} response: {error}"))?;
    if response.len() > MAX_RESPONSE_BYTES {
        return Err(format!(
            "{path} response exceeded {MAX_RESPONSE_BYTES} bytes"
        ));
    }
    let response = std::str::from_utf8(&response)
        .map_err(|error| format!("{path} response was not UTF-8: {error}"))?;
    let (headers, body) = response
        .split_once("\r\n\r\n")
        .ok_or_else(|| format!("{path} response omitted the HTTP header boundary"))?;
    let status = headers
        .lines()
        .next()
        .and_then(|line| line.split_ascii_whitespace().nth(1))
        .ok_or_else(|| format!("{path} response had an invalid status line"))?
        .parse::<u16>()
        .map_err(|error| format!("{path} response had an invalid status: {error}"))?;
    if status != 200 {
        return Err(format!("{path} returned HTTP {status}: {body}"));
    }
    serde_json::from_str(body).map_err(|error| format!("parse {path} JSON: {error}"))
}

fn read_json(path: &Path) -> Result<Value, String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("parse {}: {error}", path.display()))
}

fn parse_arguments(arguments: Vec<String>) -> Result<Arguments, String> {
    if !arguments.len().is_multiple_of(2) {
        return Err(String::from(
            "served-live arguments must be flag/value pairs",
        ));
    }
    let mut values = Vec::new();
    for pair in arguments.chunks_exact(2) {
        if !pair[0].starts_with("--") || values.iter().any(|(name, _)| name == &pair[0]) {
            return Err(format!("invalid or duplicate argument {}", pair[0]));
        }
        values.push((pair[0].as_str(), PathBuf::from(&pair[1])));
    }
    let get = |name: &str| {
        values
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value.clone())
            .ok_or_else(|| format!("missing required argument {name}"))
    };
    let allowed = [
        "--runtime-binary",
        "--workflow-binary",
        "--definition",
        "--capability-contract",
        "--target-catalog",
        "--provider-capabilities",
    ];
    if let Some((name, _)) = values.iter().find(|(key, _)| !allowed.contains(key)) {
        return Err(format!("unknown argument {name}"));
    }
    Ok(Arguments {
        runtime_binary: get("--runtime-binary")?,
        workflow_binary: get("--workflow-binary")?,
        definition: get("--definition")?,
        capability_contract: get("--capability-contract")?,
        target_catalog: get("--target-catalog")?,
        provider_capabilities: get("--provider-capabilities")?,
    })
}

fn free_port() -> Result<u16, String> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .map_err(|error| format!("reserve loopback port: {error}"))?;
    listener
        .local_addr()
        .map(|address| address.port())
        .map_err(|error| format!("read reserved loopback port: {error}"))
}

fn path_to_str(path: &Path) -> Result<&str, String> {
    path.to_str()
        .ok_or_else(|| format!("path is not UTF-8: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::parse_arguments;

    #[test]
    fn served_live_arguments_are_closed_and_unique() {
        let args = [
            "--runtime-binary",
            "runtime",
            "--workflow-binary",
            "workflow",
            "--definition",
            "definition.json",
            "--capability-contract",
            "capabilities.json",
            "--target-catalog",
            "targets.json",
            "--provider-capabilities",
            "provider.json",
        ]
        .map(str::to_owned)
        .to_vec();
        assert!(parse_arguments(args).is_ok());
        assert!(
            parse_arguments(vec![
                "--runtime-binary".to_owned(),
                "runtime".to_owned(),
                "--runtime-binary".to_owned(),
                "other-runtime".to_owned()
            ])
            .is_err()
        );
        assert!(
            parse_arguments(vec![
                "--runtime-binary".to_owned(),
                "runtime".to_owned(),
                "--unknown".to_owned(),
                "value".to_owned()
            ])
            .is_err()
        );
    }
}
