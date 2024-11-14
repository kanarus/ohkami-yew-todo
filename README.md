# Ohkami×Yew TODO Demo

## Prerequisites

- Rust toolchain of channel `1.81` with `wasm32-unknown-unknown` target
- [`worker-build`](https://crates.io/crates/worker-build) ( run `cargo install worker-build` to install )
- `trunk` CLI ( run `cargo install trunk` to install )
- `tailwindcss` CLI ( see https://tailwindcss.com/blog/standalone-cli )

In addition, `wasm-opt` is recommended to be installed.

## Setup

```sh
git clone https://github.com/kana-rus/ohkami-yew-todo.git

cd ./ohkami-yew-todo
```
```sh
npx wrangler login
```
```sh
npx wrangler d1 create ohkami-yew-todo-db

# and edit your wrangler.toml as wrangler.toml.sample
```
```sh
npx wrangler d1 execute ohkami-yew-todo-db --file ./schema.sql

npx wrangler d1 execute ohkami-yew-todo-db --file ./schema.sql --remote
```

If you push the project to your GitHub repo, **You should add `wrangler.toml` into .gitignore**！

## Local dev

```sh
npm run dev
```
```sh
trunk serve --watch src/ui --open
```

## Publish

```sh
npm run deploy
```
If you register your workers.dev subdomain at this time, it takes some minutes for DNS records to update and it's good time to take a coffee break.
