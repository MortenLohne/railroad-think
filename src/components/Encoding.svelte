<script lang="ts">
import { fly, fade } from 'svelte/transition';
import { expoOut } from 'svelte/easing';
import { createEventDispatcher } from 'svelte';

export let encoding = '';

let copied = false;
let copiedAffordanceTimeout: null | ReturnType<typeof setTimeout> = null;
async function copy() {
  const data = [
    new ClipboardItem({
      'text/plain': new Blob([encoding], { type: 'text/plain' }),
    }),
  ];

  await navigator.clipboard.write(data);

  if (copiedAffordanceTimeout !== null) return;
  copied = true;
  copiedAffordanceTimeout = setTimeout(() => {
    copied = false;
    if (copiedAffordanceTimeout !== null)
      clearTimeout(copiedAffordanceTimeout as unknown as number);
    copiedAffordanceTimeout = null;
  }, 2000);
}

const dispatch = createEventDispatcher();
function decode() {
  navigator.clipboard.readText().then(str => dispatch('decode', str));
}
</script>

<div class="encoding">
  <div>
    <button on:click={decode}>Decode clipboard</button>
    {#if encoding}
      <button on:click={copy} class:copied>Copy encoding</button>
      {#if copied}
        <div
          in:fly|local={{ y: -5, easing: expoOut }}
          out:fly|local={{ y: 5, easing: expoOut }}
          class="copied-affordance"
        >
          <span>Copied!</span>
        </div>
      {/if}
    {/if}
  </div>
  {#if encoding}
    <pre class:copied in:fade>
      <span>{encoding}</span>
    </pre>
  {/if}
</div>

<style>
.encoding {
  margin: 0 auto;
  max-width: 600px;
  width: 100%;
  display: grid;
  grid-gap: var(--s-2);
}
pre {
  white-space: pre-line;
  word-break: break-all;
  /* overflow: hidden;
  position: relative; */
  background-color: var(--gray-tint-3);
  border-color: var(--gray-tint-2);
  color: var(--gray-shade-2);
}

@keyframes blitz {
  from {
    background-color: var(--white);
    border-color: var(--green-tint-2);
    color: var(--green);
  }
  to {
    background-color: var(--gray-tint-3);
    border-color: var(--gray-tint-2);
    color: var(--gray-shade-2);
  }
}

span {
  display: block;
}

pre.copied {
  animation: blitz 2000ms linear;
}

button {
  display: inline-block;
  background-color: var(--gray-tint-3);
  color: var(--gray-shade-2);
  padding: var(--s-1);
  border-radius: var(--s-2);
  border: 2px solid var(--gray-tint-2);
}

button:hover {
  background-color: var(--white);
  border-color: var(--gray-tint-1);
}

button:active {
  background-color: var(--info-tint-3);
  border-color: var(--info);
  color: var(--gray-shade-3);
}

.copied-affordance {
  display: inline-block;
  color: var(--green-shade-1);
  position: relative;
  padding-left: calc(var(--s-3) + var(--s-1));
}

.copied-affordance span {
  font-size: var(--small);
}

.copied-affordance::after,
.copied-affordance::before {
  width: 1em;
  top: 50%;
  transform: translateY(-50%);
  position: absolute;
  left: 0;
  display: block;
  content: '';
  height: 1em;
  margin: 0 !important;
}
.copied-affordance::before {
  border-radius: 50%;
  background-color: var(--green);
}

.copied-affordance::after {
  left: 1px;
  font-weight: bold;
  color: var(--white);
  content: '✓';
  transform: translateY(calc(-50% - 2px));
}
</style>
