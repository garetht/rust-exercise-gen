use std::io::Write;
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CompilerMessage {
    message: String,
    pub code: Option<DiagnosticCode>,
    pub level: String,
    spans: Vec<DiagnosticSpan>,
    children: Vec<CompilerMessage>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
pub struct DiagnosticCode {
    pub code: String,
    explanation: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiagnosticSpan {
    file_name: String,
    line_start: u32,
    line_end: u32,
    column_start: u32,
    column_end: u32,
    text: Vec<SpanText>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpanText {
    text: String,
    highlight_start: u32,
    highlight_end: u32,
}

pub fn check_rust_code(code: &str) -> Result<Vec<CompilerMessage>, Box<dyn std::error::Error>> {
    let mut child = Command::new("rustc")
        .arg("-")  // Read from stdin
        .arg("--error-format=json")
        .arg("-o").arg("-")  // Output to stdout (though we'll ignore it)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write code to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(code.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    // Parse error messages
    let messages: Vec<CompilerMessage> = String::from_utf8(output.stderr)?
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();

    Ok(messages)
}
