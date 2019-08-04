use crate::errors::ConvertError;
use rust_base58::{FromBase58, ToBase58};
use std::convert::TryFrom;
use std::fmt::Error;
use std::str::FromStr;

pub const DIGEST_SIZE: usize = 32;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Digest([u8; DIGEST_SIZE]);

impl Digest {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }
}

impl ToString for Digest {
    fn to_string(&self) -> String {
        self.to_base58()
    }
}

impl std::fmt::Debug for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl FromStr for Digest {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Into<String> for Digest {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&[u8]> for Digest {
    type Error = ConvertError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < DIGEST_SIZE {
            return bad_args!("expected size {}, found {}", DIGEST_SIZE, value.len());
        }

        let mut a: [u8; DIGEST_SIZE] = Default::default();
        a.copy_from_slice(value);
        Ok(Digest(a))
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

impl From<[u8; DIGEST_SIZE]> for Digest {
    fn from(v: [u8; 32]) -> Self {
        Self(v)
    }
}

impl ToBase58 for Digest {
    fn to_base58(&self) -> String {
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
