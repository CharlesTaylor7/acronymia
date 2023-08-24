# Acronymia

Welcome to Acronymia, a party game based aroud, you guessed it, acronyms.

In each round, players will be given a prompt and corresponding acronym. Players will then come up with and submit a breakdown for the acronym that corresponds with the prompt. Per round, one player will be designated as the judge rather than submit an answer and will choose their favorite answer. The player with their answer chosen will be awarded points based on the length of the acronym needed.

The game will either run until one player reaches a score threshold or until the designated number of rounds passes. In the latter case, the highest score wins.

In order to play, a host will need to install and run the game. Once the game is up and running, players can join through their web browser.

Potential features include choosing themes for the game, which could change prompts given and icons players can choose to represent themselves while playing, as well as acronym length and round limit modifiers.

## Features

### Host
The first person to join the game is granted host privileges which include a debug view and the ability to impersonate or kick any player.

There is no multi-game support. It only supports one active game running at a time.

### Letter Bag
The ratio of letters matches the ratio of first letters in the english language.
Table pulled from [Wikipedia](https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_the_first_letters_of_a_word_in_English_language).

### Acronym selection
The judge pulls out letters at random from the bag.

Always 3 letters for now.

Future Ideas:
- [ ] Instead of random letters, let the judge pick an acronym for the round.
- [ ] Make the random selection enforce pronounceability. (For now the game could be called initialismia).


### Timers
60 seconds for acronym submission.
45 seconds for judging.
10 seconds for showing round winner before advancing to the next round.

## TODO
- [x] Prompts / Themed rounds
- [x] Show countdown until next round
- [ ] Able to see your point total during game play
- [ ] Warn players about penalties before they submit


### Feature requests
- Configurable Letter Distribution
- Audio cues when you need to make a decision.
- Players can pick colors / icons to represent themselves.

### Scoring Ideas
- Audience Vote mechanic for additional scoring
- Double points per the second round of play

## Implementation Notes

- Web sockets for all server <-> client communication. Polling & SSE were both leading to poor user experience.

- Central server thread with in memory state & synchronized with client handler threads via message passing.

- SPA (Single Page Application)


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

# Development

This project was bootstrapped with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Running the project

Install `cargo`, `cargo-leptos`, & `npm`.

- Run the web server: `cargo leptos watch`
- Run tailwind to bundle the css: `npm run tailwind -- --watch`
- Lint rust code: `cargo clippy`
- Run server unit tests: `cargo test --features=ssr`
- Run Playwright tests: `cargo leptos end-to-end`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future)
