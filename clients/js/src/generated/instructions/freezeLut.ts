/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  AccountMeta,
  Context,
  PublicKey,
  Serializer,
  Signer,
  WrappedInstruction,
  checkForIsWritableOverride as isWritable,
  mapSerializer,
} from '@metaplex-foundation/umi';

// Accounts.
export type FreezeLutInstructionAccounts = {
  address: PublicKey;
  authority?: Signer;
};

// Arguments.
export type FreezeLutInstructionData = { discriminator: number };

export type FreezeLutInstructionDataArgs = {};

export function getFreezeLutInstructionDataSerializer(
  context: Pick<Context, 'serializer'>
): Serializer<FreezeLutInstructionDataArgs, FreezeLutInstructionData> {
  const s = context.serializer;
  return mapSerializer<
    FreezeLutInstructionDataArgs,
    FreezeLutInstructionData,
    FreezeLutInstructionData
  >(
    s.struct<FreezeLutInstructionData>([['discriminator', s.u32()]], {
      description: 'FreezeLutInstructionData',
    }),
    (value) => ({ ...value, discriminator: 1 } as FreezeLutInstructionData)
  ) as Serializer<FreezeLutInstructionDataArgs, FreezeLutInstructionData>;
}

// Instruction.
export function freezeLut(
  context: Pick<Context, 'serializer' | 'programs' | 'identity'>,
  input: FreezeLutInstructionAccounts
): WrappedInstruction {
  const signers: Signer[] = [];
  const keys: AccountMeta[] = [];

  // Program ID.
  const programId: PublicKey = context.programs.get(
    'splAddressLookupTable'
  ).publicKey;

  // Resolved accounts.
  const addressAccount = input.address;
  const authorityAccount = input.authority ?? context.identity;

  // Address.
  keys.push({
    pubkey: addressAccount,
    isSigner: false,
    isWritable: isWritable(addressAccount, true),
  });

  // Authority.
  signers.push(authorityAccount);
  keys.push({
    pubkey: authorityAccount.publicKey,
    isSigner: true,
    isWritable: isWritable(authorityAccount, false),
  });

  // Data.
  const data = getFreezeLutInstructionDataSerializer(context).serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return {
    instruction: { keys, programId, data },
    signers,
    bytesCreatedOnChain,
  };
}
