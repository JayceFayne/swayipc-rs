use std::io::Read;
use std::os::unix::net::UnixStream;
use swayipc_types::{Error::InvalidMagic, Fallible, MAGIC};

pub(super) fn receive_from_stream(stream: &mut UnixStream) -> Fallible<(u32, Vec<u8>)> {
    let mut header_buf = [0_u8; 14];
    stream.read_exact(&mut header_buf)?;
    let magic_data: [u8; 6] = header_buf[..6].try_into().unwrap();
    if magic_data != MAGIC {
        return Err(InvalidMagic(magic_data));
    }
    let payload_len_buf: [u8; 4] = header_buf[6..10].try_into().unwrap();
    let payload_len = u32::from_ne_bytes(payload_len_buf);
    let reply_type_buf: [u8; 4] = header_buf[10..14].try_into().unwrap();
    let reply_type = u32::from_ne_bytes(reply_type_buf);
    let mut reply_payload = vec![0_u8; payload_len as usize];
    stream.read_exact(&mut reply_payload)?;
    Ok((reply_type, reply_payload))
}
