use crate::char_utils::extract_error_blocks;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CompilerMessage {
    pub message: String,
    pub code: Option<DiagnosticCode>,
    pub level: String,
    pub spans: Vec<DiagnosticSpan>,
    children: Vec<CompilerMessage>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
pub struct DiagnosticCode {
    pub code: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiagnosticSpan {
    file_name: String,
    pub line_start: u32,
    pub line_end: u32,
    pub column_start: u32,
    pub column_end: u32,
    text: Vec<SpanText>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpanText {
    text: String,
    highlight_start: u32,
    highlight_end: u32,
}

trait CommandExt {
    fn add_edition(&mut self) -> &mut Self;
    fn add_allowable_lints(&mut self) -> &mut Self;
}

impl CommandExt for Command {
    fn add_edition(&mut self) -> &mut Self {
        self.arg("--edition=2021")
    }

    fn add_allowable_lints(&mut self) -> &mut Self {
        self.arg("--allow=unused_mut")
            .arg("--allow=unused_variables")
            .arg("--allow=dead_code")
            .arg("--allow=unused_must_use")
    }
}

pub fn check_rust_code(
    code: &str,
) -> Result<(Vec<CompilerMessage>, Vec<String>), Box<dyn std::error::Error>> {
    let mut json_command = Command::new("rustc")
        .arg("-") // Read from stdin
        .arg("--error-format=json")
        .add_edition()
        .add_allowable_lints()
        .arg("-o")
        .arg("-") // Output to stdout (though we'll ignore it)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write code to stdin
    if let Some(mut stdin) = json_command.stdin.take() {
        stdin.write_all(code.as_bytes())?;
    }

    let output = json_command.wait_with_output()?;

    // Parse error messages
    let messages: Vec<CompilerMessage> = String::from_utf8(output.stderr)?
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str(line).ok())
        .filter(|message: &CompilerMessage| {
            message.level == String::from("error") && !message.code.is_none()
        })
        .collect();

    let mut human_command = Command::new("rustc")
        .arg("-") // Read from stdin
        .arg("-o").arg("-") // Output to stdout (though we'll ignore it)
        .add_edition()
        .add_allowable_lints()
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write code to stdin
    if let Some(mut stdin) = human_command.stdin.take() {
        stdin.write_all(code.as_bytes())?;
    }

    let output = human_command.wait_with_output()?;
    let string_output = String::from_utf8(output.stderr)?;
    let error_blocks = extract_error_blocks(&string_output);

    if error_blocks.len() != messages.len() {
        println!("Error blocks len: {:?}", error_blocks.len());
        println!("Messages len: {:?}", messages.len());
        println!("Error blocks: {:?}", error_blocks);
        println!("Messages: {:?}", messages);
        println!("Mismatch between error blocks and messages");
    }
    Ok((messages, error_blocks))
}
