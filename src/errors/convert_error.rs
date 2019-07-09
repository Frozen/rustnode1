#[derive(Debug, Fail)]
pub enum ConvertError {
    #[fail(display = "{}", _0)]
    InvalidLength(String),

    #[fail(display = "base58 error {}", _0)]
    Base58Error(rust_base58::base58::FromBase58Error),

    //    #[fail(display = "{}", _0)]
    //    FormatError(String),
    #[fail(display = "{}", _0)]
    BadArg(String),
}

impl From<rust_base58::base58::FromBase58Error> for ConvertError {
    fn from(e: rust_base58::base58::FromBase58Error) -> Self {
        ConvertError::Base58Error(e)
    }
}

//
//}
//pub fn badArgs() {
//    format!()
//}
