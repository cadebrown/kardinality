use dioxus::prelude::*;

use crate::ui::state::{UiSettings, UiTheme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarTab {
    Controls,
    Debug,
    Settings,
}

#[component]
pub fn Sidebar(
    mut engine: Signal<kardinality::Engine>,
    mut tab: Signal<SidebarTab>,
    mut settings: Signal<UiSettings>,
    mut kardinomicon_open: Signal<bool>,
    mut kardinomicon_target: Signal<Option<String>>,
    focused: bool,
    focus_index: usize,
) -> Element {
    let tab_value = tab();
    let cur_settings = settings();

    // Sidebar focus highlight handled inline for each button (controls are dynamic).

    let engine_read = engine.read();
    let state = &engine_read.state;

    rsx! {
        aside { class: "sidebar",
            div { class: "brand",
                h1 { class: "brand-title", "Kardinality" }
                div { class: "brand-subtitle",
                    "retro CRT deckbuilder • cards-as-code • rust+wasm"
                }
            }

            div { class: "tabs",
                button {
                    class: if tab_value == SidebarTab::Controls { "tab active" } else { "tab" },
                    onclick: move |_| tab.set(SidebarTab::Controls),
                    "Controls"
                }
                button {
                    class: if tab_value == SidebarTab::Debug { "tab active" } else { "tab" },
                    onclick: move |_| tab.set(SidebarTab::Debug),
                    "Debug"
                }
                button {
                    class: if tab_value == SidebarTab::Settings { "tab active" } else { "tab" },
                    onclick: move |_| tab.set(SidebarTab::Settings),
                    "Settings"
                }
            }

            if tab_value == SidebarTab::Controls {
                div { class: "panel sidebar-panel",
                    h3 { "Run" }
                    button {
                        "data-testid": "reset-game",
                        class: if focused && focus_index == 0 { "btn danger focused" } else { "btn danger" },
                        title: "Warning: resets your run state",
                        onclick: move |_| {
                            let mut eng = engine.write();
                            if let Err(e) = eng.dispatch(kardinality::Action::NewRun { seed: 0 }) {
                                eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                            }
                        },
                        "Reset game"
                    }
                    button {
                        "data-testid": "open-docs",
                        class: if focused && focus_index == 1 { "btn secondary focused" } else { "btn secondary" },
                        onclick: move |_| {
                            kardinomicon_target.set(None);
                            kardinomicon_open.set(true);
                        },
                        "Open docs"
                    }
                }
            } else if tab_value == SidebarTab::Debug {
                div { class: "panel sidebar-panel",
                    h3 { "Trace (latest first)" }
            button {
                class: "btn danger",
                "data-testid": "clear-trace",
                onclick: move |_| {
                    let mut eng = engine.write();
                    let _ = eng.dispatch(kardinality::Action::ClearTrace);
                },
                "Clear trace"
            }
                    if state.trace.is_empty() {
                        div { class: "empty", "No trace yet. Assemble a hand and execute." }
                    } else {
                        div { class: "trace-list selectable",
                            for evt in state.trace.iter().rev().take(120) {
                                TraceItem { evt: evt.clone() }
                            }
                        }
                    }
                }

        div { class: "panel sidebar-panel",
            h3 { "Telemetry" }
            div { class: "kv", span { "Source" } code { "{state.deck.len()}" } }
            div { class: "kv", span { "Deck" } code { "{state.collection.len()}" } }
            div { class: "kv", span { "Hand" } code { "{state.hand.len()}" } }
            div { class: "kv", span { "Pile" } code { "{state.pile.len()}" } }
            div { class: "kv", span { "Trace" } code { "{state.trace.len()}" } }
            div { class: "kv", span { "Phase" } code { "{state.phase:?}" } }
        }
            } else {
                div { class: "panel sidebar-panel",
                    h3 { "Theme" }
                    div { class: "tabs",
                        for theme in [UiTheme::Crt, UiTheme::Terminal, UiTheme::Magic] {
                            button {
                                class: if cur_settings.theme == theme { "tab active" } else { "tab" },
                                onclick: move |_| {
                                    let mut s = settings.write();
                                    s.theme = theme;
                                },
                                "{theme.label()}"
                            }
                        }
                    }

                    h3 { "Effects" }
                    button {
                        class: if cur_settings.effects { "btn" } else { "btn secondary" },
                        onclick: move |_| {
                            let mut s = settings.write();
                            s.effects = !s.effects;
                        },
                        if cur_settings.effects { "CRT overlays: ON" } else { "CRT overlays: OFF" }
                    }

                    div { class: "hint",
                        "Hotkeys: arrows select • ↑/↓ move • Shift+←/→ reorder • Enter executes."
                    }
                }
            }
        }
    }
}

#[component]
fn TraceItem(evt: kardinality::TraceEvent) -> Element {
    let (class, text) = match &evt {
        kardinality::TraceEvent::Error(_) => ("trace-item error", format!("{evt:?}")),
        kardinality::TraceEvent::Call { .. } => ("trace-item call", format!("{evt:?}")),
        kardinality::TraceEvent::EffectApplied { .. } => ("trace-item effect", format!("{evt:?}")),
        _ => ("trace-item", format!("{evt:?}")),
    };

    rsx! { div { class: "{class}", "{text}" } }
}
