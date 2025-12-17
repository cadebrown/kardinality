use dioxus::prelude::*;

use crate::ui::state::UiSettings;
use crate::ui::theme;
use crate::ui::views::{DeckWidget, KardinomiconModal, PileWidget, RegistersBody, Sidebar, SidebarTab};
use crate::ui::anim;
use std::rc::Rc;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq)]
struct FxCard {
    id: u64,
    def_id: String,
    left: f64,
    top: f64,
    width: f64,
    height: f64,
    tx: f64,
    ty: f64,
    scale: f64,
    opacity: f64,
    executing: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct FxBump {
    id: u64,
    x: f64,
    y: f64,
    text: String,
    class: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
struct FxProj {
    id: u64,
    x: f64,
    y: f64,
    tx: f64,
    ty: f64,
    text: String,
    class: &'static str,
    armed: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct FxBurst {
    id: u64,
    x: f64,
    y: f64,
    class: &'static str,
}

#[component]
fn FxOverlayCard(card: FxCard) -> Element {
    let def = kardinality::game::cards::get(&card.def_id);
    let (icon, name) = def.map(|d| (d.icon, d.name)).unwrap_or(("?", "???"));

    let cls = if card.executing {
        "card fx-card executing"
    } else {
        "card fx-card"
    };

    let style = format!(
        "left: {}px; top: {}px; width: {}px; height: {}px; opacity: {}; --fx-tx: {}px; --fx-ty: {}px; --fx-scale: {};",
        card.left, card.top, card.width, card.height, card.opacity, card.tx, card.ty, card.scale
    );

    rsx! {
        div { class: "{cls}", style: "{style}",
            div { class: "card-badge", "⚡" }
            div { class: "card-art" }
            div { class: "card-body",
                h3 { class: "card-title", "{icon} {name}" }
                div { class: "card-sub", "Executing…" }
            }
        }
    }
}

#[component]
#[component]
fn FxOverlayBump(bump: FxBump) -> Element {
    rsx! {
        div { class: "fx-bump {bump.class}", style: "left: {bump.x}px; top: {bump.y}px;", "{bump.text}" }
    }
}

#[component]
fn FxOverlayProj(proj: FxProj) -> Element {
    let style = format!(
        "left: {}px; top: {}px; transform: translate(-50%, -50%) translate({}px, {}px); opacity: {};",
        proj.x,
        proj.y,
        if proj.armed { proj.tx } else { 0.0 },
        if proj.armed { proj.ty } else { 0.0 },
        if proj.armed { 0.20 } else { 1.0 }
    );

    rsx! { div { class: "fx-proj {proj.class}", style: "{style}", "{proj.text}" } }
}

#[component]
fn FxOverlayBurst(burst: FxBurst) -> Element {
    // 12 sparks in a circle.
    let angles: [i32; 12] = [0, 30, 60, 90, 120, 150, 180, 210, 240, 270, 300, 330];
    let sparks: Vec<(i32, i32)> = angles
        .into_iter()
        .enumerate()
        .map(|(i, a)| {
            let dist = 24 + (i as i32 % 6) * 6;
            (a, dist)
        })
        .collect();
    let style = format!("--x: {}px; --y: {}px;", burst.x, burst.y);
    rsx! {
        div { class: "fx-burst {burst.class}", style: "{style}",
            for (a, dist) in sparks {
                span { style: "--a: {a}deg; --d: {dist}px;" }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusZone {
    Sidebar,
    Deck,
    Hand,
    Play,
    Shop,
}

#[derive(Debug, Clone, PartialEq)]
struct PtrDrag {
    zone: FocusZone,
    index: usize,
    card: kardinality::game::CardInstance,
    // The card's original top-left in viewport coordinates (computed from pointerdown).
    origin_left: f64,
    origin_top: f64,
    // Pointer offset within the card at pointerdown (element coords).
    off_x: f64,
    off_y: f64,
    // Current pointer position (client coords).
    client_x: f64,
    client_y: f64,
    moved: bool,
}

#[component]
pub fn App() -> Element {
    let mut engine = use_signal(|| kardinality::Engine::new(0));
    let tab = use_signal(|| SidebarTab::Controls);

    let settings = use_signal(UiSettings::default);
    let mut kardinomicon_open = use_signal(|| false);
    let mut kardinomicon_target = use_signal(|| None::<String>);

    // Keyboard-first selection state.
    let mut focus = use_signal(|| FocusZone::Deck);
    let mut sel_collection = use_signal(|| 0usize);
    let mut sel_hand = use_signal(|| 0usize);
    let mut drag = use_signal(|| None::<PtrDrag>);
    let mut sidebar_index = use_signal(|| 0usize);

    // Playback / animated execution state (UI-only).
    let pb_active = use_signal(|| false);
    let pb_cards = use_signal(|| Vec::<FxCard>::new());
    let pb_bumps = use_signal(|| Vec::<FxBump>::new());
    let pb_projs = use_signal(|| Vec::<FxProj>::new());
    let pb_bursts = use_signal(|| Vec::<FxBurst>::new());
    let pb_score = use_signal(|| 0i64);
    let pb_bank = use_signal(|| 0i64);
    let pb_len_deck = use_signal(|| 0i64);
    let pb_len_source = use_signal(|| 0i64);
    let pb_len_pile = use_signal(|| 0i64);
    let pb_pile_recent = use_signal(|| Vec::<String>::new());
    let pb_deck_shake = use_signal(|| false);
    let pb_call = use_signal(|| None::<(f64, f64, String)>); // x,y,text
    let pb_view_hand = use_signal(|| Vec::<kardinality::game::CardInstance>::new());
    let pb_view_deck = use_signal(|| Vec::<kardinality::game::CardInstance>::new());

    let engine_read = engine.read();
    let state = &engine_read.state;

    let deck_count = state.deck.len();
    let _pile_count = state.pile.len();
    let collection_count = state.collection.len();
    let hand_count = state.hand.len();

    let focus_value = focus();
    let selected_collection = sel_collection();
    let selected_hand = sel_hand();
    let drag_value = drag();
    let _dragging_id = drag_value.as_ref().map(|d| d.card.id);

    let display_score = if pb_active() { pb_score() } else { state.score };
    let display_bank = if pb_active() { pb_bank() } else { state.bankroll };
    let display_collection_count = if pb_active() { pb_len_deck().max(0) as usize } else { collection_count };
    let display_hand_count = if pb_active() { pb_view_hand().len() } else { hand_count };
    let display_source_count = if pb_active() { pb_len_source().max(0) as usize } else { deck_count };
    let display_pile_count =
        if pb_active() { pb_len_pile().max(0) as usize } else { state.pile.len() };
    let display_pile_recent = if pb_active() {
        pb_pile_recent()
    } else {
        state
            .pile
            .iter()
            .rev()
            .take(12)
            .filter_map(|c| c.def().map(|d| d.name.to_string()))
            .collect::<Vec<_>>()
    };

    let settings_value = settings();
    let mut app_class = if settings_value.effects {
        format!("app {}", settings_value.theme.class())
    } else {
        format!("app {} effects-off", settings_value.theme.class())
    };
    if drag_value.is_some() {
        app_class.push_str(" is-dragging");
    }

    let start_playback: Rc<dyn Fn()> = {
        let engine = engine.clone();
        let focus = focus.clone();
        let pb_active = pb_active.clone();
        let pb_cards = pb_cards.clone();
        let pb_bumps = pb_bumps.clone();
        let pb_projs = pb_projs.clone();
        let pb_bursts = pb_bursts.clone();
        let pb_score = pb_score.clone();
        let pb_bank = pb_bank.clone();
        let pb_len_deck = pb_len_deck.clone();
        let pb_len_source = pb_len_source.clone();
        let pb_len_pile = pb_len_pile.clone();
        let pb_pile_recent = pb_pile_recent.clone();
        let pb_deck_shake = pb_deck_shake.clone();
        let pb_call = pb_call.clone();
        let pb_view_hand = pb_view_hand.clone();
        let pb_view_deck = pb_view_deck.clone();

        Rc::new(move || {
            let mut engine = engine.clone();
            let mut focus = focus.clone();
            let mut pb_active = pb_active.clone();
            let mut pb_cards = pb_cards.clone();
            let mut pb_bumps = pb_bumps.clone();
            let mut pb_projs = pb_projs.clone();
            let mut pb_bursts = pb_bursts.clone();
            let mut pb_score = pb_score.clone();
            let mut pb_bank = pb_bank.clone();
            let mut pb_len_deck = pb_len_deck.clone();
            let mut pb_len_source = pb_len_source.clone();
            let mut pb_len_pile = pb_len_pile.clone();
            let mut pb_pile_recent = pb_pile_recent.clone();
            let mut pb_deck_shake = pb_deck_shake.clone();
            let mut pb_call = pb_call.clone();
            let mut pb_view_hand = pb_view_hand.clone();
            let mut pb_view_deck = pb_view_deck.clone();

            if pb_active() {
                return;
            }

            // Snapshot pre-state (and drop the borrow before we mutate the engine).
            let (pre_hand, pre_score, pre_bank, pre_len_deck, pre_len_source, pre_len_pile, pre_trace_len) = {
                let pre = engine.read();
                (
                    pre.state.hand.clone(),
                    pre.state.score,
                    pre.state.bankroll,
                    pre.state.collection.len() as i64,
                    pre.state.deck.len() as i64,
                    pre.state.pile.len() as i64,
                    pre.state.trace.len(),
                )
            };
            if pre_hand.is_empty() {
                return;
            }

            // Freeze hand/deck rendering during playback so cards don't jump instantly.
            let pre_collection = engine.read().state.collection.clone();
            pb_view_hand.set(pre_hand.clone());
            pb_view_deck.set(pre_collection);

            // Build overlay cards from DOM rects (hand positions).
            let ids: Vec<String> = pre_hand.iter().map(|c| format!("card-{}", c.id)).collect();
            let rects = anim::capture_rects(&ids);
            let mut overlay: Vec<FxCard> = Vec::new();
            for c in &pre_hand {
                let id = format!("card-{}", c.id);
                if let Some(r) = rects.get(&id) {
                    overlay.push(FxCard {
                        id: c.id,
                        def_id: c.def_id.clone(),
                        left: r.left,
                        top: r.top,
                        width: r.width,
                        height: r.height,
                        tx: 0.0,
                        ty: 0.0,
                        scale: 1.0,
                        opacity: 1.0,
                        executing: false,
                    });
                }
            }

            pb_score.set(pre_score);
            pb_bank.set(pre_bank);
            pb_len_deck.set(pre_len_deck);
            pb_len_source.set(pre_len_source);
            pb_len_pile.set(pre_len_pile);
            pb_pile_recent.set(Vec::new());
            pb_bumps.set(Vec::new());
            pb_projs.set(Vec::new());
            pb_bursts.set(Vec::new());
            pb_call.set(None);
            pb_deck_shake.set(false);
            pb_cards.set(overlay);
            pb_active.set(true);
            focus.set(FocusZone::Play);

            // Execute immediately to obtain trace + end state, but animate it in the UI.
            {
                let mut eng = engine.write();
                if let Err(e) = eng.dispatch(kardinality::Action::PlayHand) {
                    eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                }
            }

            let post_events: Vec<kardinality::TraceEvent> = engine
                .read()
                .state
                .trace
                .iter()
                .skip(pre_trace_len)
                .cloned()
                .collect();

            // Run playback asynchronously.
            let engine2 = engine.clone();
            let pb_cards2 = pb_cards.clone();
            let pb_bumps2 = pb_bumps.clone();
            let pb_projs2 = pb_projs.clone();
            let pb_bursts2 = pb_bursts.clone();
            let pb_score2 = pb_score.clone();
            let pb_bank2 = pb_bank.clone();
            let pb_len_deck2 = pb_len_deck.clone();
            let pb_len_source2 = pb_len_source.clone();
            let pb_len_pile2 = pb_len_pile.clone();
            let pb_pile_recent2 = pb_pile_recent.clone();
            let pb_active2 = pb_active.clone();
            let pb_deck_shake2 = pb_deck_shake.clone();
            let pb_call2 = pb_call.clone();
            let pb_view_hand2 = pb_view_hand.clone();
            let pb_view_deck2 = pb_view_deck.clone();

            spawn(async move {
                let mut pb_cards2 = pb_cards2;
                let mut pb_bumps2 = pb_bumps2;
                let mut pb_projs2 = pb_projs2;
                let mut pb_bursts2 = pb_bursts2;
                let mut pb_score2 = pb_score2;
                let mut pb_bank2 = pb_bank2;
                let mut pb_len_deck2 = pb_len_deck2;
                let mut pb_len_source2 = pb_len_source2;
                let mut pb_len_pile2 = pb_len_pile2;
                let mut pb_pile_recent2 = pb_pile_recent2;
                let mut pb_active2 = pb_active2;
                let mut pb_deck_shake2 = pb_deck_shake2;
                let mut pb_call2 = pb_call2;
                let mut pb_view_hand2 = pb_view_hand2;
                let mut pb_view_deck2 = pb_view_deck2;

                // Where cards should land after execution: center on the Pile widget.
                let pile_rect = anim::rect_for_testid("pile-zone");
                let (pile_x, pile_y) = if let Some(r) = pile_rect {
                    (r.left + r.width * 0.55, r.top + r.height * 0.35)
                } else {
                    (980.0, 520.0)
                };
                // Legacy execute point (we execute "in place" now).
                let (_exec_x, _exec_y) = (pile_x, pile_y - 70.0);

                // Determine which cards were drawn into the Deck during this playback so we can pop-animate them.
                let final_collection = engine2.read().state.collection.clone();
                let pre_ids: HashSet<u64> = pb_view_deck2.read().iter().map(|c| c.id).collect();
                let mut drawn_queue: VecDeque<kardinality::game::CardInstance> = final_collection
                    .into_iter()
                    .filter(|c| !pre_ids.contains(&c.id))
                    .collect();

                // Build a queue of overlay ids in hand order.
                let mut queue: VecDeque<u64> = VecDeque::new();
                for c in &pre_hand {
                    queue.push_back(c.id);
                }

                let mut current: Option<u64> = None;
                let mut executed_count: usize = 0;

                // Helper: spawn a bump near a testid target.
                let mut next_bump_id: u64 = 1;
                let mut push_bump = |testid: &str, text: String, class: &'static str| {
                    if let Some(r) = anim::rect_for_testid(testid) {
                        let x = r.left + r.width * 0.50;
                        let y = r.top + r.height * 0.50;
                        let bump = FxBump {
                            id: next_bump_id,
                            x,
                            y,
                            text,
                            class,
                        };
                        next_bump_id = next_bump_id.saturating_add(1);
                        pb_bumps2.write().push(bump.clone());

                        let mut pb_bumps3 = pb_bumps2.clone();
                        spawn(async move {
                            anim::sleep_ms(920).await;
                            pb_bumps3.write().retain(|b| b.id != bump.id);
                        });
                    }
                };

                // Helper: projectile from the executing card to a UI target, with sparks on impact.
                let mut next_proj_id: u64 = 1;
                let mut launch_proj = |from_x: f64, from_y: f64, to_testid: &str, text: String, class: &'static str| {
                    let Some(r) = anim::rect_for_testid(to_testid) else { return };
                    let to_x = r.left + r.width * 0.50;
                    let to_y = r.top + r.height * 0.50;
                    let proj = FxProj {
                        id: next_proj_id,
                        x: from_x,
                        y: from_y,
                        tx: to_x - from_x,
                        ty: to_y - from_y,
                        text,
                        class,
                        armed: false,
                    };
                    next_proj_id = next_proj_id.saturating_add(1);
                    pb_projs2.write().push(proj.clone());

                    let mut pb_projs3 = pb_projs2.clone();
                    let mut pb_bursts3 = pb_bursts2.clone();
                    spawn(async move {
                        anim::sleep_ms(16).await;
                        {
                            let mut v = pb_projs3.write();
                            if let Some(p) = v.iter_mut().find(|p| p.id == proj.id) {
                                p.armed = true;
                            }
                        }
                        anim::sleep_ms(560).await;
                        let burst = FxBurst {
                            id: proj.id,
                            x: from_x + proj.tx,
                            y: from_y + proj.ty,
                            class,
                        };
                        pb_bursts3.write().push(burst.clone());
                        anim::sleep_ms(520).await;
                        pb_bursts3.write().retain(|b| b.id != burst.id);
                        pb_projs3.write().retain(|p| p.id != proj.id);
                    });
                };

                for evt in post_events {
                    match evt {
                        kardinality::TraceEvent::CardStart { .. } => {
                            // Pop next overlay card from the queue and focus it in place.
                            let Some(id) = queue.pop_front() else { continue };
                            current = Some(id);

                            // Hide the real card so layout stays stable but only the overlay is visible.
                            anim::set_opacity_for_id(&format!("card-{id}"), 0.0);

                            let mut cards = pb_cards2.write();
                            if let Some(card) = cards.iter_mut().find(|c| c.id == id) {
                                card.tx = 0.0;
                                card.ty = -18.0;
                                card.scale = 1.06;
                                card.executing = true;
                            }
                            drop(cards);
                            anim::sleep_ms(180).await;
                        }
                        kardinality::TraceEvent::Call { name, args } => {
                            if let Some(id) = current {
                                // Anchor to the overlay card (the real DOM card may not exist during playback).
                                let cards = pb_cards2.read();
                                if let Some(c) = cards.iter().find(|c| c.id == id) {
                                    let x = c.left + c.width * 0.5 + c.tx;
                                    let y = c.top + c.height * 0.15 + c.ty;
                                    pb_call2.set(Some((x, y, format!("{name}({})", args.join(", ")))));
                                    anim::sleep_ms(260).await;
                                    pb_call2.set(None);
                                }
                            }
                        }
                        kardinality::TraceEvent::EffectApplied { effect } => {
                            let (from_x, from_y) = if let Some(id) = current {
                                let cards = pb_cards2.read();
                                if let Some(c) = cards.iter().find(|c| c.id == id) {
                                    (
                                        c.left + c.width * 0.5 + c.tx,
                                        c.top + c.height * 0.5 + c.ty,
                                    )
                                } else {
                                    (pile_x, pile_y)
                                }
                            } else {
                                (pile_x, pile_y)
                            };
                            match effect {
                                kardinality::vm::Effect::AddScore(n) => {
                                    let cls = if n >= 0 { "pos" } else { "neg" };
                                    launch_proj(from_x, from_y, "score-value", format!("{:+}", n), cls);
                                    anim::sleep_ms(620).await;
                                    push_bump("score-value", format!("{:+}", n), cls);
                                    *pb_score2.write() += n;
                                }
                                kardinality::vm::Effect::AddBankroll(n) => {
                                    let cls = if n >= 0 { "pos" } else { "neg" };
                                    launch_proj(from_x, from_y, "money-value", format!("${:+}", n), cls);
                                    anim::sleep_ms(620).await;
                                    push_bump("money-value", format!("${:+}", n), cls);
                                    *pb_bank2.write() += n;
                                }
                                kardinality::vm::Effect::MulBankroll(n) => {
                                    launch_proj(from_x, from_y, "money-value", format!("×{n}"), "mul");
                                    anim::sleep_ms(620).await;
                                    push_bump("money-value", format!("×{n}"), "mul");
                                    *pb_bank2.write() *= n;
                                }
                                kardinality::vm::Effect::Draw(n) => {
                                    launch_proj(from_x, from_y, "deck-count", format!("draw(+{n})"), "info");
                                    pb_deck_shake2.set(true);
                                    let mut pb_deck_shake3 = pb_deck_shake2.clone();
                                    spawn(async move {
                                        anim::sleep_ms(520).await;
                                        pb_deck_shake3.set(false);
                                    });
                                    // Pop in the newly drawn cards into the Deck row.
                                    let want = n.clamp(0, 25);
                                    let mut added: i64 = 0;
                                    for _ in 0..want {
                                        let Some(c) = drawn_queue.pop_front() else { break };
                                        let id = c.id;
                                        pb_view_deck2.write().push(c);
                                        added += 1;
                                        // After the DOM updates, pop-in animate this new card.
                                        let card_dom_id = format!("card-{id}");
                                        spawn(async move {
                                            anim::sleep_ms(32).await;
                                            anim::add_temp_class_for_id(&card_dom_id, "pop-in", 650);
                                        });
                                    }
                                    anim::sleep_ms(520).await;
                                    if added > 0 {
                                        push_bump("deck-count", format!("+{added}"), "pos");
                                        *pb_len_deck2.write() += added;
                                        *pb_len_source2.write() -= added;
                                    } else {
                                        push_bump("deck-count", "+0".to_string(), "info");
                                    }
                                }
                                kardinality::vm::Effect::SetAcc(v) => {
                                    launch_proj(from_x, from_y, "deck-zone", format!("acc={v}"), "info");
                                    anim::sleep_ms(620).await;
                                    push_bump("deck-zone", format!("acc={v}"), "info");
                                }
                                kardinality::vm::Effect::Clone(n) => {
                                    launch_proj(from_x, from_y, "deck-count", format!("clone({n})"), "info");
                                    anim::sleep_ms(620).await;
                                    push_bump("deck-count", format!("+{n}"), "pos");
                                }
                                kardinality::vm::Effect::Again(n) => {
                                    launch_proj(from_x, from_y, "deck-zone", format!("again({n})"), "info");
                                    anim::sleep_ms(620).await;
                                    push_bump("deck-zone", format!("again({n})"), "info");
                                }
                                kardinality::vm::Effect::Mutate => {
                                    launch_proj(from_x, from_y, "deck-zone", "mutate()".to_string(), "info");
                                    anim::sleep_ms(620).await;
                                    push_bump("deck-zone", "mutate()".to_string(), "info");
                                }
                            }
                        }
                        kardinality::TraceEvent::CardEnd { .. } => {
                            let Some(id) = current.take() else { continue };

                            // Smoothly fly to the Pile, then disappear there.
                            let mut cards = pb_cards2.write();
                            if let Some(card) = cards.iter_mut().find(|c| c.id == id) {
                                let cx = card.left + card.width * 0.5;
                                let cy = card.top + card.height * 0.5;
                                let stack_i = executed_count as f64;
                                let stack_x = pile_x + stack_i * 3.0;
                                let stack_y = pile_y + stack_i * 2.0;
                                card.tx = stack_x - cx;
                                card.ty = stack_y - cy;
                                card.scale = 0.90;
                                card.executing = false;
                                card.opacity = 0.92;
                            }
                            drop(cards);
                            anim::sleep_ms(420).await;
                            {
                                let mut cards = pb_cards2.write();
                                if let Some(card) = cards.iter_mut().find(|c| c.id == id) {
                                    card.opacity = 0.0;
                                    card.scale = 0.86;
                                }
                            }
                            anim::sleep_ms(180).await;
                            pb_cards2.write().retain(|c| c.id != id);
                            executed_count = executed_count.saturating_add(1);
                            *pb_len_pile2.write() += 1;

                            // Remove this card from the frozen Hand list so it disappears from the row.
                            pb_view_hand2.write().retain(|c| c.id != id);

                            // Update Pile recent list (most recent first).
                            if let Some(card) = pre_hand.iter().find(|c| c.id == id) {
                                if let Some(def) = kardinality::game::cards::get(&card.def_id) {
                                    let mut v = pb_pile_recent2.write();
                                    v.insert(0, def.name.to_string());
                                    v.truncate(12);
                                }
                            }
                            // Restore opacity if the DOM element still exists (it might not).
                            anim::set_opacity_for_id(&format!("card-{id}"), 1.0);
                        }
                        _ => {}
                    }
                }

                // End playback.
                pb_call2.set(None);
                pb_deck_shake2.set(false);
                // Fade overlays out, then end playback.
                {
                    for c in pb_cards2.write().iter_mut() {
                        c.opacity = 0.0;
                        c.scale = 0.96;
                    }
                }
                anim::sleep_ms(220).await;
                pb_cards2.set(Vec::new());
                pb_bumps2.set(Vec::new());
                pb_projs2.set(Vec::new());
                pb_bursts2.set(Vec::new());
                pb_active2.set(false);

                // Ensure HUD lands on real state.
                let st = engine2.read();
                pb_score2.set(st.state.score);
                pb_bank2.set(st.state.bankroll);
                pb_len_deck2.set(st.state.collection.len() as i64);
                pb_len_source2.set(st.state.deck.len() as i64);
                pb_len_pile2.set(st.state.pile.len() as i64);
                pb_pile_recent2.set(Vec::new());

                pb_view_hand2.set(Vec::new());
                pb_view_deck2.set(Vec::new());
            });
        })
    };

    let start_playback_kb = start_playback.clone();
    let start_playback_btn = start_playback.clone();

    // Pointer-drag: we move the actual card element via CSS transform (no ghost overlay).

    // When playback is active, freeze what the user sees for hand/deck to avoid jumpy re-layout.
    let view_hand = if pb_active() {
        pb_view_hand()
    } else {
        state.hand.clone()
    };
    let view_collection = if pb_active() {
        pb_view_deck()
    } else {
        state.collection.clone()
    };

    // Precompute drag styles outside `rsx!` (the macro doesn't like `let` inside loops).
    let hand_ui: Vec<(kardinality::game::CardInstance, bool, String)> = view_hand
        .iter()
        .cloned()
        .map(|card| {
            if let Some(d) = drag_value.as_ref() {
                if d.card.id == card.id && d.moved {
                    let tx = d.client_x - d.off_x - d.origin_left;
                    let ty = d.client_y - d.off_y - d.origin_top;
                    return (card, true, format!("--drag-tx:{tx}px; --drag-ty:{ty}px;"));
                }
            }
            (card, false, String::new())
        })
        .collect();

    let deck_ui: Vec<(kardinality::game::CardInstance, bool, String)> = view_collection
        .iter()
        .cloned()
        .map(|card| {
            if let Some(d) = drag_value.as_ref() {
                if d.card.id == card.id && d.moved {
                    let tx = d.client_x - d.off_x - d.origin_left;
                    let ty = d.client_y - d.off_y - d.origin_top;
                    return (card, true, format!("--drag-tx:{tx}px; --drag-ty:{ty}px;"));
                }
            }
            (card, false, String::new())
        })
        .collect();

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

                // E2E / debug harness: allow pre-filling the Deck with many cards via `?prefill=N`.
                if let Some(v) = anim::query_param("prefill") {
                    if let Ok(n) = v.parse::<usize>() {
                        let mut eng = engine.write();
                        let _ = eng.dispatch(kardinality::Action::DrawToCollection { count: n });
                    }
                }
            },
            onpointermove: move |evt: PointerEvent| {
                let Some(mut d) = drag() else { return };
                if !d.moved {
                    // Don't start a drag until the pointer moves a bit.
                    let p = evt.data().client_coordinates();
                    let dx = (p.x - d.client_x).abs();
                    let dy = (p.y - d.client_y).abs();
                    if dx.max(dy) < 3.0 {
                        return;
                    }
                    d.moved = true;
                }

                let p = evt.data().client_coordinates();
                d.client_x = p.x;
                d.client_y = p.y;
                drag.set(Some(d));
            },
            onpointerup: move |evt: PointerEvent| {
                let Some(d) = drag() else { return };

                // If we never moved far enough, treat it like a click (let normal click handlers run).
                if !d.moved {
                    drag.set(None);
                    return;
                }

                evt.prevent_default();

                // Decide drop target using hit-testing (WASM) or fallbacks.
                let p = evt.data().client_coordinates();
                let hit = anim::hit_test(p.x, p.y);

                let mut eng = engine.write();

                let src_zone = d.zone;
                let src_index = d.index;

                // Determine target zone/index from card_id if possible.
                let mut target_zone: Option<FocusZone> = None;
                let mut target_index: Option<usize> = None;
                if let Some(id) = hit.card_id {
                    if let Some(i) = eng.state.hand.iter().position(|c| c.id == id) {
                        target_zone = Some(FocusZone::Hand);
                        target_index = Some(i);
                    } else if let Some(i) = eng.state.collection.iter().position(|c| c.id == id) {
                        target_zone = Some(FocusZone::Deck);
                        target_index = Some(i);
                    }
                } else if let Some(z) = hit.zone {
                    target_zone = Some(match z {
                        anim::HitZone::Hand => FocusZone::Hand,
                        anim::HitZone::Deck => FocusZone::Deck,
                    });
                }

                let rel = hit.rel_x.unwrap_or(0.5);
                let hint = if rel < 0.28 { 0 } else if rel > 0.72 { 2 } else { 1 }; // 0 before,1 swap,2 after

                match (src_zone, target_zone, target_index) {
                    // Hand -> Hand
                    (FocusZone::Hand, Some(FocusZone::Hand), Some(ti)) => {
                        if hint == 1 {
                            let _ = eng.dispatch(kardinality::Action::SwapHand { a: src_index, b: ti });
                            focus.set(FocusZone::Hand);
                            sel_hand.set(ti);
                        } else {
                            let mut to = ti;
                            if hint == 2 { to = to.saturating_add(1); }
                            if src_index < to { to = to.saturating_sub(1); }
                            let len = eng.state.hand.len();
                            let to = to.min(len.saturating_sub(1));
                            let _ = eng.dispatch(kardinality::Action::ReorderHand { from: src_index, to });
                            focus.set(FocusZone::Hand);
                            sel_hand.set(to);
                        }
                    }
                    // Deck -> Deck
                    (FocusZone::Deck, Some(FocusZone::Deck), Some(ti)) => {
                        if hint == 1 {
                            let _ = eng.dispatch(kardinality::Action::SwapCollection { a: src_index, b: ti });
                            focus.set(FocusZone::Deck);
                            sel_collection.set(ti);
                        } else {
                            let mut to = ti;
                            if hint == 2 { to = to.saturating_add(1); }
                            if src_index < to { to = to.saturating_sub(1); }
                            let len = eng.state.collection.len();
                            let to = to.min(len.saturating_sub(1));
                            let _ = eng.dispatch(kardinality::Action::ReorderCollection { from: src_index, to });
                            focus.set(FocusZone::Deck);
                            sel_collection.set(to);
                        }
                    }
                    // Deck -> Hand
                    (FocusZone::Deck, Some(FocusZone::Hand), maybe_ti) => {
                        if let Err(e) = eng.dispatch(kardinality::Action::MoveCollectionToHand { index: src_index }) {
                            eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                        }
                        let from = eng.state.hand.len().saturating_sub(1);
                        if let Some(ti) = maybe_ti {
                            let mut to = ti.min(from);
                            if hint == 2 { to = (ti.saturating_add(1)).min(from); }
                            let _ = eng.dispatch(kardinality::Action::ReorderHand { from, to });
                            focus.set(FocusZone::Hand);
                            sel_hand.set(to);
                        } else {
                            focus.set(FocusZone::Hand);
                            sel_hand.set(from);
                        }
                    }
                    // Hand -> Deck
                    (FocusZone::Hand, Some(FocusZone::Deck), maybe_ti) => {
                        if let Err(e) = eng.dispatch(kardinality::Action::MoveHandToCollection { index: src_index }) {
                            eng.state.trace.push(kardinality::TraceEvent::Error(e.to_string()));
                        }
                        let from = eng.state.collection.len().saturating_sub(1);
                        if let Some(ti) = maybe_ti {
                            let mut to = ti.min(from);
                            if hint == 2 { to = (ti.saturating_add(1)).min(from); }
                            let _ = eng.dispatch(kardinality::Action::ReorderCollection { from, to });
                            focus.set(FocusZone::Deck);
                            sel_collection.set(to);
                        } else {
                            focus.set(FocusZone::Deck);
                            sel_collection.set(from);
                        }
                    }
                    // Drop on whitespace: append/move-to-end within same zone
                    (FocusZone::Hand, Some(FocusZone::Hand), None) => {
                        let len = eng.state.hand.len();
                        if len > 0 {
                            let to = len - 1;
                            let _ = eng.dispatch(kardinality::Action::ReorderHand { from: src_index, to });
                            focus.set(FocusZone::Hand);
                            sel_hand.set(to);
                        }
                    }
                    (FocusZone::Deck, Some(FocusZone::Deck), None) => {
                        let len = eng.state.collection.len();
                        if len > 0 {
                            let to = len - 1;
                            let _ = eng.dispatch(kardinality::Action::ReorderCollection { from: src_index, to });
                            focus.set(FocusZone::Deck);
                            sel_collection.set(to);
                        }
                    }
                    // If we couldn't hit-test a zone, do nothing.
                    _ => {}
                }

                drag.set(None);
            },
            onkeydown: move |evt: KeyboardEvent| {
                use dioxus::prelude::Key;
                use keyboard_types::Modifiers;

                let key = evt.key();
                let shift = evt.modifiers().contains(Modifiers::SHIFT);

                if pb_active() {
                    // During playback, block input (it's a little cutscene).
                    evt.prevent_default();
                    return;
                }

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
                                let next = clamp(idx.saturating_sub(1), len);
                                sel_collection.set(next);
                                if let Some(card) = engine.read().state.collection.get(next) {
                                    anim::scroll_card_into_view(&format!("card-{}", card.id));
                                }
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
                                let next = clamp(idx.saturating_sub(1), len);
                                sel_hand.set(next);
                                if let Some(card) = engine.read().state.hand.get(next) {
                                    anim::scroll_card_into_view(&format!("card-{}", card.id));
                                }
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
                                let next = clamp(idx + 1, len);
                                sel_collection.set(next);
                                if let Some(card) = engine.read().state.collection.get(next) {
                                    anim::scroll_card_into_view(&format!("card-{}", card.id));
                                }
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
                                let next = clamp(idx + 1, len);
                                sel_hand.set(next);
                                if let Some(card) = engine.read().state.hand.get(next) {
                                    anim::scroll_card_into_view(&format!("card-{}", card.id));
                                }
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

                        if focus() == FocusZone::Play {
                            (start_playback_kb)();
                        }
                        if focus() == FocusZone::Shop {
                            let mut eng = engine.write();
                            eng.state
                                .trace
                                .push(kardinality::TraceEvent::Info("Shop: coming soon".to_string()));
                        }
                    }
                    Key::Tab => {
                        // Cycle focus through Hand -> Deck -> Play -> Shop.
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
                                focus.set(FocusZone::Play);
                            }
                            FocusZone::Play => {
                                focus.set(FocusZone::Shop);
                            }
                            FocusZone::Shop => {
                                focus.set(FocusZone::Hand);
                                let len = engine.read().state.hand.len();
                                sel_hand.set(clamp(sel_hand(), len));
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
                    // Single "Run" pane: Play Hand on top, score/bank below.
                    div { class: "panel run-pane",
                        {
                            let pct = if state.target_score <= 0 {
                                0.0
                            } else {
                                (display_score as f64 / state.target_score as f64).clamp(0.0, 1.0)
                            };
                            let pct100 = (pct * 100.0).clamp(0.0, 100.0);
                            let fill_style = format!("width: {pct100:.2}%");

                            rsx! {
                                div { class: "run-buttons",
                                    button {
                                        class: if focus_value == FocusZone::Play { "play-btn focused play-btn2" } else { "play-btn play-btn2" },
                                        "data-testid": "play",
                                        title: "Play the current hand (Enter)",
                                        onclick: move |_| {
                                            focus.set(FocusZone::Play);
                                            (start_playback_btn)();
                                        },
                                        div { class: "play-head",
                                            span { class: "play-icon", "▶" }
                                            span { class: "play-text", "Play" }
                                        }
                                    }
                                    button {
                                        class: if focus_value == FocusZone::Shop { "play-btn focused shop-btn" } else { "play-btn shop-btn" },
                                        "data-testid": "shop",
                                        title: "Shop (coming soon)",
                                        onclick: move |_| {
                                            focus.set(FocusZone::Shop);
                                            let mut eng = engine.write();
                                            eng.state.trace.push(kardinality::TraceEvent::Info("Shop: coming soon".to_string()));
                                        },
                                        div { class: "play-head",
                                            span { class: "play-icon", "◆" }
                                            span { class: "play-text", "Shop" }
                                        }
                                    }
                                }

                                div { class: "run-strip",
                                    div { class: "strip-item",
                                        span { class: "strip-k", "Level" }
                                        span { class: "strip-v", "data-testid": "level-value", "{state.level}" }
                                    }
                                    div { class: "strip-item",
                                        span { class: "strip-k", "Score" }
                                        span { class: "strip-v", "data-testid": "score-value", "{display_score}/{state.target_score}" }
                                    }
                                    div { class: "strip-item",
                                        span { class: "strip-k", "Money" }
                                        span { class: "strip-v", "data-testid": "money-value", "${display_bank}" }
                                    }
                                }

                                div { class: "run-progress",
                                    div { class: "run-progress-fill", style: "{fill_style}" }
                                }
                            }
                        }
                    }

                    div { class: "panel hud-panel",
                        RegistersBody {
                            collection_count: display_collection_count,
                            hand_count: display_hand_count,
                        }
                    }

                    div { class: if pb_deck_shake() { "panel deck-widget deck-shake" } else { "panel deck-widget" },
                        DeckWidget {
                            deck_count: display_source_count,
                            collection_count: display_collection_count,
                            level: state.level,
                        }
                    }
                }

                div { class: "content",
                div { class: "handrow",
                div { class: "handbar", "data-testid": "hand-zone",
                    div { class: "hand-title",
                        span { "Hand" }
                        span { class: "hint", "{display_hand_count} cards" }
                    }

                    div { class: "row-scroll",
                        div { class: "row",
                            if view_hand.is_empty() {
                                div {
                                    class: "ghost-card dropzone",
                                    "data-testid": "hand-dropzone",
                                    div { class: "ghost-plus", "+" }
                                    div { class: "ghost-hint", "Drop to add" }
                                }
                            }
                            for (idx, item) in hand_ui.iter().enumerate() {
                                crate::ui::views::CardView {
                                    key: "hand-{item.0.id}",
                                    index: idx,
                                    card: item.0.clone(),
                                    selected: focus_value == FocusZone::Hand && selected_hand == idx,
                                    dragging: item.1,
                                    drag_style: item.2.clone(),
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
                                        let mut eng = engine.write();
                                        let _ = eng.dispatch(kardinality::Action::ReorderHand { from: idx, to: idx - 1 });
                                        focus.set(FocusZone::Hand);
                                        sel_hand.set(idx - 1);
                                    },
                                    on_move_right: move |idx| {
                                        let len = engine.read().state.hand.len();
                                        if idx + 1 >= len { return; }
                                        let mut eng = engine.write();
                                        let _ = eng.dispatch(kardinality::Action::ReorderHand { from: idx, to: idx + 1 });
                                        focus.set(FocusZone::Hand);
                                        sel_hand.set(idx + 1);
                                    },
                                    on_docs: move |id| {
                                        kardinomicon_target.set(Some(id));
                                        kardinomicon_open.set(true);
                                    },
                                    on_ptr_down: move |pd: crate::ui::views::PtrDown| {
                                        if let Some(card) = engine.read().state.hand.get(pd.index).cloned() {
                                            drag.set(Some(PtrDrag {
                                                zone: FocusZone::Hand,
                                                index: pd.index,
                                                card,
                                                origin_left: pd.client_x - pd.elem_x,
                                                origin_top: pd.client_y - pd.elem_y,
                                                off_x: pd.elem_x,
                                                off_y: pd.elem_y,
                                                client_x: pd.client_x,
                                                client_y: pd.client_y,
                                                moved: false,
                                            }));
                                        }
                                        focus.set(FocusZone::Hand);
                                        sel_hand.set(pd.index);
                                    },
                                }
                            }
                        }
                    }
                }
                PileWidget { count: display_pile_count, recent: display_pile_recent }
                }

                // Deck at the bottom: this is your owned collection of cards you can put into your hand.
                div { class: "deckbar", "data-testid": "deck-zone",
                    div { class: "hand-title",
                        span { "Deck" }
                        span { class: "hint", "{display_collection_count} cards" }
                    }

                    div { class: "row-scroll",
                        div { class: "row",
                            if view_collection.is_empty() {
                                div {
                                    class: "ghost-card dropzone",
                                    "data-testid": "deck-dropzone",
                                    div { class: "ghost-plus", "+" }
                                    div { class: "ghost-hint", "Drop to add" }
                                }
                            }
                            for (idx, item) in deck_ui.iter().enumerate() {
                                crate::ui::views::CardView {
                                    key: "deck-{item.0.id}",
                                    index: idx,
                                    card: item.0.clone(),
                                    selected: focus_value == FocusZone::Deck && selected_collection == idx,
                                    dragging: item.1,
                                    drag_style: item.2.clone(),
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
                                            let mut eng = engine.write();
                                            let _ = eng.dispatch(kardinality::Action::ReorderCollection { from: idx, to: idx - 1 });
                                            focus.set(FocusZone::Deck);
                                            sel_collection.set(idx - 1);
                                    },
                                    on_move_right: move |idx| {
                                            let len = engine.read().state.collection.len();
                                            if idx + 1 >= len { return; }
                                            let mut eng = engine.write();
                                            let _ = eng.dispatch(kardinality::Action::ReorderCollection { from: idx, to: idx + 1 });
                                            focus.set(FocusZone::Deck);
                                            sel_collection.set(idx + 1);
                                    },
                                    on_docs: move |id| {
                                            kardinomicon_target.set(Some(id));
                                            kardinomicon_open.set(true);
                                    },
                                    on_ptr_down: move |pd: crate::ui::views::PtrDown| {
                                            if let Some(card) = engine.read().state.collection.get(pd.index).cloned() {
                                                drag.set(Some(PtrDrag {
                                                    zone: FocusZone::Deck,
                                                    index: pd.index,
                                                    card,
                                                    origin_left: pd.client_x - pd.elem_x,
                                                    origin_top: pd.client_y - pd.elem_y,
                                                    off_x: pd.elem_x,
                                                    off_y: pd.elem_y,
                                                    client_x: pd.client_x,
                                                    client_y: pd.client_y,
                                                    moved: false,
                                                }));
                                            }
                                            focus.set(FocusZone::Deck);
                                            sel_collection.set(pd.index);
                                    },
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

            // Pointer-drag uses the real card element (no overlay).

            // Animated playback overlay.
            if pb_active() {
                div { class: "fx-blocker" }
                div { class: "fx-layer",
                    if let Some((x, y, text)) = pb_call() {
                        div { class: "fx-call", style: "left: {x}px; top: {y}px;", "{text}" }
                    }
                    for p in pb_projs() {
                        FxOverlayProj { proj: p }
                    }
                    for burst in pb_bursts() {
                        FxOverlayBurst { burst: burst }
                    }
                    for c in pb_cards() {
                        FxOverlayCard { card: c }
                    }
                    for b in pb_bumps() {
                        FxOverlayBump { bump: b }
                    }
                }
            }
        }
    }
}


