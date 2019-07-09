#[macro_export]
macro_rules! bad_args {
    ($($arg:tt)*) => (Result::Err(ConvertError::BadArg(format!($($arg)*))))
}
