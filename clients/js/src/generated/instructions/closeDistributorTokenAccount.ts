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
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type CloseDistributorTokenAccountInstructionAccounts = {
  /** Base key of the distributor. */
  base: Signer;
  /** [MerkleDistributor]. */
  distributor: PublicKey | Pda;
  /** Distributor containing the tokens to distribute. */
  from: PublicKey | Pda;
  /** Account to send the claimed tokens to. */
  to: PublicKey | Pda;
  /** Who is receiving the remaining rent allocation. */
  receiver: PublicKey | Pda;
  /** The [System] program. */
  systemProgram?: PublicKey | Pda;
  /** SPL [Token] program. */
  tokenProgram?: PublicKey | Pda;
};

// Data.
export type CloseDistributorTokenAccountInstructionData = {
  discriminator: Array<number>;
  bump: number;
};

export type CloseDistributorTokenAccountInstructionDataArgs = { bump: number };

export function getCloseDistributorTokenAccountInstructionDataSerializer(): Serializer<
  CloseDistributorTokenAccountInstructionDataArgs,
  CloseDistributorTokenAccountInstructionData
> {
  return mapSerializer<
    CloseDistributorTokenAccountInstructionDataArgs,
    any,
    CloseDistributorTokenAccountInstructionData
  >(
    struct<CloseDistributorTokenAccountInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['bump', u8()],
      ],
      { description: 'CloseDistributorTokenAccountInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [156, 174, 153, 120, 102, 150, 134, 142],
    })
  ) as Serializer<
    CloseDistributorTokenAccountInstructionDataArgs,
    CloseDistributorTokenAccountInstructionData
  >;
}

// Args.
export type CloseDistributorTokenAccountInstructionArgs =
  CloseDistributorTokenAccountInstructionDataArgs;

// Instruction.
export function closeDistributorTokenAccount(
  context: Pick<Context, 'programs'>,
  input: CloseDistributorTokenAccountInstructionAccounts &
    CloseDistributorTokenAccountInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'gumdrop',
    'gdrpGjVffourzkdDRrQmySw4aTHr8a3xmQzzxSwFD1a'
  );

  // Accounts.
  const resolvedAccounts = {
    base: { index: 0, isWritable: false as boolean, value: input.base ?? null },
    distributor: {
      index: 1,
      isWritable: false as boolean,
      value: input.distributor ?? null,
    },
    from: { index: 2, isWritable: true as boolean, value: input.from ?? null },
    to: { index: 3, isWritable: true as boolean, value: input.to ?? null },
    receiver: {
      index: 4,
      isWritable: true as boolean,
      value: input.receiver ?? null,
    },
    systemProgram: {
      index: 5,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    tokenProgram: {
      index: 6,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: CloseDistributorTokenAccountInstructionArgs = {
    ...input,
  };

  // Default values.
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
  const data =
    getCloseDistributorTokenAccountInstructionDataSerializer().serialize(
      resolvedArgs as CloseDistributorTokenAccountInstructionDataArgs
    );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
