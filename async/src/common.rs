use async_io::Async;
use futures_lite::AsyncReadExt;
use std::os::unix::net::UnixStream;
use swayipc_types::{Error::InvalidMagic, Fallible, MAGIC};

pub(super) async fn receive_from_stream(
    stream: &mut Async<UnixStream>,
) -> Fallible<(u32, Vec<u8>)> {
    let mut magic_data = [0_u8; 6];
    stream.read_exact(&mut magic_data).await?;
    if magic_data != MAGIC {
        return Err(InvalidMagic(magic_data));
    }
    let mut payload_len_buf = [0_u8; 4];
    stream.read_exact(&mut payload_len_buf).await?;
    let payload_len = u32::from_ne_bytes(payload_len_buf);
    let mut reply_type_buf = [0_u8; 4];
    stream.read_exact(&mut reply_type_buf).await?;
    let reply_type = u32::from_ne_bytes(reply_type_buf);
    let mut reply_payload = vec![0_u8; payload_len as usize];
    stream.read_exact(&mut reply_payload).await?;
    Ok((reply_type, reply_payload))
}
