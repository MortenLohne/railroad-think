<script>
export let disabled = false;
const style = `--r: ${Math.random() * 2 - 1}`;
</script>

<button {disabled} {style} on:click><slot /></button>

<style>
button {
  font-size: var(--large);
  display: block;
  margin: 0 auto;
  border: 2px solid var(--gray-tint-2);
  border-radius: var(--s-2);
  padding: var(--s-0-5) var(--s-2);
  background-color: var(--gray-tint-3);
  color: var(--black);
  transform: rotate(calc(var(--r) * 3deg));
  transition: transform 100ms;
}

button[disabled] {
  cursor: initial;
  font-weight: normal;
  color: var(--gray);
  border-color: var(--gray-tint-3);
}

button:not([disabled]):hover {
  transform: rotate(calc(var(--r) * 0.5deg));
  background-color: var(--white);
}

button:not([disabled]):active {
  color: var(--gray-shade-3);
  background-color: var(--info-tint-3);
  border-color: var(--info);
  transform: rotate(0) translateY(1px);
}
</style>
