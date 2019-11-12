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
            Ok(Some(Asset::from_bytes(buf)))
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
}

impl<R: io::Read> Deserializer for R {}
