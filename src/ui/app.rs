use dioxus::prelude::*;

use crate::ui::state::UiSettings;
use crate::ui::theme;
use crate::ui::views::{DeckWidget, KardinomiconModal, Sidebar, SidebarTab, TopHud};
use crate::ui::anim;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusZone {
    Sidebar,
    Deck,
    Hand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DragPayload {
    zone: FocusZone,
    index: usize,
}

#[component]
pub fn App() -> Element {
    let mut engine = use_signal(|| kardinality::Engine::new(0));
    let mut tab = use_signal(|| SidebarTab::Controls);

    let settings = use_signal(UiSettings::default);
    let mut kardinomicon_open = use_signal(|| false);
    let mut kardinomicon_target = use_signal(|| None::<String>);

    // Keyboard-first selection state.
    let mut focus = use_signal(|| FocusZone::Deck);
    let mut sel_collection = use_signal(|| 0usize);
    let mut sel_hand = use_signal(|| 0usize);
    let mut drag = use_signal(|| None::<DragPayload>);
    let mut sidebar_index = use_signal(|| 0usize);

    let engine_read = engine.read();
    let state = &engine_read.state;

    let deck_count = state.deck.len();
    let discard_count = state.discard.len();
    let collection_count = state.collection.len();
    let hand_count = state.hand.len();

    let focus_value = focus();
    let selected_collection = sel_collection();
    let selected_hand = sel_hand();

    let settings_value = settings();
    let app_class = if settings_value.effects {
        format!("app {}", settings_value.theme.class())
    } else {
        format!("app {} effects-off", settings_value.theme.class())
    };

    rsx! {
        style { {theme::CSS} }

        div {
            class: "{app_class}",
            tabindex: "0",
            onmounted: move |evt| {
                // Keep the app keyboard-first: focus the root so arrow keys work immediately.
                let node = evt.data();
                spawn(async move {
                    let _ = node.set_focus(true).await;
                });
            },
            onkeydown: move |evt: KeyboardEvent| {
                use dioxus::prelude::Key;
                use keyboard_types::Modifiers;

                let key = evt.key();
                let shift = evt.modifiers().contains(Modifiers::SHIFT);

                // Avoid the browser interpreting arrows/tab as scrolling/focus shifts.
                if matches!(
                    key,
                    Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown | Key::Tab
                ) {
                    evt.prevent_default();
                }

                // Helper: clamp an index into [0, len-1] (or 0 if empty).
                let clamp = |idx: usize, len: usize| -> usize {
                    if len == 0 { 0 } else { idx.min(len - 1) }
                };

                match key {
                    Key::ArrowLeft => {
                        if focus() == FocusZone::Sidebar {
                            // no-op for now (reserved for tab switching)
                        } else if focus() == FocusZone::Deck {
                            let len = engine.read().state.collection.len();
                            let idx = sel_collection();
                            if shift && len > 0 && idx > 0 {
                                let before = {
                                    let st = engine.read();
                                    let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                    anim::capture_rects(&ids)
                                };
                                let mut eng = engine.write();
                                let _ = eng.dispatch(kardinality::Action::ReorderCollection { from: idx, to: idx - 1 });
                                sel_collection.set(idx - 1);
                                anim::play_flip(before, 260.0);
                            } else {
                                sel_collection.set(clamp(idx.saturating_sub(1), len));
                            }
                        } else if focus() == FocusZone::Hand {
                            let len = engine.read().state.hand.len();
                            let idx = sel_hand();
                            if shift && len > 0 && idx > 0 {
                                let before = {
                                    let st = engine.read();
                                    let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                    anim::capture_rects(&ids)
                                };
                                let mut eng = engine.write();
                                let _ = eng.dispatch(kardinality::Action::ReorderHand { from: idx, to: idx - 1 });
                                sel_hand.set(idx - 1);
                                anim::play_flip(before, 260.0);
                            } else {
                                sel_hand.set(clamp(idx.saturating_sub(1), len));
                            }
                        }
                    }
                    Key::ArrowRight => {
                        if focus() == FocusZone::Sidebar {
                            // no-op for now (reserved for tab switching)
                        } else if focus() == FocusZone::Deck {
                            let len = engine.read().state.collection.len();
                            let idx = sel_collection();
                            if shift && len > 0 && idx + 1 < len {
                                let before = {
                                    let st = engine.read();
                                    let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                    anim::capture_rects(&ids)
                                };
                                let mut eng = engine.write();
                                let _ = eng.dispatch(kardinality::Action::ReorderCollection { from: idx, to: idx + 1 });
                                sel_collection.set(idx + 1);
                                anim::play_flip(before, 260.0);
                            } else {
                                sel_collection.set(clamp(idx + 1, len));
                            }
                        } else if focus() == FocusZone::Hand {
                            let len = engine.read().state.hand.len();
                            let idx = sel_hand();
                            if shift && len > 0 && idx + 1 < len {
                                let before = {
                                    let st = engine.read();
                                    let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                    anim::capture_rects(&ids)
                                };
                                let mut eng = engine.write();
                                let _ = eng.dispatch(kardinality::Action::ReorderHand { from: idx, to: idx + 1 });
                                sel_hand.set(idx + 1);
                                anim::play_flip(before, 260.0);
                            } else {
                                sel_hand.set(clamp(idx + 1, len));
                            }
                        }
                    }
                    Key::ArrowUp => {
                        if focus() == FocusZone::Sidebar {
                            let idx = sidebar_index();
                            sidebar_index.set(idx.saturating_sub(1));
                            return;
                        }
                        // Move from collection -> hand and focus hand.
                        if focus() == FocusZone::Deck {
                            let idx = sel_collection();
                            let before = {
                                let st = engine.read();
                                let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                anim::capture_rects(&ids)
                            };
                            let mut eng = engine.write();
                            if let Err(e) = eng.dispatch(kardinality::Action::MoveCollectionToHand { index: idx }) {
                                eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                            }
                            focus.set(FocusZone::Hand);
                            let new_hand_len = eng.state.hand.len();
                            if new_hand_len > 0 {
                                sel_hand.set(new_hand_len - 1);
                            }
                            let new_coll_len = eng.state.collection.len();
                            sel_collection.set(clamp(idx, new_coll_len));
                            anim::play_flip(before, 280.0);
                        }
                    }
                    Key::ArrowDown => {
                        if focus() == FocusZone::Sidebar {
                            let idx = sidebar_index();
                            // Controls has 3 actions; Debug has 1 action (clear trace); Settings has 0 focusable actions.
                            let max = match tab() {
                                SidebarTab::Controls => 1usize,
                                SidebarTab::Debug => 0usize,
                                SidebarTab::Settings => 0usize,
                            };
                            sidebar_index.set((idx + 1).min(max));
                            return;
                        }
                        // Move from hand -> collection and focus collection.
                        if focus() == FocusZone::Hand {
                            let idx = sel_hand();
                            let before = {
                                let st = engine.read();
                                let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                anim::capture_rects(&ids)
                            };
                            let mut eng = engine.write();
                            if let Err(e) = eng.dispatch(kardinality::Action::MoveHandToCollection { index: idx }) {
                                eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                            }
                            focus.set(FocusZone::Deck);
                            let new_coll_len = eng.state.collection.len();
                            if new_coll_len > 0 {
                                sel_collection.set(new_coll_len - 1);
                            }
                            let new_hand_len = eng.state.hand.len();
                            sel_hand.set(clamp(idx, new_hand_len));
                            anim::play_flip(before, 280.0);
                        }
                    }
                    Key::Enter => {
                        if focus() == FocusZone::Sidebar {
                            let idx = sidebar_index();
                            match tab() {
                                SidebarTab::Controls => {
                                    let mut eng = engine.write();
                                    match idx {
                                        0 => {
                                            if let Err(e) = eng.dispatch(kardinality::Action::NewRun { seed: 0 }) {
                                                eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                            }
                                        }
                                        1 => {
                                            kardinomicon_target.set(None);
                                            kardinomicon_open.set(true);
                                        }
                                        _ => {}
                                    }
                                }
                                SidebarTab::Debug => {
                                    let mut eng = engine.write();
                                    let _ = eng.dispatch(kardinality::Action::ClearTrace);
                                }
                                SidebarTab::Settings => {}
                            }
                            return;
                        }

                        let mut eng = engine.write();
                        if let Err(e) = eng.dispatch(kardinality::Action::PlayHand) {
                            eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                        }
                    }
                    Key::Tab => {
                        // Cycle focus through Sidebar -> Hand -> Deck.
                        match focus() {
                            FocusZone::Sidebar => {
                                focus.set(FocusZone::Hand);
                                let len = engine.read().state.hand.len();
                                sel_hand.set(clamp(sel_hand(), len));
                            }
                            FocusZone::Hand => {
                                focus.set(FocusZone::Deck);
                                let len = engine.read().state.collection.len();
                                sel_collection.set(clamp(sel_collection(), len));
                            }
                            FocusZone::Deck => {
                                focus.set(FocusZone::Sidebar);
                                tab.set(SidebarTab::Controls);
                                sidebar_index.set(sidebar_index().min(1));
                            }
                        }
                    }
                    _ => {}
                }
            },
            Sidebar {
                engine,
                tab,
                settings,
                kardinomicon_open,
                kardinomicon_target,
                focused: focus_value == FocusZone::Sidebar,
                focus_index: sidebar_index(),
            }

            div { class: "main",
                div { class: "topbar",
                    div { class: "hud",
                        TopHud {
                            bankroll: state.bankroll,
                            score: state.score,
                            target: state.target_score,
                            collection_count,
                            hand_count,
                            level: state.level,
                        }
                    }

                    div { class: "right-rail",
                        button {
                            class: "play-btn",
                            "data-testid": "play-hand",
                            onclick: move |_| {
                                let before = {
                                    let st = engine.read();
                                    let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                    anim::capture_rects(&ids)
                                };
                                let mut eng = engine.write();
                                if let Err(e) = eng.dispatch(kardinality::Action::PlayHand) {
                                    eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                }
                                anim::play_flip(before, 360.0);
                            },
                            span { class: "play-icon", "▶" }
                            span { class: "play-text", "Play Hand" }
                        }

                        DeckWidget {
                            deck_count,
                            discard_count,
                            collection_count,
                            hand_count,
                            level: state.level,
                            target: state.target_score,
                        }
                    }
                }

                div { class: "handbar", "data-testid": "hand-zone",
                    div { class: "hand-title",
                        span { "Hand" }
                        span { class: "hint", "{hand_count} cards" }
                    }

                    if state.hand.is_empty() {
                        div { class: "row-scroll",
                            div { class: "row center",
                                div {
                                    class: "ghost-card dropzone",
                                    ondragover: move |evt| evt.prevent_default(),
                                    ondrop: move |evt| {
                                        evt.prevent_default();
                                        if let Some(payload) = drag() {
                                            let before = {
                                                let st = engine.read();
                                                let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                anim::capture_rects(&ids)
                                            };
                                            let mut eng = engine.write();
                                            if payload.zone == FocusZone::Deck {
                                                if let Err(e) = eng.dispatch(kardinality::Action::MoveCollectionToHand { index: payload.index }) {
                                                    eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                                }
                                                focus.set(FocusZone::Hand);
                                                sel_hand.set(eng.state.hand.len().saturating_sub(1));
                                            }
                                            anim::play_flip(before, 280.0);
                                        }
                                        drag.set(None);
                                    },
                                    div { class: "ghost-plus", "+" }
                                    div { class: "ghost-hint", "Drop to add" }
                                }
                            }
                        }
                    } else {
                        div {
                            class: "row-scroll",
                            ondragover: move |evt| evt.prevent_default(),
                            ondrop: move |evt| {
                                evt.prevent_default();
                                if let Some(payload) = drag() {
                                    let before = {
                                        let st = engine.read();
                                        let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                        anim::capture_rects(&ids)
                                    };
                                    let mut eng = engine.write();
                                    match payload.zone {
                                        FocusZone::Hand => {
                                            let len = eng.state.hand.len();
                                            if len > 0 {
                                                let to = len - 1;
                                                let _ = eng.dispatch(kardinality::Action::ReorderHand { from: payload.index, to });
                                                focus.set(FocusZone::Hand);
                                                sel_hand.set(to);
                                            }
                                        }
                                        FocusZone::Deck => {
                                            if let Err(e) = eng.dispatch(kardinality::Action::MoveCollectionToHand { index: payload.index }) {
                                                eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                            }
                                            focus.set(FocusZone::Hand);
                                            sel_hand.set(eng.state.hand.len().saturating_sub(1));
                                        }
                                        FocusZone::Sidebar => {}
                                    }
                                    anim::play_flip(before, 280.0);
                                }
                                drag.set(None);
                            },
                            div { class: "row",
                                // Drop slots allow inserting between cards. Dropping onto a card swaps.
                                for slot in 0..=state.hand.len() {
                                    div {
                                        key: "hand-slot-{slot}",
                                        class: "drop-slot",
                                        ondragover: move |evt| evt.prevent_default(),
                                        ondrop: move |evt| {
                                            evt.prevent_default();
                                            if let Some(payload) = drag() {
                                                let before = {
                                                    let st = engine.read();
                                                    let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                    anim::capture_rects(&ids)
                                                };
                                                let mut eng = engine.write();
                                                match payload.zone {
                                                    FocusZone::Hand => {
                                                        let mut to = slot;
                                                        if payload.index < to { to = to.saturating_sub(1); }
                                                        let _ = eng.dispatch(kardinality::Action::ReorderHand { from: payload.index, to });
                                                        focus.set(FocusZone::Hand);
                                                        sel_hand.set(to.min(eng.state.hand.len().saturating_sub(1)));
                                                    }
                                                    FocusZone::Deck => {
                                                        if let Err(e) = eng.dispatch(kardinality::Action::MoveCollectionToHand { index: payload.index }) {
                                                            eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                                        }
                                                        let from = eng.state.hand.len().saturating_sub(1);
                                                        let to = slot.min(from);
                                                        let _ = eng.dispatch(kardinality::Action::ReorderHand { from, to });
                                                        focus.set(FocusZone::Hand);
                                                        sel_hand.set(to);
                                                    }
                                                    FocusZone::Sidebar => {}
                                                }
                                                anim::play_flip(before, 320.0);
                                            }
                                            drag.set(None);
                                        }
                                    }
                                    if slot < state.hand.len() {
                                        crate::ui::views::CardView {
                                            index: slot,
                                            card: state.hand[slot].clone(),
                                            selected: focus_value == FocusZone::Hand && selected_hand == slot,
                                            primary_icon: "↓",
                                            on_select: move |idx| { focus.set(FocusZone::Hand); sel_hand.set(idx); },
                                            on_primary: move |idx| {
                                            let before = {
                                                let st = engine.read();
                                                let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                anim::capture_rects(&ids)
                                            };
                                            let mut eng = engine.write();
                                            if let Err(e) = eng.dispatch(kardinality::Action::MoveHandToCollection { index: idx }) {
                                                eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                            }
                                            focus.set(FocusZone::Deck);
                                            let new_coll_len = eng.state.collection.len();
                                            if new_coll_len > 0 { sel_collection.set(new_coll_len - 1); }
                                            let new_hand_len = eng.state.hand.len();
                                            sel_hand.set(if new_hand_len == 0 { 0 } else { idx.min(new_hand_len - 1) });
                                            anim::play_flip(before, 280.0);
                                        },
                                        on_move_left: move |idx| {
                                            if idx == 0 { return; }
                                            let before = {
                                                let st = engine.read();
                                                let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                anim::capture_rects(&ids)
                                            };
                                            let mut eng = engine.write();
                                            let _ = eng.dispatch(kardinality::Action::ReorderHand { from: idx, to: idx - 1 });
                                            focus.set(FocusZone::Hand);
                                            sel_hand.set(idx - 1);
                                            anim::play_flip(before, 260.0);
                                        },
                                        on_move_right: move |idx| {
                                            let len = engine.read().state.hand.len();
                                            if idx + 1 >= len { return; }
                                            let before = {
                                                let st = engine.read();
                                                let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                anim::capture_rects(&ids)
                                            };
                                            let mut eng = engine.write();
                                            let _ = eng.dispatch(kardinality::Action::ReorderHand { from: idx, to: idx + 1 });
                                            focus.set(FocusZone::Hand);
                                            sel_hand.set(idx + 1);
                                            anim::play_flip(before, 260.0);
                                        },
                                        on_docs: move |id| {
                                            kardinomicon_target.set(Some(id));
                                            kardinomicon_open.set(true);
                                        },
                                        on_drag_start: move |idx| {
                                            drag.set(Some(DragPayload { zone: FocusZone::Hand, index: idx }));
                                            focus.set(FocusZone::Hand);
                                            sel_hand.set(idx);
                                        },
                                        on_drag_end: move |_| drag.set(None),
                                            on_drop: move |drop_idx| {
                                            if let Some(payload) = drag() {
                                                let before = {
                                                    let st = engine.read();
                                                    let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                    anim::capture_rects(&ids)
                                                };
                                                let mut eng = engine.write();
                                                match payload.zone {
                                                    FocusZone::Hand => {
                                                        let _ = eng.dispatch(kardinality::Action::SwapHand { a: payload.index, b: drop_idx });
                                                        focus.set(FocusZone::Hand);
                                                        sel_hand.set(drop_idx);
                                                    }
                                                    FocusZone::Deck => {
                                                        if let Err(e) = eng.dispatch(kardinality::Action::MoveCollectionToHand { index: payload.index }) {
                                                            eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                                        }
                                                        let from = eng.state.hand.len().saturating_sub(1);
                                                        let to = drop_idx.min(from);
                                                        let _ = eng.dispatch(kardinality::Action::ReorderHand { from, to });
                                                        focus.set(FocusZone::Hand);
                                                        sel_hand.set(to);
                                                    }
                                                    FocusZone::Sidebar => {}
                                                }
                                                anim::play_flip(before, 280.0);
                                            }
                                            drag.set(None);
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Deck at the bottom: this is your owned collection of cards you can put into your hand.
                div { class: "deckbar", "data-testid": "deck-zone",
                    div { class: "hand-title",
                        span { "Deck" }
                        span { class: "hint", "{collection_count} cards" }
                    }

                    if state.collection.is_empty() {
                        div { class: "row-scroll",
                            div { class: "row center",
                                div {
                                    class: "ghost-card dropzone",
                                    ondragover: move |evt| evt.prevent_default(),
                                    ondrop: move |evt| {
                                        evt.prevent_default();
                                        if let Some(payload) = drag() {
                                            let before = {
                                                let st = engine.read();
                                                let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                anim::capture_rects(&ids)
                                            };
                                            let mut eng = engine.write();
                                            if payload.zone == FocusZone::Hand {
                                                if let Err(e) = eng.dispatch(kardinality::Action::MoveHandToCollection { index: payload.index }) {
                                                    eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                                }
                                                focus.set(FocusZone::Deck);
                                                sel_collection.set(eng.state.collection.len().saturating_sub(1));
                                            }
                                            anim::play_flip(before, 280.0);
                                        }
                                        drag.set(None);
                                    },
                                    div { class: "ghost-plus", "+" }
                                    div { class: "ghost-hint", "Drop to add" }
                                }
                            }
                        }
                    } else {
                        div {
                            class: "row-scroll",
                            ondragover: move |evt| evt.prevent_default(),
                            ondrop: move |evt| {
                                evt.prevent_default();
                                if let Some(payload) = drag() {
                                    let before = {
                                        let st = engine.read();
                                        let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                        anim::capture_rects(&ids)
                                    };
                                    let mut eng = engine.write();
                                    match payload.zone {
                                        FocusZone::Deck => {
                                            let len = eng.state.collection.len();
                                            if len > 0 {
                                                let to = len - 1;
                                                let _ = eng.dispatch(kardinality::Action::ReorderCollection { from: payload.index, to });
                                                focus.set(FocusZone::Deck);
                                                sel_collection.set(to);
                                            }
                                        }
                                        FocusZone::Hand => {
                                            if let Err(e) = eng.dispatch(kardinality::Action::MoveHandToCollection { index: payload.index }) {
                                                eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                            }
                                            focus.set(FocusZone::Deck);
                                            sel_collection.set(eng.state.collection.len().saturating_sub(1));
                                        }
                                        FocusZone::Sidebar => {}
                                    }
                                    anim::play_flip(before, 280.0);
                                }
                                drag.set(None);
                            },
                            div { class: "row",
                                for slot in 0..=state.collection.len() {
                                    div {
                                        key: "deck-slot-{slot}",
                                        class: "drop-slot",
                                        ondragover: move |evt| evt.prevent_default(),
                                        ondrop: move |evt| {
                                            evt.prevent_default();
                                            if let Some(payload) = drag() {
                                                let before = {
                                                    let st = engine.read();
                                                    let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                    anim::capture_rects(&ids)
                                                };
                                                let mut eng = engine.write();
                                                match payload.zone {
                                                    FocusZone::Deck => {
                                                        let mut to = slot;
                                                        if payload.index < to { to = to.saturating_sub(1); }
                                                        let _ = eng.dispatch(kardinality::Action::ReorderCollection { from: payload.index, to });
                                                        focus.set(FocusZone::Deck);
                                                        sel_collection.set(to.min(eng.state.collection.len().saturating_sub(1)));
                                                    }
                                                    FocusZone::Hand => {
                                                        if let Err(e) = eng.dispatch(kardinality::Action::MoveHandToCollection { index: payload.index }) {
                                                            eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                                        }
                                                        let from = eng.state.collection.len().saturating_sub(1);
                                                        let to = slot.min(from);
                                                        let _ = eng.dispatch(kardinality::Action::ReorderCollection { from, to });
                                                        focus.set(FocusZone::Deck);
                                                        sel_collection.set(to);
                                                    }
                                                    FocusZone::Sidebar => {}
                                                }
                                                anim::play_flip(before, 320.0);
                                            }
                                            drag.set(None);
                                        }
                                    }
                                    if slot < state.collection.len() {
                                        crate::ui::views::CardView {
                                            index: slot,
                                            card: state.collection[slot].clone(),
                                            selected: focus_value == FocusZone::Deck && selected_collection == slot,
                                            primary_icon: "↑",
                                            on_select: move |idx| { focus.set(FocusZone::Deck); sel_collection.set(idx); },
                                            on_primary: move |idx| {
                                            let before = {
                                                let st = engine.read();
                                                let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                anim::capture_rects(&ids)
                                            };
                                            let mut eng = engine.write();
                                            if let Err(e) = eng.dispatch(kardinality::Action::MoveCollectionToHand { index: idx }) {
                                                eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                            }
                                            focus.set(FocusZone::Hand);
                                            let new_hand_len = eng.state.hand.len();
                                            if new_hand_len > 0 { sel_hand.set(new_hand_len - 1); }
                                            let new_coll_len = eng.state.collection.len();
                                            sel_collection.set(if new_coll_len == 0 { 0 } else { idx.min(new_coll_len - 1) });
                                            anim::play_flip(before, 280.0);
                                        },
                                        on_move_left: move |idx| {
                                            if idx == 0 { return; }
                                            let before = {
                                                let st = engine.read();
                                                let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                anim::capture_rects(&ids)
                                            };
                                            let mut eng = engine.write();
                                            let _ = eng.dispatch(kardinality::Action::ReorderCollection { from: idx, to: idx - 1 });
                                            focus.set(FocusZone::Deck);
                                            sel_collection.set(idx - 1);
                                            anim::play_flip(before, 260.0);
                                        },
                                        on_move_right: move |idx| {
                                            let len = engine.read().state.collection.len();
                                            if idx + 1 >= len { return; }
                                            let before = {
                                                let st = engine.read();
                                                let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                anim::capture_rects(&ids)
                                            };
                                            let mut eng = engine.write();
                                            let _ = eng.dispatch(kardinality::Action::ReorderCollection { from: idx, to: idx + 1 });
                                            focus.set(FocusZone::Deck);
                                            sel_collection.set(idx + 1);
                                            anim::play_flip(before, 260.0);
                                        },
                                        on_docs: move |id| {
                                            kardinomicon_target.set(Some(id));
                                            kardinomicon_open.set(true);
                                        },
                                        on_drag_start: move |idx| {
                                            drag.set(Some(DragPayload { zone: FocusZone::Deck, index: idx }));
                                            focus.set(FocusZone::Deck);
                                            sel_collection.set(idx);
                                        },
                                        on_drag_end: move |_| drag.set(None),
                                            on_drop: move |drop_idx| {
                                            if let Some(payload) = drag() {
                                                let before = {
                                                    let st = engine.read();
                                                    let ids = anim::visible_card_ids(&st.state.collection, &st.state.hand);
                                                    anim::capture_rects(&ids)
                                                };
                                                let mut eng = engine.write();
                                                match payload.zone {
                                                    FocusZone::Deck => {
                                                        let _ = eng.dispatch(kardinality::Action::SwapCollection { a: payload.index, b: drop_idx });
                                                        focus.set(FocusZone::Deck);
                                                        sel_collection.set(drop_idx);
                                                    }
                                                    FocusZone::Hand => {
                                                        if let Err(e) = eng.dispatch(kardinality::Action::MoveHandToCollection { index: payload.index }) {
                                                            eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                                                        }
                                                        let from = eng.state.collection.len().saturating_sub(1);
                                                        let to = drop_idx.min(from);
                                                        let _ = eng.dispatch(kardinality::Action::ReorderCollection { from, to });
                                                        focus.set(FocusZone::Deck);
                                                        sel_collection.set(to);
                                                    }
                                                    FocusZone::Sidebar => {}
                                                }
                                                anim::play_flip(before, 280.0);
                                            }
                                            drag.set(None);
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            KardinomiconModal {
                open: kardinomicon_open(),
                target: kardinomicon_target(),
                on_close: move |_| {
                    kardinomicon_open.set(false);
                    kardinomicon_target.set(None);
                }
            }
        }
    }
}


