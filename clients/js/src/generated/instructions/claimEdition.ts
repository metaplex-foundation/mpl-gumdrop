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
export type ClaimEditionInstructionAccounts = {
  /**
   * Given a token account containing the master edition token to prove authority, and a brand new non-metadata-ed mint with one token
   * make a new Metadata + Edition that is a child of the master edition denoted by this authority token.
   * 4. `[writable]` Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number])
   * where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
   * 8. `[]` token account containing token from master metadata mint
   * The [MerkleDistributor].
   */

  distributor: PublicKey | Pda;
  /** Status of the claim. Created on first invocation of this function */
  claimCount: PublicKey | Pda;
  /** Extra signer expected for claims */
  temporal: Signer;
  /**
   * Payer of the claim. Should be `mint_authority` for `candy_machine_mint` and will be
   * `update_authority` for `candy_machine_metadata`
   */

  payer?: Signer;
  /** PDA of `metadata_new_mint` */
  metadataNewMetadata: PublicKey | Pda;
  /** PDA of `metadata_new_mint` */
  metadataNewEdition: PublicKey | Pda;
  /** PDA of `metadata_master_mint` */
  metadataMasterEdition: PublicKey | Pda;
  /** Generated mint */
  metadataNewMint: PublicKey | Pda;
  /** PDA of `metadata_master_mint` and edition number */
  metadataEditionMarkPda: PublicKey | Pda;
  /** Mint authority for `metadata_new_mint` */
  metadataNewMintAuthority: Signer;
  /**
   * Owner of token account containing master token (#8)
   * distributor
   * Token account
   */

  metadataMasterTokenAccount: PublicKey | Pda;
  /** Update authority for new metadata */
  metadataNewUpdateAuthority: PublicKey | Pda;
  /** Master record metadata account */
  metadataMasterMetadata: PublicKey | Pda;
  /** Master metadata mint account */
  metadataMasterMint: PublicKey | Pda;
  /** The [System] program. */
  systemProgram?: PublicKey | Pda;
  /** SPL [Token] program. */
  tokenProgram?: PublicKey | Pda;
  /** SPL [TokenMetadata] program. */
  tokenMetadataProgram?: PublicKey | Pda;
  rent?: PublicKey | Pda;
};

// Data.
export type ClaimEditionInstructionData = {
  discriminator: Array<number>;
  claimBump: number;
  index: bigint;
  amount: bigint;
  edition: bigint;
  claimantSecret: PublicKey;
  proof: Array<Uint8Array>;
};

export type ClaimEditionInstructionDataArgs = {
  claimBump: number;
  index: number | bigint;
  amount: number | bigint;
  edition: number | bigint;
  claimantSecret: PublicKey;
  proof: Array<Uint8Array>;
};

export function getClaimEditionInstructionDataSerializer(): Serializer<
  ClaimEditionInstructionDataArgs,
  ClaimEditionInstructionData
> {
  return mapSerializer<
    ClaimEditionInstructionDataArgs,
    any,
    ClaimEditionInstructionData
  >(
    struct<ClaimEditionInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['claimBump', u8()],
        ['index', u64()],
        ['amount', u64()],
        ['edition', u64()],
        ['claimantSecret', publicKeySerializer()],
        ['proof', array(bytes({ size: 32 }))],
      ],
      { description: 'ClaimEditionInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [150, 83, 124, 180, 53, 35, 144, 248],
    })
  ) as Serializer<ClaimEditionInstructionDataArgs, ClaimEditionInstructionData>;
}

// Args.
export type ClaimEditionInstructionArgs = ClaimEditionInstructionDataArgs;

// Instruction.
export function claimEdition(
  context: Pick<Context, 'payer' | 'programs'>,
  input: ClaimEditionInstructionAccounts & ClaimEditionInstructionArgs
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
    claimCount: {
      index: 1,
      isWritable: true as boolean,
      value: input.claimCount ?? null,
    },
    temporal: {
      index: 2,
      isWritable: false as boolean,
      value: input.temporal ?? null,
    },
    payer: {
      index: 3,
      isWritable: false as boolean,
      value: input.payer ?? null,
    },
    metadataNewMetadata: {
      index: 4,
      isWritable: true as boolean,
      value: input.metadataNewMetadata ?? null,
    },
    metadataNewEdition: {
      index: 5,
      isWritable: true as boolean,
      value: input.metadataNewEdition ?? null,
    },
    metadataMasterEdition: {
      index: 6,
      isWritable: true as boolean,
      value: input.metadataMasterEdition ?? null,
    },
    metadataNewMint: {
      index: 7,
      isWritable: true as boolean,
      value: input.metadataNewMint ?? null,
    },
    metadataEditionMarkPda: {
      index: 8,
      isWritable: true as boolean,
      value: input.metadataEditionMarkPda ?? null,
    },
    metadataNewMintAuthority: {
      index: 9,
      isWritable: false as boolean,
      value: input.metadataNewMintAuthority ?? null,
    },
    metadataMasterTokenAccount: {
      index: 10,
      isWritable: false as boolean,
      value: input.metadataMasterTokenAccount ?? null,
    },
    metadataNewUpdateAuthority: {
      index: 11,
      isWritable: false as boolean,
      value: input.metadataNewUpdateAuthority ?? null,
    },
    metadataMasterMetadata: {
      index: 12,
      isWritable: false as boolean,
      value: input.metadataMasterMetadata ?? null,
    },
    metadataMasterMint: {
      index: 13,
      isWritable: false as boolean,
      value: input.metadataMasterMint ?? null,
    },
    systemProgram: {
      index: 14,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    tokenProgram: {
      index: 15,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    tokenMetadataProgram: {
      index: 16,
      isWritable: false as boolean,
      value: input.tokenMetadataProgram ?? null,
    },
    rent: {
      index: 17,
      isWritable: false as boolean,
      value: input.rent ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: ClaimEditionInstructionArgs = { ...input };

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
  const data = getClaimEditionInstructionDataSerializer().serialize(
    resolvedArgs as ClaimEditionInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
