//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CloseDistributor {
    /// Base key of the distributor.
    pub base: solana_program::pubkey::Pubkey,
    /// [MerkleDistributor].
    pub distributor: solana_program::pubkey::Pubkey,

    pub distributor_wallet: solana_program::pubkey::Pubkey,
    /// Who is receiving the remaining tokens and rent allocations.
    pub receiver: solana_program::pubkey::Pubkey,
    /// The [System] program.
    pub system_program: solana_program::pubkey::Pubkey,
    /// SPL [Token] program.
    pub token_program: solana_program::pubkey::Pubkey,
}

impl CloseDistributor {
    pub fn instruction(
        &self,
        args: CloseDistributorInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CloseDistributorInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.base, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.distributor,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.distributor_wallet,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.receiver,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CloseDistributorInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::GUMDROP_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CloseDistributorInstructionData {
    discriminator: [u8; 8],
}

impl CloseDistributorInstructionData {
    fn new() -> Self {
        Self {
            discriminator: [202, 56, 180, 143, 46, 104, 106, 112],
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CloseDistributorInstructionArgs {
    pub bump: u8,
    pub wallet_bump: u8,
}

/// Instruction builder for `CloseDistributor`.
///
/// ### Accounts:
///
///   0. `[signer]` base
///   1. `[writable]` distributor
///   2. `[writable]` distributor_wallet
///   3. `[]` receiver
///   4. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   5. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Default)]
pub struct CloseDistributorBuilder {
    base: Option<solana_program::pubkey::Pubkey>,
    distributor: Option<solana_program::pubkey::Pubkey>,
    distributor_wallet: Option<solana_program::pubkey::Pubkey>,
    receiver: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    bump: Option<u8>,
    wallet_bump: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CloseDistributorBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Base key of the distributor.
    #[inline(always)]
    pub fn base(&mut self, base: solana_program::pubkey::Pubkey) -> &mut Self {
        self.base = Some(base);
        self
    }
    /// [MerkleDistributor].
    #[inline(always)]
    pub fn distributor(&mut self, distributor: solana_program::pubkey::Pubkey) -> &mut Self {
        self.distributor = Some(distributor);
        self
    }
    #[inline(always)]
    pub fn distributor_wallet(
        &mut self,
        distributor_wallet: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.distributor_wallet = Some(distributor_wallet);
        self
    }
    /// Who is receiving the remaining tokens and rent allocations.
    #[inline(always)]
    pub fn receiver(&mut self, receiver: solana_program::pubkey::Pubkey) -> &mut Self {
        self.receiver = Some(receiver);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// The [System] program.
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    /// SPL [Token] program.
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn bump(&mut self, bump: u8) -> &mut Self {
        self.bump = Some(bump);
        self
    }
    #[inline(always)]
    pub fn wallet_bump(&mut self, wallet_bump: u8) -> &mut Self {
        self.wallet_bump = Some(wallet_bump);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = CloseDistributor {
            base: self.base.expect("base is not set"),
            distributor: self.distributor.expect("distributor is not set"),
            distributor_wallet: self
                .distributor_wallet
                .expect("distributor_wallet is not set"),
            receiver: self.receiver.expect("receiver is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = CloseDistributorInstructionArgs {
            bump: self.bump.clone().expect("bump is not set"),
            wallet_bump: self.wallet_bump.clone().expect("wallet_bump is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `close_distributor` CPI accounts.
pub struct CloseDistributorCpiAccounts<'a, 'b> {
    /// Base key of the distributor.
    pub base: &'b solana_program::account_info::AccountInfo<'a>,
    /// [MerkleDistributor].
    pub distributor: &'b solana_program::account_info::AccountInfo<'a>,

    pub distributor_wallet: &'b solana_program::account_info::AccountInfo<'a>,
    /// Who is receiving the remaining tokens and rent allocations.
    pub receiver: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [System] program.
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL [Token] program.
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `close_distributor` CPI instruction.
pub struct CloseDistributorCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Base key of the distributor.
    pub base: &'b solana_program::account_info::AccountInfo<'a>,
    /// [MerkleDistributor].
    pub distributor: &'b solana_program::account_info::AccountInfo<'a>,

    pub distributor_wallet: &'b solana_program::account_info::AccountInfo<'a>,
    /// Who is receiving the remaining tokens and rent allocations.
    pub receiver: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [System] program.
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL [Token] program.
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CloseDistributorInstructionArgs,
}

impl<'a, 'b> CloseDistributorCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CloseDistributorCpiAccounts<'a, 'b>,
        args: CloseDistributorInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            base: accounts.base,
            distributor: accounts.distributor,
            distributor_wallet: accounts.distributor_wallet,
            receiver: accounts.receiver,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.base.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.distributor.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.distributor_wallet.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.receiver.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CloseDistributorInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::GUMDROP_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.base.clone());
        account_infos.push(self.distributor.clone());
        account_infos.push(self.distributor_wallet.clone());
        account_infos.push(self.receiver.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `CloseDistributor` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` base
///   1. `[writable]` distributor
///   2. `[writable]` distributor_wallet
///   3. `[]` receiver
///   4. `[]` system_program
///   5. `[]` token_program
pub struct CloseDistributorCpiBuilder<'a, 'b> {
    instruction: Box<CloseDistributorCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CloseDistributorCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CloseDistributorCpiBuilderInstruction {
            __program: program,
            base: None,
            distributor: None,
            distributor_wallet: None,
            receiver: None,
            system_program: None,
            token_program: None,
            bump: None,
            wallet_bump: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Base key of the distributor.
    #[inline(always)]
    pub fn base(&mut self, base: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.base = Some(base);
        self
    }
    /// [MerkleDistributor].
    #[inline(always)]
    pub fn distributor(
        &mut self,
        distributor: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.distributor = Some(distributor);
        self
    }
    #[inline(always)]
    pub fn distributor_wallet(
        &mut self,
        distributor_wallet: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.distributor_wallet = Some(distributor_wallet);
        self
    }
    /// Who is receiving the remaining tokens and rent allocations.
    #[inline(always)]
    pub fn receiver(
        &mut self,
        receiver: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.receiver = Some(receiver);
        self
    }
    /// The [System] program.
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// SPL [Token] program.
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn bump(&mut self, bump: u8) -> &mut Self {
        self.instruction.bump = Some(bump);
        self
    }
    #[inline(always)]
    pub fn wallet_bump(&mut self, wallet_bump: u8) -> &mut Self {
        self.instruction.wallet_bump = Some(wallet_bump);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = CloseDistributorInstructionArgs {
            bump: self.instruction.bump.clone().expect("bump is not set"),
            wallet_bump: self
                .instruction
                .wallet_bump
                .clone()
                .expect("wallet_bump is not set"),
        };
        let instruction = CloseDistributorCpi {
            __program: self.instruction.__program,

            base: self.instruction.base.expect("base is not set"),

            distributor: self
                .instruction
                .distributor
                .expect("distributor is not set"),

            distributor_wallet: self
                .instruction
                .distributor_wallet
                .expect("distributor_wallet is not set"),

            receiver: self.instruction.receiver.expect("receiver is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CloseDistributorCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    base: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    distributor: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    distributor_wallet: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    receiver: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bump: Option<u8>,
    wallet_bump: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
