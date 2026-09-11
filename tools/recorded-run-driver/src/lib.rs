//! Bounded local IO and lifecycle control for trusted, non-daemonizing tools.
use nix::{
    errno::Errno,
    sys::signal::{Signal, killpg},
    unistd::Pid,
};
use std::{
    fs::{self, File},
    io::{self, Read},
    os::unix::process::CommandExt,
    path::Path,
    process::{Child, Command},
    time::{Duration, Instant},
};

pub const REPORT_LIMIT: u64 = 4 * 1024 * 1024;

pub fn read_bounded(path: &Path, bound: u64) -> io::Result<Vec<u8>> {
    if !fs::symlink_metadata(path)?.is_file() {
        return Err(io::Error::other("file type rejected"));
    }
    read_descriptor(File::open(path)?, bound)
}

fn read_descriptor(file: File, bound: u64) -> io::Result<Vec<u8>> {
    let meta = file.metadata()?;
    if !meta.is_file() || meta.len() > bound {
        return Err(io::Error::other("file type/size rejected"));
    }
    read_limited(file, bound)
}

fn read_limited(reader: impl Read, bound: u64) -> io::Result<Vec<u8>> {
    // Growth after metadata inspection cannot cause an unbounded allocation.
    let mut bytes = Vec::new();
    reader
        .take(
            bound
                .checked_add(1)
                .ok_or_else(|| io::Error::other("invalid bound"))?,
        )
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > bound {
        return Err(io::Error::other("file grew beyond bound"));
    }
    Ok(bytes)
}

struct Group(Child, bool);
impl Group {
    fn cleanup(&mut self) -> io::Result<()> {
        if self.1 {
            return Ok(());
        }
        let signal = killpg(Pid::from_raw(self.0.id() as i32), Signal::SIGKILL);
        let waited = self.0.wait();
        self.1 = true;
        match signal {
            Ok(()) | Err(Errno::ESRCH) => {}
            Err(error) => return Err(io::Error::other(error)),
        }
        waited.map(|_| ())
    }
}
impl Drop for Group {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

pub fn execute_bounded(command: &mut Command, report: &Path, timeout: Duration) -> io::Result<()> {
    let mut group = Group(command.process_group(0).spawn()?, false);
    let start = Instant::now();
    let result = loop {
        if start.elapsed() > timeout || fs::metadata(report).is_ok_and(|m| m.len() > REPORT_LIMIT) {
            break Err(io::Error::other("stage timed out or exceeded report bound"));
        }
        if let Some(status) = group.0.try_wait()? {
            break if status.success() {
                Ok(())
            } else {
                Err(io::Error::other(format!("stage exit {:?}", status.code())))
            };
        }
        std::thread::sleep(Duration::from_millis(25));
    };
    // Also terminate surviving descendants after a successful/nonzero parent exit.
    // This runs before consumers read or hash the completed report.
    group.cleanup()?;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        process::Stdio,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    #[test]
    fn growing_input_never_reads_beyond_cap_plus_detection_byte() {
        struct Growing(usize);
        impl Read for Growing {
            fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
                buffer.fill(b'x');
                self.0 += buffer.len();
                Ok(buffer.len())
            }
        }
        // Models continuous growth after the descriptor's metadata check.
        let mut growing = Growing(0);
        assert!(read_limited(&mut growing, 100).is_err());
        assert_eq!(growing.0, 101);
    }
    fn directory() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "recorded-lifecycle-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        path
    }
    #[test]
    fn single_descriptor_and_bounds() {
        let dir = directory();
        let path = dir.join("report");
        fs::write(&path, b"1234").unwrap();
        let opened = File::open(&path).unwrap();
        fs::rename(&path, dir.join("original")).unwrap();
        fs::write(&path, b"replacement exceeds cap").unwrap();
        assert_eq!(read_descriptor(opened, 4).unwrap(), b"1234");
        assert!(read_bounded(&path, 4).is_err());
        fs::write(&path, b"12345").unwrap();
        assert_eq!(read_bounded(&path, 5).unwrap(), b"12345");
        assert!(read_bounded(&path, 4).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
    fn descendants(script: &str, timeout: Duration, success: bool) {
        let dir = directory();
        let marker = dir.join("late-marker");
        let report = dir.join("report");
        let mut command = Command::new("/bin/sh");
        command
            .args(["-c", script, "test"])
            .arg(&marker)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(File::create(&report).unwrap());
        assert_eq!(
            execute_bounded(&mut command, &report, timeout).is_ok(),
            success
        );
        std::thread::sleep(Duration::from_millis(700));
        assert!(!marker.exists(), "descendant survived runner completion");
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn timeout_cleans_descendants() {
        descendants(
            "(sleep 0.5; echo late > \"$1\") & sleep 10",
            Duration::from_millis(100),
            false,
        );
    }
    #[test]
    fn report_overflow_cleans_descendants() {
        descendants(
            "(sleep 0.5; echo late > \"$1\") & head -c 4194305 /dev/zero; sleep 10",
            Duration::from_secs(3),
            false,
        );
    }
    #[test]
    fn normal_exit_cleans_descendants() {
        descendants(
            "(sleep 0.5; echo late > \"$1\") & exit 0",
            Duration::from_secs(3),
            true,
        );
    }
    #[test]
    fn nonzero_exit_cleans_descendants() {
        descendants(
            "(sleep 0.5; echo late > \"$1\") & exit 7",
            Duration::from_secs(3),
            false,
        );
    }
}
