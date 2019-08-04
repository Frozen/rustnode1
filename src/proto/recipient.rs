use crate::proto;
//use crate::proto::recipient::Recipient::Address;
use crate::errors::ConvertError;
use crate::proto::Serializer;

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
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::proto::Alias;
    //    use crate::proto::recipient::Recipient::Alias;

    #[test]
    fn test_recipient_bytes() {
        let a = Alias::new(proto::scheme::MainNet, "blabla").unwrap();
        let rec = Recipient::Alias(a);

        //        let mut t = UnsignedTransferV1::new()
    }

}
