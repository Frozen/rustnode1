use crate::crypto::PublicKey;
use crate::errors::ConvertError;
use crate::proto::Asset;
use byteorder::{BigEndian, ReadBytesExt};
use std::io;

pub trait Deserializer: io::Read {
    fn byte(&mut self) -> io::Result<u8> {
        let mut x: [u8; 1] = [0];
        self.read_exact(&mut x)?;
        Ok(x[0])
    }

    fn public_key(&mut self) -> io::Result<PublicKey> {
        let mut buf = [0u8; PublicKey::size()];
        self.read_exact(&mut buf)?;
        Ok(PublicKey::from_bytes(buf))
    }

    fn asset(&mut self) -> Result<Option<Asset>, ConvertError> {
        let rs = self.bool()?;
        if rs {
            let mut buf = [0u8; Asset::size()];
            self.read_exact(&mut buf)?;
            Ok(Some(Asset::from(buf)))
        } else {
            Ok(None)
        }
    }

    fn timestamp(&mut self) -> io::Result<u64> {
        self.u64()
    }

    fn u64(&mut self) -> io::Result<u64> {
        self.read_u64::<BigEndian>()
    }

    fn bool(&mut self) -> Result<bool, ConvertError> {
        let b = self.byte()?;
        match b {
            1 => Ok(true),
            0 => Ok(false),
            _ => return bad_args!("expected bool 1 or 0, found {}", b),
        }
    }

    fn u8string(&mut self) -> Result<String, ConvertError> {
        let len = self.byte()?;
        let mut s = vec![0; len as usize];
        self.read_exact(&mut s)?;
        let out = String::from_utf8(s)?;
        Ok(out)
    }
}

impl<R: io::Read> Deserializer for R {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn u8string() {
        let bts = [3u8, 'a' as u8, 'b' as u8, 'c' as u8];
        let mut x = &bts[..];
        let rs = x.u8string().unwrap();
        assert_eq!("abc", &rs);
    }
}
