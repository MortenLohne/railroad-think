<script>
import Button from './Button.svelte';
import { rollable, placeable, codes, names } from '../lib/pieces';
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();

export let selected = null;
export let toPlace = null;
export let expendedSpecials = null;
export let specialPlaced = null;
export let sandbox = false;

$: specialsDisabled = expendedSpecials
  ? expendedSpecials.filter(d => d).length === 3 || specialPlaced !== null
  : false;

let pieces = [];
let placed = [];
$: updateToPlace(toPlace);

function updateToPlace(toPlace) {
  if (!toPlace) return;
  if (toPlace.length < pieces.length) {
    const toPlaceSlice = [...toPlace];
    placed = pieces.map((_, i) => {
      const idx = toPlaceSlice.indexOf(pieces[i]);
      if (idx === -1) return true;
      toPlaceSlice.splice(idx, 1);
      return false;
    });
  } else {
    pieces = toPlace;
    placed = pieces.map(_ => false);
  }
}

const categories = { placeable, rollable };

function handleClick(code) {
  return function (evt) {
    if (selected === code) {
      evt.preventDefault();
      selected = null;
    }
  };
}

function roll() {
  dispatch('roll');
}
</script>

<ul class:some-selected={selected}>
  {#each Object.entries(categories) as [category, pieces]}
    {#each pieces as piece}
      <li
        on:click={handleClick(codes[piece])}
        class:placeable={category === 'placeable'}
        class:rollable={category === 'rollable'}
        class:selected={selected === codes[piece]}
      >
        <label>
          <input
            type="radio"
            aria-hidden="true"
            disabled={!sandbox &&
              (category !== 'placeable' || specialsDisabled)}
            bind:group={selected}
            value={codes[piece]}
            name={piece}
          />
          <img src="./assets/{piece}.png" alt={piece} />
        </label>
      </li>
    {/each}
  {/each}

  {#if toPlace}
    {#each pieces as piece, i}
      <li
        on:click={handleClick(codes[piece])}
        class="rollable"
        class:selected={selected === i}
      >
        <label>
          <input
            type="radio"
            aria-hidden="true"
            disabled={placed[i]}
            bind:group={selected}
            value={piece.toString()}
            name={names[piece]}
          />
          <img src="./assets/{names[piece]}.png" alt={names[piece]} />
        </label>
      </li>
    {/each}
    <div class="btn">
      <Button disabled={toPlace.length > 0} on:click={roll}>Roll</Button>
    </div>
  {/if}
</ul>

<div class="selected-affordance">
  {#if selected}
    Selected:
    <span>{names[selected]}</span>
    <code>{selected}</code>
  {:else}
    <span class="null">No piece selected</span>
  {/if}
</div>

<style>
ul {
  list-style-type: none;
  display: grid;
  padding: 0;
  max-width: 500px;
  margin: 0 auto;
  --unit: 1fr;
  grid-template-columns: repeat(18, var(--unit));
  grid-template-rows: repeat(5, var(--unit));
  grid-gap: var(--s-1);
  justify-content: center;
}

li {
  display: block;
  line-height: 0;
}

label {
  display: block;
}

[aria-hidden] {
  display: none;
}

img {
  cursor: pointer;
  display: block;
  border: 2px solid var(--black);
  border-radius: var(--s-2);
  background-color: var(--white);
}

.placeable {
  grid-column: span 3;
  grid-row: span 3;
}
.rollable {
  grid-column: span 2;
  grid-row: span 2;
}

img {
  width: 100%;
  height: 100%;
}

/* .some-selected img, */
input:disabled + img {
  opacity: 0.25;
}

label:hover img {
  border-color: var(--blue);
  opacity: 1;
}

.some-selected label:hover img {
  border-color: var(--black);
}

.selected img {
  opacity: 1;
}

.selected img {
  border-color: var(--blue);
}

.selected-affordance {
  margin-top: var(--s-1);
  text-align: center;
  margin-bottom: var(--s-2);
}

.selected-affordance span.null {
  color: var(--gray);
}

.selected-affordance span:not(.null) {
  font-weight: 600;
  background-color: var(--blue);
  padding: 1px var(--s-1) 2px;
  border-radius: var(--s-1);
  color: var(--white);
}

.btn {
  grid-column: span 3;
  grid-row: span 2;
  width: 100%;
  display: block;
  place-self: center;
}
</style>
