# Ohkami×Yew TODO Demo

## Prerequisites

- Latest Rust toolchain with `wasm32-unknown-unknown` target
- npm
- `trunk` CLI ( installable by `cargo install trunk` )
- `tailwindcss` CLI ( see https://tailwindcss.com/blog/standalone-cli )

In addition, `wasm-opt` is recommended to be installed.

## Setup

```sh
npm create cloudflare ./path/to/project-dir -- --template https://github.com/kana-rus/ohkami-templates/worker
```
```sh
cd ./path/to/project-dir
```
```sh
npx wrangler login
```
```sh
npx wrangler d1 create ohkami-yew-todo-db

# and edit your wrangler.toml as wrangler.toml.sample
```

If you push the project to your GitHub repo, **You should add `wrangler.toml` into .gitignore**！

## Local dev

```sh
npm run dev
```

## Publish

```sh
npm run deploy
```
If you register your workers.dev subdomain at this time, it takes some minutes for DNS records to update and it's good time to take a coffee break.
