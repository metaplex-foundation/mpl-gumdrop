import { UmiPlugin } from '@metaplex-foundation/umi';
import { createMplGumdropProgram } from './generated';

export const mplGumdrop = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createMplGumdropProgram(), false);
  },
});
