use crate::{
    keys::{PublicKey, SecretKey},
    utils::{prove, verify_proof},
};

pub struct Proof {
    pub proof_bytes: Vec<u8>,
    pub signer: PublicKey,
    pub message_bytes: Vec<u8>,
}

impl Proof {
    pub fn new(secret_key: &SecretKey, message: impl AsRef<[u8]>) -> Self {
        prove(secret_key, message.as_ref())
    }
    
    pub fn verify(&self) -> Result<[u8; 64], ()> {
        verify_proof(self)
    }
}
