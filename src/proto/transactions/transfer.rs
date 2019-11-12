use crate::crypto;
use crate::crypto::Digest;
use crate::crypto::PublicKey;
use crate::crypto::Signature;
use crate::errors::ConvertError;
use crate::proto::asset::Asset;
use crate::proto::attachment::Attachment;
use crate::proto::attachment::MAX_ATTACHMENT_LENGTH_BYTES;
use crate::proto::deserializer::Deserializer;
use crate::proto::recipient::Recipient;
use crate::proto::serializer::Serializer;
use crate::proto::timestamp::Timestamp;
use crate::proto::transactions::{TransactionType, TRANSFER_TRANSACTION};
use std::io;
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

struct SignerTransferV1 {
    body: UnsignedTransferV1,
    signature: crypto::Signature,
}

struct UnsignedTransferV1 {
    id: Option<Digest>,
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
        //        if attachment.len() > MAX_ATTACHMENT_LENGTH_BYTES {
        //            return Err(ConvertError::BadArg("attachment too long".into()));
        //        }

        let out = UnsignedTransferV1 {
            id: None,
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

    pub fn bytes(&self, s: &mut Serializer) -> Result<(), failure::Error> {
        Ok(())
    }

    pub fn sign(&self, secret: crypto::SecretKey) -> Result<SignerTransferV1, ConvertError> {
        return bad_args!("not implemented");
    }

    pub fn deserialize<T>(mut r: &mut T) -> Result<UnsignedTransferV1, ConvertError>
    where
        T: io::Read,
    {
        let r#type = r.byte()?;
        if r#type != TRANSFER_TRANSACTION {
            return bad_args!("expected type {}, found {}", TRANSFER_TRANSACTION, r#type);
        }

        let pk = r.public_key()?;

        let amount_asset = r.asset()?;
        let fee_asset = r.asset();
        let ts = r.timestamp()?;
        let Amount = r.u64()?;
        let Fee = r.u64()?;
        //        let version = r.byte()?;

        Err(ConvertError::BadArg("bla".to_string()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Read;

    #[test]
    fn unsigned_transfer_v1_from_bytes() {
        //        let mut buf = vec![0u8; 0];

        let mut b: Vec<u8> = vec![
            4, 51, 96, 125, 11, 97, 38, 220, 232, 10, 189, 12, 82, 163, 164, 224, 227, 202, 165,
            127, 137, 249, 88, 174, 105, 228, 61, 128, 171, 244, 4, 212, 112, 11, 175, 174, 60,
            127, 148, 85, 153, 87, 22, 36, 3, 7, 28, 124, 47, 143, 231, 52, 86, 181, 238, 226, 39,
            38, 108, 182, 141, 22, 134, 169, 5, 4, 217, 213, 197, 121, 113, 238, 251, 8, 94, 58,
            186, 247, 165, 164, 166, 205, 184, 24, 95, 48, 16, 85, 131, 205, 176, 154, 216, 246,
            24, 134, 236, 101, 0, 0, 0, 0, 1, 103, 168, 57, 210, 136, 0, 0, 0, 0, 0, 0, 39, 16, 0,
            0, 0, 0, 0, 0, 39, 16, 1, 87, 194, 6, 114, 186, 86, 233, 186, 233, 243, 109, 72, 253,
            83, 72, 141, 25, 253, 57, 148, 96, 195, 41, 20, 23, 0, 0,
        ];

        let mut slice = &b[..];

        let rs = UnsignedTransferV1::deserialize(&mut slice);
    }
}
