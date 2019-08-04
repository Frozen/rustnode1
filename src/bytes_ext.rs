use crate::crypto::{Digest, DIGEST_SIZE};
use crate::proto::Asset;
use bytes::Buf;
use bytes::BufMut;
use std::convert::TryFrom;
use std::io;
use std::io::Cursor;

pub trait BufMutExt: bytes::BufMut
where
    Self: std::marker::Sized,
{
    fn put_optional_asset(&mut self, t: Option<Asset>) {
        match t {
            Some(asset) => {
                self.put(1u8);
                self.put_slice(asset.as_bytes());
            }
            None => {
                self.put(0u8);
            }
        }
    }
}

impl BufMutExt for Vec<u8> {}
//
//pub trait BufExt: Buf {
//    fn get_optional_asset(&self) -> Option<Asset> {
//        //        self.get_u8()
//    }
//}

//struct Deserializer {}

pub trait Deserializer: io::Read {
    fn get_bool(&mut self) -> io::Result<bool> {
        Ok(self.byte()? == 1)
    }

    fn byte(&mut self) -> Result<u8, io::Error> {
        let mut buf = [0u8];
        let rs = self.read_exact(&mut buf[..])?;
        Ok(buf[0])
    }

    fn get_optional_asset(&mut self) -> io::Result<Asset> {
        let mut buf = [0u8; DIGEST_SIZE];

        self.read_exact(&mut buf)?;
        Ok(Asset::from(buf))
    }
}

impl Deserializer for Cursor<Vec<u8>> {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pub_optional_asset() {
        let mut buf = vec![];
        let digest = "BXBUNddxTGTQc3G4qHYn5E67SBwMj18zLncUr871iuRD"
            .parse()
            .unwrap();
        buf.put_optional_asset(Some(digest));

        //        buf.get_u8();
        let mut c = Cursor::new(buf);

        assert_eq!(true, c.get_bool().unwrap());
        assert_eq!(digest, c.get_optional_asset().unwrap());
        //        assert_eq!(digest, c.)
    }
}
