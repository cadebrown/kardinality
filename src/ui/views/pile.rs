use dioxus::prelude::*;

#[component]
pub fn PileWidget(count: usize, recent: Vec<String>) -> Element {
    let top = recent.first().cloned().unwrap_or_default();
    rsx! {
        div { class: "panel pile-widget", "data-testid": "pile-zone",
            h2 { class: "deck-title", "Pile" }
            div { class: "pile-face",
                div { class: "pile-face-title", if top.is_empty() { "Empty" } else { "{top}" } }
                div { class: "pile-face-sub", "Most recent" }
            }
            div { class: "pile-meta",
                div { class: "pill", span { "Cards" } strong { "data-testid": "pile-count", "{count}" } }
            }

            div { class: "pile-menu",
                div { class: "pile-menu-title", "Recent" }
                if recent.is_empty() {
                    div { class: "pile-item empty", "—" }
                } else {
                    for (i, name) in recent.iter().take(8).enumerate() {
                        div { class: "pile-item", "#{i + 1} {name}" }
                    }
                }
            }
        }
    }
}
