# 📎 ln
**ln** is a very simple url shortener. It's powered by Cloudflare workers/KV. It's meant to be speedy and *edgy*. 

![demo](./demo.gif)

## features
**ln** is simple, set-and-forget, and idempotent.
* **ln** will recognize a link, and return the same shortlink created previously.
* **ln** is simple to use. No user data is collected through the system.
* **ln** plays nice with [**hop**](https://github.com/jchen/hop).

## prereqs
You'll need `rustup` and `cargo`: 
```sh
curl https://sh.rustup.rs -sSf | sh
```
```sh
npm install -g wrangler
```

## build/deploy/test
```sh
cargo build && wrangler publish
```
You can also `cargo test` and `wrangler dev`, etc...

GitHub actions isn't powerful enough to build Rust/wasm so it must be manually published. 
