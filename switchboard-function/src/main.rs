use std::str::FromStr;

pub use switchboard_solana::get_ixn_discriminator;
pub use switchboard_solana::prelude::*;

mod params;
pub use params::*;

pub mod lib;
pub use lib::*;

#[tokio::main(worker_threads = 12)]
async fn main() {
    // First, initialize the runner instance with a freshly generated Gramine keypair
    let runner = FunctionRunner::from_env(None).unwrap();

    // parse and validate user provided request params
    let params = ContainerParams::decode(
        &runner
            .function_request_data
            .as_ref()
            .unwrap()
            .container_params,
    )
    .unwrap();


    let rand_seed = generate_randomness(0, 500_000_000);

    // IXN DATA:
    let mut ixn_data = get_ixn_discriminator("process_mint").to_vec();
    ixn_data.append(&mut rand_seed.to_le_bytes().to_vec());

    let request_pubkey = runner.function_request_key.unwrap();

    ixn_data.append(&mut runner.function_request_data.as_ref().unwrap().container_params.clone());
    // ACCOUNTS:
    let ixn = Instruction {
        program_id: params.program_id,
        data: ixn_data,
        accounts: vec![
            AccountMeta::new(*params.candy_machine.key, false),
            AccountMeta::new(*params.authority_pda.key, false),
            AccountMeta::new_readonly(*params.mint_authority.key, true),
            AccountMeta::new(*params.payer.key, true),
            AccountMeta::new_readonly(*params.nft_owner.key, false),
            AccountMeta::new(*params.nft_mint.key, false),
            AccountMeta::new_readonly(*params.nft_mint_authority.key, true),
            AccountMeta::new(*params.nft_metadata.key, false),
            AccountMeta::new(*params.nft_master_edition.key, false),
            AccountMeta::new_readonly(*params.token.key, false),
            AccountMeta::new_readonly(*params.token_record.key, false),
            AccountMeta::new(*params.collection_delegate_record.key, false),
            AccountMeta::new(*params.collection_mint.key, false),
            AccountMeta::new(*params.collection_metadata.key, false),
            AccountMeta::new(*params.collection_master_edition.key, false),
            AccountMeta::new(*params.collection_update_authority.key, false),
            AccountMeta::new_readonly(*params.token_metadata_program.key, false),
            AccountMeta::new_readonly(*params.spl_token_program.key, false),
            AccountMeta::new_readonly(*params.spl_ata_program.key, false),
            AccountMeta::new_readonly(*params.system_program.key, false),
            AccountMeta::new_readonly(*params.instruction_sysvar_account.key, false),
            AccountMeta::new_readonly(*params.authorization_rules_program.key, false),
            AccountMeta::new_readonly(*params.authorization_rules.key, false),
            AccountMeta::new(*request_pubkey, false),
            AccountMeta::new_readonly(*params.switchboard.key, false),
            AccountMeta::new(*params.switchboard_state.key, false),
            AccountMeta::new(*params.switchboard_attestation_queue.key, false),
            AccountMeta::new(*params.switchboard_function.key, false),
            AccountMeta::new(*params.switchboard_request.key, false),
            AccountMeta::new(*params.switchboard_request_escrow.key, false),
            AccountMeta::new_readonly(*params.switchboard_mint.key, false),
            AccountMeta::new_readonly(*params.token_program.key, false),
            AccountMeta::new_readonly(*params.associated_token_program.key, false),

        ],
    };

    // Then, write your own Rust logic and build a Vec of instructions.
    // Should  be under 700 bytes after serialization
    // Finally, emit the signed quote and partially signed transaction to the functionRunner oracle
    // The functionRunner oracle will use the last outputted word to stdout as the serialized result. This is what gets executed on-chain.
    runner.emit(vec![ixn]).await.unwrap();
}

fn generate_randomness(min: u32, max: u32) -> u32 {
    if min == max {
        return min;
    }
    if min > max {
        return generate_randomness(max, min);
    }

    // We add one so its inclusive [min, max]
    let window = (max + 1) - min;

    let mut bytes: [u8; 4] = [0u8; 4];
    Gramine::read_rand(&mut bytes).expect("gramine failed to generate randomness");
    let raw_result: &[u32] = bytemuck::cast_slice(&bytes[..]);

    (raw_result[0] % window) + min
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1. Check when lower_bound is greater than upper_bound
    #[test]
    fn test_generate_randomness_with_flipped_bounds() {
        let min = 100;
        let max = 50;

        let result = generate_randomness(100, 50);
        assert!(result >= max && result < min);
    }

    // 2. Check when lower_bound is equal to upper_bound
    #[test]
    fn test_generate_randomness_with_equal_bounds() {
        let bound = 100;
        assert_eq!(generate_randomness(bound, bound), bound);
    }

    // 3. Test within a range
    #[test]
    fn test_generate_randomness_within_bounds() {
        let min = 100;
        let max = 200;

        let result = generate_randomness(min, max);

        assert!(result >= min && result < max);
    }

    // 4. Test randomness distribution (not truly deterministic, but a sanity check)
    #[test]
    fn test_generate_randomness_distribution() {
        let min = 0;
        let max = 9;

        let mut counts = vec![0; 10];
        for _ in 0..1000 {
            let result = generate_randomness(min, max);
            let index: usize = result as usize;
            counts[index] += 1;
        }

        // Ensure all counts are non-zero (probabilistically should be the case)
        for count in counts.iter() {
            assert!(*count > 0);
        }
    }
}
