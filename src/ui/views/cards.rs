use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PtrDown {
    pub index: usize,
    pub client_x: f64,
    pub client_y: f64,
    pub elem_x: f64,
    pub elem_y: f64,
}

#[component]
pub fn CardView(
    index: usize,
    card: kardinality::game::CardInstance,
    selected: bool,
    focused: bool,
    dragging: bool,
    drag_style: String,
    primary_icon: &'static str,
    on_select: EventHandler<usize>,
    on_primary: EventHandler<usize>,
    on_move_left: EventHandler<usize>,
    on_move_right: EventHandler<usize>,
    on_docs: EventHandler<String>,
    on_ptr_down: EventHandler<PtrDown>,
) -> Element {
    let badge = format!("#{index}");

    let def_id = card.def_id.clone();
    let (name, script, budget, icon, kind_label, kind_icon, kind_class, fn_visuals) = card
        .def()
        .map(|d| {
            let kind_class = match d.kind {
                kardinality::game::cards::CardKind::Economy => "kind-economy",
                kardinality::game::cards::CardKind::Score => "kind-score",
                kardinality::game::cards::CardKind::Control => "kind-control",
                kardinality::game::cards::CardKind::Meta => "kind-meta",
            };
            let kind_visual = kardinality::game::cards::kind_visual(d.kind);
            (
                d.name,
                d.script,
                d.budget,
                d.icon,
                kind_visual.label.to_string(),
                kind_visual.icon,
                kind_class,
                kardinality::game::cards::script_function_visuals(d.script),
            )
        })
        .unwrap_or((
            "Missing Card",
            "/* missing */",
            0,
            "?",
            "Missing".to_string(),
            "?",
            "kind-missing",
            Vec::new(),
        ));

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
                            "draw"
                                | "d"
                                | "score"
                                | "s"
                                | "bank"
                                | "b"
                                | "dbl"
                                | "x"
                                | "tri"
                                | "t"
                                | "fibo"
                                | "f"
                                | "clone"
                                | "c"
                                | "again"
                                | "a"
                                | "mutate"
                                | "m"
                                | "jam"
                                | "j"
                                | "mint"
                                | "i"
                                | "cash"
                                | "v"
                                | "hedge"
                                | "h"
                                | "wild"
                                | "w"
                        );
                        let is_reg = matches!(
                            name.as_str(),
                            "len_deck"
                                | "len_pool"
                                | "len_collection"
                                | "len_hand"
                                | "len_source"
                                | "len_draw"
                                | "len_pile"
                                | "len_discard"
                                | "level"
                                | "lvl"
                                | "target"
                                | "bankroll"
                                | "money"
                                | "score"
                                | "deck"
                                | "hand"
                                | "D"
                                | "H"
                                | "S"
                                | "P"
                                | "L"
                                | "T"
                                | "B"
                                | "Q"
                        );

                        if name == "acc" || name == "A" {
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

    let mut class = if selected {
        format!("card selected {kind_class}")
    } else {
        format!("card {kind_class}")
    };
    if focused {
        class.push_str(" focused");
    }
    if dragging {
        class.push_str(" dragging");
    }

    rsx! {
        div {
            class: "{class}",
            id: "card-{card.id}",
            "data-selected": if selected { "true" } else { "false" },
            "data-dragging": if dragging { "true" } else { "false" },
            title: "Click to select • Drag to move • 📖 for docs",
            style: "{drag_style}",
            onclick: move |_| on_select.call(index),
            onpointerdown: move |evt: PointerEvent| {
                let c = evt.data().client_coordinates();
                let e = evt.data().element_coordinates();
                on_ptr_down.call(PtrDown {
                    index,
                    client_x: c.x,
                    client_y: c.y,
                    elem_x: e.x,
                    elem_y: e.y,
                });
            },
            div { class: "card-top",
                div { class: "card-top-left",
                    div { class: "card-index", "{badge}" }
                    div { class: "card-kind-tag",
                        span { class: "kind-glyph", "{kind_icon}" }
                        span { class: "kind-name", "{kind_label}" }
                    }
                }
                button {
                    class: "card-docs",
                    title: "Docs",
                    draggable: "false",
                    onpointerdown: move |evt| {
                        evt.stop_propagation();
                        evt.prevent_default();
                    },
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_docs.call(def_id.clone());
                    },
                    "📖"
                }
            }
            div { class: "card-art",
                div { class: "card-main-icon", "{icon}" }
                div { class: "card-fx-ribbon",
                    if fn_visuals.is_empty() {
                        div { class: "fx-dot fx-unknown", "∅" }
                    } else {
                        for fx in fn_visuals.iter().take(4) {
                            div { class: "fx-dot fx-{fx.accent}", title: "{fx.label}",
                                span { class: "fx-glyph", "{fx.icon}" }
                                span { class: "fx-short", "{fx.short}" }
                            }
                        }
                        if fn_visuals.len() > 4 {
                            div { class: "fx-dot fx-more", "+{fn_visuals.len() - 4}" }
                        }
                    }
                }
            }
            div { class: "card-body",
                h3 { class: "card-title", "{name}" }
                div { class: "card-sub", "{kind_label} • Budget: {budget}" }
                div { class: "card-script",
                    for (text, cls) in script_spans {
                        span { class: "tok tok-{cls}", "{text}" }
                    }
                }
                if !fn_visuals.is_empty() {
                    div { class: "card-fx-row",
                        for fx in fn_visuals {
                            div { class: "card-fx-chip fx-{fx.accent}", title: "{fx.label}",
                                span { class: "chip-icon", "{fx.icon}" }
                                span { class: "chip-label", "{fx.label}" }
                            }
                        }
                    }
                }
            }

            div { class: "card-actions",
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
