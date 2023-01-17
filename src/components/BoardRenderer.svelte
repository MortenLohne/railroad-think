<script>
import exits from '../data/exits.csv';
import VisibleGrid from './VisibleGrid.svelte';
import Candidates from './Candidates.svelte';
import Placements from './Placements.svelte';
import Suggest from './Suggest.svelte';

export let aiSuggestion = null;
export let candidates = null;
export let selectedCandidate = null;
export let placements = null;
export let frontier = null;

const displayTile = tile => `
  grid-column: ${+tile.x + 2};
  grid-row: ${+tile.y + 2};
  transform: rotate(${tile.rotation * 90}deg);
`;

const displayConnection = ({ x, y }) => `
  grid-column: ${x + 2};
  grid-row: ${y + 2};
`;
</script>

<div class="outer-board">
  <VisibleGrid />
  {#if aiSuggestion}
    <Suggest suggestion={aiSuggestion} />
  {/if}
  <Candidates {candidates} bind:selected={selectedCandidate} />

  <Placements {placements} />
  {#each exits as exit}
    <img
      class="exit"
      src="./assets/exit {exit.type}.png"
      alt=""
      style={displayTile(exit)}
    />
  {/each}

  {#each frontier as [square, arr]}
    {#each arr as [direction, type]}
      <img
        alt=""
        class="frontier"
        src="./assets/connection {type.toLowerCase()}.png"
        class:north={direction === 'North'}
        class:east={direction === 'East'}
        class:south={direction === 'South'}
        class:west={direction === 'West'}
        style={displayConnection(square, type)}
      />
    {/each}
  {/each}

  <div class="board-square" />
  <div class="center-square" />
</div>

<style>
.outer-board {
  display: grid;
  justify-content: center;
  --outer-sides: calc(7 + 2);
  grid-template-rows: repeat(var(--outer-sides), 64px);
  grid-template-columns: repeat(var(--outer-sides), 64px);
}

.board-square {
  grid-row: 2 / calc(7 + 2);
  grid-column: 2 / calc(7 + 2);
  box-shadow: inset 0 0 0 2px black;
}
.center-square {
  grid-row: 4 / 7;
  grid-column: 4 / 7;
  box-shadow: inset 0 0 0 2px red;
  background-color: #f502;
  position: relative;
  z-index: 1;
  mix-blend-mode: multiply;
  pointer-events: none;
}

img {
  display: block;
  user-select: none;
}

.exit {
  box-shadow: none;
}

.frontier {
  width: 20px;
  height: 20px;
  place-self: center;
  position: relative;
  z-index: 1;
}

.frontier.north {
  align-self: start;
}
.frontier.east {
  justify-self: end;
}
.frontier.south {
  align-self: end;
}
.frontier.west {
  justify-self: start;
}
</style>
