<script lang="ts">
import Game from '/src/components/Game.svelte';
</script>

{#await import('/src/railroad-ink-solver/pkg/railroad_ink_solver') then solver}
  <Game {solver} />
{/await}
