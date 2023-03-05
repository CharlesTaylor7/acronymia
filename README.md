# Acronymia

Welcome to Acronymia, a party game based aroud, you guessed it, acronyms.

In each round, players will be given a prompt and corresponding acronym. Players will then come up with and submit a breakdown for the acronym that corresponds with the prompt. Per round, one player will be designated as the judge rather than submit an answer and will choose their favorite answer. The player with their answer chosen will be awarded points based on the length of the acronym needed.

The game will either run until one player reaches a score threshold or until the designated number of rounds passes. In the latter case, the highest score wins.

In order to play, a host will need to install and run the game. Once the game is up and running, players can join through their web browser (whether or not room codes or passwords will be used is to be determined.)

Potential features include choosing themes for the game, which could change prompts given and icons players can choose to represent themselves while playing, as well as acronym length and round limit modifiers.

## Features

### Bag
There's a scrabble (ish) bag of letters.

1 of each letter for now.

Future:
ratio matches the ratio of first letters in the english language

### Acronym selection
The judge pulls out letters at random from the bag.

Always 3 for now.

Future Ideas:
- [ ] Roll a 1 d6. Add 2. So a random range from 3 to 8.

- [ ] Make acronyms pronounceable. (For now the game could be called initialismia).


## Implementation Notes

Web sockets or Polling?
> whichever the framework makes easier

No database.

Central server thread with in memory state.

Pages:
- Lobby: "/"
- Active Game (SPA): `/game/<id>/`
- Game configaration `/game/<id>/config`


### TODO 

- [ ] Lobby with no passcode
- [ ] passcode

# Development

This project was bootstrapped with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Running your project

`cargo leptos watch`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future)
