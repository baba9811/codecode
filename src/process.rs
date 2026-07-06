use anyhow::{Context, Result};
use std::{
    env,
    io::{ErrorKind, Write},
    path::PathBuf,
    process::{Command, Stdio},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use wait_timeout::ChildExt;

#[derive(Clone, Debug)]
pub struct CommandSpec {
    pub program: PathBuf,
    pub args: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct RunOutput {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

pub fn run_capture(command: &mut Command, input: &str, timeout: Duration) -> Result<RunOutput> {
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().context("spawn command")?;
    if let Some(stdin) = child.stdin.as_mut() {
        match stdin.write_all(input.as_bytes()) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::BrokenPipe => {}
            Err(error) => return Err(error).context("write stdin"),
        }
    }
    drop(child.stdin.take());

    let timed_out = match child.wait_timeout(timeout).context("wait for command")? {
        Some(_) => false,
        None => {
            let _ = child.kill();
            true
        }
    };
    let output = child.wait_with_output().context("read command output")?;
    Ok(RunOutput {
        code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        timed_out,
    })
}

pub fn which(name: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths).find_map(|dir| {
        let path = dir.join(name);
        if path.is_file() { Some(path) } else { None }
    })
}

pub fn shell_process(command: &str) -> Command {
    if cfg!(windows) {
        let mut process = Command::new("cmd");
        process.args(["/C", command]);
        process
    } else {
        let mut process = Command::new("sh");
        process.args(["-c", command]);
        process
    }
}

pub fn sh_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

pub fn unique_temp_path(prefix: &str, ext: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    env::temp_dir().join(format!("{prefix}-{}-{nanos}.{ext}", std::process::id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_capture_tolerates_child_that_exits_before_reading_stdin() {
        let mut command = shell_process("exit 0");
        let output = run_capture(
            &mut command,
            &"x".repeat(1024 * 1024),
            Duration::from_secs(5),
        )
        .unwrap();

        assert_eq!(output.code, Some(0));
        assert!(!output.timed_out);
    }
}
