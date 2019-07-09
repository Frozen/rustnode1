#![feature(try_from)]
use rust_base58::{FromBase58, ToBase58};
use std::convert::TryFrom;

impl From<rust_base58::base58::FromBase58Error> for ConvertError {
    fn from(e: rust_base58::base58::FromBase58Error) -> Self {
        ConvertError::Base58Error(e)
    }
}

impl Into<String> for Digest {
    fn into(self) -> String {
        self.0.to_base58()
    }
}
//
//#[cfg(test)]
//mod tests {
//
//    use super::*;
//
//    #[test]
//    fn digest_to_and_from_base58() {
//        let s = "FXYA6oQLoBuGKNbmRSrXAk3N539Qd6mLWxRGHD44NQoi";
//        let a = Digest::try_from(s).unwrap();
//        assert!(Digest::try_from("").is_err());
//
//        let z: String = a.into();
//
//        assert_eq!("FXYA6oQLoBuGKNbmRSrXAk3N539Qd6mLWxRGHD44NQoi", &z)
//    }
//
//    #[test]
//    fn signature_to_and_from_base58() {
//        let s = "2Nnm3hfK6CnG5NHK3BKzvxFuAuPRW67PUrC5Ssw6NcWqc4x5D9T2NVWrNozkyCCpD4wSP2Z9JxcZfdKmNEXtBTzx";
//        let a = Signature::try_from(s).unwrap();
//        assert!(Signature::try_from("").is_err());
//
//        let z: String = a.into();
//
//        assert_eq!("2Nnm3hfK6CnG5NHK3BKzvxFuAuPRW67PUrC5Ssw6NcWqc4x5D9T2NVWrNozkyCCpD4wSP2Z9JxcZfdKmNEXtBTzx", &z)
//    }
//
//}
