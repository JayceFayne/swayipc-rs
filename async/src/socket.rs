use async_io::Async;
use async_pidfd::AsyncPidFd;
use futures_lite::AsyncReadExt;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use swayipc_types::{Error, Fallible};

pub async fn get_socketpath() -> Fallible<PathBuf> {
    if let Ok(socketpath) = env::var("I3SOCK") {
        Ok(socketpath)
    } else if let Ok(socketpath) = env::var("SWAYLOCK") {
        Ok(socketpath)
    } else if let Ok(socketpath) = spawn("i3").await {
        Ok(socketpath)
    } else if let Ok(socketpath) = spawn("sway").await {
        Ok(socketpath)
    } else {
        Err(Error::SocketNotFound)
    }
    .map(PathBuf::from)
}

async fn spawn(wm: &str) -> Fallible<String> {
    let mut child = Command::new(wm)
        .arg("--get-socketpath")
        .stdout(Stdio::piped())
        .spawn()?;
    let mut buf = String::new();
    if let Some(stdout) = child.stdout.take() {
        Async::new(stdout)?.read_to_string(&mut buf).await?;
        buf.pop();
    }
    AsyncPidFd::from_pid(child.id() as i32)?.wait().await?;
    Ok(buf)
}
