<script>
import Suggest from '/src/components/nrk/Suggest.svelte';

export let value = '';
export let options = [];
export let placeholder = '';
export let icon = null;

function handleSelect(evt) {
  value = evt.detail.detail.innerText;
}

const search =
  'M62.54,55.47,51.06,44h0l-6.77-6.78a24.1,24.1,0,1,0-7.08,7.08L44,51.06h0L55.47,62.54a5,5,0,0,0,7.07-7.07ZM11.27,36.88a18.12,18.12,0,1,1,12.8,5.3A18.13,18.13,0,0,1,11.27,36.88Z';
const dropdown = 'M48.37,27.22,32,46.78,15.63,27.22Z';
function getIcon(icon) {
  switch (icon) {
    case 'search':
      return search;
    case 'dropdown':
      return dropdown;
    default:
      return icon;
  }
}
</script>

<div>
  <input bind:value type="text" {placeholder} class:icon={icon !== null} />
  <Suggest on:suggest.select={handleSelect} hidden={true}>
    <ul>
      {#each options as option}
        <li>
          <button>{option}</button>
        </li>
      {/each}
    </ul>
  </Suggest>

  {#if icon !== null}
    <svg aria-hidden="true" viewBox="0 0 64 64" width="1em" height="1em">
      <path d={getIcon(icon)} />
    </svg>
  {/if}
</div>

<style>
div {
  position: relative;
  border-radius: 999px;
  background-color: var(--white);
}

input {
  padding: var(--s-2) var(--s-3);
  font-size: inherit;
  font-family: inherit;
  color: inherit;
  border-radius: 999px;
  border: 1px solid var(--gray-tint-2);
  width: 100%;
  position: relative;
  z-index: 1;
  background-color: transparent;
}

input.icon {
  padding-left: 5ch;
}

input:global([aria-expanded='true']) {
  border-radius: var(--s-3);
  border-bottom-left-radius: 0;
  border-bottom-right-radius: 0;
  border-bottom-color: var(--gray-tint-3);
}

ul {
  position: absolute;
  background-color: var(--white);
  list-style-type: none;
  width: 100%;
  padding: var(--s-1) var(--s-2);
  border: 1px solid var(--gray-tint-2);
  border-top: none;
  border-bottom-right-radius: var(--s-2);
  border-bottom-left-radius: var(--s-2);
}

button {
  display: block;
  width: calc(100% + 2 * var(--s-2));
  margin: 0 calc(-1 * var(--s-2));
  padding: var(--s-0-5) var(--s-3);
}

div :global(mark) {
  background-color: var(--info-tint-3);
  mix-blend-mode: multiply;
  border: 1px solid var(--info-tint-2);
  border-radius: 3px;
  color: var(--black);
  margin: 0 -2px;
  padding: 0 1px;
}

button:hover,
button:focus {
  background-color: var(--info-tint-3);
}

svg {
  position: absolute;
  top: 0;
  left: var(--s-1-5);
  height: 100%;
  width: 2.25em;
  padding: var(--s-1-5) var(--s-2);
  color: inherit;
  border-right: 1px solid var(--gray-tint-3);
  pointer-events: none;
  user-select: none;
  z-index: 0;
}

svg path {
  fill: currentColor;
}
</style>
