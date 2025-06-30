use solana_sdk::signature::{Keypair, Signature, Signer};
use solana_sdk::pubkey::Pubkey;
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use crate::types::{KeypairResponse, SignMessageResponse, VerifyMessageResponse};

pub fn generate_keypair() -> Result<KeypairResponse> {
    let keypair = Keypair::new();
    
    Ok(KeypairResponse {
        pubkey: keypair.pubkey().to_string(),
        secret: bs58::encode(keypair.to_bytes()).into_string(),
    })
}

pub fn sign_message(message: &str, secret_key_b58: &str) -> Result<SignMessageResponse> {
    let secret_bytes = bs58::decode(secret_key_b58)
        .into_vec()
        .map_err(|_| anyhow!("Invalid base58 secret key"))?;
    
    if secret_bytes.len() != 64 {
        return Err(anyhow!("Secret key must be 64 bytes"));
    }
    
    let keypair = Keypair::from_bytes(&secret_bytes)
        .map_err(|_| anyhow!("Invalid secret key format"))?;
    
    let message_bytes = message.as_bytes();
    let signature = keypair.sign_message(message_bytes);
    
    Ok(SignMessageResponse {
        signature: general_purpose::STANDARD.encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: message.to_string(),
    })
}

pub fn verify_message(message: &str, signature_b64: &str, pubkey_b58: &str) -> Result<VerifyMessageResponse> {
    let pubkey = pubkey_b58.parse::<Pubkey>()
        .map_err(|_| anyhow!("Invalid public key"))?;
    
    let signature_bytes = general_purpose::STANDARD.decode(signature_b64)
        .map_err(|_| anyhow!("Invalid base64 signature"))?;
    
    if signature_bytes.len() != 64 {
        return Err(anyhow!("Signature must be 64 bytes"));
    }
    
    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| anyhow!("Invalid signature format"))?;
    
    let message_bytes = message.as_bytes();
    let is_valid = signature.verify(&pubkey.to_bytes(), message_bytes);
    
    Ok(VerifyMessageResponse {
        valid: is_valid,
        message: message.to_string(),
        pubkey: pubkey_b58.to_string(),
    })
}
