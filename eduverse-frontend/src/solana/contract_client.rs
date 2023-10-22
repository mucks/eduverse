use std::str::FromStr;

use crate::types::CreateTeacherInstruction;

use super::rpc_client::SolanaRpcClient;
use anyhow::{Context, Result};
use borsh::{to_vec, BorshDeserialize};
use eduverse_contract::state::{Config, Teacher};
use num_traits::ToBytes;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    transaction::Transaction,
};

lazy_static::lazy_static! {
    pub static ref PROGRAM_ID: Pubkey = Pubkey::from_str("DMk4dLgAZvP84jxzgZZgS1R5WXGMi4wXHdj7cdi3sKuR").expect("invalid PROGRAM_ID");
    pub static ref SYSTEM_PROGRAM_ID: Pubkey = Pubkey::from_str("11111111111111111111111111111111").expect("invalid SYSTEM_PROGRAM_ID");
    pub static ref SYS_VAR_RENT: Pubkey = Pubkey::from_str("SysvarRent111111111111111111111111111111111").expect("invalid SYS_VAR_RENT");
}

pub struct ContractClient {
    pub inner: SolanaRpcClient,
}

#[derive(Debug)]
pub enum EduverseInstruction {
    Initialize,
    CreateTeacher(CreateTeacherInstruction),
    RegisterSubject(u32),
}

impl EduverseInstruction {
    pub fn sighash(&self) -> [u8; 8] {
        match self {
            EduverseInstruction::Initialize => sighash("initialize"),
            EduverseInstruction::CreateTeacher(_) => sighash("create_teacher_profile"),
            EduverseInstruction::RegisterSubject(_) => sighash("teacher_register_subject"),
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = self.sighash().to_vec();

        let args = match self {
            EduverseInstruction::CreateTeacher(teacher) => to_vec(&teacher)?,
            EduverseInstruction::RegisterSubject(subject_id) => subject_id.to_le_bytes().to_vec(),
            _ => vec![],
        };

        if !args.is_empty() {
            bytes.extend(args);
        }

        Ok(bytes)
    }
}

impl ContractClient {
    pub fn new(inner: SolanaRpcClient) -> Self {
        Self { inner }
    }

    pub fn dev() -> Self {
        let inner = SolanaRpcClient::devnet();
        Self::new(inner)
    }
    pub fn local() -> Self {
        let inner = SolanaRpcClient::local();
        Self::new(inner)
    }

    pub fn get_all_accounts(&self) -> Result<()> {
        Ok(())
    }

    pub fn create_tx(
        &self,
        payer: &Pubkey,
        instr: EduverseInstruction,
        accounts: Vec<AccountMeta>,
    ) -> Result<Transaction> {
        let instr_data = instr.to_bytes()?;

        let instr = Instruction::new_with_bytes(*PROGRAM_ID, &instr_data, accounts);

        let msg = Message::new(&[instr], Some(payer));

        log::debug!("\n\nmsg: {:?}\n\n", msg);

        let tx = Transaction::new_unsigned(msg);

        Ok(tx)
    }

    pub fn create_config(&self, payer: &Pubkey) -> Result<Transaction> {
        let (conf_pda, _) = derive_program_address_str("config", &None)?;

        let accounts = vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(conf_pda, false),
            AccountMeta::new(*SYS_VAR_RENT, false),
            AccountMeta::new(*SYSTEM_PROGRAM_ID, false),
        ];

        let tx = self.create_tx(payer, EduverseInstruction::Initialize, accounts)?;

        Ok(tx)
    }

    pub async fn get_accounts_by_struct_name<T: BorshDeserialize>(
        &self,
        struct_name: &str,
    ) -> Result<Vec<T>> {
        let accounts = self.inner.get_program_accounts(&PROGRAM_ID).await?;
        let acc_sighash = account_sighash(struct_name);

        log::debug!("\n\nacc_sighash: {}\n\n", hex::encode(acc_sighash));

        let structs: Vec<T> = accounts
            .iter()
            .filter(|a| a.data[..8] == acc_sighash)
            .filter_map(|a| match BorshDeserialize::deserialize(&mut &a.data[8..]) {
                Ok(data) => Some(data),
                Err(err) => {
                    log::error!(
                        "could not deserialize account data for account: {a:?} | err {err:?}"
                    );
                    None
                }
            })
            .collect();

        Ok(structs)
    }

    pub async fn get_account_by_pda<T: BorshDeserialize>(&self, pda: &Pubkey) -> Result<T> {
        let acc = self
            .inner
            .get_account(pda)
            .await
            .context("could not get account")?;

        let account = BorshDeserialize::deserialize(&mut &acc.data[8..])?;

        Ok(account)
    }

    pub async fn get_config(&self) -> Result<Config> {
        let (config_pda, _) = derive_program_address_str("config", &None)?;

        self.get_account_by_pda::<Config>(&config_pda).await
    }

    pub async fn get_teacher_by_pubkey(&self, pubkey: &Pubkey) -> Result<Teacher> {
        let (teacher_pda, _) = derive_program_address_str("teacher", &Some(*pubkey))?;

        self.get_account_by_pda::<Teacher>(&teacher_pda).await
    }

    pub async fn get_teachers(&self) -> Result<Vec<Teacher>> {
        self.get_accounts_by_struct_name::<Teacher>("Teacher").await
    }

    pub async fn register_subject(&self, payer: &Pubkey, subject_id: u32) -> Result<Transaction> {
        let (teacher_pda, _) = derive_program_address_str("teacher", &Some(*payer))?;
        let (subject_config_pda, _) = derive_program_address(
            &[
                b"subject_config" as &[u8],
                &subject_id.to_le_bytes() as &[u8],
            ],
            &None,
        )?;

        let (subject_teacher_pda, _) = derive_program_address(
            &[
                b"subject_teacher" as &[u8],
                &subject_id.to_le_bytes() as &[u8],
                // TODO: query subject_config and get the count_teachers if subject_config exists
                &0u32.to_le_bytes() as &[u8],
            ],
            &None,
        )?;

        let accounts = vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(teacher_pda, false),
            AccountMeta::new(subject_config_pda, false),
            AccountMeta::new(subject_teacher_pda, false),
            AccountMeta::new(*SYS_VAR_RENT, false),
            AccountMeta::new(*SYSTEM_PROGRAM_ID, false),
        ];
        let tx = self.create_tx(
            payer,
            EduverseInstruction::RegisterSubject(subject_id),
            accounts,
        )?;
        Ok(tx)
    }

    pub async fn build_create_teacher_tx(
        &self,
        instr: CreateTeacherInstruction,
        payer: &Pubkey,
    ) -> Result<Transaction> {
        let conf = self.get_config().await?;

        let (conf_pda, _) = derive_program_address_str("config", &None)?;
        let (teacher_pda, _) = derive_program_address_str("teacher", &Some(*payer))?;

        let (teacher_lookup_pda, _) = derive_program_address(
            &vec![
                "teacher_by_id".as_bytes(),
                &conf.count_teachers.to_le_bytes(),
            ],
            &None,
        )?;

        let accounts = vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(conf_pda, false),
            AccountMeta::new(teacher_pda, false),
            AccountMeta::new(teacher_lookup_pda, false),
            AccountMeta::new(*SYS_VAR_RENT, false),
            AccountMeta::new(*SYSTEM_PROGRAM_ID, false),
        ];

        let tx = self.create_tx(payer, EduverseInstruction::CreateTeacher(instr), accounts)?;
        Ok(tx)
    }

    #[cfg(test)]
    pub async fn send_tx(&self, tx: Transaction) -> Result<String> {
        self.inner.send_transaction(&tx).await
    }
}

pub fn derive_program_address_str(key: &str, pubkey: &Option<Pubkey>) -> Result<(Pubkey, u8)> {
    let seeds = vec![key.as_bytes()];

    derive_program_address(&seeds, pubkey)
}

// TODO: optimize this functions as references are converted to vectors and back
pub fn derive_program_address(seeds: &[&[u8]], pubkey: &Option<Pubkey>) -> Result<(Pubkey, u8)> {
    let mut seeds: Vec<Vec<u8>> = seeds.to_vec().into_iter().map(|s| s.to_vec()).collect();
    if let Some(pk) = pubkey {
        seeds.push(pk.to_bytes().to_vec());
    }

    let seeds: Vec<&[u8]> = seeds.iter().map(|s| s.as_ref()).collect();

    let option_pk = Pubkey::try_find_program_address(&seeds, &PROGRAM_ID);
    let pk = option_pk.context("could not derive program address")?;

    Ok(pk)
}

pub fn sighash(name: &str) -> [u8; 8] {
    let preimage = format!("global:{}", name);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&solana_sdk::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}

pub fn account_sighash(name: &str) -> [u8; 8] {
    let preimage = format!("account:{}", name);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&solana_sdk::hash::hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}

#[cfg(test)]
mod tests {
    use crate::{solana::rpc_client::TEST_KEYPAIR, types::Subject};

    use super::*;
    use solana_sdk::signer::Signer;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    async fn test_create_config() -> Result<()> {
        wasm_logger::init(wasm_logger::Config::default());

        let client = ContractClient::local();

        let tx = client.create_config(&TEST_KEYPAIR.pubkey())?;
        let tx = client.inner.sign_tx(tx, &TEST_KEYPAIR).await?;

        let tx_hash = client.send_tx(tx).await?;

        log::debug!("\n\n ||||| tx_hash: {:?} ||||\n\n", tx_hash);

        Ok(())
    }
    #[wasm_bindgen_test]
    async fn test_get_config() -> Result<()> {
        wasm_logger::init(wasm_logger::Config::default());

        let client = ContractClient::local();

        let conf = client.get_config().await?;

        log::debug!("{:?}", conf);

        Ok(())
    }

    #[wasm_bindgen_test]
    async fn test_get_teachers() -> Result<()> {
        wasm_logger::init(wasm_logger::Config::default());

        let client = ContractClient::local();

        let teachers = client.get_teachers().await?;

        log::debug!("{:?}", teachers);

        Ok(())
    }

    #[wasm_bindgen_test]
    async fn test_create_teacher() -> Result<()> {
        wasm_logger::init(wasm_logger::Config::default());

        let client = ContractClient::local();

        let instr = CreateTeacherInstruction {
            title: "test".to_string(),
            website: "test".to_string(),
            telegram: "test".to_string(),
            twitter: "test".to_string(),
        };

        let unsigned_tx = client
            .build_create_teacher_tx(instr, &TEST_KEYPAIR.pubkey())
            .await?;
        let tx = client.inner.sign_tx(unsigned_tx, &TEST_KEYPAIR).await?;

        let tx_hash = client.send_tx(tx).await?;

        log::debug!("\n\ntx_hash: {:?}\n\n", tx_hash);
        Ok(())
    }

    #[wasm_bindgen_test]
    async fn test_register_subject() -> Result<()> {
        wasm_logger::init(wasm_logger::Config::default());

        let client = ContractClient::local();

        let pk = TEST_KEYPAIR.pubkey();

        let mut tx = client
            .register_subject(&pk, Subject::English as u32)
            .await?;

        tx.message.recent_blockhash = client.inner.get_latest_blockhash().await?;

        let tx = client.inner.sign_tx(tx, &TEST_KEYPAIR).await?;

        client.send_tx(tx).await?;

        Ok(())
    }

    #[wasm_bindgen_test]
    async fn get_get_teachers() -> Result<()> {
        wasm_logger::init(wasm_logger::Config::default());

        let client = ContractClient::local();
        let teachers = client.get_teachers().await?;

        log::debug!("{:?}", teachers);

        Ok(())
    }
}
