use crate::proto;
//use crate::proto::recipient::Recipient::Address;
use crate::errors::ConvertError;
use crate::proto::deserializer::Deserializer;
use crate::proto::recipient::Recipient::Address;
use crate::proto::Serializer;
use std::io;

const ADDRESS_VERSION: u8 = 1;
const ALIAS_VERSION: u8 = 2;

pub enum Recipient {
    Address(proto::Address),
    Alias(proto::Alias),
}

impl Recipient {
    pub fn bytes(&self, s: &mut Serializer) -> Result<(), ConvertError> {
        match self {
            Recipient::Address(addr) => addr.bytes(s),
            Recipient::Alias(alias) => alias.bytes(s),
        }
    }

    //    pub fn deserialize<R>(r: &mut R) -> Result<Recipient, ConvertError>
    //    where
    //        R: io::Read,
    //    {
    //        let v = r.byte();
    //
    //
    //
    //
    //
    //
    //        match v {
    //            ADDRESS_VERSION => Ok(Recipient::Address(proto::Address::deserialize(&mut r)?)),
    //            ALIAS_VERSION => Ok(Recipient::Alias(proto::Alias::deserialize(&mut r)?)),
    //            _ => return bad_args!("unable to parse Recipient"),
    //        }
    //    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::proto::Alias;

    #[test]
    fn test_recipient_bytes() {
        let a = Alias::new(proto::scheme::MainNet, "blabla").unwrap();
        let rec = Recipient::Alias(a);
    }
}
