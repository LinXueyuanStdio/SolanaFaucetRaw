#![cfg(feature = "program")]
use byteorder::{ByteOrder, LittleEndian};
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    info,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke_signed,
    instruction::{Instruction}
};
use spl_token;
use std::mem;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 给 A 账户 100 个币
    // accounts = {
    //     A,
    //     token program,
    //     faucet token account,
    //     faucet token owner (auth)
    // }
    // instruction_data 生成授权凭证
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();
    let receiver = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let faucet = next_account_info(accounts_iter)?;
    let faucet_authority = next_account_info(accounts_iter)?;

    let seed = instruction_data;
    let instruction = spl_token::instruction::transfer(
        token_program.key,
        faucet.key,
        receiver.key,
        faucet_authority.key,
        &[],
        100000000)?;

    invoke_signed(&instruction, &[
        faucet.clone(),
        receiver.clone(),
        faucet_authority.clone(),
        token_program.clone(),
    ], &[&[seed]])?;

    Ok(())
}

#[cfg(test)]
mod test {
    use {
        super::*,
        assert_matches::*,
        solana_program::instruction::{AccountMeta, Instruction},
        solana_program_test::*,
        solana_sdk::{signature::Signer, transaction::Transaction},
    };

    #[tokio::test]
    async fn test_transaction() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "bpf_program_template",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![1, 2, 3],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        assert_matches!(banks_client.process_transaction(transaction).await, Ok(()));
    }
}

#[cfg(not(target_arch = "bpf"))]
solana_sdk::program_stubs!();
