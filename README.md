<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos`

Then run

`cargo leptos new --git leptos-rs/start`

to generate a new project template.

`cd {projectname}`

to go to your newly created project.

Of course you should explore around the project structure, but the best place to start with your application code is in `src/app.rs`.

## Running your project

`cargo leptos watch`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future)


# Acronymia

## What is?

Acroynymia is a game you play with your friends.
It's like party games such as Apples to apples or quiplash.


## Scrabble bag
There's a scrabble (ish) bag of letters.

1 of each letter for now.

Future:
ratio matches the ratio of first letters in the english language

## Pick a judge ( rotates)

The judge pulls out letters at random from the bag.

Always 3 for now.

Future:
Roll a 1 d6. Add 2. So a random range from 3 to 8.


## People submit 

Private submission of choices. 
They get an input box per letter and type words.


## Judge selects

Points assigned, next round. 
each person is judge twice. 

### Architecture

Web sockets or Polling?
> whichever the framework makes easier

No database.

Central server thread with in memory state.


Pages:
(1) Lobby (/) (root)
(2) Active Game (SPA) (/game/<code>/)
(3) Game configaration (Future) (/game/<code>/config)


### TODO 

- [ ] Lobby with no passcode
- [ ] passcode
