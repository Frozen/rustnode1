use super::transfer::SignedTransferV1;
use crate::crypto;

pub enum Transaction {
    TransferV1(SignedTransferV1),
}

#[derive(Debug)]
pub enum ID {
    Digest(crypto::Digest),
}

//impl ID {
//
//}

pub trait TransactionFields {
    fn id(&self) -> ID;
}
