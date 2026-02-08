use dioxus::prelude::*;

#[component]
pub fn DeckWidget(deck_count: usize, collection_count: usize, level: u32) -> Element {
    rsx! {
        div { class: "panel deck-widget",
            h2 { class: "deck-title", "⚙ Source Reactor · L{level}" }
            div { class: "hint", "Generates new cards that flow into your Code Bank." }

            div { class: "deck-stack",
                div { class: "deck-card" }
                div { class: "deck-card" }
            }

            div { class: "deck-meta",
                div { class: "pill", span { "Source Flux" } strong { "data-testid": "source-count", "{deck_count}" } }
                div { class: "pill", span { "Code Deck" } strong { "data-testid": "deck-count", "{collection_count}" } }
            }
        }
    }
}
