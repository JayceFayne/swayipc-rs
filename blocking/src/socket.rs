use std::env;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use swayipc_types::{Error, Fallible};

pub fn get_socketpath() -> Fallible<PathBuf> {
    env::var("I3SOCK")
        .or_else(|_| env::var("SWAYSOCK"))
        .or_else(|_| spawn("i3"))
        .or_else(|_| spawn("sway"))
        .map_err(|_| Error::SocketNotFound)
        .map(PathBuf::from)
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
    child.wait()?;
    Ok(buf)
}
