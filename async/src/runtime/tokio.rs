pub use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::process::Stdio;
use std::time::Duration;
use swayipc_types::Fallible;
use tokio::net::UnixStream;
use tokio::process::Command;

pub type Socket = UnixStream;

pub async fn get_socketpath_from_wm(wm: &str) -> Fallible<String> {
    let mut child = Command::new(wm)
        .arg("--get-socketpath")
        .stdout(Stdio::piped())
        .spawn()?;
    let mut buf = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut buf).await?;
        buf.pop();
    }
    child.wait().await?;
    Ok(buf)
}

pub async fn sleep(ms: u64) {
    tokio::time::sleep(Duration::from_millis(ms)).await
}
