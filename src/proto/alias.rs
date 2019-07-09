//#[macro_use]
use crate::errors::ConvertError;
use crate::macros;
use crate::proto::scheme;
use crate::proto::scheme::Scheme;
use rust_base58::FromBase58;
use std::convert::TryFrom;

const ALIAS_VERSION: u8 = 0x02;
const ALIAS_MIN_LENGTH: usize = 4;
const ALIAS_MAX_LENGTH: usize = 30;
const ALIAS_PREFIX: &str = "alias";

pub struct Alias {
    version: u8,
    scheme: u8,
    pub alias: String,
}

impl Alias {
    pub fn new(scheme: Scheme, alias: &str) -> Result<Alias, ConvertError> {
        if alias.len() < ALIAS_MIN_LENGTH || alias.len() > ALIAS_MAX_LENGTH {
            return bad_args!(
                "alias length should be > {} and < {}, found {}",
                ALIAS_MIN_LENGTH,
                ALIAS_MAX_LENGTH,
                alias.len()
            );
        }
        if !correct_alphabet(alias) {
            return bad_args!("alias should contain only following characters: {}", alias);
        }
        return Ok(Alias {
            version: ALIAS_VERSION,
            scheme,
            alias: alias.to_string(),
        });
    }

    fn to_string(&self) -> String {
        format!("{}:{}:{}", ALIAS_PREFIX, self.scheme, self.alias)
    }
}

fn correct_alphabet(s: &str) -> bool {
    for c in s.chars() {
        if (c < '0' || c > '9')
            && (c < 'a' || c > 'z')
            && c != '_'
            && c != '@'
            && c != '-'
            && c != '.'
        {
            return false;
        }
    }
    return true;
}

impl TryFrom<&str> for Alias {
    type Error = ConvertError;

    /// Tries create Alias from input string
    /// # Examples
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use rustnode1::proto::Alias;
    ///
    /// let x = Alias::try_from("alias:W:blah-blah-blah").unwrap();
    /// assert_eq!("blah-blah-blah", x.alias)
    /// ```
    ///
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rs = value.split(":").collect::<Vec<&str>>();
        if rs.len() != 3 {
            return bad_args!("invalid alias");
        }

        if rs[0] != ALIAS_PREFIX {
            return bad_args!(
                "invalid alias prefix, expected {}, found {}",
                ALIAS_PREFIX,
                rs[0]
            );
        }

        if rs[1].len() != 1 {
            return bad_args!("incorrect alias chainID '{}'", rs[1]);
        }

        Alias::new(rs[0].bytes().nth(0).unwrap(), rs[2])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_alias() {
        let a = Alias::new(scheme::MainNet, "stonescissors").unwrap();
    }

    //    #[test]
    //    fn test_alias_try_from() {
    //        let s = "alias:T:blah-blah-blah";
    //        let a = Alias::try_from(s).unwrap();
    //
    //        assert_eq!(a.alias, "blah-blah-blah")
    //    }
}