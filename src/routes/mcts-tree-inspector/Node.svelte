<script context="module">
const fmt = new Intl.NumberFormat('nb-NO').format;
</script>

<script lang="ts">
import { max } from 'd3-array';
import Edge from './Edge.svelte';

export let children: null | any[] = null;
export let total_score = 0;
export let visits = 0;
export let roll: null | boolean = null;
export let heuristic = 0;
export let is_terminal = false;
// silence unused warnings
heuristic;
is_terminal;

const id = (Math.random() * 1e20).toString(16);

$: cap = max(children ?? [], d => d.visits);
$: v = cap;

$: view = children && children.filter(d => d.visits >= v);
</script>

<div class="node">
  {#if roll}
    Roll(<code>{roll}</code>)
  {/if}
  <p><span>Visits:</span> {fmt(visits)}</p>
  <p><span>Total:</span> {fmt(total_score)}</p>

  {#if children && visits > 0}
    <hr />
    <label>
      <p><span>Min:</span> {v}</p>
      <input {id} type="range" min="0" max={cap + 1} bind:value={v} />
    </label>
  {/if}
</div>

{#if visits > 0 && view}
  <div class="children">
    {#if children && view.length !== children.length}
      <p class="omitted-affordance">
        {children.length - view.length} edges omitted
      </p>
    {/if}
    {#each view as child}
      <Edge {...child} />
    {/each}
  </div>
{/if}

<style>
.node {
  display: inline-block;
  flex-direction: column;
  font-size: var(--small);
  background-color: var(--green-shade-1);
  color: var(--white);
  font-weight: 550;
  padding: var(--s-0-5) var(--s-2-5);
  border-radius: var(--s-2-5);
  border: 1px solid var(--green-shade-2);
}

.children {
  padding-left: var(--s-3);
  padding-top: var(--s-1);
  border-left: 2px solid var(--green-tint-2);
}

.omitted-affordance {
  color: var(--gray-shade-1);
  font-style: italic;
  font-size: var(--small);
  margin-bottom: var(--s-0-5);
}

span {
  font-size: var(--xsmall);
  color: var(--green-tint-2);
}

code {
  background-color: transparent;
  color: inherit;
  border: none;
  padding: none;
}

hr {
  border: none;
  border-top: 1px solid var(--green-shade-2);
  margin: var(--s-1) calc(-1 * var(--s-2-5)) var(--s-0-5);
}

label {
  display: grid;
  font-size: var(--xsmall);
  grid-template-columns: 8ch 1fr;
}

input[type='range'] {
  -webkit-appearance: none; /* Hides the slider so that custom slider can be made */
  width: 100%; /* Specific width is required for Firefox. */
  background: transparent; /* Otherwise white in Chrome */
}
input[type='range']::-webkit-slider-thumb {
  -webkit-appearance: none;
}

input[type='range']::-webkit-slider-thumb {
  -webkit-appearance: none;
  height: var(--s-2);
  width: var(--s-2);
  border-radius: var(--s-3);
  background: var(--black);
  cursor: pointer;
  margin-top: calc(-0.5 * var(--s-1));
}

input[type='range']::-webkit-slider-runnable-track {
  width: 100%;
  height: var(--s-1-5);
  cursor: pointer;
  background: var(--green-tint-3);
  border-radius: 9999px;
  border: 1px solid var(--green-shade-2);
}

input[type='range']:focus::-webkit-slider-runnable-track {
  background: white;
}
</style>
