use dioxus::prelude::*;

#[component]
pub fn ScoreBankBody(bankroll: i64, score: i64, target: i64) -> Element {
    let pct = if target <= 0 {
        0.0
    } else {
        (score as f64 / target as f64).clamp(0.0, 1.25)
    };
    let pct100 = (pct * 100.0).clamp(0.0, 125.0);
    let fill_style = format!("width: {pct100:.2}%");

    let stage = if pct < 0.33 {
        "low"
    } else if pct < 0.66 {
        "mid"
    } else if pct < 1.0 {
        "high"
    } else {
        "clear"
    };

    rsx! {
        h2 { class: "hud-title", "Score" }

        div { class: "scoreline",
            div { class: "scorebig", "data-testid": "score-value", "{score}" }
            div { class: "scoremeta",
                div { class: "hint", "Target" }
                div { class: "scoretarget", "{target}" }
            }
        }

        div { class: "progress",
            div { class: "progress-fill {stage}", style: "{fill_style}" }
        }

        div { class: "bankline",
            div { class: "hint", "Money" }
            div { class: "bankbig", "data-testid": "money-value", "${bankroll}" }
        }
    }
}

#[component]
pub fn RegistersBody(collection_count: usize, hand_count: usize) -> Element {
    rsx! {
        h2 { class: "hud-title", "Registers" }
        div { class: "kv", span { "len_deck" } code { "{collection_count}" } }
        div { class: "kv", span { "len_hand" } code { "{hand_count}" } }
    }
}

#[component]
pub fn TopHud(
    bankroll: i64,
    score: i64,
    target: i64,
    collection_count: usize,
    hand_count: usize,
    level: u32,
) -> Element {
    rsx! {
        div { class: "panel hud-panel",
            ScoreBankBody { bankroll, score, target }
        }

        div { class: "panel hud-panel",
            RegistersBody { collection_count, hand_count }
        }
    }
}
