# Packages

In alphabetical order:

* [`./android`](./android) - The Android App
* [`./bridge-c`](./bridge-c) - The glue between the Android/iOS apps and the search engine
* [`./bridge-wasm`](./bridge-wasm) - The glue between the web app and the search engine
* [`./crawler`](./crawler) - Scripts for scanning [Common Crawl](https://commoncrawl.org/) data for Bible verses
* [`./engine`](./engine) - The search engine and index building tooling
* [`./ios`](./ios) - The iOS app
* [`./kube`](./kube) - Kubernetes deployment configuration
* [`./proto`](./proto) - [Protocol Buffers](https://developers.google.com/protocol-buffers) definitions
* [`./server`](./server) - The search server
* [`./text`](./text) - Scripts for assembling various translations of the Bible into usable data
* [`./web`](./web) - The web app

## Prerequisites

1. Install [`direnv`](https://direnv.net/) and approve this project's `.envrc`
1. Install [`rustup`](https://rustup.rs/) and install Rust

## Quick Start

```sh
# Download index
wget https://f001.backblazeb2.com/file/instant-bible/index.pb
# Run server
cargo run --release --bin server
# Run web app
cd web
npm ci
npm run dev
```

## Less Quick Start

1. Follow instructions in `./text` to create Bible data
1. Follow instructions in `./crawler` to create popularity data
1. Run `cargo run --release --bin engine` to create search index
1. Run `cargo run --release --bin server` to run the server
1. Follow instructions in `./web` to run the web app
1. Follow instructions in `./android` and `./ios` to set up those projects
