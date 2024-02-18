import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import yaml from '@rollup/plugin-yaml';
import dsv from '@rollup/plugin-dsv';
import json from './custom-plugins/json.js';
import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin';
import wasm from 'vite-plugin-wasm';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    sveltekit(),
    json(),
    yaml(),
    dsv(),
    wasm(),
    vanillaExtractPlugin(),
    // add more as needed
  ],
  server: {
    port: 4173,
    open: true,
  },
  build: {
    target: 'es2015',
  },
});
