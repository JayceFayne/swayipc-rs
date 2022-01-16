use std::env;
use std::path::PathBuf;

use crate::runtime::get_socketpath_from_wm as spawn;

pub async fn get_socketpath() -> PathBuf {
    PathBuf::from(if let Ok(socketpath) = env::var("I3SOCK") {
        socketpath
    } else if let Ok(socketpath) = env::var("SWAYSOCK") {
        socketpath
    } else if let Ok(socketpath) = spawn("i3").await {
        socketpath
    } else if let Ok(socketpath) = spawn("sway").await {
        socketpath
    } else {
        unreachable!()
    })
}
