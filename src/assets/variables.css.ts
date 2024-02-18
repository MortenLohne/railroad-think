import {
  createGlobalThemeContract,
  createGlobalTheme,
} from '@vanilla-extract/css';

import * as colors from './colors';

// A better way would probably be to explicitly handle the typing here ...
export const vars = {
  ...(colors as any),
};

export const globalThemeContract = createGlobalThemeContract(vars, (_, path) =>
  path.join('-').replace('-base', '')
);

export const globalStyle = createGlobalTheme(
  '.railroad-think',
  globalThemeContract,
  vars
);
