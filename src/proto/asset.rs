use crate::crypto;

pub type Asset = crypto::Digest;

impl Asset {
    pub const fn size() -> usize {
        crypto::DIGEST_SIZE
    }
}
