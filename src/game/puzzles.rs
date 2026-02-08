#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PuzzleDef {
    pub id: &'static str,
    pub name: &'static str,
    pub blurb: &'static str,
    pub hint: &'static str,
    pub theme: &'static str,
    pub start_bankroll: i64,
    pub start_score: i64,
    pub start_level: u32,
    pub target_score: i64,
    pub goal_bankroll: Option<i64>,
    pub play_limit: Option<u32>,
    pub source_ids: &'static [&'static str],
    pub collection_ids: &'static [&'static str],
    pub hand_ids: &'static [&'static str],
}

pub fn catalog() -> &'static [PuzzleDef] {
    &PUZZLES
}

pub fn get(id: &str) -> Option<&'static PuzzleDef> {
    PUZZLES.iter().find(|p| p.id == id)
}

static PUZZLES: [PuzzleDef; 6] = [
    PuzzleDef {
        id: "lesson_score_ping",
        name: "Lesson 1: Score Ping",
        blurb: "Play a tiny script to hit a clean score target in one move.",
        hint: "Run `Tap Score` once. This teaches the shortest reliable scorer pattern.",
        theme: "terminal",
        start_bankroll: 10,
        start_score: 0,
        start_level: 1,
        target_score: 2,
        goal_bankroll: None,
        play_limit: Some(1),
        source_ids: &["tap_score", "tap_bank", "spark_draw"],
        collection_ids: &[],
        hand_ids: &["tap_score"],
    },
    PuzzleDef {
        id: "lesson_money_loop",
        name: "Lesson 2: Money Loop",
        blurb: "Build money, multiply it, then convert a slice into score.",
        hint: "The intended line is `Tap Bank -> Double Bank -> Cashout 2 -> Tap Score`.",
        theme: "crt",
        start_bankroll: 8,
        start_score: 0,
        start_level: 1,
        target_score: 4,
        goal_bankroll: Some(12),
        play_limit: Some(1),
        source_ids: &[
            "tap_bank",
            "double_bank",
            "cash_two",
            "tap_score",
            "tap_bank",
        ],
        collection_ids: &[],
        hand_ids: &["tap_bank", "double_bank", "cash_two", "tap_score"],
    },
    PuzzleDef {
        id: "lesson_draw_math",
        name: "Lesson 3: Draw Math",
        blurb: "Use draw sequencing to inflate deck size before a math finisher.",
        hint: "Play `Spark Draw` before `Tri Deck` so `D` is larger when `t(D)` runs.",
        theme: "magic",
        start_bankroll: 10,
        start_score: 0,
        start_level: 1,
        target_score: 10,
        goal_bankroll: None,
        play_limit: Some(1),
        source_ids: &[
            "tap_bank",
            "tap_score",
            "tap_bank",
            "tap_score",
            "spark_draw",
        ],
        collection_ids: &["tap_bank", "tap_score"],
        hand_ids: &["spark_draw", "tri_deck"],
    },
    PuzzleDef {
        id: "lesson_adaptive_branch",
        name: "Lesson 4: Adaptive Branch",
        blurb: "Use hedge logic to pivot your reward based on current progress.",
        hint: "Because score starts below target, `h(11)` adds score first, then `v(11)` cashes.",
        theme: "terminal",
        start_bankroll: 6,
        start_score: 8,
        start_level: 1,
        target_score: 12,
        goal_bankroll: Some(4),
        play_limit: Some(1),
        source_ids: &["hedge_two", "cash_two", "tap_score", "tap_bank"],
        collection_ids: &[],
        hand_ids: &["hedge_two", "cash_two", "tap_score"],
    },
    PuzzleDef {
        id: "lesson_meta_clone",
        name: "Lesson 5: Meta Clone",
        blurb: "Chain replay mechanics to multiply a cheap scoring card.",
        hint: "`Clone 2` requeues the previous card twice. Put it after your scorer.",
        theme: "magic",
        start_bankroll: 10,
        start_score: 0,
        start_level: 1,
        target_score: 6,
        goal_bankroll: None,
        play_limit: Some(1),
        source_ids: &["tap_score", "clone_pair", "tap_bank", "spark_draw"],
        collection_ids: &[],
        hand_ids: &["tap_score", "clone_pair"],
    },
    PuzzleDef {
        id: "lesson_fibo_sprint",
        name: "Lesson 6: Fibo Sprint",
        blurb: "Exploit level scaling with one compressed math card.",
        hint: "At level 5, `f(L+11);s(A)` yields Fibonacci(7)=13 for an instant clear.",
        theme: "crt",
        start_bankroll: 10,
        start_score: 0,
        start_level: 5,
        target_score: 13,
        goal_bankroll: None,
        play_limit: Some(1),
        source_ids: &["fibo_level", "tap_bank", "tap_score"],
        collection_ids: &[],
        hand_ids: &["fibo_level"],
    },
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::game::cards;

    #[test]
    fn puzzle_ids_are_unique() {
        let mut ids = HashSet::new();
        for p in catalog() {
            assert!(ids.insert(p.id), "duplicate puzzle id: {}", p.id);
        }
    }

    #[test]
    fn puzzle_cards_exist() {
        for p in catalog() {
            for id in p
                .source_ids
                .iter()
                .chain(p.collection_ids.iter())
                .chain(p.hand_ids.iter())
            {
                assert!(
                    cards::get(id).is_some(),
                    "puzzle {} references unknown card {}",
                    p.id,
                    id
                );
            }
        }
    }

    #[test]
    fn puzzle_core_fields_are_valid() {
        for p in catalog() {
            assert!(
                !p.name.trim().is_empty(),
                "name must not be empty: {}",
                p.id
            );
            assert!(
                !p.hint.trim().is_empty(),
                "hint must not be empty: {}",
                p.id
            );
            assert!(
                !p.hand_ids.is_empty(),
                "puzzle should provide a starter hand: {}",
                p.id
            );
            assert!(p.target_score >= 1, "target must be positive: {}", p.id);
            if let Some(limit) = p.play_limit {
                assert!(limit >= 1, "play limit must be at least 1: {}", p.id);
            }
        }
    }
}
