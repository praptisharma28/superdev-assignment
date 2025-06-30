use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    system_instruction,
};
use spl_token::{
    instruction as spl_instruction,
    ID as TOKEN_PROGRAM_ID,
};
use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account,
};
use anyhow::{Result, anyhow};
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose};
use crate::types::{AccountMeta, InstructionResponse};

pub fn create_mint_instruction(
    mint_authority: &str,
    mint: &str,
    decimals: u8,
) -> Result<InstructionResponse> {
    let mint_authority_pubkey = Pubkey::from_str(mint_authority)
        .map_err(|_| anyhow!("Invalid mint authority address"))?;
    
    let mint_pubkey = Pubkey::from_str(mint)
        .map_err(|_| anyhow!("Invalid mint address"))?;
    
    let rent_pubkey = solana_sdk::sysvar::rent::id();
    
    let instruction = spl_instruction::initialize_mint(
        &TOKEN_PROGRAM_ID,
        &mint_pubkey,
        &mint_authority_pubkey,
        Some(&mint_authority_pubkey),
        decimals,
    )?;
    
    let accounts = instruction.accounts.iter().map(|acc| AccountMeta {
        pubkey: acc.pubkey.to_string(),
        is_signer: acc.is_signer,
        is_writable: acc.is_writable,
    }).collect();
    
    Ok(InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    })
}

pub fn create_mint_to_instruction(
    mint: &str,
    destination: &str,
    authority: &str,
    amount: u64,
) -> Result<InstructionResponse> {
    let mint_pubkey = Pubkey::from_str(mint)
        .map_err(|_| anyhow!("Invalid mint address"))?;
    
    let destination_pubkey = Pubkey::from_str(destination)
        .map_err(|_| anyhow!("Invalid destination address"))?;
    
    let authority_pubkey = Pubkey::from_str(authority)
        .map_err(|_| anyhow!("Invalid authority address"))?;
    
    let instruction = spl_instruction::mint_to(
        &TOKEN_PROGRAM_ID,
        &mint_pubkey,
        &destination_pubkey,
        &authority_pubkey,
        &[],
        amount,
    )?;
    
    let accounts = instruction.accounts.iter().map(|acc| AccountMeta {
        pubkey: acc.pubkey.to_string(),
        is_signer: acc.is_signer,
        is_writable: acc.is_writable,
    }).collect();
    
    Ok(InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    })
}

pub fn create_sol_transfer_instruction(
    from: &str,
    to: &str,
    lamports: u64,
) -> Result<InstructionResponse> {
    let from_pubkey = Pubkey::from_str(from)
        .map_err(|_| anyhow!("Invalid sender address"))?;
    
    let to_pubkey = Pubkey::from_str(to)
        .map_err(|_| anyhow!("Invalid recipient address"))?;
    
    if lamports == 0 {
        return Err(anyhow!("Amount must be greater than 0"));
    }
    
    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, lamports);
    
    let accounts = instruction.accounts.iter().map(|acc| AccountMeta {
        pubkey: acc.pubkey.to_string(),
        is_signer: acc.is_signer,
        is_writable: acc.is_writable,
    }).collect();
    
    Ok(InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    })
}

pub fn create_token_transfer_instruction(
    mint: &str,
    owner: &str,
    destination: &str,
    amount: u64,
) -> Result<InstructionResponse> {
    let mint_pubkey = Pubkey::from_str(mint)
        .map_err(|_| anyhow!("Invalid mint address"))?;
    
    let owner_pubkey = Pubkey::from_str(owner)
        .map_err(|_| anyhow!("Invalid owner address"))?;
    
    let destination_pubkey = Pubkey::from_str(destination)
        .map_err(|_| anyhow!("Invalid destination address"))?;
    
    if amount == 0 {
        return Err(anyhow!("Amount must be greater than 0"));
    }
    
    let source_ata = get_associated_token_address(&owner_pubkey, &mint_pubkey);
    let dest_ata = get_associated_token_address(&destination_pubkey, &mint_pubkey);
    
    let instruction = spl_instruction::transfer(
        &TOKEN_PROGRAM_ID,
        &source_ata,
        &dest_ata,
        &owner_pubkey,
        &[],
        amount,
    )?;
    
    let accounts = instruction.accounts.iter().map(|acc| AccountMeta {
        pubkey: acc.pubkey.to_string(),
        is_signer: acc.is_signer,
        is_writable: acc.is_writable,
    }).collect();
    
    Ok(InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    })
}
