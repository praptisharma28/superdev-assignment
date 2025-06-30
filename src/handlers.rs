use warp::{Rejection, Reply};
use crate::types::*;
use crate::crypto;
use crate::solana_ops;

pub async fn handle_generate_keypair() -> Result<impl Reply, Rejection> {
    match crypto::generate_keypair() {
        Ok(keypair) => {
            let response = ApiResponse::success(serde_json::to_value(keypair).unwrap());
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            let response = ApiResponse::error(format!("Failed to generate keypair: {}", e));
            Ok(warp::reply::json(&response))
        }
    }
}

pub async fn handle_create_token(req: CreateTokenRequest) -> Result<impl Reply, Rejection> {
    if req.mint_authority.is_empty() || req.mint.is_empty() {
        let response = ApiResponse::error("Missing required fields".to_string());
        return Ok(warp::reply::json(&response));
    }
    
    match solana_ops::create_mint_instruction(&req.mint_authority, &req.mint, req.decimals) {
        Ok(instruction) => {
            let response = ApiResponse::success(serde_json::to_value(instruction).unwrap());
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            let response = ApiResponse::error(format!("Failed to create token instruction: {}", e));
            Ok(warp::reply::json(&response))
        }
    }
}

pub async fn handle_mint_token(req: MintTokenRequest) -> Result<impl Reply, Rejection> {
    if req.mint.is_empty() || req.destination.is_empty() || req.authority.is_empty() {
        let response = ApiResponse::error("Missing required fields".to_string());
        return Ok(warp::reply::json(&response));
    }
    
    if req.amount == 0 {
        let response = ApiResponse::error("Amount must be greater than 0".to_string());
        return Ok(warp::reply::json(&response));
    }
    
    match solana_ops::create_mint_to_instruction(&req.mint, &req.destination, &req.authority, req.amount) {
        Ok(instruction) => {
            let response = ApiResponse::success(serde_json::to_value(instruction).unwrap());
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            let response = ApiResponse::error(format!("Failed to create mint instruction: {}", e));
            Ok(warp::reply::json(&response))
        }
    }
}

pub async fn handle_sign_message(req: SignMessageRequest) -> Result<impl Reply, Rejection> {
    if req.message.is_empty() || req.secret.is_empty() {
        let response = ApiResponse::error("Missing required fields".to_string());
        return Ok(warp::reply::json(&response));
    }
    
    match crypto::sign_message(&req.message, &req.secret) {
        Ok(signature_response) => {
            let response = ApiResponse::success(serde_json::to_value(signature_response).unwrap());
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            let response = ApiResponse::error(format!("Failed to sign message: {}", e));
            Ok(warp::reply::json(&response))
        }
    }
}

pub async fn handle_verify_message(req: VerifyMessageRequest) -> Result<impl Reply, Rejection> {
    if req.message.is_empty() || req.signature.is_empty() || req.pubkey.is_empty() {
        let response = ApiResponse::error("Missing required fields".to_string());
        return Ok(warp::reply::json(&response));
    }
    
    match crypto::verify_message(&req.message, &req.signature, &req.pubkey) {
        Ok(verify_response) => {
            let response = ApiResponse::success(serde_json::to_value(verify_response).unwrap());
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            let response = ApiResponse::error(format!("Failed to verify message: {}", e));
            Ok(warp::reply::json(&response))
        }
    }
}

pub async fn handle_send_sol(req: SendSolRequest) -> Result<impl Reply, Rejection> {
    if req.from.is_empty() || req.to.is_empty() {
        let response = ApiResponse::error("Missing required fields".to_string());
        return Ok(warp::reply::json(&response));
    }
    
    if req.lamports == 0 {
        let response = ApiResponse::error("Amount must be greater than 0".to_string());
        return Ok(warp::reply::json(&response));
    }
    
    match solana_ops::create_sol_transfer_instruction(&req.from, &req.to, req.lamports) {
        Ok(instruction) => {
            let response = ApiResponse::success(serde_json::to_value(instruction).unwrap());
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            let response = ApiResponse::error(format!("Failed to create transfer instruction: {}", e));
            Ok(warp::reply::json(&response))
        }
    }
}

pub async fn handle_send_token(req: SendTokenRequest) -> Result<impl Reply, Rejection> {
    if req.destination.is_empty() || req.mint.is_empty() || req.owner.is_empty() {
        let response = ApiResponse::error("Missing required fields".to_string());
        return Ok(warp::reply::json(&response));
    }
    
    if req.amount == 0 {
        let response = ApiResponse::error("Amount must be greater than 0".to_string());
        return Ok(warp::reply::json(&response));
    }
    
    match solana_ops::create_token_transfer_instruction(&req.mint, &req.owner, &req.destination, req.amount) {
        Ok(instruction) => {
            let response = ApiResponse::success(serde_json::to_value(instruction).unwrap());
            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            let response = ApiResponse::error(format!("Failed to create token transfer instruction: {}", e));
            Ok(warp::reply::json(&response))
        }
    }
}
