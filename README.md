# Let's count the workouts

This is just a playground project, so that I have some fun with Rust and WASM

Go here to see it working: https://marinho.github.io/lets-count-the-workouts/

## Required toolings

If you don't have Rust nightly, you can install it with

```sh
rustup toolchain install nightly --allow-downgrade
```

You can add the `wasm` compilation target to rust using

```sh
rustup target add wasm32-unknown-unknown
```

## Running

To execute project, run:

```sh
trunk serve --port 3000
```

Then load your browser at `http://localhost:3000`.

## Building

To build an app for release, use the command

```sh
trunk build --release
```

This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.

For further information about hosting Leptos CSR apps, please refer to [the Leptos Book chapter on deployment available here][deploy-csr].

[Leptos]: https://github.com/leptos-rs/leptos
[Trunk]: https://github.com/trunk-rs/trunk
[Trunk-instructions]: https://trunkrs.dev/assets/
[deploy-csr]: https://book.leptos.dev/deployment/csr.html
