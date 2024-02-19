# A Railroad Ink bot

Railroad Ink is a game where you draw roads and railways on a grid. You get points for connecting exits, creating unbroken roads and railways, using the center of the board, and you lose points for open paths. The game is played over 7 rounds, and at the start of every round you roll dice to see the roads and railways you must draw.

This is a program that plays that game. It uses a Monte Carlo Tree Search to evaluate moves, and a neural network to evaluate the board state.

Inspired by [Tiltak](https://github.com/MortenLohne/tiltak), an AI for the board game Tak.

## Running

Due to some depency shenanigans I'm hoping will be resolved soon, you'll need to run this with a nightly version of rust:

```sh
cargo +nightly-2024-02-04 run -r -- <command>
```

Install the nightly version of rust with:

```sh
rustup toolchain install nightly-2024-02-04
```

## Commands

### `play`

Let the AI play some Railroad Ink. By default, it will play a single game, spending a second to evaluate each move. The moves will be printed to the console.

`-c`, `--count` - The number of games to play. Default is 1.

`-d`, `--duration` - The time to spend evaluating each move, in seconds. Default is 1.

`-i`, `--iterations` - The number of iterations to run the MCTS algorithm. If you specify this, the program will search the tree for the specified number of iterations, instead a specified duration.

### `train`

Train the AI. Sorta. This is all just a pile of things that are vaguely useful for me as I go along.

`-t`, `--train` - A boolean flag that determines if the program should train the neural network it uses to evaluate moves. Defaults to true.

`-g`, `--generate-training-data` - A boolean flag that determines if the program should run some games to create more data to train on. Defaults to false.

`-l`, `--loop-training` - This boolean flag tells the program to never stop training.
