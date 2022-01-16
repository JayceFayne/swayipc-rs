pub use futures_lite::{AsyncReadExt, AsyncWriteExt};

use async_io::Async;
use async_io::Timer;
use async_pidfd::AsyncPidFd;
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use swayipc_types::Fallible;

pub type Socket = Async<UnixStream>;

pub async fn get_socketpath_from_wm(wm: &str) -> Fallible<String> {
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

pub async fn sleep(ms: u64) {
    Timer::after(Duration::from_millis(ms)).await;
}
