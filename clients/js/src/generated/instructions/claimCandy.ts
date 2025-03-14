/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  publicKey,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  bytes,
  mapSerializer,
  publicKey as publicKeySerializer,
  struct,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type ClaimCandyInstructionAccounts = {
  /** The [MerkleDistributor]. */
  distributor: PublicKey | Pda;
  /** The [MerkleDistributor] wallet */
  distributorWallet: PublicKey | Pda;
  /** Status of the claim. Created on first invocation of this function */
  claimCount: PublicKey | Pda;
  /** Extra signer expected for claims */
  temporal: Signer;
  /**
   * Payer of the claim. Should be `mint_authority` for `candy_machine_mint` and will be
   * `update_authority` for `candy_machine_metadata`
   */

  payer?: Signer;
  /** Candy-machine Config */
  candyMachineConfig: PublicKey | Pda;
  /** Candy-Machine. Verified through CPI */
  candyMachine: PublicKey | Pda;
  /** Candy-Machine-Wallet. Verified through CPI */
  candyMachineWallet: PublicKey | Pda;
  /** Generated mint */
  candyMachineMint: PublicKey | Pda;
  /** PDA of `candy_machine_mint` */
  candyMachineMetadata: PublicKey | Pda;
  /** PDA of `candy_machine_mint` */
  candyMachineMasterEdition: PublicKey | Pda;
  /** The [System] program. */
  systemProgram?: PublicKey | Pda;
  /** SPL [Token] program. */
  tokenProgram?: PublicKey | Pda;
  /** SPL [TokenMetadata] program. */
  tokenMetadataProgram?: PublicKey | Pda;
  /** SPL [CandyMachine] program. */
  candyMachineProgram?: PublicKey | Pda;
  rent?: PublicKey | Pda;
  clock: PublicKey | Pda;
};

// Data.
export type ClaimCandyInstructionData = {
  discriminator: Array<number>;
  walletBump: number;
  claimBump: number;
  index: bigint;
  amount: bigint;
  claimantSecret: PublicKey;
  proof: Array<Uint8Array>;
};

export type ClaimCandyInstructionDataArgs = {
  walletBump: number;
  claimBump: number;
  index: number | bigint;
  amount: number | bigint;
  claimantSecret: PublicKey;
  proof: Array<Uint8Array>;
};

export function getClaimCandyInstructionDataSerializer(): Serializer<
  ClaimCandyInstructionDataArgs,
  ClaimCandyInstructionData
> {
  return mapSerializer<
    ClaimCandyInstructionDataArgs,
    any,
    ClaimCandyInstructionData
  >(
    struct<ClaimCandyInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['walletBump', u8()],
        ['claimBump', u8()],
        ['index', u64()],
        ['amount', u64()],
        ['claimantSecret', publicKeySerializer()],
        ['proof', array(bytes({ size: 32 }))],
      ],
      { description: 'ClaimCandyInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [87, 176, 177, 90, 136, 95, 83, 242],
    })
  ) as Serializer<ClaimCandyInstructionDataArgs, ClaimCandyInstructionData>;
}

// Args.
export type ClaimCandyInstructionArgs = ClaimCandyInstructionDataArgs;

// Instruction.
export function claimCandy(
  context: Pick<Context, 'payer' | 'programs'>,
  input: ClaimCandyInstructionAccounts & ClaimCandyInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'gumdrop',
    'gdrpGjVffourzkdDRrQmySw4aTHr8a3xmQzzxSwFD1a'
  );

  // Accounts.
  const resolvedAccounts = {
    distributor: {
      index: 0,
      isWritable: true as boolean,
      value: input.distributor ?? null,
    },
    distributorWallet: {
      index: 1,
      isWritable: true as boolean,
      value: input.distributorWallet ?? null,
    },
    claimCount: {
      index: 2,
      isWritable: true as boolean,
      value: input.claimCount ?? null,
    },
    temporal: {
      index: 3,
      isWritable: false as boolean,
      value: input.temporal ?? null,
    },
    payer: {
      index: 4,
      isWritable: false as boolean,
      value: input.payer ?? null,
    },
    candyMachineConfig: {
      index: 5,
      isWritable: false as boolean,
      value: input.candyMachineConfig ?? null,
    },
    candyMachine: {
      index: 6,
      isWritable: true as boolean,
      value: input.candyMachine ?? null,
    },
    candyMachineWallet: {
      index: 7,
      isWritable: true as boolean,
      value: input.candyMachineWallet ?? null,
    },
    candyMachineMint: {
      index: 8,
      isWritable: true as boolean,
      value: input.candyMachineMint ?? null,
    },
    candyMachineMetadata: {
      index: 9,
      isWritable: true as boolean,
      value: input.candyMachineMetadata ?? null,
    },
    candyMachineMasterEdition: {
      index: 10,
      isWritable: true as boolean,
      value: input.candyMachineMasterEdition ?? null,
    },
    systemProgram: {
      index: 11,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    tokenProgram: {
      index: 12,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    tokenMetadataProgram: {
      index: 13,
      isWritable: false as boolean,
      value: input.tokenMetadataProgram ?? null,
    },
    candyMachineProgram: {
      index: 14,
      isWritable: false as boolean,
      value: input.candyMachineProgram ?? null,
    },
    rent: {
      index: 15,
      isWritable: false as boolean,
      value: input.rent ?? null,
    },
    clock: {
      index: 16,
      isWritable: false as boolean,
      value: input.clock ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: ClaimCandyInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'splSystem',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }
  if (!resolvedAccounts.tokenProgram.value) {
    resolvedAccounts.tokenProgram.value = context.programs.getPublicKey(
      'splToken',
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
    );
    resolvedAccounts.tokenProgram.isWritable = false;
  }
  if (!resolvedAccounts.tokenMetadataProgram.value) {
    resolvedAccounts.tokenMetadataProgram.value = context.programs.getPublicKey(
      'mplTokenMetadata',
      'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
    );
    resolvedAccounts.tokenMetadataProgram.isWritable = false;
  }
  if (!resolvedAccounts.candyMachineProgram.value) {
    resolvedAccounts.candyMachineProgram.value = context.programs.getPublicKey(
      'mplCandyMachine',
      'CndyV3LdqHUfDLmE5naZjVN8rBZz4tqhdefbAnjHG3JR'
    );
    resolvedAccounts.candyMachineProgram.isWritable = false;
  }
  if (!resolvedAccounts.rent.value) {
    resolvedAccounts.rent.value = publicKey(
      'SysvarRent111111111111111111111111111111111'
    );
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getClaimCandyInstructionDataSerializer().serialize(
    resolvedArgs as ClaimCandyInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
