use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use crate::types::{KeypairResponse, SignMessageResponse, VerifyMessageResponse};

pub fn generate_keypair() -> Result<KeypairResponse> {
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);
    
    let public_key_bytes = keypair.public.to_bytes();
    let secret_key_bytes = keypair.secret.to_bytes();
    
    Ok(KeypairResponse {
        pubkey: bs58::encode(public_key_bytes).into_string(),
        secret: bs58::encode(secret_key_bytes).into_string(),
    })
}

pub fn sign_message(message: &str, secret_key_b58: &str) -> Result<SignMessageResponse> {
    let secret_bytes = bs58::decode(secret_key_b58)
        .into_vec()
        .map_err(|_| anyhow!("Invalid base58 secret key"))?;
    
    if secret_bytes.len() != 32 {
        return Err(anyhow!("Secret key must be 32 bytes"));
    }
    
    let secret_key = SecretKey::from_bytes(&secret_bytes)
        .map_err(|_| anyhow!("Invalid secret key format"))?;
    
    let public_key = PublicKey::from(&secret_key);
    let keypair = Keypair { secret: secret_key, public: public_key };
    
    let message_bytes = message.as_bytes();
    let signature = keypair.sign(message_bytes);
    
    Ok(SignMessageResponse {
        signature: general_purpose::STANDARD.encode(signature.to_bytes()),
        public_key: bs58::encode(public_key.to_bytes()).into_string(),
        message: message.to_string(),
    })
}

pub fn verify_message(message: &str, signature_b64: &str, pubkey_b58: &str) -> Result<VerifyMessageResponse> {
    let pubkey_bytes = bs58::decode(pubkey_b58)
        .into_vec()
        .map_err(|_| anyhow!("Invalid base58 public key"))?;
    
    if pubkey_bytes.len() != 32 {
        return Err(anyhow!("Public key must be 32 bytes"));
    }
    
    let public_key = PublicKey::from_bytes(&pubkey_bytes)
        .map_err(|_| anyhow!("Invalid public key format"))?;
    
    let signature_bytes = general_purpose::STANDARD.decode(signature_b64)
        .map_err(|_| anyhow!("Invalid base64 signature"))?;
    
    if signature_bytes.len() != 64 {
        return Err(anyhow!("Signature must be 64 bytes"));
    }
    
    let signature = Signature::from_bytes(&signature_bytes)
        .map_err(|_| anyhow!("Invalid signature format"))?;
    
    let message_bytes = message.as_bytes();
    let is_valid = public_key.verify(message_bytes, &signature).is_ok();
    
    Ok(VerifyMessageResponse {
        valid: is_valid,
        message: message.to_string(),
        pubkey: pubkey_b58.to_string(),
    })
}
