use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardKind {
    Economy,
    Score,
    Control,
    Meta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardDef {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: CardKind,
    pub budget: usize,
    pub script: &'static str,
    pub icon: &'static str,
    pub doc: &'static str,
}

pub fn catalog() -> &'static [CardDef] {
    &CATALOG
}

pub fn get(id: &str) -> Option<&'static CardDef> {
    CATALOG.iter().find(|c| c.id == id)
}

pub fn starter_deck_ids() -> &'static [&'static str] {
    &["draw_5", "score_4", "bank_6"]
}

static CATALOG: [CardDef; 8] = [
    CardDef {
        id: "draw_5",
        name: "Draw 5",
        kind: CardKind::Control,
        budget: 12,
        script: "draw(5)",
        icon: "⇣",
        doc: r#"
Generates new cards.

* **Effect**: `draw(n)` adds `n` new cards into your Deck (up to availability).
* **Default script**: `draw(5)`
* **Notes**: This is your primary way to expand options early.
"#,
    },
    CardDef {
        id: "score_4",
        name: "Score +4",
        kind: CardKind::Score,
        budget: 12,
        script: "score(4)",
        icon: "▲",
        doc: r#"
Adds to your score.

* **Effect**: `score(n)` increases `score` by `n`.
* **Default script**: `score(4)`
"#,
    },
    CardDef {
        id: "bank_6",
        name: "Bank +6",
        kind: CardKind::Economy,
        budget: 13,
        script: "bank(6)",
        icon: "$",
        doc: r#"
Adds to your bankroll.

* **Effect**: `bank(n)` increases `bankroll` by `n`.
* **Default script**: `bank(6)`
"#,
    },
    CardDef {
        id: "score_2",
        name: "Score +2",
        kind: CardKind::Score,
        budget: 12,
        script: "score(11)",
        icon: "▲",
        doc: r#"
Adds to your score.

* **Effect**: `score(n)` increases `score` by `n`.
* **Default script**: `score(11)` (unary 2)
* **Notes**: Score is compared against `target` to clear the level.
"#,
    },
    CardDef {
        id: "score_len_deck",
        name: "Score +len_source",
        kind: CardKind::Score,
        budget: 24,
        script: "score(len_source)",
        icon: "Σ",
        doc: r#"
Adds the current source size to your score.

* **Effect**: `score(len_source)` increases `score` by `len_source`.
* **Register**: `len_source` is the number of cards remaining in the source pile.
"#,
    },
    CardDef {
        id: "bank_3",
        name: "Bank +3",
        kind: CardKind::Economy,
        budget: 12,
        script: "bank(111)",
        icon: "$",
        doc: r#"
Adds to your bankroll.

* **Effect**: `bank(n)` increases `bankroll` by `n`.
* **Default script**: `bank(111)` (unary 3)
"#,
    },
    CardDef {
        id: "bank_len_pool",
        name: "Bank +len_deck",
        kind: CardKind::Economy,
        budget: 24,
        script: "bank(len_deck)",
        icon: "$",
        doc: r#"
Adds the current deck size to your bankroll.

* **Effect**: `bank(len_deck)` increases `bankroll` by `len_deck`.
* **Register**: `len_deck` is the number of cards currently in your deck.
"#,
    },
    CardDef {
        id: "dbl_bank",
        name: "Double Bank",
        kind: CardKind::Economy,
        budget: 12,
        script: "dbl()",
        icon: "×2",
        doc: r#"
Doubles your bankroll.

* **Effect**: `dbl()` multiplies `bankroll` by 2.
* **Default script**: `dbl()`
"#,
    },
];
