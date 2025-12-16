# Kardinality

Kardinality is a **single‑player, browser‑deployed** card game where your “hand” is an ordered stack of executable code‑cards. The vibe is roguelike deck‑building (Balatro‑adjacent), but the mechanics are intentionally **programming‑shaped**: cards compose, control flow exists, and you win by hitting ever‑higher score targets.

## Tech stack

* **Rust**: all game logic (rules, state, interpreter)
* **WASM (web)**: cross‑platform browser deploy
* **Dioxus**: UI rendered via **HTML DOM + CSS**, no traditional rendering engine
* **Browser APIs**: accessed from Rust (`web_sys` / `wasm_bindgen`) with minimal/no hand‑written JS

## Quick start (dev)

Recommended workflow:

* Install Rust (stable) and add the WASM target:
  * `rustup target add wasm32-unknown-unknown`
* Install Dioxus CLI:
  * `cargo install dioxus-cli`
  * (optional, faster installs) `cargo install cargo-binstall && cargo binstall dioxus-cli`
* Run the dev server:
  * `dx serve`
* Build for release:
  * `dx build --release`
* Run core logic tests:
  * `cargo test`

## Automated GUI smoke tests (E2E)

This repo includes a headless browser harness (Playwright) that can boot the WASM app and drive the UI.
It’s intended for regression checks and future automated deployment confidence.

Prereqs:

* Node.js 18+
* `dioxus-cli` installed (`dx serve` must work)

Install deps:

* `npm install`

Run E2E tests (headless):

* `npm run e2e`

Run E2E tests with the Playwright UI:

* `npm run e2e:ui`

## Docs

* `AGENTS.md`: guidance for future AI agents (commands, constraints, conventions)
* `KARDLANG.md`: language philosophy + cost model + registers + function reference
