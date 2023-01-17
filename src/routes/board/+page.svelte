<script>
import Board from '/src/components/Board.svelte';
import initSolver, * as solver from '/src/railroad-ink-solver/pkg';
</script>

{#await initSolver()}
  loading ...
{:then _}
  <Board {solver} />
{/await}
