use std::io;
use tokio_util::bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct LspCodec;

impl Decoder for LspCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        // Look for the end of headers (\r\n\r\n)
        let header_end =
            match src.windows(4).position(|window| window == b"\r\n\r\n") {
                Some(pos) => pos,
                None => return Ok(None), // Need more data
            };

        let headers_str = String::from_utf8_lossy(&src[..header_end]);
        let mut content_length: Option<usize> = None;

        for line in headers_str.lines() {
            if line.to_lowercase().starts_with("content-length:") {
                if let Some(val) = line.split(':').nth(1) {
                    content_length = val.trim().parse().ok();
                }
            }
        }

        let length = match content_length {
            Some(len) => len,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Missing Content-Length header",
                ));
            }
        };

        let total_frame_len = header_end + 4 + length;
        if src.len() < total_frame_len {
            return Ok(None); // Need full payload body
        }

        // Advance buffer past headers
        src.advance(header_end + 4);

        // Extract payload body
        let body_bytes = src.split_to(length);
        let body_str = String::from_utf8(body_bytes.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(Some(body_str))
    }
}

impl Encoder<String> for LspCodec {
    type Error = io::Error;

    fn encode(
        &mut self,
        item: String,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        let header = format!("Content-Length: {}\r\n\r\n", item.len());
        dst.extend_from_slice(header.as_bytes());
        dst.extend_from_slice(item.as_bytes());
        Ok(())
    }
}
