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

### Acronym Selection
Acronyms are by default between 2-6 letters long, but any range of lengths can be chosen at game setup time.

The probability of any letter being selected is proportional to its frequencey amongst the first letter of words in the English language. 
The letter frequency table has been adapted from [Wikipedia](https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_the_first_letters_of_a_word_in_English_language).

### Timers
60 seconds for acronym submission.
45 seconds for judging.
10 seconds for showing round winner before advancing to the next round.

### Feature requests
- Configurable Letter Distribution
- Audio cues when you need to make a decision.
- Players can pick colors / icons to represent themselves.

### Scoring Ideas
- Audience Vote mechanic for additional scoring
- Double points per the second round of play

# Development
This project is proudly built with the [Leptos](https://github.com/leptos-rs/leptos) web framework and bootstrapped from the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Project Setup

You will need `rustup` & `npm` for these steps: 
```sh
rustup toolchain install nightly --allow-downgrade  # ensure you have Rust nightly
rustup target add wasm32-unknown-unknown            # ensure you can compile to Web Assembly
cargo install cargo-leptos                          # installs development scripts for a leptos project
npm install                                         # installs tailwind & playwright 

# (optional) if you want to run playwright tests 
npm run playwright-setup
```

## Development commands
- Run the dev server: `cargo leptos watch --features=dev`
- Run tailwind to bundle the css: `npm run tailwind -- --watch`
- Run unit tests: `cargo leptos test`
- Run Playwright tests: `cargo leptos end-to-end`
- Build the production server: `cargo leptos build --release`

## Using Cargo by itself
`cargo build ` will fail by itself will fail to build.
That's because there is conditionally compiled code in the project, that requires either the `hydrate` or `ssr` flag to be set.

So for example, if you want to just focus on the server code. You can still run commands like `test`, `clippy`, `check` with cargo by itself. You just need to append `--features=ssr` to all those commands.

## Using cargo-leptos
To manage the project as a whole, it's best to use the `cargo-leptos` extension.
`cargo leptos watch` is equivalent to watching both frontend & backend builds at the same time.
