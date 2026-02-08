## Kardlang

Kardlang is the tiny cards-as-code language inside **Kardinality**.

### Philosophy

* Card power is budgeted by script length.
* Every character matters.
* Digits are intentionally expensive to discourage brute-force constants.
* Programs stay deterministic and integer-only.

### Effective Length Cost

Budgets are checked against **effective length**:

* Most characters cost `1`.
* Digits cost their numeric value.
* `0` still costs `1`.

Examples:

* `s(11)` costs `5` (`s`,`(`,`1`,`1`,`)`)
* `s(9)` costs `12` because `9` costs `9`

### Core Syntax

Programs are call sequences:

```text
s(11); b(11); d(1)
```

* Calls are separated by whitespace and/or `;`.
* Arguments are integer expressions using:
* numbers (`111` or `4`)
* identifiers (`len_deck` or short aliases like `D`)
* `+`, `*`, and parentheses

### Grammar

The canonical grammar string is in `src/kardlang/grammar.rs`.

### Registers

Long and short forms are both supported:

* `len_deck` / `len_pool` / `len_collection` / `D`
* `len_hand` / `H`
* `len_source` / `len_draw` / `S`
* `len_pile` / `len_discard` / `P`
* `level` / `lvl` / `L`
* `target` / `T`
* `score` / `Q`
* `bankroll` / `money` / `B`
* writable accumulator: `acc` / `A`
* safety registers: `max_steps`, `max_step`, `max_loop_iters`, `max_loop`

### Functions

All core ops have long + short forms:

* `score(n)` / `s(n)`: add score
* `bank(n)` / `b(n)`: add bankroll
* `dbl()` / `x()`: multiply bankroll by 2
* `draw(n)` / `d(n)`: draw cards into Deck
* `tri(n)` / `t(n)`: set `A = n*(n+1)/2`
* `fibo(n)` / `f(n)`: set `A = F(n)`
* `clone(n)` / `c(n)`: queue copies of last played card
* `again(n)` / `a(n)`: replay last played card
* `mutate()` / `m()`: mutate last played card

Combo ops:

* `jam(n)` / `j(n)`: `score += n` and draw `1`
* `mint(n)` / `i(n)`: `bankroll += n` and draw `1`
* `cash(n)` / `v(n)`: `score += n`, `bankroll -= n`
* `hedge(n)` / `h(n)`: add score if below target, else add bankroll
* `wild(n)` / `w(n)`: mutate then replay `n`

### Safety Limits

Execution is always bounded:

* `max_steps`: cap on evaluated calls per execution
* `max_loop_iters`: reserved for future control-flow extensions

On limit hit, execution aborts cleanly and emits a trace error.

### Tutorial Puzzles

Use **Controls → Puzzles / Tutorials** in the UI to launch curated hand/deck scenarios.

Each puzzle includes:

* A fixed opening setup
* A play limit
* A hint
* A clear goal (score target, sometimes bankroll target)

This doubles as both tutorial flow and regression coverage for core combo patterns.
