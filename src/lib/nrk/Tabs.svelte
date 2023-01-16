<!--
  This component is a wrapper for @nrk/core-tabs.
  Documentation for that can be found here:
  https://static.nrk.no/core-components/latest/index.html?core-tabs/readme.md
-->
<script context="module">
import { browser } from '$app/env';
import CoreTabs from '@nrk/core-tabs';

if (browser && customElements.get('b4-core-tabs') === undefined) {
  window.customElements.define('b4-core-tabs', CoreTabs);
}
</script>

<script>
// @ts-nocheck
import { createEventDispatcher } from 'svelte';

export let tab = undefined;

let tabsElement;
const dispatch = createEventDispatcher();
function init(node) {
  tabsElement = node;
  node.tab = tab;

  node.addEventListener('tabs.toggle', event => {
    dispatch('toggle', event);
  });
}

$: if (tabsElement) tabsElement.tab = tab;
</script>

<b4-core-tabs use:init>
  <slot />
</b4-core-tabs>

<slot name="panels" />
