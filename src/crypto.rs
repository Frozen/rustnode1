#![feature(try_from)]
use rust_base58::{FromBase58, ToBase58};
use std::convert::TryFrom;

const DIGEST_SIZE: usize = 32;

#[derive(Debug, Fail)]
enum ConvertError {
    #[fail(display = "invalid length: expected {}, found {}", expected, found)]
    InvalidLength { expected: usize, found: usize },

    #[fail(display = "base58 error {}", _0)]
    Base58Error(rust_base58::base58::FromBase58Error),
}

impl From<rust_base58::base58::FromBase58Error> for ConvertError {
    fn from(e: rust_base58::base58::FromBase58Error) -> Self {
        ConvertError::Base58Error(e)
    }
}

struct Digest([u8; DIGEST_SIZE]);

impl TryFrom<&str> for Digest {
    type Error = ConvertError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rs = value.from_base58()?;
        if rs.len() != DIGEST_SIZE {
            return Err(ConvertError::InvalidLength {
                expected: DIGEST_SIZE,
                found: rs.len(),
            }
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
    fn check_to_and_from_base58() {
        let s = "FXYA6oQLoBuGKNbmRSrXAk3N539Qd6mLWxRGHD44NQoi";
        let a = Digest::try_from(s).unwrap();
        assert!(Digest::try_from("").is_err());

        let z: String = a.into();

        assert_eq!("FXYA6oQLoBuGKNbmRSrXAk3N539Qd6mLWxRGHD44NQoi", &z)
    }

}
