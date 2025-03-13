/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  ACCOUNT_HEADER_SIZE,
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u16,
  u32,
  u8,
} from '@metaplex-foundation/umi/serializers';
import { getMyAccountSize } from '../accounts';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type CreateInstructionAccounts = {
  /** The address of the new account */
  address: Signer;
  /** The authority of the new account */
  authority?: PublicKey | Pda;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** The system program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type CreateInstructionData = {
  discriminator: number;
  arg1: number;
  arg2: number;
};

export type CreateInstructionDataArgs = { arg1: number; arg2: number };

export function getCreateInstructionDataSerializer(): Serializer<
  CreateInstructionDataArgs,
  CreateInstructionData
> {
  return mapSerializer<CreateInstructionDataArgs, any, CreateInstructionData>(
    struct<CreateInstructionData>(
      [
        ['discriminator', u8()],
        ['arg1', u16()],
        ['arg2', u32()],
      ],
      { description: 'CreateInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 0 })
  ) as Serializer<CreateInstructionDataArgs, CreateInstructionData>;
}

// Args.
export type CreateInstructionArgs = CreateInstructionDataArgs;

// Instruction.
export function create(
  context: Pick<Context, 'identity' | 'payer' | 'programs'>,
  input: CreateInstructionAccounts & CreateInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplGumdrop',
    'gdrpGjVffourzkdDRrQmySw4aTHr8a3xmQzzxSwFD1a'
  );

  // Accounts.
  const resolvedAccounts = {
    address: {
      index: 0,
      isWritable: true as boolean,
      value: input.address ?? null,
    },
    authority: {
      index: 1,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
    payer: {
      index: 2,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    systemProgram: {
      index: 3,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: CreateInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.authority.value) {
    resolvedAccounts.authority.value = context.identity.publicKey;
  }
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
  const data = getCreateInstructionDataSerializer().serialize(
    resolvedArgs as CreateInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = getMyAccountSize() + ACCOUNT_HEADER_SIZE;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
