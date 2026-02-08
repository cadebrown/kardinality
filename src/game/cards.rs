use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::kardlang::parse_program;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KindVisual {
    pub icon: &'static str,
    pub label: &'static str,
    pub accent: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionVisual {
    pub canonical: &'static str,
    pub icon: &'static str,
    pub short: &'static str,
    pub label: &'static str,
    pub accent: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GenerationRule {
    id: &'static str,
    min_level: u32,
    max_level: u32,
    weight: u16,
}

const ANY_LEVEL: u32 = u32::MAX;

pub fn catalog() -> &'static [CardDef] {
    &CATALOG
}

pub fn get(id: &str) -> Option<&'static CardDef> {
    CATALOG.iter().find(|c| c.id == id)
}

pub fn starter_deck_ids() -> &'static [&'static str] {
    &["spark_draw", "tap_score", "tap_bank"]
}

pub fn kind_visual(kind: CardKind) -> KindVisual {
    match kind {
        CardKind::Economy => KindVisual {
            icon: "◎",
            label: "Economy",
            accent: "economy",
        },
        CardKind::Score => KindVisual {
            icon: "✹",
            label: "Score",
            accent: "score",
        },
        CardKind::Control => KindVisual {
            icon: "⇄",
            label: "Control",
            accent: "control",
        },
        CardKind::Meta => KindVisual {
            icon: "◇",
            label: "Meta",
            accent: "meta",
        },
    }
}

pub fn function_visual(name: &str) -> Option<FunctionVisual> {
    let canonical = match name {
        "score" | "s" => "score",
        "bank" | "b" => "bank",
        "dbl" | "x" => "dbl",
        "draw" | "d" => "draw",
        "tri" | "t" => "tri",
        "fibo" | "f" => "fibo",
        "clone" | "c" => "clone",
        "again" | "a" => "again",
        "mutate" | "m" => "mutate",
        "jam" | "j" => "jam",
        "mint" | "i" => "mint",
        "cash" | "v" => "cash",
        "hedge" | "h" => "hedge",
        "wild" | "w" => "wild",
        _ => return None,
    };

    Some(match canonical {
        "score" => FunctionVisual {
            canonical,
            icon: "🎯",
            short: "S",
            label: "Score",
            accent: "score",
        },
        "bank" => FunctionVisual {
            canonical,
            icon: "💰",
            short: "B",
            label: "Bank",
            accent: "economy",
        },
        "dbl" => FunctionVisual {
            canonical,
            icon: "✖",
            short: "×2",
            label: "Double",
            accent: "economy",
        },
        "draw" => FunctionVisual {
            canonical,
            icon: "🃏",
            short: "D",
            label: "Draw",
            accent: "control",
        },
        "tri" => FunctionVisual {
            canonical,
            icon: "△",
            short: "Tri",
            label: "Tri",
            accent: "score",
        },
        "fibo" => FunctionVisual {
            canonical,
            icon: "Φ",
            short: "Fib",
            label: "Fibo",
            accent: "score",
        },
        "clone" => FunctionVisual {
            canonical,
            icon: "🪞",
            short: "Cln",
            label: "Clone",
            accent: "meta",
        },
        "again" => FunctionVisual {
            canonical,
            icon: "↻",
            short: "Agn",
            label: "Again",
            accent: "meta",
        },
        "mutate" => FunctionVisual {
            canonical,
            icon: "🧬",
            short: "Mut",
            label: "Mutate",
            accent: "meta",
        },
        "jam" => FunctionVisual {
            canonical,
            icon: "⚡",
            short: "Jam",
            label: "Jam",
            accent: "control",
        },
        "mint" => FunctionVisual {
            canonical,
            icon: "🫧",
            short: "Mint",
            label: "Mint",
            accent: "economy",
        },
        "cash" => FunctionVisual {
            canonical,
            icon: "💸",
            short: "Cash",
            label: "Cash",
            accent: "economy",
        },
        "hedge" => FunctionVisual {
            canonical,
            icon: "🛡",
            short: "Hdg",
            label: "Hedge",
            accent: "meta",
        },
        "wild" => FunctionVisual {
            canonical,
            icon: "🃟",
            short: "Wild",
            label: "Wild",
            accent: "meta",
        },
        _ => unreachable!("canonical function mapping must stay exhaustive"),
    })
}

pub fn script_function_visuals(script: &str) -> Vec<FunctionVisual> {
    let Ok(program) = parse_program(script) else {
        return Vec::new();
    };

    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for call in program.calls {
        if let Some(fx) = function_visual(&call.name) {
            if seen.insert(fx.canonical) {
                out.push(fx);
            }
        }
    }
    out
}

pub fn source_count_for_level(level: u32) -> usize {
    let level = level.max(1) as usize;
    (56 + level.saturating_sub(1) * 8).min(128)
}

pub fn generate_source_ids(seed: u64, level: u32) -> Vec<&'static str> {
    generate_source_ids_with_count(seed, level, source_count_for_level(level))
}

pub fn generate_source_ids_with_count(seed: u64, level: u32, count: usize) -> Vec<&'static str> {
    let level = level.max(1);
    let mut rng =
        ChaCha8Rng::seed_from_u64(seed ^ (level as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let rules = active_rules_for_level(level);

    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        if let Some(id) = pick_weighted_rule_id(&mut rng, &rules) {
            out.push(id);
        }
    }
    out
}

pub fn roll_card_id_for_level<R: Rng + ?Sized>(rng: &mut R, level: u32) -> Option<&'static str> {
    let rules = active_rules_for_level(level.max(1));
    pick_weighted_rule_id(rng, &rules)
}

fn active_rules_for_level(level: u32) -> Vec<&'static GenerationRule> {
    let mut rules: Vec<&GenerationRule> = GENERATION_RULES
        .iter()
        .filter(|r| level >= r.min_level && level <= r.max_level)
        .collect();

    if rules.is_empty() {
        rules = GENERATION_RULES.iter().collect();
    }
    rules
}

fn pick_weighted_rule_id<R: Rng + ?Sized>(
    rng: &mut R,
    rules: &[&'static GenerationRule],
) -> Option<&'static str> {
    let total: u32 = rules.iter().map(|r| r.weight as u32).sum();
    if total == 0 {
        return None;
    }

    let mut roll = rng.random_range(0..total);
    for rule in rules {
        let weight = rule.weight as u32;
        if roll < weight {
            return Some(rule.id);
        }
        roll -= weight;
    }

    rules.last().map(|r| r.id)
}

static CATALOG: [CardDef; 33] = [
    CardDef {
        id: "tap_score",
        name: "Tap Score",
        kind: CardKind::Score,
        budget: 8,
        script: "s(11)",
        icon: "▲",
        doc: r#"
Starter score ping.

* Script: `s(11)`
* Effect: +2 score.
"#,
    },
    CardDef {
        id: "tap_bank",
        name: "Tap Bank",
        kind: CardKind::Economy,
        budget: 8,
        script: "b(11)",
        icon: "$",
        doc: r#"
Starter economy ping.

* Script: `b(11)`
* Effect: +2 bankroll.
"#,
    },
    CardDef {
        id: "spark_draw",
        name: "Spark Draw",
        kind: CardKind::Control,
        budget: 8,
        script: "d(11)",
        icon: "⇣",
        doc: r#"
Starter card generation.

* Script: `d(11)`
* Effect: draw 2 into your Deck.
"#,
    },
    CardDef {
        id: "pulse_score",
        name: "Pulse Score",
        kind: CardKind::Score,
        budget: 9,
        script: "s(D)",
        icon: "Σ",
        doc: r#"
Scales with deck size.

* Script: `s(D)`
* Register: `D` = `len_deck`.
"#,
    },
    CardDef {
        id: "hand_score",
        name: "Hand Score",
        kind: CardKind::Score,
        budget: 9,
        script: "s(H)",
        icon: "✦",
        doc: r#"
Scores from current hand size.

* Script: `s(H)`
* Register: `H` = `len_hand`.
"#,
    },
    CardDef {
        id: "level_ping",
        name: "Level Ping",
        kind: CardKind::Score,
        budget: 10,
        script: "s(L+1)",
        icon: "▴",
        doc: r#"
Steady level scaling.

* Script: `s(L+1)`
* Register: `L` = `level`.
"#,
    },
    CardDef {
        id: "deck_bank",
        name: "Deck Dividend",
        kind: CardKind::Economy,
        budget: 9,
        script: "b(D)",
        icon: "₿",
        doc: r#"
Turns deck size into bankroll.

* Script: `b(D)`
* Register: `D` = `len_deck`.
"#,
    },
    CardDef {
        id: "source_bank",
        name: "Source Dividend",
        kind: CardKind::Economy,
        budget: 9,
        script: "b(S)",
        icon: "₵",
        doc: r#"
Banks from source pile size.

* Script: `b(S)`
* Register: `S` = `len_source`.
"#,
    },
    CardDef {
        id: "level_bank",
        name: "Level Capital",
        kind: CardKind::Economy,
        budget: 12,
        script: "b(L+11)",
        icon: "◫",
        doc: r#"
Economy scaling by level.

* Script: `b(L+11)`
* Register: `L` = `level`.
"#,
    },
    CardDef {
        id: "double_bank",
        name: "Double Bank",
        kind: CardKind::Economy,
        budget: 8,
        script: "x()",
        icon: "×2",
        doc: r#"
Classic multiplier.

* Script: `x()`
* Effect: bankroll *= 2.
"#,
    },
    CardDef {
        id: "jam_two",
        name: "Jam +2",
        kind: CardKind::Control,
        budget: 9,
        script: "j(11)",
        icon: "⚡",
        doc: r#"
Tempo card: score plus refill.

* Script: `j(11)`
* Effect: +2 score and draw 1.
"#,
    },
    CardDef {
        id: "mint_two",
        name: "Mint +2",
        kind: CardKind::Economy,
        budget: 9,
        script: "i(11)",
        icon: "◎",
        doc: r#"
Tempo economy.

* Script: `i(11)`
* Effect: +2 bankroll and draw 1.
"#,
    },
    CardDef {
        id: "hedge_two",
        name: "Hedge +2",
        kind: CardKind::Meta,
        budget: 10,
        script: "h(11)",
        icon: "⤳",
        doc: r#"
Adaptive payoff.

* Script: `h(11)`
* Effect: if `score < target`, gain score; else gain bankroll.
"#,
    },
    CardDef {
        id: "cash_two",
        name: "Cashout 2",
        kind: CardKind::Meta,
        budget: 10,
        script: "v(11)",
        icon: "↔",
        doc: r#"
Convert bankroll into score.

* Script: `v(11)`
* Effect: +2 score, -2 bankroll.
"#,
    },
    CardDef {
        id: "clone_one",
        name: "Clone 1",
        kind: CardKind::Meta,
        budget: 8,
        script: "c(1)",
        icon: "⧉",
        doc: r#"
Queue one copy of the last played card.

* Script: `c(1)`
* Works best after a strong card.
"#,
    },
    CardDef {
        id: "again_one",
        name: "Again 1",
        kind: CardKind::Meta,
        budget: 8,
        script: "a(1)",
        icon: "↺",
        doc: r#"
Replay last card one extra time.

* Script: `a(1)`
* Pairs with scaling cards.
"#,
    },
    CardDef {
        id: "mutator",
        name: "Mutator",
        kind: CardKind::Meta,
        budget: 8,
        script: "m()",
        icon: "✸",
        doc: r#"
Randomize the last played card.

* Script: `m()`
* Chaos tool for high-roll lines.
"#,
    },
    CardDef {
        id: "wild_one",
        name: "Wild 1",
        kind: CardKind::Meta,
        budget: 9,
        script: "w(1)",
        icon: "🜂",
        doc: r#"
Mutate then replay once.

* Script: `w(1)`
* Effect: `m(); a(1)`.
"#,
    },
    CardDef {
        id: "tri_deck",
        name: "Tri Deck",
        kind: CardKind::Score,
        budget: 14,
        script: "t(D);s(A)",
        icon: "△",
        doc: r#"
Triangular deck scaling.

* Script: `t(D);s(A)`
* Effect: `A = tri(D)`, then score `A`.
"#,
    },
    CardDef {
        id: "fibo_level",
        name: "Fibo Level",
        kind: CardKind::Score,
        budget: 16,
        script: "f(L+11);s(A)",
        icon: "Φ",
        doc: r#"
Level-based Fibonacci scorer.

* Script: `f(L+11);s(A)`
* Effect: score Fibonacci(level+2).
"#,
    },
    CardDef {
        id: "tri_hand_bank",
        name: "Tri Hand Bank",
        kind: CardKind::Economy,
        budget: 14,
        script: "t(H);b(A)",
        icon: "◇",
        doc: r#"
Triangular hand-to-cash line.

* Script: `t(H);b(A)`
* Register: `H` = `len_hand`.
"#,
    },
    CardDef {
        id: "fibo_hand_bank",
        name: "Fibo Hand Bank",
        kind: CardKind::Economy,
        budget: 15,
        script: "f(H+1);b(A)",
        icon: "◈",
        doc: r#"
Fibonacci hand banking.

* Script: `f(H+1);b(A)`
* Strong with wide hands.
"#,
    },
    CardDef {
        id: "clone_pair",
        name: "Clone 2",
        kind: CardKind::Meta,
        budget: 10,
        script: "c(11)",
        icon: "⧉",
        doc: r#"
Queue two copies of the last card.

* Script: `c(11)`
* Burst enabler.
"#,
    },
    CardDef {
        id: "again_pair",
        name: "Again 2",
        kind: CardKind::Meta,
        budget: 10,
        script: "a(11)",
        icon: "↻",
        doc: r#"
Replay the last card twice.

* Script: `a(11)`
* Combo extender.
"#,
    },
    CardDef {
        id: "wild_pair",
        name: "Wild 2",
        kind: CardKind::Meta,
        budget: 11,
        script: "w(11)",
        icon: "🜃",
        doc: r#"
Mutate then replay twice.

* Script: `w(11)`
* High-variance finisher.
"#,
    },
    CardDef {
        id: "meta_weave",
        name: "Meta Weave",
        kind: CardKind::Meta,
        budget: 12,
        script: "m();c(1)",
        icon: "∞",
        doc: r#"
Mutate then clone.

* Script: `m();c(1)`
* Creates fresh follow-up lines.
"#,
    },
    CardDef {
        id: "meta_recursion",
        name: "Meta Recursion",
        kind: CardKind::Meta,
        budget: 13,
        script: "m();a(11)",
        icon: "∴",
        doc: r#"
Mutate then replay twice.

* Script: `m();a(11)`
* Expensive but explosive.
"#,
    },
    CardDef {
        id: "deck_loop",
        name: "Deck Loop",
        kind: CardKind::Control,
        budget: 11,
        script: "d(H+1)",
        icon: "⇆",
        doc: r#"
Draw scales with hand width.

* Script: `d(H+1)`
* Good in hand-stacking builds.
"#,
    },
    CardDef {
        id: "source_skim",
        name: "Source Skim",
        kind: CardKind::Control,
        budget: 12,
        script: "d(S+11)",
        icon: "⇊",
        doc: r#"
Source-aware draw burst.

* Script: `d(S+11)`
* Register: `S` = `len_source`.
"#,
    },
    CardDef {
        id: "jackpot",
        name: "Jackpot",
        kind: CardKind::Economy,
        budget: 12,
        script: "b(B+11)",
        icon: "⬢",
        doc: r#"
Bankroll snowball.

* Script: `b(B+11)`
* Register: `B` = `bankroll`.
"#,
    },
    CardDef {
        id: "surge_score",
        name: "Surge Score",
        kind: CardKind::Score,
        budget: 12,
        script: "s(Q+11)",
        icon: "⬡",
        doc: r#"
Score snowball.

* Script: `s(Q+11)`
* Register: `Q` = `score`.
"#,
    },
    CardDef {
        id: "liquidity",
        name: "Liquidity",
        kind: CardKind::Economy,
        budget: 10,
        script: "b(Q)",
        icon: "≋",
        doc: r#"
Turn score momentum into cash.

* Script: `b(Q)`
* Register: `Q` = `score`.
"#,
    },
    CardDef {
        id: "all_in_score",
        name: "All-in Score",
        kind: CardKind::Score,
        budget: 10,
        script: "v(B)",
        icon: "!",
        doc: r#"
Convert all bankroll into score.

* Script: `v(B)`
* Effect: `score += bankroll`, then `bankroll = 0`.
"#,
    },
];

static GENERATION_RULES: [GenerationRule; 33] = [
    GenerationRule {
        id: "tap_score",
        min_level: 1,
        max_level: 4,
        weight: 16,
    },
    GenerationRule {
        id: "tap_bank",
        min_level: 1,
        max_level: 4,
        weight: 16,
    },
    GenerationRule {
        id: "spark_draw",
        min_level: 1,
        max_level: 4,
        weight: 12,
    },
    GenerationRule {
        id: "pulse_score",
        min_level: 1,
        max_level: ANY_LEVEL,
        weight: 10,
    },
    GenerationRule {
        id: "hand_score",
        min_level: 2,
        max_level: ANY_LEVEL,
        weight: 10,
    },
    GenerationRule {
        id: "level_ping",
        min_level: 2,
        max_level: ANY_LEVEL,
        weight: 8,
    },
    GenerationRule {
        id: "deck_bank",
        min_level: 1,
        max_level: ANY_LEVEL,
        weight: 10,
    },
    GenerationRule {
        id: "source_bank",
        min_level: 2,
        max_level: ANY_LEVEL,
        weight: 8,
    },
    GenerationRule {
        id: "level_bank",
        min_level: 2,
        max_level: ANY_LEVEL,
        weight: 6,
    },
    GenerationRule {
        id: "double_bank",
        min_level: 1,
        max_level: ANY_LEVEL,
        weight: 6,
    },
    GenerationRule {
        id: "jam_two",
        min_level: 1,
        max_level: ANY_LEVEL,
        weight: 8,
    },
    GenerationRule {
        id: "mint_two",
        min_level: 1,
        max_level: ANY_LEVEL,
        weight: 8,
    },
    GenerationRule {
        id: "hedge_two",
        min_level: 2,
        max_level: ANY_LEVEL,
        weight: 7,
    },
    GenerationRule {
        id: "cash_two",
        min_level: 2,
        max_level: ANY_LEVEL,
        weight: 7,
    },
    GenerationRule {
        id: "clone_one",
        min_level: 1,
        max_level: ANY_LEVEL,
        weight: 7,
    },
    GenerationRule {
        id: "again_one",
        min_level: 1,
        max_level: ANY_LEVEL,
        weight: 7,
    },
    GenerationRule {
        id: "mutator",
        min_level: 2,
        max_level: ANY_LEVEL,
        weight: 6,
    },
    GenerationRule {
        id: "wild_one",
        min_level: 2,
        max_level: ANY_LEVEL,
        weight: 5,
    },
    GenerationRule {
        id: "tri_deck",
        min_level: 3,
        max_level: ANY_LEVEL,
        weight: 6,
    },
    GenerationRule {
        id: "fibo_level",
        min_level: 3,
        max_level: ANY_LEVEL,
        weight: 5,
    },
    GenerationRule {
        id: "tri_hand_bank",
        min_level: 3,
        max_level: ANY_LEVEL,
        weight: 6,
    },
    GenerationRule {
        id: "fibo_hand_bank",
        min_level: 3,
        max_level: ANY_LEVEL,
        weight: 5,
    },
    GenerationRule {
        id: "clone_pair",
        min_level: 3,
        max_level: ANY_LEVEL,
        weight: 5,
    },
    GenerationRule {
        id: "again_pair",
        min_level: 3,
        max_level: ANY_LEVEL,
        weight: 5,
    },
    GenerationRule {
        id: "wild_pair",
        min_level: 4,
        max_level: ANY_LEVEL,
        weight: 3,
    },
    GenerationRule {
        id: "meta_weave",
        min_level: 3,
        max_level: ANY_LEVEL,
        weight: 4,
    },
    GenerationRule {
        id: "meta_recursion",
        min_level: 4,
        max_level: ANY_LEVEL,
        weight: 3,
    },
    GenerationRule {
        id: "deck_loop",
        min_level: 2,
        max_level: ANY_LEVEL,
        weight: 6,
    },
    GenerationRule {
        id: "source_skim",
        min_level: 2,
        max_level: ANY_LEVEL,
        weight: 6,
    },
    GenerationRule {
        id: "jackpot",
        min_level: 3,
        max_level: ANY_LEVEL,
        weight: 4,
    },
    GenerationRule {
        id: "surge_score",
        min_level: 3,
        max_level: ANY_LEVEL,
        weight: 4,
    },
    GenerationRule {
        id: "liquidity",
        min_level: 3,
        max_level: ANY_LEVEL,
        weight: 4,
    },
    GenerationRule {
        id: "all_in_score",
        min_level: 4,
        max_level: ANY_LEVEL,
        weight: 3,
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kardlang::{effective_len, parse_program};

    #[test]
    fn starter_cards_exist_in_catalog() {
        for id in starter_deck_ids() {
            assert!(get(id).is_some(), "starter card missing: {id}");
        }
    }

    #[test]
    fn every_card_script_parses_and_fits_budget() {
        for card in catalog() {
            parse_program(card.script).unwrap_or_else(|e| {
                panic!("parse failed for {} ({}): {e}", card.id, card.script);
            });
            let cost = effective_len(card.script);
            assert!(
                cost <= card.budget,
                "card {} over budget: cost={cost}, budget={}",
                card.id,
                card.budget
            );
        }
    }

    #[test]
    fn source_generation_is_deterministic() {
        let a = generate_source_ids_with_count(12345, 3, 40);
        let b = generate_source_ids_with_count(12345, 3, 40);
        assert_eq!(a, b);
    }

    #[test]
    fn level_unlocks_expand_the_generation_pool() {
        let l1 = active_rules_for_level(1).len();
        let l4 = active_rules_for_level(4).len();
        assert!(l4 > l1, "expected more cards at level 4 than level 1");
    }

    #[test]
    fn generation_never_emits_locked_cards() {
        let level = 2;
        let ids = generate_source_ids_with_count(7, level, 200);
        for id in ids {
            let rule = GENERATION_RULES
                .iter()
                .find(|r| r.id == id)
                .expect("generated id must exist in rules");
            assert!(
                level >= rule.min_level && level <= rule.max_level,
                "card {id} should not be available at level {level}"
            );
        }
    }

    #[test]
    fn function_aliases_resolve_to_shared_visuals() {
        let full = function_visual("score").expect("score visual");
        let short = function_visual("s").expect("s visual");
        assert_eq!(full.canonical, short.canonical);
        assert_eq!(full.icon, short.icon);
        assert_eq!(full.label, short.label);
    }

    #[test]
    fn script_visuals_dedupe_and_keep_call_order() {
        let visuals = script_function_visuals("s(11);s(D);b(11);v(11)");
        let names = visuals.iter().map(|v| v.canonical).collect::<Vec<_>>();
        assert_eq!(names, vec!["score", "bank", "cash"]);
    }

    #[test]
    fn every_catalog_card_exposes_function_visuals() {
        for card in catalog() {
            let visuals = script_function_visuals(card.script);
            assert!(
                !visuals.is_empty(),
                "card {} did not expose function visuals",
                card.id
            );
        }
    }
}
