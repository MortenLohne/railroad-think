<!--
  This component is a wrapper for @nrk/core-toggle.
  Documentation for that can be found here:
  https://static.nrk.no/core-components/latest/index.html?core-toggle/readme.md
-->
<script context="module">
import { browser } from '$app/env';
import CoreToggle from '@nrk/core-toggle';

if (browser && customElements.get('b4-core-toggle') === undefined) {
  window.customElements.define('b4-core-toggle', CoreToggle);
}
</script>

<script>
// @ts-nocheck
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();

export let hidden = true;
export let popup = false;
export let value = undefined;

let toggle;
function init(node) {
  toggle = node;
  toggle.popup = popup;
  toggle.hidden = hidden;
  if (value !== undefined) toggle.value = value;

  toggle.addEventListener('toggle', event => {
    hidden = toggle.hidden;
    dispatch('toggle', event);
  });

  toggle.addEventListener('toggle.select', event => {
    dispatch('toggle.select', event);
  });
}

$: if (toggle) toggle.hidden = hidden;
$: if (toggle) toggle.popup = popup;
</script>

<b4-core-toggle use:init>
  <slot />
</b4-core-toggle>
