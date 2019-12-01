use crate::{ensure, Fallible, MAGIC};
use std::io::Read;
use std::os::unix::net::UnixStream;

pub(crate) fn receive_from_stream(stream: &mut UnixStream) -> Fallible<(u32, Vec<u8>)> {
    let mut magic_data = [0_u8; 6];
    stream.read_exact(&mut magic_data)?;
    ensure!(
        magic_data == MAGIC,
        "unexpected magic string: expected 'i3-ipc' but got {}",
        String::from_utf8_lossy(&magic_data)
    );
    let mut payload_len_buf = [0_u8; 4];
    stream.read_exact(&mut payload_len_buf)?;
    let payload_len = u32::from_ne_bytes(payload_len_buf);
    let mut message_type_buf = [0_u8; 4];
    stream.read_exact(&mut message_type_buf)?;
    let message_type = u32::from_ne_bytes(message_type_buf);
    let mut payload_data = vec![0_u8; payload_len as usize];
    stream.read_exact(&mut payload_data[..])?;
    Ok((message_type, payload_data))
}
