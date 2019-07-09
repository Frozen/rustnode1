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

struct UnsignedTransferV1 {
    id: Option<Digest>,
    fee: u64,
    amount: u64,
    r#type: TransactionType,
    version: u8,
    //    signature: Option<Signature>,
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
        if amount <= 0 {
            return Err(ConvertError::BadArg("amount should be positive".into()));
        }
        if fee <= 0 {
            return Err(ConvertError::BadArg("fee should be positive".into()));
        }
        //        if attachment.len() > MAX_ATTACHMENT_LENGTH_BYTES {
        //            return Err(ConvertError::BadArg("attachment too long".into()));
        //        }

        let out = UnsignedTransferV1 {
            id: None,
            fee,
            amount,
            r#type: TransactionType::TransferTransaction,
            version: 1,
            //            signature: None,
            sender_pk,
            fee_asset,
            timestamp,
            recipient,
            attachment,
            amount_asset,
        };

        return Ok(out);
    }

    pub fn bytes(s: &mut Serializer) -> Result<(), failure::Error> {
        Ok(())
    }

    fn body_bytes(&self, s: &mut Serializer) -> Result<(), failure::Error> {
        //        self.recipient.bytes(s)?; // TODO

        Ok(())
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
