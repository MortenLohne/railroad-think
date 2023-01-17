<script lang="ts">
import { names } from '../lib/pieces';
export let candidates = new Map();
export let selected = null;

let tile: null | string = null;
let rotation = 0;

$: candidates && reset();

function reset() {
  tile = null;
  rotation = 0;
}

function handleClick(evtTile: any, _candidate: any) {
  return function () {
    if (evtTile === tile) {
      rotation = (rotation + 1) % candidates.get(tile).length;
    } else {
      tile = evtTile;
    }

    selected = candidates.get(tile)[rotation];
  };
}

const displayTile = (tile: any) => `
  grid-column: ${tile.square.x + 2};
  grid-row: ${tile.square.y + 2};
  transform:
    rotate(${
      tile.orientation.rotation * 90 + (tile.orientation.flip ? 0 : 0)
    }deg)
    scaleX(${tile.orientation.flip ? -1 : 1});
`;
</script>

{#each [...candidates.keys()] as candidateTile}
  {#each candidates.get(candidateTile) as candidate, i}
    <button
      class="candidate tile"
      style={displayTile(candidate)}
      class:selected={tile === candidateTile && rotation === i}
      class:some-selected={tile}
      on:click={handleClick(candidateTile, candidate)}
    >
      <img src="./assets/{names[candidate.piece]}.png" alt="" />
    </button>
  {/each}
{/each}

<style>
.candidate {
  opacity: 0.5;
}

.some-selected {
  opacity: 0.05;
}

.selected {
  opacity: 1;
}
</style>
