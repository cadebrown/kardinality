use dioxus::prelude::*;

#[component]
pub fn CardView(
    index: usize,
    card: kardinality::game::CardInstance,
    selected: bool,
    primary_icon: &'static str,
    on_select: EventHandler<usize>,
    on_primary: EventHandler<usize>,
    on_move_left: EventHandler<usize>,
    on_move_right: EventHandler<usize>,
    on_docs: EventHandler<String>,
    on_drag_start: EventHandler<usize>,
    on_drag_end: EventHandler<()>,
    on_drop: EventHandler<usize>,
) -> Element {
    let badge = format!("#{index}");

    let def_id = card.def_id.clone();
    let (name, script, budget, icon, doc_hint) = card
        .def()
        .map(|d| (d.name, d.script, d.budget, d.icon, d.kind))
        .map(|(name, script, budget, icon, kind)| (name, script, budget, icon, format!("{kind:?}")))
        .unwrap_or(("Missing Card", "/* missing */", 0, "?", "Missing".to_string()));

    let script_spans: Vec<(String, &'static str)> = match kardinality::kardlang::lex(script) {
        Ok(tokens) => tokens
            .into_iter()
            .filter(|t| !matches!(t.kind, kardinality::kardlang::TokenKind::Eof))
            .map(|t| {
                let text = script
                    .get(t.span.start..t.span.end)
                    .unwrap_or("")
                    .to_string();
                let class = match t.kind {
                    kardinality::kardlang::TokenKind::Ident(name) => {
                        let is_fn = matches!(
                            name.as_str(),
                            "draw" | "score" | "bank" | "dbl" | "tri" | "fibo" | "clone" | "again" | "mutate"
                        );
                        let is_reg = matches!(
                            name.as_str(),
                            "len_deck" | "len_hand" | "lvl" | "score"
                        );

                        if name == "acc" {
                            "acc"
                        } else if is_fn {
                            "fn"
                        } else if is_reg {
                            "reg"
                        } else {
                            "ident"
                        }
                    }
                    kardinality::kardlang::TokenKind::NumUnary(_) => "num",
                    kardinality::kardlang::TokenKind::NumDigit(_) => "num",
                    kardinality::kardlang::TokenKind::Plus => "op",
                    kardinality::kardlang::TokenKind::Star => "op",
                    kardinality::kardlang::TokenKind::LParen => "punct",
                    kardinality::kardlang::TokenKind::RParen => "punct",
                    kardinality::kardlang::TokenKind::Comma => "punct",
                    kardinality::kardlang::TokenKind::Semi => "punct",
                    kardinality::kardlang::TokenKind::Eof => "punct",
                };
                (text, class)
            })
            .collect(),
        Err(_) => vec![(script.to_string(), "raw")],
    };

    let class = if selected { "card selected" } else { "card" };

    rsx! {
        div {
            class: "{class}",
            id: "card-{card.id}",
            onclick: move |_| on_select.call(index),
            draggable: "true",
            ondragstart: move |evt: DragEvent| {
                let _ = evt
                    .data()
                    .data_transfer()
                    .set_data("text/plain", "kardinality-card");
                evt.data().data_transfer().set_effect_allowed("move");
                on_drag_start.call(index);
            },
            ondragend: move |_| on_drag_end.call(()),
            ondragover: move |evt| evt.prevent_default(),
            ondrop: move |evt| {
                evt.prevent_default();
                evt.stop_propagation();
                on_drop.call(index);
            },
            div { class: "card-badge", "{badge}" }
            div { class: "card-art" }
            div { class: "card-body",
                h3 { class: "card-title", "{icon} {name}" }
                div { class: "card-sub", "{doc_hint} • Budget: {budget}" }
                div { class: "card-script",
                    for (text, cls) in script_spans {
                        span { class: "tok tok-{cls}", "{text}" }
                    }
                }
            }

            div { class: "card-actions",
                button {
                    class: "card-btn",
                    title: "Docs",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_docs.call(def_id.clone());
                    },
                    "📖"
                }
                button {
                    class: "card-btn",
                    title: "Move",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_primary.call(index);
                    },
                    "{primary_icon}"
                }
                button {
                    class: "card-btn",
                    title: "Move left",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_move_left.call(index);
                    },
                    "←"
                }
                button {
                    class: "card-btn",
                    title: "Move right",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_move_right.call(index);
                    },
                    "→"
                }
            }
        }
    }
}


