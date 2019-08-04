use super::types::TransactionFields;
use crate::crypto;
use crate::crypto::Digest;
use crate::crypto::PublicKey;
use crate::crypto::Signature;
use crate::errors::ConvertError;
use crate::proto::asset::Asset;
use crate::proto::attachment::Attachment;
use crate::proto::attachment::MAX_ATTACHMENT_LENGTH_BYTES;
use crate::proto::recipient::Recipient;
use crate::proto::serializer::Serializer;
use crate::proto::timestamp::Timestamp;
use crate::proto::transactions::types::ID;
use crate::proto::transactions::TransactionType;
use std::io::Write;
//
//struct UnsignedTransferV1Body {
//    sender_pk: PublicKey,
//    amount_asset: Option<Asset>,
//    FeeAsset    OptionalAsset    `json:"feeAssetId"`
//    Timestamp   uint64           `json:"timestamp,omitempty"`
//    Amount      uint64           `json:"amount"`
//    Fee         uint64           `json:"fee"`
//    Recipient   Recipient        `json:"recipient"`
//    Attachment  Attachment       `json:"attachment,omitempty"`
//}

pub struct SignedTransferV1 {
    body: UnsignedTransferV1,
    signature: crypto::Signature,
}

pub struct UnsignedTransferV1 {
    //    id: Option<Digest>,
    fee: u64,
    amount: u64,
    r#type: TransactionType,
    version: u8,
    sender_pk: PublicKey,
    fee_asset: Option<Asset>,
    timestamp: Timestamp,
    recipient: Recipient,
    attachment: Attachment,
    amount_asset: Option<Asset>,
}

impl UnsignedTransferV1 {
    pub fn new(
        sender_pk: PublicKey,
        amount_asset: Option<Asset>,
        fee_asset: Option<Asset>,
        timestamp: Timestamp,
        amount: u64,
        fee: u64,
        recipient: Recipient,
        attachment: Attachment,
    ) -> Result<UnsignedTransferV1, ConvertError> {
        let out = UnsignedTransferV1 {
            fee,
            amount,
            r#type: TransactionType::TransferTransaction,
            version: 1,
            sender_pk,
            fee_asset,
            timestamp,
            recipient,
            attachment,
            amount_asset,
        };

        return Ok(out);
    }

    pub fn bytes(&self, s: &mut dyn bytes::buf::BufMut) -> Result<(), failure::Error> {
        Ok(())
    }

    pub fn sign(&self, secret: crypto::SecretKey) -> Result<SignedTransferV1, ConvertError> {
        return bad_args!("not implemented");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn unsigned_transfer_v1_from_bytes() {
        let mut buf = vec![0u8; 0];

        //        let mut t = UnsignedTransferV1::new()
    }

}
