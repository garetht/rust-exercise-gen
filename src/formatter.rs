use std::io::Write;
use std::process::{Command, Stdio};

pub fn format_rust_code(code: &str) -> Result<String, std::io::Error> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(code.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "rustfmt failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}
