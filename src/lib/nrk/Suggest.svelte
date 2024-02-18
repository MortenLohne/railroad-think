<!--
  This component is a wrapper for @nrk/core-tabs.
  Documentation for that can be found here:
  https://static.nrk.no/core-components/latest/index.html?core-suggest/readme.md
-->
<script context="module">
import { browser } from '$app/env';
import CoreSuggest from '@nrk/core-suggest';

if (browser && customElements.get('core-suggest') === undefined) {
  window.customElements.define('core-suggest', CoreSuggest);
}
</script>

<script>
// @ts-nocheck
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();

export let limit = 5;
export let highlight = 'on';
export let hidden = false;
export let id = undefined;

let suggest;
function init(node) {
  suggest = node;
  suggest.limit = limit;
  suggest.highlight = highlight;
  suggest.hidden = hidden;
  if (id !== undefined) suggest.id = id;

  const forward = eventName => event => dispatch(eventName, event);

  suggest.addEventListener('suggest.filter', forward('suggest.filter'));
  suggest.addEventListener('suggest.select', forward('suggest.select'));
  suggest.addEventListener(
    'suggest.ajax.beforeSend',
    forward('suggest.ajax.beforeSend')
  );
  suggest.addEventListener('suggest.ajax', forward('suggest.ajax'));
  suggest.addEventListener('suggest.ajax.error', forward('suggest.ajax.error'));
}

$: if (suggest) suggest.limit = limit;
$: if (suggest) suggest.highlight = highlight;
$: if (suggest) suggest.hidden = hidden;
</script>

<core-suggest use:init>
  <slot />
</core-suggest>
