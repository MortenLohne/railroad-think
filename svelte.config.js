import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

import { readFileSync } from 'fs';

const paths = {};

const target = readFileSync('deploy.sh', 'utf-8')
  .match(/TARGET=[^\n]+/)[0]
  .split('=')
  .pop();

if (process.env.NODE_ENV === 'production') {
  const stage = process.env.B4_STAGE === 'true';
  const base = stage ? 'http://lisatest.bt.no' : 'https://www.bt.no/spesial';
  paths.assets = `${base}/${target}`;
}

export default {
  preprocess: preprocess(), // Documentation: https://github.com/sveltejs/svelte-preprocess
  kit: {
    paths,

    adapter: adapter({
      pages: 'dist',
    }),

    appDir: 'files',

    files: {
      template: 'index.html',
    },

    prerender: { default: true },
  },
};
