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
