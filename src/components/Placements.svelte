<script>
import { names } from '../lib/pieces';
export let placements = [];

const displayTile = tile => `
  grid-column: ${tile.square.x + 2};
  grid-row: ${tile.square.y + 2};
  transform:
    rotate(${
      tile.orientation.rotation * 90 + (tile.orientation.flip ? 0 : 0)
    }deg)
    scaleX(${tile.orientation.flip ? -1 : 1});
`;
</script>

{#each placements as tile}
  <img
    src="./assets/{names[tile.piece]}.png"
    alt=""
    style={displayTile(tile)}
  />
{/each}
