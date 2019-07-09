use crate::errors::ConvertError;
use rust_base58::{FromBase58, ToBase58};
use std::convert::TryFrom;

const SIGNATURE_SIZE: usize = 64;

pub struct Signature([u8; SIGNATURE_SIZE]);

impl TryFrom<&str> for Signature {
    type Error = ConvertError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rs = value.from_base58()?;
        if rs.len() != SIGNATURE_SIZE {
            return Err(ConvertError::InvalidLength(format!(
                "expected size {}, found {}",
                SIGNATURE_SIZE,
                rs.len()
            ))
            .into());
        }

        let mut a = [0; SIGNATURE_SIZE];
        a.copy_from_slice(&rs);
        Ok(Signature(a))
    }
}

impl Into<String> for Signature {
    fn into(self) -> String {
        self.0.to_base58()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn signature_to_and_from_base58() {
        let s = "2Nnm3hfK6CnG5NHK3BKzvxFuAuPRW67PUrC5Ssw6NcWqc4x5D9T2NVWrNozkyCCpD4wSP2Z9JxcZfdKmNEXtBTzx";
        let a = Signature::try_from(s).unwrap();
        assert!(Signature::try_from("").is_err());

        let z: String = a.into();

        assert_eq!("2Nnm3hfK6CnG5NHK3BKzvxFuAuPRW67PUrC5Ssw6NcWqc4x5D9T2NVWrNozkyCCpD4wSP2Z9JxcZfdKmNEXtBTzx", &z)
    }

}
