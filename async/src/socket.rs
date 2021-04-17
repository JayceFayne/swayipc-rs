use async_io::Async;
use futures_lite::AsyncReadExt;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use swayipc_types::Fallible;

pub async fn get_socketpath() -> Fallible<PathBuf> {
    async {
        if let Ok(socketpath) = env::var("I3SOCK") {
            return Ok(socketpath);
        }
        if let Ok(socketpath) = env::var("SWAYSOCK") {
            return Ok(socketpath);
        }
        if let Ok(socketpath) = spawn("i3").await {
            return Ok(socketpath);
        }
        if let Ok(socketpath) = spawn("sway").await {
            return Ok(socketpath);
        }
        unreachable!()
    }
    .await
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
    Ok(buf)
}
