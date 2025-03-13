//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

/// Accounts.
pub struct Claim {
    /// The [MerkleDistributor].
    pub distributor: solana_program::pubkey::Pubkey,
    /// Status of the claim.
    pub claim_status: solana_program::pubkey::Pubkey,
    /// Distributor containing the tokens to distribute.
    pub from: solana_program::pubkey::Pubkey,
    /// Account to send the claimed tokens to.
    pub to: solana_program::pubkey::Pubkey,
    /// Extra signer expected for claims
    pub temporal: solana_program::pubkey::Pubkey,
    /// Payer of the claim.
    pub payer: solana_program::pubkey::Pubkey,
    /// The [System] program.
    pub system_program: solana_program::pubkey::Pubkey,
    /// SPL [Token] program.
    pub token_program: solana_program::pubkey::Pubkey,
}

impl Claim {
    pub fn instruction(
        &self,
        args: ClaimInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: ClaimInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.distributor,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.claim_status,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.from, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.to, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.temporal,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
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
        let mut data = ClaimInstructionData::new().try_to_vec().unwrap();
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
struct ClaimInstructionData {
    discriminator: [u8; 8],
}

impl ClaimInstructionData {
    fn new() -> Self {
        Self {
            discriminator: [62, 198, 214, 193, 213, 159, 108, 210],
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimInstructionArgs {
    pub claim_bump: u8,
    pub index: u64,
    pub amount: u64,
    pub claimant_secret: Pubkey,
    pub proof: Vec<[u8; 32]>,
}

/// Instruction builder for `Claim`.
///
/// ### Accounts:
///
///   0. `[writable]` distributor
///   1. `[writable]` claim_status
///   2. `[writable]` from
///   3. `[writable]` to
///   4. `[signer]` temporal
///   5. `[writable, signer]` payer
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Default)]
pub struct ClaimBuilder {
    distributor: Option<solana_program::pubkey::Pubkey>,
    claim_status: Option<solana_program::pubkey::Pubkey>,
    from: Option<solana_program::pubkey::Pubkey>,
    to: Option<solana_program::pubkey::Pubkey>,
    temporal: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    claim_bump: Option<u8>,
    index: Option<u64>,
    amount: Option<u64>,
    claimant_secret: Option<Pubkey>,
    proof: Option<Vec<[u8; 32]>>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ClaimBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The [MerkleDistributor].
    #[inline(always)]
    pub fn distributor(&mut self, distributor: solana_program::pubkey::Pubkey) -> &mut Self {
        self.distributor = Some(distributor);
        self
    }
    /// Status of the claim.
    #[inline(always)]
    pub fn claim_status(&mut self, claim_status: solana_program::pubkey::Pubkey) -> &mut Self {
        self.claim_status = Some(claim_status);
        self
    }
    /// Distributor containing the tokens to distribute.
    #[inline(always)]
    pub fn from(&mut self, from: solana_program::pubkey::Pubkey) -> &mut Self {
        self.from = Some(from);
        self
    }
    /// Account to send the claimed tokens to.
    #[inline(always)]
    pub fn to(&mut self, to: solana_program::pubkey::Pubkey) -> &mut Self {
        self.to = Some(to);
        self
    }
    /// Extra signer expected for claims
    #[inline(always)]
    pub fn temporal(&mut self, temporal: solana_program::pubkey::Pubkey) -> &mut Self {
        self.temporal = Some(temporal);
        self
    }
    /// Payer of the claim.
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
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
    pub fn claim_bump(&mut self, claim_bump: u8) -> &mut Self {
        self.claim_bump = Some(claim_bump);
        self
    }
    #[inline(always)]
    pub fn index(&mut self, index: u64) -> &mut Self {
        self.index = Some(index);
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    #[inline(always)]
    pub fn claimant_secret(&mut self, claimant_secret: Pubkey) -> &mut Self {
        self.claimant_secret = Some(claimant_secret);
        self
    }
    #[inline(always)]
    pub fn proof(&mut self, proof: Vec<[u8; 32]>) -> &mut Self {
        self.proof = Some(proof);
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
        let accounts = Claim {
            distributor: self.distributor.expect("distributor is not set"),
            claim_status: self.claim_status.expect("claim_status is not set"),
            from: self.from.expect("from is not set"),
            to: self.to.expect("to is not set"),
            temporal: self.temporal.expect("temporal is not set"),
            payer: self.payer.expect("payer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = ClaimInstructionArgs {
            claim_bump: self.claim_bump.clone().expect("claim_bump is not set"),
            index: self.index.clone().expect("index is not set"),
            amount: self.amount.clone().expect("amount is not set"),
            claimant_secret: self
                .claimant_secret
                .clone()
                .expect("claimant_secret is not set"),
            proof: self.proof.clone().expect("proof is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `claim` CPI accounts.
pub struct ClaimCpiAccounts<'a, 'b> {
    /// The [MerkleDistributor].
    pub distributor: &'b solana_program::account_info::AccountInfo<'a>,
    /// Status of the claim.
    pub claim_status: &'b solana_program::account_info::AccountInfo<'a>,
    /// Distributor containing the tokens to distribute.
    pub from: &'b solana_program::account_info::AccountInfo<'a>,
    /// Account to send the claimed tokens to.
    pub to: &'b solana_program::account_info::AccountInfo<'a>,
    /// Extra signer expected for claims
    pub temporal: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer of the claim.
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [System] program.
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL [Token] program.
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `claim` CPI instruction.
pub struct ClaimCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [MerkleDistributor].
    pub distributor: &'b solana_program::account_info::AccountInfo<'a>,
    /// Status of the claim.
    pub claim_status: &'b solana_program::account_info::AccountInfo<'a>,
    /// Distributor containing the tokens to distribute.
    pub from: &'b solana_program::account_info::AccountInfo<'a>,
    /// Account to send the claimed tokens to.
    pub to: &'b solana_program::account_info::AccountInfo<'a>,
    /// Extra signer expected for claims
    pub temporal: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer of the claim.
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [System] program.
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL [Token] program.
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: ClaimInstructionArgs,
}

impl<'a, 'b> ClaimCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ClaimCpiAccounts<'a, 'b>,
        args: ClaimInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            distributor: accounts.distributor,
            claim_status: accounts.claim_status,
            from: accounts.from,
            to: accounts.to,
            temporal: accounts.temporal,
            payer: accounts.payer,
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
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.distributor.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.claim_status.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.from.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.to.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.temporal.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
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
        let mut data = ClaimInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::GUMDROP_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.distributor.clone());
        account_infos.push(self.claim_status.clone());
        account_infos.push(self.from.clone());
        account_infos.push(self.to.clone());
        account_infos.push(self.temporal.clone());
        account_infos.push(self.payer.clone());
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

/// Instruction builder for `Claim` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` distributor
///   1. `[writable]` claim_status
///   2. `[writable]` from
///   3. `[writable]` to
///   4. `[signer]` temporal
///   5. `[writable, signer]` payer
///   6. `[]` system_program
///   7. `[]` token_program
pub struct ClaimCpiBuilder<'a, 'b> {
    instruction: Box<ClaimCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ClaimCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ClaimCpiBuilderInstruction {
            __program: program,
            distributor: None,
            claim_status: None,
            from: None,
            to: None,
            temporal: None,
            payer: None,
            system_program: None,
            token_program: None,
            claim_bump: None,
            index: None,
            amount: None,
            claimant_secret: None,
            proof: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The [MerkleDistributor].
    #[inline(always)]
    pub fn distributor(
        &mut self,
        distributor: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.distributor = Some(distributor);
        self
    }
    /// Status of the claim.
    #[inline(always)]
    pub fn claim_status(
        &mut self,
        claim_status: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.claim_status = Some(claim_status);
        self
    }
    /// Distributor containing the tokens to distribute.
    #[inline(always)]
    pub fn from(&mut self, from: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.from = Some(from);
        self
    }
    /// Account to send the claimed tokens to.
    #[inline(always)]
    pub fn to(&mut self, to: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.to = Some(to);
        self
    }
    /// Extra signer expected for claims
    #[inline(always)]
    pub fn temporal(
        &mut self,
        temporal: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.temporal = Some(temporal);
        self
    }
    /// Payer of the claim.
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
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
    pub fn claim_bump(&mut self, claim_bump: u8) -> &mut Self {
        self.instruction.claim_bump = Some(claim_bump);
        self
    }
    #[inline(always)]
    pub fn index(&mut self, index: u64) -> &mut Self {
        self.instruction.index = Some(index);
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.instruction.amount = Some(amount);
        self
    }
    #[inline(always)]
    pub fn claimant_secret(&mut self, claimant_secret: Pubkey) -> &mut Self {
        self.instruction.claimant_secret = Some(claimant_secret);
        self
    }
    #[inline(always)]
    pub fn proof(&mut self, proof: Vec<[u8; 32]>) -> &mut Self {
        self.instruction.proof = Some(proof);
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
        let args = ClaimInstructionArgs {
            claim_bump: self
                .instruction
                .claim_bump
                .clone()
                .expect("claim_bump is not set"),
            index: self.instruction.index.clone().expect("index is not set"),
            amount: self.instruction.amount.clone().expect("amount is not set"),
            claimant_secret: self
                .instruction
                .claimant_secret
                .clone()
                .expect("claimant_secret is not set"),
            proof: self.instruction.proof.clone().expect("proof is not set"),
        };
        let instruction = ClaimCpi {
            __program: self.instruction.__program,

            distributor: self
                .instruction
                .distributor
                .expect("distributor is not set"),

            claim_status: self
                .instruction
                .claim_status
                .expect("claim_status is not set"),

            from: self.instruction.from.expect("from is not set"),

            to: self.instruction.to.expect("to is not set"),

            temporal: self.instruction.temporal.expect("temporal is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

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

struct ClaimCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    distributor: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    claim_status: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    from: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    to: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    temporal: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    claim_bump: Option<u8>,
    index: Option<u64>,
    amount: Option<u64>,
    claimant_secret: Option<Pubkey>,
    proof: Option<Vec<[u8; 32]>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
