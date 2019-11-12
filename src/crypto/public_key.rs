use crate::errors::ConvertError;
use rust_base58::{FromBase58, ToBase58};
use std::convert::TryFrom;

const PUBLIC_KEY_SIZE: usize = 32;

pub struct PublicKey([u8; PUBLIC_KEY_SIZE]);

//impl PublicKey {
//    fn generate(seed : &[u8]) -> PublicKey {
//
////        func GeneratePublicKey(sk SecretKey) PublicKey {
//            var pk PublicKey
//            s := [SecretKeySize]byte(sk)
//            var ed edwards25519.ExtendedGroupElement
//            edwards25519.GeScalarMultBase(&ed, &s)
//            var edYPlusOne = new(edwards25519.FieldElement)
//            edwards25519.FeAdd(edYPlusOne, &ed.Y, &ed.Z)
//            var oneMinusEdY = new(edwards25519.FieldElement)
//            edwards25519.FeSub(oneMinusEdY, &ed.Z, &ed.Y)
//            var invOneMinusEdY = new(edwards25519.FieldElement)
//            edwards25519.FeInvert(invOneMinusEdY, oneMinusEdY)
//            var montX = new(edwards25519.FieldElement)
//            edwards25519.FeMul(montX, edYPlusOne, invOneMinusEdY)
//            p := new([PublicKeySize]byte)
//            edwards25519.FeToBytes(p, montX)
//            copy(pk[:], p[:])
//        return pk
////    }
//    }
//}

impl PublicKey {
    pub const fn size() -> usize {
        PUBLIC_KEY_SIZE
    }

    pub fn from_bytes(b: [u8; 32]) -> Self {
        PublicKey(b)
    }
}

impl TryFrom<&str> for PublicKey {
    type Error = ConvertError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rs = value.from_base58()?;
        if rs.len() != PUBLIC_KEY_SIZE {
            return Err(ConvertError::InvalidLength(format!(
                "expected size {}, found {}",
                PUBLIC_KEY_SIZE,
                rs.len()
            ))
            .into());
        }

        let mut a = [0; PUBLIC_KEY_SIZE]; // = Default::default();
        a.copy_from_slice(&rs);
        Ok(PublicKey(a))
    }
}

impl Into<String> for PublicKey {
    fn into(self) -> String {
        self.0.to_base58()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn digest_to_and_from_base58() {
        let s = "CG3tYXqAngjnzbx5z4b2x2TUy9mijWBSUJ5dTmmevFuo";
        let a = PublicKey::try_from(s).unwrap();
        assert!(PublicKey::try_from("").is_err());

        let z: String = a.into();

        assert_eq!(s, &z)
    }
}
