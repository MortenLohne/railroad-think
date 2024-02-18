<!--
  This component is a wrapper for @nrk/core-datepicker.
  Documentation for that can be found here:
  https://static.nrk.no/core-components/latest/index.html?core-datepicker/readme.md
-->
<script context="module">
import { browser } from '$app/env';
import CoreDatepicker from '@nrk/core-datepicker';

if (browser && customElements.get('core-datepicker') === undefined) {
  window.customElements.define('core-datepicker', CoreDatepicker);
}
</script>

<script>
// @ts-nocheck
import { createEventDispatcher } from 'svelte';

// Initial state and bindings
export let date = 'now';
export let months = '';
export let days = '';
export let disabled = false;

// // Additional elements
// export let input = false;
// export let select = false;
// export let table = false;

// State
export let timestamp = null;
export let year = null;
export let month = null;
export let day = null;
export let hour = null;
export let minute = null;
export let second = null;

const dispatch = createEventDispatcher();

function init(datepicker) {
  dispatch('init', datepicker);

  // Set initial custom element state
  datepicker.date = date;
  datepicker.months = months;
  datepicker.days = days;
  datepicker.disabled = disabled;

  ({ timestamp, year, month, day, hour, minute, second } = datepicker);

  datepicker.addEventListener(
    'datepicker.change',
    ({ target: datepicker, detail: date }) => {
      // Sync component state
      ({ timestamp, year, month, day, hour, minute, second } = datepicker);
      // Forward as component event
      dispatch('change', date);
    }
  );
}
</script>

<div>
  <core-datepicker {date} {months} use:init>
    <slot />
  </core-datepicker>
</div>

<style>
core-datepicker {
  position: relative;
}

core-datepicker :global(table) {
  width: 100%;
  table-layout: fixed;
  border-collapse: collapse;
  text-align: center;
}

core-datepicker :global(th) {
  color: var(--gray-shade-1);
  font-weight: normal;
  font-size: var(--small);
}

core-datepicker :global(thead) {
  border-bottom: 4px solid transparent;
}

core-datepicker :global(td) {
  width: 0;
  position: relative;
}

core-datepicker :global(caption) {
  margin-bottom: var(--s-1);
}

core-datepicker :global(td > button) {
  width: 100%;
  padding: 4px;
  cursor: pointer;
  border: none;

  --background-color: transparent;
  --border-color: transparent;
}

core-datepicker :global(td > button::after) {
  display: block;
  content: '';
  box-sizing: border-box;
  position: absolute;
  z-index: -1;
  left: 1px;
  top: 1px;
  width: calc(100% - 2px);
  height: calc(100% - 2px);
  border-radius: var(--s-1);
  border: 1px solid transparent;
  background-color: var(--background-color);
  border-color: var(--border-color);
}

core-datepicker :global(button[data-adjacent='true']) {
  color: var(--gray-tint-1);
}

core-datepicker :global(button[aria-current='date']) {
  /* Targets the current date (today) in month view */
  color: var(--blue);
}

core-datepicker :global(td:nth-last-child(1) button),
core-datepicker :global(td:nth-last-child(2) button) {
  color: var(--red);
}

core-datepicker :global(td:nth-last-child(1) button[data-adjacent='true']),
core-datepicker :global(td:nth-last-child(2) button[data-adjacent='true']) {
  color: var(--red-tint-2);
}

core-datepicker :global(td button[autofocus]) {
  /* Targets the chosen date in month view */
  color: var(--white);
  --background-color: var(--blue);
}

core-datepicker :global(td > button:hover) {
  color: var(--black);
  --background-color: var(--blue-tint-4);
  --border-color: var(--blue-tint-1);
}

core-datepicker :global(button[autofocus]:hover) {
  color: var(--white);
  --background-color: var(--blue-shade-1);
  --border-color: var(--blue-shade-1);
}

core-datepicker :global(button:focus-visible) {
  outline: 2px solid var(--blue);
  outline-offset: 1px;
}
</style>
