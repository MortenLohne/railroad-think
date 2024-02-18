import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

const paths = {};

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
