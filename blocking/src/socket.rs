use std::env;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use swayipc_types::Fallible;

pub fn get_socketpath() -> PathBuf {
    PathBuf::from(if let Ok(socketpath) = env::var("I3SOCK") {
        socketpath
    } else if let Ok(socketpath) = env::var("SWAYSOCK") {
        socketpath
    } else if let Ok(socketpath) = spawn("i3") {
        socketpath
    } else if let Ok(socketpath) = spawn("sway") {
        socketpath
    } else {
        unreachable!()
    })
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
