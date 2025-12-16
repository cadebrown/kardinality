## Kardlang

Kardlang is the tiny “cards-as-code” language inside **Kardinality**. It’s designed for a gamejam reality: **simple to read**, **cheap to parse**, and **easy to balance**.

### Philosophy

* **Character budgets are the balance lever**: every card has a strict budget. If you want more power, you spend more characters.
* **Big constants are intentionally expensive**: digits cost their numeric value, so typing `9` is “9 chars worth of power”. Unary numbers (`111`) are always available but still cost length.
* **Arithmetic is minimal**: only `+` and `*` exist inside expressions. No subtraction, division, exponentiation, or function composition.
* **No nested calls (for now)**: you can’t write `score(sqr(len_hand))`. A call’s arguments are **expressions**, not other calls.
* **Everything is an integer**: makes the VM deterministic, fast, and easier to reason about.

### The “effective length” cost model

Kardlang budgets are enforced using an **effective length**, not raw character count:

* Normal characters cost **1**
* Digit characters cost **their value**
  * `0` costs **1**
  * `4` costs **4**
  * `9` costs **9**

This prevents “type 999999” strategies and pushes players toward **state-derived values** and **sequencing** instead of raw constants.

### Core syntax (v0)

* Programs are a sequence of calls:

```text
score(4)
bank(6)
dbl()
```

* Calls can be separated by whitespace and/or `;`.
* Arguments are integer expressions using:
  * **numbers** (unary `111` or digit shorthand `4`)
  * **register identifiers** (e.g. `len_deck`)
  * **operators** `+` and `*`
  * **parentheses** for grouping

### Grammar

The codebase ships aimage.png canonical grammar string in `src/kardlang/grammar.rs`. (The Kardinomicon shows it in-game.)

### Registers

Registers are named reads of game state. They are intentionally verbose (referencing state costs characters).

Current registers include:

* `len_deck` / `len_pool` / `len_collection`
* `len_hand`
* `len_source` / `len_draw`
* `len_discard`
* `score`
* `bankroll`
* `level`
* `target`
* `max_steps` / `max_step`
* `max_loop_iters` / `max_loop`

### Functions

These are the “opcodes” of Kardlang. A card is basically a (budgeted) program made of these calls.

* `draw(n)`: generate/draw `n` new cards into your Deck (bounded in the engine)
* `score(n)`: add `n` to score
* `bank(n)`: add `n` to bankroll
* `dbl()`: multiply bankroll by 2

### Safety limits

Execution is always bounded:

* `max_steps`: caps how many calls can be evaluated in one execution
* `max_loop_iters`: reserved for future control-flow cards

If a limit is exceeded, execution aborts cleanly and the UI shows it in the trace/debug panel.


