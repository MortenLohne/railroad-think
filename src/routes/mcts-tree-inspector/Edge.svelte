<script context="module" lang="ts">
const fmt = new Intl.NumberFormat('nb-NO', { maximumFractionDigits: 2 }).format;
function fmtMv(move: string) {
  if (move.indexOf('(') === -1) {
    return move;
  }

  let [action, content] = move.split('(');
  content = content.replace(')', '');

  if (content.startsWith('Place')) {
    const pos = content.slice(0, 2);
    const piece = content.slice(2, 4);
    const rot = content.slice(4, 5);
    content = [pos, piece, rot].map(str => `<code>${str}</code>`).join('');
    return `${action}(${content})`;
  } else {
    content = content.replace('[', '').replace(']', '').replace(/\s/g, '');
    return `${action}(<code>${content}</code>)`;
  }
}
</script>

<script lang="ts">
import Node from './Node.svelte';
export let child: null | { visits: number } | object = null;
export let mean_score = 0;
export let visits = 0;
export let mv: any;

const singleChild =
  // @ts-ignore
  child && child.visits !== undefined ? (child as { visits: number }) : null;
</script>

<div class="edge">
  <div class="main">
    <p class="mv"><span>Move:</span> {@html fmtMv(mv)}</p>
    {#if visits > 0}
      <div>
        <p><span>Visits:</span> {fmt(visits)}</p>
        <p><span>Mean score:</span> {fmt(mean_score)}</p>
      </div>
    {/if}
  </div>

  {#if child}
    <div class="child">
      {#if singleChild}
        <Node {...singleChild} />
      {:else}
        {#each Object.entries(child) as [roll, rollChild]}
          <Node {...rollChild} {roll} />
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
.main {
  display: inline-block;
  font-size: var(--small);
  background-color: var(--blue-tint-3);
  border: 1px solid var(--blue);
  border-radius: var(--s-1);
  padding: var(--s-0-5) var(--s-1-5);
  color: var(--blue-shade-2);
}

.edge + :global(.edge) {
  margin-top: var(--s-0-5);
}

.child {
  margin-top: calc(-1 * var(--s-1));
  padding-left: var(--s-1);
  padding-top: var(--s-2);
  border-left: 2px solid var(--blue);
}

span {
  font-size: var(--xsmall);
  color: var(--blue-shade-1);
}

.mv > :global(code) {
  background-color: var(--blue-tint-2);
  padding: 0 var(--s-1);
  border: none;
  color: var(--blue-shade-2);
}

.mv > :global(code + code) {
  margin-left: var(--s-0-5);
}
</style>
