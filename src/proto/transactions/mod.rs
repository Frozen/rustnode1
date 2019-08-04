mod transfer;
mod types;

//pub type TransactionType = u32;
use self::transfer::SignedTransferV1;

pub enum TransactionType {
    GenesisTransaction,
    TransferTransaction,
}


