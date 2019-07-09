use crate::proto;
use crate::proto::recipient::Recipient::Address;

pub enum Recipient {
    Address(proto::Address),
    Alias(proto::Alias),
}

//impl Recipient {
//    pub fn bytes(&self, s: &mut Serializer) -> Result<(), failure::Error> {
//
//        match self {
//            Address(addr) => addr.bytes(s),
//            Alias(alias) =>
//        }
//
//    }
//}
