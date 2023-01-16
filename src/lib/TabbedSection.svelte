<script>
import Section from '/src/lib/Section.svelte';
import Tabs from '/src/lib/nrk/Tabs.svelte';
</script>

<Section>
  <Tabs>
    <div class="tabs">
      <slot name="tabs" />
    </div>
    <slot slot="panels" />
  </Tabs>
</Section>

<style>
.tabs :global(button),
.tabs :global(a) {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--color, var(--black));
  padding: var(--s-2) var(--s-3);
  border: 1px solid transparent;
  border-bottom-color: var(--gray-tint-3);
  border-top-color: var(--gray-tint-3);
}

.tabs :global(button[aria-selected='false']),
.tabs :global(a[aria-selected='false']) {
  background-color: var(--background-color-selected, var(--gray-tint-4));
  color: var(--color-closed, var(--gray-shade-1));
}

.tabs :global(button:hover),
.tabs :global(a:hover),
.tabs :global(button:focus),
.tabs :global(a:focus),
.tabs :global(button:active),
.tabs :global(a:active) {
  color: var(--color, var(--black));
}

.tabs :global(:first-child) {
  border-top-left-radius: var(--s-2);
  border-left-color: var(--gray-tint-3);
}

.tabs :global(:last-child) {
  border-top-right-radius: var(--s-2);
  border-right-color: var(--gray-tint-3);
}

.tabs :global(button[aria-selected='true']),
.tabs :global(a[aria-selected='true']) {
  border-color: var(--border-color-selected, var(--gray-tint-3));
  border-bottom-color: var(--gray-tint-3);
  border-bottom-style: dashed;
  font-weight: 550;
}

.tabs {
  margin: calc(-1 * var(--s-3) - 1px);
  margin-top: calc(-1 * var(--s-3) - 1px);
  margin-bottom: var(--s-2-5);
  display: flex;
  flex-direction: row;
}
</style>
