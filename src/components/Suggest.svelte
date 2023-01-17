<script>
import { names } from '../lib/pieces';
export let suggestion;

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

<img
  src="./assets/{names[suggestion.piece]}.png"
  alt=""
  style={displayTile(suggestion)}
/>

<style>
img {
  opacity: 0.7;
  box-shadow: 0 0 0 3px var(--blue);
}
</style>
