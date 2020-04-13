use crate::socket::get_path as get_path_blocking;
use crate::Fallible;
use async_std::task;

//TODO: impl without task::spawn_blocking soon as async-std supports async process spawning

pub async fn get_path() -> Fallible<String> {
    task::spawn_blocking(|| get_path_blocking()).await
}
