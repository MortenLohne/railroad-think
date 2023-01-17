<script>
let tiles = [];

for (let y = 0; y < 7; y++) {
  for (let x = 0; x < 7; x++) {
    tiles.push([x, y]);
  }
}

const letter = n => (n + 10).toString(17);
</script>

{#each tiles as [x, y]}
  <div class="grid" style="--x: {x + 2}; --y: {y + 2}" />
{/each}

{#each [...Array(7).keys()] as n}
  <p style="--x: 1; --y: {n + 2}; justify-self: start">{letter(n)}</p>
  <p style="--x: 9; --y: {n + 2}; justify-self: end">{letter(n)}</p>
  <p style="--y: 1; --x: {n + 2}; align-self: start">{n}</p>
  <p style="--y: 9; --x: {n + 2}; align-self: end">{n}</p>
{/each}

<style>
div,
p {
  grid-row: var(--y);
  grid-column: var(--x);
}

.grid {
  box-shadow: inset 0 0 0 0.5px #0002;
  pointer-events: none;
}

p {
  user-select: none;
  color: var(--gray-shade-1);
  font-size: var(--small);
  place-self: center;
}
</style>
