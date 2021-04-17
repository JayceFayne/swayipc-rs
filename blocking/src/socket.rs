use std::env;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use swayipc_types::Fallible;

//TODO: try block instead of function
pub fn get_socketpath() -> Fallible<PathBuf> {
    _get_socketpath().map(PathBuf::from)
}

fn _get_socketpath() -> Fallible<String> {
    if let Ok(socketpath) = env::var("I3SOCK") {
        return Ok(socketpath);
    }
    if let Ok(socketpath) = env::var("SWAYSOCK") {
        return Ok(socketpath);
    }
    if let Ok(socketpath) = spawn("i3") {
        return Ok(socketpath);
    }
    if let Ok(socketpath) = spawn("sway") {
        return Ok(socketpath);
    }
    unreachable!()
}

fn spawn(wm: &str) -> Fallible<String> {
    let mut child = Command::new(wm)
        .arg("--get-socketpath")
        .stdout(Stdio::piped())
        .spawn()?;
    let mut buf = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut buf)?;
        buf.pop();
    }
    Ok(buf)
}
