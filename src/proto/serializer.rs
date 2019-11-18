use crate::errors::ConvertError;
use bytes::{BigEndian, BufMut, BytesMut};

pub struct Serializer {
    bytes: BytesMut,
}

impl Serializer {
    pub fn new() -> Serializer {
        Serializer {
            bytes: BytesMut::with_capacity(1024 * 256),
        }
    }

    pub fn put(&mut self, bytes: &[u8]) {
        self.bytes.put(bytes);
    }

    pub fn byte(&mut self, b: u8) {
        self.bytes.put_u8(b)
    }

    pub fn u16string(&mut self, s: &str) -> Result<(), ConvertError> {
        if s.len() > std::u16::MAX as usize {
            return bad_args!(
                "invalid length for string, expected max {}, found {}",
                std::u16::MAX,
                s.len()
            );
        }

        self.bytes.put_u16_be(s.len() as u16);
        self.bytes.put_slice(s.as_bytes());
        Ok(())
    }

    pub fn u8string(&mut self, s: &str) -> Result<(), ConvertError> {
        if s.len() > std::u8::MAX as usize {
            return bad_args!(
                "invalid length for string, expected max {}, found {}",
                std::u8::MAX,
                s.len()
            );
        }
        self.bytes.put_u8(s.len() as u8);
        self.bytes.put_slice(s.as_bytes());
        Ok(())
    }

    pub fn u32(&mut self, v: u32) -> Result<(), ConvertError> {
        self.bytes.put_u32_be(v);
        Ok(())
    }

    pub fn u64(&mut self, v: u64) -> Result<(), ConvertError> {
        self.bytes.put_u64_be(v);
        Ok(())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_put() {
        let mut s = Serializer::new();
        s.put(b"abc");
        let rs = s.bytes.take();
        assert_eq!(b"abc"[..], rs[..]);
    }
}
