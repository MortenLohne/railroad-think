<script>
import Game from '/src/components/Game.svelte';
import initSolver, * as solver from '/src/railroad-ink-solver/pkg';

// export let dev; // Are we in production?
// export let target; // Element the app attaches to
</script>

{#await initSolver()}
  loading ...
{:then _}
  <Game {solver} />
{/await}
