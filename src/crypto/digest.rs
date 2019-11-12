use crate::errors::ConvertError;
use rust_base58::{FromBase58, ToBase58};
use std::convert::TryFrom;

pub const DIGEST_SIZE: usize = 32;

pub struct Digest([u8; DIGEST_SIZE]);

impl Digest {
    pub const fn from_bytes(b: [u8; DIGEST_SIZE]) -> Digest {
        Digest(b)
    }
}

impl TryFrom<&str> for Digest {
    type Error = ConvertError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rs = value.from_base58()?;
        if rs.len() != DIGEST_SIZE {
            return Err(ConvertError::InvalidLength(format!(
                "expected size {}, found {}",
                DIGEST_SIZE,
                rs.len()
            ))
            .into());
        }

        let mut a: [u8; DIGEST_SIZE] = Default::default();
        a.copy_from_slice(&rs);
        Ok(Digest(a))
    }
}

impl Into<String> for Digest {
    fn into(self) -> String {
        self.0.to_base58()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn digest_to_and_from_base58() {
        let s = "FXYA6oQLoBuGKNbmRSrXAk3N539Qd6mLWxRGHD44NQoi";
        let a = Digest::try_from(s).unwrap();
        assert!(Digest::try_from("").is_err());

        let z: String = a.into();

        assert_eq!("FXYA6oQLoBuGKNbmRSrXAk3N539Qd6mLWxRGHD44NQoi", &z)
    }
}
