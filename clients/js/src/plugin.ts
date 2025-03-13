import { UmiPlugin } from '@metaplex-foundation/umi';
import { createGumdropProgram } from './generated';

export const mplGumdrop = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createGumdropProgram(), false);
  },
});
