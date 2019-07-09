mod digest;
mod public_key;
mod secret_key;
mod signature;

pub use crate::crypto::public_key::PublicKey;
pub use crate::crypto::secret_key::SecretKey;
pub use digest::Digest;
pub use signature::Signature;

use sha2::Sha256;

pub struct KeyPair {
    secret: SecretKey,
    public: PublicKey,
}

//fn generate_key_pair(seed: &[u8]) -> KeyPair {
//    let mut h = Sha256::new();
//    h.input(seed);
//    let digest = h.result();
//    var sk SecretKey
//    var pk PublicKey
//    sk = GenerateSecretKey(digest)
//    pk = GeneratePublicKey(sk)
//    return sk, pk
//}

#[cfg(test)]
mod tests {}
