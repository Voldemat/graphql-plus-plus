use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn format_using_rustfmt(
    path_to_config: &str,
    code: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut child = Command::new("rustfmt")
        .arg("--config-path")
        .arg(path_to_config)
        .arg("--edition")
        .arg("2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let mut stdin = child.stdin.take().ok_or("Failed to open stdin")?;
        stdin.write_all(code.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        let err = String::from_utf8(output.stderr)?;
        Err(format!("rustfmt failed: {}", err).into())
    }
}
