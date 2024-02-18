import {
  createGlobalThemeContract,
  createGlobalTheme,
} from '@vanilla-extract/css';

import colors from './colors';

export const globalThemeContract = createGlobalThemeContract(
  colors,
  (_, path) => path.join('-').replace('-DEFAULT', '')
);

export const globalStyle = createGlobalTheme(
  '.railroad-think',
  globalThemeContract,
  // @ts-ignore I haven't figured out how to type this, but it works. :)
  colors
);
