<script>
import Select from './Select.svelte';
import Button from './Button.svelte';
import Encoding from './Encoding.svelte';
import BoardRenderer from './BoardRenderer.svelte';
import { group } from 'd3-array';

export let solver;

const BOARD_SIZE = 7;

let controller = new solver.BoardController();
let turn, board;
$: board = controller.get();
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

function expandSquare(sq) {
  return {
    raw: sq,
    x: sq % BOARD_SIZE,
    y: Math.floor(sq / BOARD_SIZE),
  };
}
function expandPlacement(p) {
  return {
    ...p,
    square: expandSquare(p.square.raw),
  };
}

const cachedCandidates = new Map();
function getCandidates(piece) {
  if (!piece) return new Map();
  if (!cachedCandidates.has(piece)) {
    let possible = controller.findPossible(+piece);
    console.log(possible);
    possible = possible.map(expandPlacement);
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
      board = controller.get();
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
  cachedCandidates.clear();
}

function reset() {
  clearState();
  controller = new solver.BoardController();
}

let someScore;
function score() {
  someScore = controller.score();
}

function decode({ detail: encoding }) {
  try {
    controller = solver.BoardController.decode(encoding);
  } catch (e) {
    console.error(e, 'Could not decode');
  }
}

let selected;
$: candidates = getCandidates(selected);
let selectedCandidate = null;
</script>

<Select bind:selected sandbox={true} />

<BoardRenderer {candidates} bind:selectedCandidate {placements} {frontier} />

<div class="control-panel">
  <Button disabled={!selectedCandidate} on:click={place(selectedCandidate)}>
    Place tile
  </Button>
  <Button disabled={turn === 0} on:click={reset}>Reset</Button>
  <div>
    <Button on:click={score}
      >Score
      {#if someScore}
        = {someScore}
      {/if}
    </Button>
  </div>
  <Encoding {encoding} on:decode={decode} />
</div>

<style>
.control-panel {
  margin-top: var(--s-2);
  display: grid;
  grid-gap: var(--s-2-5);
}
</style>
