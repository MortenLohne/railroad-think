import Error from './Error.svelte';

/**
 * Catches initialization errors.
 * This ErrorBoundary-component is [fully based on this repl](https://svelte.dev/repl/006facb65ece4f808cd733e838783228?version=3.22.2).
 *
 * Wrap your crashing component with this,
 * and handle the error if it binds to anything.
 * ```svelte
 * {#if !error}
 *   <ErrorBoundary bind:error>
 *     <PotentiallyCrashingComponent />
 *   </ErrorBoundary>
 * {:else}
 *   <p>Oh noes, it crashed! :(</p>
 * {/if}
 * ```
 */
export default class errorBoundary extends Error {
  constructor(config) {
    let error = null;
    config.props.$$slots.default = config.props.$$slots.default.map(
      x =>
        (...args) => {
          try {
            return x(...args);
          } catch (e) {
            error = e;
          }
        }
    );
    super(config);
    if (error) this.$set({ error: error });
  }
}
