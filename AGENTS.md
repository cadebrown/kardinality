## Kardinality — AGENTS.md (AI / automation guide)

This file exists so future AI agents (or automation) can make **coherent changes quickly** without re‑deriving project intent, constraints, and workflow.

### Mission

Ship a fun, jam‑scoped, browser game where **cards are code** and card execution is unified through a tiny language (“Kardlang”), with the UI rendered using **DOM + CSS** via Dioxus.

### Hard constraints (do not violate)

* **Rust owns the logic**: game rules, interpreter, progression, persistence.
* **DOM‑first rendering**: no traditional rendering engine; lean on Dioxus + CSS.
* **Minimal/no hand‑written JS**: use browser APIs through Rust (`web_sys` / `wasm_bindgen`) when needed.
* **Safety limits always on**: execution must have max steps / max loop iterations and abort cleanly.

### MVP cutline (Wednesday)

* The game is playable end‑to‑end: draw → play hand → score/bankroll changes → level targets → advance.
* A **trace panel** exists (it’s both debugging and core UX).
* Kardlang v0 works (unary numbers + cost model) and cards execute through one evaluator.

If the build isn’t solid by Thu, cut shop/wildcards before touching fundamentals.

### Commands (intended)

* Dev server: `dx serve`
* Release build: `dx build --release`
* Format: `cargo fmt`
* Tests: `cargo test`

### Conventions

* Prefer small, composable modules: `game/`, `kardlang/`, `vm/`, `ui/`.
* Keep runs deterministic: seedable RNG and reproducible outcomes for balancing/debugging.
* “Trace‑first” development: if you add new effects/control flow, add trace output too.
* Markdown style in this repo:
  * Blank line after headings
  * Use `*` bullets (not `-`)


