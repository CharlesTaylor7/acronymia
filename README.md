# Acronymia

Welcome to Acronymia, a party game based aroud, you guessed it, acronyms.

In each round, players will be given a prompt and corresponding acronym. Players will then come up with and submit a breakdown for the acronym that corresponds with the prompt. Per round, one player will be designated as the judge rather than submit an answer and will choose their favorite answer. The player with their answer chosen will be awarded points based on the length of the acronym needed.

The game will either run until one player reaches a score threshold or until the designated number of rounds passes. In the latter case, the highest score wins.

In order to play, a host will need to install and run the game. Once the game is up and running, players can join through their web browser (whether or not room codes or passwords will be used is to be determined.)

Potential features include choosing themes for the game, which could change prompts given and icons players can choose to represent themselves while playing, as well as acronym length and round limit modifiers.

## Features

### Letter Bag
The ratio of letters matches the ratio of first letters in the english language.
Table pulled from [Wikipedia](https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_the_first_letters_of_a_word_in_English_language).

### Acronym selection
The judge pulls out letters at random from the bag.

Always 3 letters for now.

Future Ideas:
- [ ] Instead of random letters, let the judge pick an acronym for the round.
- [ ] The number of letters in the acronym is itself random, between 3 to 8. 
- [ ] Make the random selection enforce pronounceability. (For now the game could be called initialismia).


## Implementation Notes

Uses Web sockets. Polling & SSE were both leading to poor user experience.

Central server thread with in memory state & message passing.

Pages:
- Lobby: "/"
- Active Game (SPA): `/game/<id>/`
- Game configaration `/game/<id>/config`


### Known Leptos bugs
These are bugs we've observed in Leptos.
This list serves 2 purposes:
  (1) a todo list of bug reports/PRs that I'd like to submit to the upstream repo.
  (2) provide workarounds for each issue.

- Putting a raw text element at the top level of component that lives inside a dynamic child can cause panics:
https://github.com/leptos-rs/leptos/blob/586f524015a543c9599205a91eac59bd7fca3c47/leptos_dom/src/components/dyn_child.rs#L245
  - Workaround: wrap text nodes in an html tag: span, div, etc.

- Server api functions can silently fail. If you use composite types in the function arguments the code compiles but fails completely silently at runtime. There's no panic, no console warning, no nothing. Strings & numbers work fine, vectors & structs don't. I believe it's actually supposed to work with any type that's serializable with Serde. 
  Workaround: apply the "Cbor" argument to the server macro. It uses a different serialization format then the default and actually works. The default deserialization format is Form data. 

### TODO for MVP
- [x] Setup step
  - [x] Players can set their nickname
  - [x] Allow game creator to start game (for MVP creator is the first registered player)
- [x] Submission step
  - [x] a synchronized 30 second timer.
  - [x] acronym input
  - [x] of submissions received
- [x] Judging step
  - [x] player view
      - [x] readonly list of submissions
  - [x] judge view 
      - [x] list of submissions each of which is a button for selecting winner
      - [ ] after a winner for the round is selected the next round starts with a small delay. Say 5 seconds for everyone to see the result
- [x] Results step
  - [x] Show a scoreboard of player points

- [x] Refactor SSE & server functions into using websockets for a more realtime experience


# Development

This project was bootstrapped with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Running your project

Install `cargo`, `cargo-leptos`, & `npm`.

- Run the web server: `cargo leptos watch`
- Run tailwind to bundle the css: `npm run watch-css`
- Lint rust code: `cargo clippy`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future)
