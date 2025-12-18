use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KTab {
    Overview,
    Functions,
    Examples,
    Reference,
}

#[component]
pub fn KardinomiconModal(
    open: bool,
    target: Option<String>,
    on_close: EventHandler<()>,
) -> Element {
    let catalog = kardinality::game::cards::catalog();
    let mut tab = use_signal(|| KTab::Overview);

    // If a target is provided, show it first.
    let mut ordered: Vec<kardinality::game::cards::CardDef> = Vec::with_capacity(catalog.len());
    if let Some(t) = target.as_deref() {
        if let Some(found) = catalog.iter().find(|c| c.id == t) {
            ordered.push(*found);
        }
    }
    for c in catalog {
        if ordered.first().is_some_and(|first| first.id == c.id) {
            continue;
        }
        ordered.push(*c);
    }

    let tab_value = tab();

    rsx! {
        if open {
            div {
                class: "modal-overlay",
                onclick: move |_| on_close.call(()),

                div {
                    class: "modal panel",
                    onclick: move |evt| evt.stop_propagation(),

                    div { class: "modal-header",
                        div { class: "modal-title",
                            span { class: "modal-glyph", "⟡" }
                            span { "Kardinomicon" }
                        }
                        button { class: "btn danger", onclick: move |_| on_close.call(()), "Close" }
                    }

                    div { class: "modal-body",
                        div { class: "tabs",
                            button {
                                class: if tab_value == KTab::Overview { "tab active" } else { "tab" },
                                onclick: move |_| tab.set(KTab::Overview),
                                "Overview"
                            }
                            button {
                                class: if tab_value == KTab::Functions { "tab active" } else { "tab" },
                                onclick: move |_| tab.set(KTab::Functions),
                                "Functions"
                            }
                            button {
                                class: if tab_value == KTab::Examples { "tab active" } else { "tab" },
                                onclick: move |_| tab.set(KTab::Examples),
                                "Examples"
                            }
                            button {
                                class: if tab_value == KTab::Reference { "tab active" } else { "tab" },
                                onclick: move |_| tab.set(KTab::Reference),
                                "Reference"
                            }
                        }

                        if tab_value == KTab::Overview {
                            h2 { class: "hud-title", "What is this?" }
                            div { class: "kdoc selectable",
                                "Cards are little programs. You assemble a Hand (top), then hit "
                                strong { "Play Hand" }
                                " to execute them in order."
                                br {}
                                "Kardlang is integer-only with strict budgets. Power scales with length."
                                br {}
                                "Key registers: "
                                code { "len_deck" } " • " code { "len_hand" } " • " code { "lvl" }
                                br {}
                                "Tip: drop between cards to insert, or on a card to swap."
                            }

                            h2 { class: "hud-title", "Cards" }
                            div { class: "kcard-list",
                                for c in ordered {
                                    KardinomiconCard {
                                        card: c,
                                        highlight: target.as_deref().is_some_and(|t| t == c.id),
                                    }
                                }
                            }
                        } else if tab_value == KTab::Functions {
                            h2 { class: "hud-title", "Built-in functions" }
                            div { class: "kdoc selectable",
                                strong { "Economy / score" } br {}
                                code { "score(n)" } " → score += n" br {}
                                code { "bank(n)" } " → bankroll += n" br {}
                                code { "dbl()" } " → bankroll *= 2" br {}
                                br {}
                                strong { "Deck ops" } br {}
                                code { "draw(n)" } " → generate n new cards into your Deck" br {}
                                br {}
                                strong { "Acc / math" } br {}
                                code { "tri(x)" } " → acc = x*(x+1)/2" br {}
                                code { "fibo(x)" } " → acc = F(x)" br {}
                                br {}
                                strong { "Meta" } br {}
                                code { "clone(n)" } " → queue n copies of last played card" br {}
                                code { "again(n)" } " → queue n replays of last played card" br {}
                                code { "mutate()" } " → mutate last played card into a random new one" br {}
                            }
                        } else if tab_value == KTab::Examples {
                            h2 { class: "hud-title", "Example combos" }
                            div { class: "kdoc selectable",
                                strong { "Build then score" } br {}
                                code { "draw(5)" } " → expand options" br {}
                                code { "tri(len_deck); score(acc)" } " → score based on deck size" br {}
                                br {}
                                strong { "Snowball economy" } br {}
                                code { "bank(6); dbl()" } " → bankroll growth" br {}
                                br {}
                                strong { "Chaos loop" } br {}
                                code { "mutate(); again(2)" } " → roll the dice twice" br {}
                            }
                        } else {
                            h2 { class: "hud-title", "Language reference" }
                            div { class: "kdoc selectable",
                                "Numbers are unary (" code { "111" } " = 3) or digit shorthand (" code { "4" } " = 4 but costs 4)."
                                br {}
                                "Expressions support " code { "+" } " and " code { "*" } " with parentheses."
                                br {}
                                "Registers: " code { "len_deck" } ", " code { "len_hand" } ", " code { "lvl" } ", and writable " code { "acc" } "."
                            }
                            pre { class: "kcard-doc selectable", "{kardinality::kardlang::GRAMMAR.trim()}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn KardinomiconCard(card: kardinality::game::cards::CardDef, highlight: bool) -> Element {
    let class = if highlight {
        "kcard highlight"
    } else {
        "kcard"
    };
    rsx! {
        div { class: "{class}",
            div { class: "kcard-head",
                span { class: "kcard-icon", "{card.icon}" }
                div { class: "kcard-title",
                    div { class: "kcard-name", "{card.name}" }
                    div { class: "kcard-id", "{card.id}" }
                }
                div { class: "kcard-script selectable", "{card.script}" }
            }
            pre { class: "kcard-doc selectable", "{card.doc.trim()}" }
        }
    }
}
