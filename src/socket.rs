use crate::{bail, Fallible};
use std::{env, process};

pub fn get_path() -> Fallible<String> {
    // Check I3SOCK first to allow compatibility with i3.
    // sway sets I3SOCK to point to SWAYSOCK, so this also works for sway.
    if let Ok(sockpath) = env::var("I3SOCK") {
        return Ok(sockpath);
    }
    let output = process::Command::new("i3")
        .arg("--get-socketpath")
        .output()?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_owned());
    }
    if let Ok(sockpath) = env::var("SWAYSOCK") {
        return Ok(sockpath);
    }
    let output = process::Command::new("sway")
        .arg("--get-socketpath")
        .output()?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_owned());
    }
    let prefix = if let Some(code) = output.status.code() {
        format!("sway --get-socketpath returned with exit code {}", code)
    } else {
        "sway --get-socketpath didn't return with exit code 0".to_owned()
    };
    bail!(if output.stderr.is_empty() {
        prefix
    } else {
        format!("{}. stderr: {:?}", prefix, output.stderr)
    })
}
