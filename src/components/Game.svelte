<script>
import Select from '/src/components/Select.svelte';
import Button from '/src/components/Button.svelte';
import Encoding from '/src/components/Encoding.svelte';
import { group } from 'd3-array';

import BoardRenderer from './BoardRenderer.svelte';

export let solver;

const BOARD_SIZE = 7;

let controller = new solver.GameController();

let turn, board, toPlace, expendedSpecials, specialPlaced;
$: game = controller.get();
$: ({ turn, board, toPlace, expendedSpecials, specialPlaced } = game);
// TODO: Remove all this and just use the raw output from WASM?
// Or maybe this is the right place for it ... other components shouldn't need to care
// about the serialized representation of the game ...
$: frontier = getFrontier(board.frontier);
$: placements = board.placements.filter(d => d).map(expandPlacement);
$: encoding = controller.encode();
function getFrontier(frontier) {
  return Object.entries(frontier).map(([sq, kind]) => [parseSquare(sq), kind]);
}
function parseSquare(sq) {
  let [x, y] = sq.split('');
  x = +x;
  y = parseInt(y, 17) - 10;
  return {
    x,
    y,
    raw: x + y * BOARD_SIZE,
  };
}
$: console.log({ turn, board, toPlace, expendedSpecials, specialPlaced });

function expandSquare(square) {
  return {
    raw: square.raw,
    x: square.raw % BOARD_SIZE,
    y: Math.floor(square.raw / BOARD_SIZE),
  };
}
function expandPlacement(p) {
  return {
    ...p,
    square: expandSquare(p.square),
  };
}

const cachedCandidates = new Map();
function getCandidates(piece) {
  if (!piece) return new Map();
  if (!cachedCandidates.has(piece)) {
    const possible = controller.findPossible(+piece).map(expandPlacement);
    cachedCandidates.set(
      piece,
      group(possible, d => d.square.raw)
    );
  }
  return cachedCandidates.get(piece);
}

function place(candidate) {
  return function () {
    try {
      controller.place(candidate);
      selected = null;
      selectedCandidate = null;
      cachedCandidates.clear();
      game = controller.get();
      encoding = controller.encode();
    } catch (e) {
      console.error(e);
    }
  };
}

function clearState() {
  selected = null;
  someScore = null;
  selectedCandidate = null;
  encoding = '';
  aiSuggestion = null;
  cachedCandidates.clear();
}

function reset() {
  clearState();
  controller = new solver.GameController();
}

let someScore;
function score() {
  someScore = controller.score();
}

function decode({ detail: encoding }) {
  // clearState();
  try {
    controller.decode(encoding);
    game = controller.get();
  } catch (e) {
    console.error(e, 'Could not decode');
  }
}

function roll() {
  try {
    controller.roll();
    clearState();
    game = controller.get();
  } catch (e) {
    console.error(e);
  }
}

let aiSuggestion = null;
function mcts() {
  console.log(controller);
  let move = controller.searchFor(100);

  if (move.startsWith('Place')) {
    const square = parseSquare(move.slice(6, 8));
    const piece = parseInt(move.slice(8, 10), 16);
    const orientation = +move.slice(10, 11);
    aiSuggestion = {
      square,
      piece,
      orientation: {
        flipped: orientation > 3,
        rotation: orientation % 4,
      },
    };
  } else {
    aiSuggestion = null;
  }
}

function autoplay() {
  controller.autoplay(100);
  selected = null;
  selectedCandidate = null;
  // cachedCandidates.clear();
  game = controller.get();
  encoding = controller.encode();
}

let selected;
$: candidates = getCandidates(selected);
let selectedCandidate = null;
</script>

<Select
  bind:selected
  {toPlace}
  {expendedSpecials}
  {specialPlaced}
  on:roll={roll}
/>

<BoardRenderer
  {aiSuggestion}
  {candidates}
  bind:selectedCandidate
  {placements}
  {frontier}
/>

<div class="control-panel">
  <Button disabled={!selectedCandidate} on:click={place(selectedCandidate)}>
    Place tile
  </Button>
  <Button disabled={turn === 0} on:click={reset}>Reset</Button>
  <Button on:click={mcts}>MCTS!</Button>
  <Button on:click={autoplay}>Autoplay</Button>
  <div>
    <Button on:click={score}
      >Score
      {#if someScore}
        = {someScore}
      {/if}
    </Button>
  </div>
  <Encoding encoding={turn === 0 ? '' : encoding} on:decode={decode} />
</div>

<style>
.control-panel {
  margin-top: var(--s-2);
  display: grid;
  grid-gap: var(--s-2-5);
}
</style>
