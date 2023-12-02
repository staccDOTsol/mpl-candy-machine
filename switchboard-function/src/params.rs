use crate::*;

pub struct ContainerParams {
    pub program_id: Pubkey,
    pub request_key: Pubkey,
    pub candy_machine: Pubkey,
    pub spl_ata_program: Pubkey,
    pub authority_pda: Pubkey,
    pub collection_delegate_record: Pubkey,
    pub collection_master_edition: Pubkey,
    pub collection_metadata: Pubkey,
    pub collection_mint: Pubkey,
    pub collection_update_authority: Pubkey,
    pub nft_owner: Pubkey,
    pub nft_master_edition: Pubkey,
    pub nft_metadata: Pubkey,
    pub nft_mint: Pubkey,
    pub nft_mint_authority: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub sysvar_instructions: Pubkey,
    pub token: Pubkey,
    pub bump: u8,
}

impl ContainerParams {
    pub fn decode(container_params: &Vec<u8>) -> std::result::Result<Self, SbError> {
        let params = String::from_utf8(container_params.clone()).unwrap();

        let mut program_id: Pubkey = Pubkey::default();
        let mut request_key: Pubkey = Pubkey::default();


        println!("-----> {:?}", params);
        for env_pair in params.split(',') {
            let pair: Vec<&str> = env_pair.splitn(2, '=').collect();
            if pair.len() == 2 {
                match pair[0] {
                    "PID" => program_id = Pubkey::from_str(pair[1]).unwrap(),
                    "CANDY_MACHINE" => candy_machine = Pubkey::from_str(pair[1]).unwrap(),
                    "SPL_ATA_PROGRAM" => spl_ata_program = Pubkey::from_str(pair[1]).unwrap(),
                    "AUTHORITY_PDA" => authority_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "COLLECTION_DELEGATE_RECORD" => collection_delegate_record = Pubkey::from_str(pair[1]).unwrap(),
                    "COLLECTION_MASTER_EDITION" => collection_master_edition = Pubkey::from_str(pair[1]).unwrap(),
                    "COLLECTION_METADATA" => collection_metadata = Pubkey::from_str(pair[1]).unwrap(),
                    "COLLECTION_MINT" => collection_mint = Pubkey::from_str(pair[1]).unwrap(),
                    "COLLECTION_UPDATE_AUTHORITY" => collection_update_authority = Pubkey::from_str(pair[1]).unwrap(),
                    "NFT_OWNER" => nft_owner = Pubkey::from_str(pair[1]).unwrap(),
                    "NFT_MASTER_EDITION" => nft_master_edition = Pubkey::from_str(pair[1]).unwrap(),
                    "NFT_METADATA" => nft_metadata = Pubkey::from_str(pair[1]).unwrap(),
                    "NFT_MINT" => nft_mint = Pubkey::from_str(pair[1]).unwrap(),
                    "NFT_MINT_AUTHORITY" => nft_mint_authority = Pubkey::from_str(pair[1]).unwrap(),
                    "PAYER" => payer = Pubkey::from_str(pair[1]).unwrap(),
                    "SYSTEM_PROGRAM" => system_program = Pubkey::from_str(pair[1]).unwrap(),
                    "SYSVAR_INSTRUCTIONS" => sysvar_instructions = Pubkey::from_str(pair[1]).unwrap(),
                    "TOKEN" => token = Pubkey::from_str(pair[1]).unwrap(),
                    "TOKEN_METADATA_PROGRAM" => token_metadata_program = Pubkey::from_str(pair[1]).unwrap(),
                    "SPL_TOKEN_PROGRAM" => spl_token_program = Pubkey::from_str(pair[1]).unwrap(),
                    "TOKEN_RECORD" => token_record = Pubkey::from_str(pair[1]).unwrap(),
                    "REQUEST_KEY" => request_key = Pubkey::from_str(pair[1]).unwrap(),
                    "BUMP" => bump = pair[1].parse::<u8>().unwrap(),
                    _ => {}
                }
            }
        }

        if program_id == Pubkey::default() {
            return Err(SbError::CustomMessage(
                "PID cannot be undefined".to_string(),
            ));
        }
        if request_key == Pubkey::default() {
            return Err(SbError::CustomMessage(
                "REQUEST_KEY cannot be undefined".to_string(),
            ));
        }


        Ok(Self {
            program_id,
            request_key,
            candy_machine,
            spl_ata_program,
            authority_pda,
            collection_delegate_record,
            collection_master_edition,
            collection_metadata,
            collection_mint,
            collection_update_authority,
            nft_owner,
            nft_master_edition,
            nft_metadata,
            nft_mint,
            nft_mint_authority,
            payer,
            system_program,
            sysvar_instructions,
            token,
            token_metadata_program,
            spl_token_program,
            token_record,
            bump,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_decode() {
        let request_params_string = format!(
            "PID={},REQUEST={}",
            anchor_spl::token::ID,
            anchor_spl::token::ID
        );
        let request_params_bytes = request_params_string.into_bytes();

        let params = ContainerParams::decode(&request_params_bytes).unwrap();

        assert_eq!(params.program_id, anchor_spl::token::ID);
        assert_eq!(params.request_key, anchor_spl::token::ID);
    }
}
