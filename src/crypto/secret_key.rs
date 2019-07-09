use crate::errors::ConvertError;
use rust_base58::{FromBase58, ToBase58};
use std::cmp::min;
use std::convert::TryFrom;

const SECRET_KEY_SIZE: usize = 32;

#[derive(Debug, Copy, Clone)]
pub struct SecretKey([u8; SECRET_KEY_SIZE]);

impl SecretKey {
    fn generate(seed: &[u8]) -> SecretKey {
        let mut sk: [u8; SECRET_KEY_SIZE] = Default::default();
        for i in 0..min(SECRET_KEY_SIZE, seed.len()) {
            sk[i] = seed[i];
        }
        sk[0] &= 248;
        sk[31] &= 127;
        sk[31] |= 64;
        return SecretKey(sk);
    }

    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&str> for SecretKey {
    type Error = ConvertError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rs = value.from_base58()?;
        if rs.len() != SECRET_KEY_SIZE {
            return Err(ConvertError::InvalidLength(format!(
                "expected size {}, found {}",
                SECRET_KEY_SIZE,
                rs.len()
            ))
            .into());
        }

        let mut a: [u8; SECRET_KEY_SIZE] = Default::default();
        a.copy_from_slice(&rs);
        Ok(SecretKey(a))
    }
}

impl Into<String> for SecretKey {
    fn into(self) -> String {
        self.0.to_base58()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn to_and_from_base58() {
        let s = "DR6Dy986fgC5C8cWv6a7AYowx35jw2wYBFMLpFe4tiQk";
        let a = SecretKey::try_from(s).unwrap();
        assert!(SecretKey::try_from("").is_err());

        let z: String = a.into();

        assert_eq!(s, &z)
    }

    #[test]
    fn test_gen_secret_key() {
        let rs = SecretKey::generate(b"abc");
        assert_eq!(
            [
                96, 98, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 64
            ],
            rs.as_bytes()
        )
    }
}
