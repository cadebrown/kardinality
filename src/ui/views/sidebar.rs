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
    let puzzles = kardinality::game::puzzles::catalog();

    // Sidebar focus highlight handled inline for each button (controls are dynamic).

    let engine_read = engine.read();
    let state = &engine_read.state;

    rsx! {
        aside { class: "sidebar",
            div { class: "brand",
                h1 { class: "brand-title", "Kardinality // Command Deck" }
                div { class: "brand-subtitle",
                    "Grid-first card programming roguelite • Rust logic • trace-driven"
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
                    h3 { "Run Controls" }
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

                div { class: "panel sidebar-panel",
                    h3 { "Puzzles / Tutorials" }
                    div { class: "hint",
                        "Preset deck-hand combos that teach patterns with guided hints and explicit goals."
                    }

                    if state.mode == kardinality::game::RunMode::Puzzle {
                        div { class: "kv", span { "Puzzle" } code { "{state.puzzle_title.clone().unwrap_or_else(|| \"Untitled\".to_string())}" } }
                        div { class: "kv", span { "Status" } code {
                            if state.puzzle_solved {
                                "Solved"
                            } else if state.puzzle_failed {
                                "Failed"
                            } else {
                                "In Progress"
                            }
                        } }
                        div { class: "kv", span { "Goal" } code { "{puzzle_goal_text(state)}" } }
                        if let Some(limit) = state.puzzle_play_limit {
                            div { class: "kv", span { "Plays Left" } code { "{limit.saturating_sub(state.turn)}" } }
                        }
                        if let Some(hint) = state.puzzle_hint.as_deref() {
                            div { class: "puzzle-hint-hero selectable",
                                div { class: "puzzle-hint-kicker", "Hint:" }
                                p { class: "puzzle-hint-text", "{hint}" }
                            }
                        }
                        if let Some(msg) = state.puzzle_message.as_deref() {
                            div { class: "puzzle-message-banner selectable", "{msg}" }
                        }
                        if let Some(id) = state.puzzle_id.as_deref() {
                            button {
                                class: "btn secondary",
                                onclick: {
                                    let retry_id = id.to_string();
                                    move |_| {
                                        let mut eng = engine.write();
                                        if let Err(e) = eng.dispatch(kardinality::Action::StartPuzzle { id: retry_id.clone() }) {
                                            eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                        }
                                    }
                                },
                                "Retry puzzle"
                            }
                        }
                    } else {
                        div { class: "hint", "No puzzle active. Pick one below." }
                    }

                    div { class: "trace-list",
                        for p in puzzles {
                            button {
                                class: "btn secondary",
                                "data-testid": "puzzle-{p.id}",
                                onclick: move |_| {
                                    {
                                        let mut s = settings.write();
                                        if let Some(theme) = theme_from_puzzle_key(p.theme) {
                                            s.theme = theme;
                                        }
                                    }
                                    let mut eng = engine.write();
                                    if let Err(e) = eng.dispatch(kardinality::Action::StartPuzzle { id: p.id.to_string() }) {
                                        eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                    }
                                },
                                "{p.name}"
                            }
                        }
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

fn theme_from_puzzle_key(key: &str) -> Option<UiTheme> {
    match key {
        "crt" => Some(UiTheme::Crt),
        "terminal" => Some(UiTheme::Terminal),
        "magic" => Some(UiTheme::Magic),
        _ => None,
    }
}

fn puzzle_goal_text(state: &kardinality::game::GameState) -> String {
    match state.puzzle_bankroll_goal {
        Some(goal) => format!("score >= {} and bank >= {}", state.target_score, goal),
        None => format!("score >= {}", state.target_score),
    }
}
