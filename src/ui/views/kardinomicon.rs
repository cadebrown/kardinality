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
                                "Kardlang is integer-only with strict budgets. Every character matters."
                                br {}
                                "Key registers: "
                                code { "D" } " • " code { "H" } " • " code { "S" } " • " code { "L" } " • " code { "T" } " • writable " code { "A" }
                                br {}
                                "Want a guided start? Open "
                                strong { "Controls → Puzzles / Tutorials" }
                                " and launch a lesson."
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
                                code { "score(n) / s(n)" } " → score += n" br {}
                                code { "bank(n) / b(n)" } " → bankroll += n" br {}
                                code { "dbl() / x()" } " → bankroll *= 2" br {}
                                code { "cash(n) / v(n)" } " → score += n, bankroll -= n" br {}
                                code { "hedge(n) / h(n)" } " → score if behind target, bankroll if ahead" br {}
                                br {}
                                strong { "Deck ops" } br {}
                                code { "draw(n) / d(n)" } " → generate n new cards into your Deck" br {}
                                code { "jam(n) / j(n)" } " → score += n and draw 1" br {}
                                code { "mint(n) / i(n)" } " → bankroll += n and draw 1" br {}
                                br {}
                                strong { "Acc / math" } br {}
                                code { "tri(x) / t(x)" } " → A = x*(x+1)/2" br {}
                                code { "fibo(x) / f(x)" } " → A = F(x)" br {}
                                br {}
                                strong { "Meta" } br {}
                                code { "clone(n) / c(n)" } " → queue n copies of last played card" br {}
                                code { "again(n) / a(n)" } " → queue n replays of last played card" br {}
                                code { "mutate() / m()" } " → mutate last played card into a random new one" br {}
                                code { "wild(n) / w(n)" } " → mutate then replay n times" br {}
                            }
                        } else if tab_value == KTab::Examples {
                            h2 { class: "hud-title", "Example combos" }
                            div { class: "kdoc selectable",
                                strong { "Grow and pressure" } br {}
                                code { "j(11); d(H)" } " → score while refilling options" br {}
                                br {}
                                strong { "Math scaler" } br {}
                                code { "t(D); s(A)" } " → triangular score burst from deck size" br {}
                                br {}
                                strong { "Adaptive line" } br {}
                                code { "h(11); v(11)" } " → push score when behind, cash out when ahead" br {}
                            }
                        } else {
                            h2 { class: "hud-title", "Language reference" }
                            div { class: "kdoc selectable",
                                "Numbers are unary (" code { "111" } " = 3) or digit shorthand (" code { "4" } " = 4 but costs 4)."
                                br {}
                                "Expressions support " code { "+" } " and " code { "*" } " with parentheses."
                                br {}
                                "Registers (long + short): "
                                code { "len_deck/D" } ", "
                                code { "len_hand/H" } ", "
                                code { "len_source/S" } ", "
                                code { "len_pile/P" } ", "
                                code { "level/L" } ", "
                                code { "target/T" } ", "
                                code { "bankroll/B" } ", "
                                code { "score/Q" } ", writable "
                                code { "acc/A" } "."
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
    let kind = kardinality::game::cards::kind_visual(card.kind);
    let functions = kardinality::game::cards::script_function_visuals(card.script);
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
            div { class: "kcard-signals",
                div { class: "kcard-kind",
                    span { class: "kcard-kind-icon", "{kind.icon}" }
                    span { class: "kcard-kind-label", "{kind.label}" }
                }
                for fx in functions {
                    div { class: "kcard-fx fx-{fx.accent}",
                        span { class: "kcard-fx-icon", "{fx.icon}" }
                        span { class: "kcard-fx-label", "{fx.label}" }
                    }
                }
            }
            pre { class: "kcard-doc selectable", "{card.doc.trim()}" }
        }
    }
}
