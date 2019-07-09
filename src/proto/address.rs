use crate::errors::ConvertError;
use crate::proto::serializer::Serializer;
use rust_base58::{FromBase58, ToBase58};
use std::convert::TryFrom;

const headerSize: usize = 2;
const bodySize: usize = 20;
const checksumSize: usize = 4;
const ADDRESS_SIZE: usize = headerSize + bodySize + checksumSize;

pub struct Address([u8; ADDRESS_SIZE]);

impl TryFrom<&str> for Address {
    type Error = ConvertError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rs = value.from_base58()?;
        if rs.len() != ADDRESS_SIZE {
            return Err(ConvertError::InvalidLength(format!(
                "expected size {}, found {}",
                ADDRESS_SIZE,
                rs.len()
            ))
            .into());
        }

        let mut a = [0; ADDRESS_SIZE]; // = Default::default();
        a.copy_from_slice(&rs);
        Ok(Address(a))
    }
}

impl Into<String> for Address {
    fn into(self) -> String {
        self.0.to_base58()
    }
}

impl Address {
    pub fn bytes(&self, s: &mut Serializer) -> Result<(), failure::Error> {
        //        self.0.
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn signature_to_and_from_base58() {
        let s = "3PAASSqnygiyYoQuqmXpwaSUJmRkqytwPaw";
        let a = Address::try_from(s).unwrap();
        assert!(Address::try_from("").is_err());

        let z: String = a.into();

        assert_eq!(s, &z)
    }

}
