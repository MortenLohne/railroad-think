<script lang="ts">
import Edge from './Edge.svelte';
import Node from './Node.svelte';
const tree_a = fetch('./mcts-trees/tree_a.json').then(r => r.json());
const tree_b = fetch('./mcts-trees/tree_b.json').then(r => r.json());
const rootIsEdge = (tree: any) => Boolean(tree.mv);
// console.log(tree_a, tree_b);
</script>

<div class="top">
  {#await Promise.all([tree_a, tree_b]) then trees}
    {#each trees as tree}
      <div>
        <svelte:component this={rootIsEdge(tree) ? Edge : Node} {...tree} />
      </div>
    {/each}
  {/await}
</div>

<style>
.top {
  display: grid;
  grid-template-columns: 1fr 1fr;

  grid-gap: var(--s-5);
}
</style>
