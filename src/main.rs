use anyhow::{Context, Result};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];

    let mut child = Command::new(command)
        .args(command_args)
        .stdout(Stdio::piped()) // Capture stdout
        .stderr(Stdio::piped()) // Capture stderr
        .spawn()
        .with_context(|| {
            format!(
                "Failed to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        let std_out = std::str::from_utf8(&output.stdout)?;
        println!("Stdout:\n{}", std_out);

        let std_err = std::str::from_utf8(&output.stderr)?;
        println!("Stderr:\n{}", std_err);
    } else {
        std::process::exit(1);
    }

    Ok(())
}
