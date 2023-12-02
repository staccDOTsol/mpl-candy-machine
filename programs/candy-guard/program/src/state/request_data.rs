use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct RequestAccountData {
    pub bump: u8,
    pub pubkey_hash: [u8; 32],
    pub switchboard_request: Pubkey,
    pub seed: u32,
    pub blockhash: [u8; 32],
    pub result: [u8; 32],
    pub request_timestamp: i64,
    pub seed_timestamp: i64,
    pub reveal_timestamp: i64,
}