use dioxus::prelude::*;

#[component]
pub fn DeckWidget(
    deck_count: usize,
    discard_count: usize,
    collection_count: usize,
    hand_count: usize,
    level: u32,
    target: i64,
) -> Element {
    rsx! {
        div { class: "panel deck-widget",
            h2 { class: "deck-title", "Source // Level {level}" }

            div { class: "deck-stack",
                div { class: "deck-card" }
                div { class: "deck-card" }
                div { class: "deck-card" }
            }

            div { class: "deck-meta",
                div { class: "pill", span { "Source" } strong { "{deck_count}" } }
                div { class: "pill", span { "Discard" } strong { "{discard_count}" } }
                div { class: "pill", span { "Deck" } strong { "{collection_count}" } }
                div { class: "pill", span { "Hand" } strong { "{hand_count}" } }
                div { class: "pill", span { "Target" } strong { "{target}" } }
                div { class: "pill", span { "RNG" } strong { "seeded" } }
            }
        }
    }
}


