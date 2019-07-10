#[derive(Debug, Fail)]
pub enum ConvertError {
    #[fail(display = "{}", _0)]
    InvalidLength(String),

    #[fail(display = "base58 error {}", _0)]
    Base58Error(rust_base58::base58::FromBase58Error),

    #[fail(display = "{}", _0)]
    BadArg(String),

    #[fail(display = "{}", _0)]
    IO(std::io::Error),

    #[fail(display = "{}", _0)]
    Utf8Error(std::str::Utf8Error),
}

impl From<rust_base58::base58::FromBase58Error> for ConvertError {
    fn from(e: rust_base58::base58::FromBase58Error) -> Self {
        ConvertError::Base58Error(e)
    }
}

impl From<std::io::Error> for ConvertError {
    fn from(e: std::io::Error) -> Self {
        ConvertError::IO(e)
    }
}

impl From<std::str::Utf8Error> for ConvertError {
    fn from(e: std::str::Utf8Error) -> Self {
        ConvertError::Utf8Error(e)
    }
}

//
//}
//pub fn badArgs() {
//    format!()
//}
