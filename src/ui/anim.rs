// FLIP animations are only meaningful in the browser build.
// Provide no-op stubs for native builds so `cargo test` remains pleasant.

#[cfg(target_arch = "wasm32")]
mod imp {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use wasm_bindgen::JsCast;

    #[derive(Debug, Clone, Copy)]
    pub struct Rect {
        pub left: f64,
        pub top: f64,
        pub width: f64,
        pub height: f64,
    }

    pub fn visible_card_ids(
        deck: &[kardinality::game::CardInstance],
        hand: &[kardinality::game::CardInstance],
    ) -> Vec<String> {
        let mut ids = Vec::with_capacity(deck.len() + hand.len());
        for c in deck {
            ids.push(format!("card-{}", c.id));
        }
        for c in hand {
            ids.push(format!("card-{}", c.id));
        }
        ids
    }

    pub fn capture_rects(ids: &[String]) -> HashMap<String, Rect> {
        let mut out = HashMap::new();
        let Some(window) = web_sys::window() else {
            return out;
        };
        let Some(doc) = window.document() else {
            return out;
        };

        for id in ids {
            if let Some(el) = doc.get_element_by_id(id) {
                let rect = el.get_bounding_client_rect();
                out.insert(
                    id.clone(),
                    Rect {
                        left: rect.left(),
                        top: rect.top(),
                        width: rect.width(),
                        height: rect.height(),
                    },
                );
            }
        }
        out
    }

    pub fn rect_for_testid(testid: &str) -> Option<Rect> {
        let Some(window) = web_sys::window() else {
            return None;
        };
        let Some(doc) = window.document() else {
            return None;
        };
        let selector = format!(r#"[data-testid="{testid}"]"#);
        let el = doc.query_selector(&selector).ok().flatten()?;
        let rect = el.get_bounding_client_rect();
        Some(Rect {
            left: rect.left(),
            top: rect.top(),
            width: rect.width(),
            height: rect.height(),
        })
    }

    pub fn rect_for_id(id: &str) -> Option<Rect> {
        let Some(window) = web_sys::window() else {
            return None;
        };
        let Some(doc) = window.document() else {
            return None;
        };
        let el = doc.get_element_by_id(id)?;
        let rect = el.get_bounding_client_rect();
        Some(Rect {
            left: rect.left(),
            top: rect.top(),
            width: rect.width(),
            height: rect.height(),
        })
    }

    pub fn set_opacity_for_id(id: &str, opacity: f64) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(doc) = window.document() else {
            return;
        };
        let Some(el) = doc.get_element_by_id(id) else {
            return;
        };
        if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() {
            let _ = html
                .style()
                .set_property("opacity", &format!("{opacity:.3}"));
        }
    }

    pub fn add_temp_class_for_id(id: &str, class: &str, ms: i32) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(doc) = window.document() else {
            return;
        };
        let Some(el) = doc.get_element_by_id(id) else {
            return;
        };
        let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        let class = class.to_string();
        let id = id.to_string();

        let cur = html.class_name();
        let has = cur.split_whitespace().any(|c| c == class);
        if !has {
            let next = if cur.trim().is_empty() {
                class.clone()
            } else {
                format!("{cur} {class}")
            };
            html.set_class_name(&next);
        }

        let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let Some(window) = web_sys::window() else {
                return;
            };
            let Some(doc) = window.document() else { return };
            let Some(el) = doc.get_element_by_id(&id) else {
                return;
            };
            let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() else {
                return;
            };
            let cur = html.class_name();
            let next = cur
                .split_whitespace()
                .filter(|c| *c != class)
                .collect::<Vec<_>>()
                .join(" ");
            html.set_class_name(&next);
        }) as Box<dyn FnMut()>);

        let _ = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), ms);
        cb.forget();
    }

    pub async fn sleep_ms(ms: i32) {
        use wasm_bindgen_futures::JsFuture;
        let promise = js_sys::Promise::new(&mut |resolve, _reject| {
            let Some(window) = web_sys::window() else {
                return;
            };
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
        });
        let _ = JsFuture::from(promise).await;
    }

    pub fn start_focus_halo_loop() {
        thread_local! {
            static HALO_CB: RefCell<Option<wasm_bindgen::closure::Closure<dyn FnMut(f64)>>> =
                RefCell::new(None);
        }

        // Already started?
        let already = HALO_CB.with(|c| c.borrow().is_some());
        if already {
            return;
        }

        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(doc) = window.document() else {
            return;
        };

        let window2 = window.clone();
        let doc2 = doc.clone();

        // The callback reschedules itself via the thread-local stored closure.
        let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move |_ts: f64| {
            let app = doc2.get_element_by_id("app-root");
            let halo_el = doc2.get_element_by_id("focus-halo");

            if let (Some(app), Some(halo_el)) = (app, halo_el) {
                if let Ok(halo) = halo_el.dyn_into::<web_sys::HtmlElement>() {
                    let enabled =
                        app.get_attribute("data-halo-enabled").unwrap_or_default() == "true";
                    let target = app.get_attribute("data-halo-target").unwrap_or_default();
                    let zone = app.get_attribute("data-halo-zone").unwrap_or_default();

                    if !enabled || target.is_empty() {
                        let _ = halo.style().set_property("display", "none");
                    } else if let Some(t) = doc2.get_element_by_id(&target) {
                        let rect = t.get_bounding_client_rect();
                        let style = halo.style();
                        let _ = style.set_property("display", "block");
                        let _ = style.set_property("left", &format!("{}px", rect.left()));
                        let _ = style.set_property("top", &format!("{}px", rect.top()));
                        let _ = style.set_property("width", &format!("{}px", rect.width()));
                        let _ = style.set_property("height", &format!("{}px", rect.height()));
                        halo.set_class_name(&format!("focus-halo {zone}"));
                    } else {
                        let _ = halo.style().set_property("display", "none");
                    }
                }
            }

            HALO_CB.with(|c| {
                if let Some(cb) = c.borrow().as_ref() {
                    let _ = window2.request_animation_frame(cb.as_ref().unchecked_ref());
                }
            });
        }) as Box<dyn FnMut(f64)>);

        HALO_CB.with(|c| {
            *c.borrow_mut() = Some(cb);
            if let Some(cb) = c.borrow().as_ref() {
                let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
            }
        });
    }

    pub fn query_param(key: &str) -> Option<String> {
        let Some(window) = web_sys::window() else {
            return None;
        };
        let loc = window.location();
        let search = loc.search().ok()?;
        // search is like "?a=1&b=2"
        let s = search.trim_start_matches('?');
        for part in s.split('&') {
            if part.is_empty() {
                continue;
            }
            let mut it = part.splitn(2, '=');
            let k = it.next().unwrap_or("");
            let v = it.next().unwrap_or("");
            if k == key {
                // URL decode '+' is space, %XX sequences.
                return Some(
                    js_sys::decode_uri_component(&v.replace('+', " "))
                        .ok()
                        .and_then(|js| js.as_string())
                        .unwrap_or_else(|| v.to_string()),
                );
            }
        }
        None
    }

    pub fn play_flip(before: HashMap<String, Rect>, duration_ms: f64) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(doc) = window.document() else {
            return;
        };

        // Run on the next frame so Dioxus has applied DOM updates after state changes.
        let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            // Step 1: apply inverse transforms (no transition)
            let mut animated: Vec<web_sys::HtmlElement> = Vec::new();
            let mut containers: Vec<web_sys::HtmlElement> = Vec::new();
            let mut bars: Vec<web_sys::HtmlElement> = Vec::new();
            for (id, b) in before.iter() {
                let Some(el) = doc.get_element_by_id(id) else {
                    continue;
                };
                let a = el.get_bounding_client_rect();
                let dx = b.left - a.left();
                let dy = b.top - a.top();
                if dx.abs() < 0.5 && dy.abs() < 0.5 {
                    continue;
                }

                let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() else {
                    continue;
                };

                // Avoid clipping while a card travels between rows/containers.
                // NOTE: we do this without forcing scrollbars, to avoid any layout shift.
                if let Ok(Some(parent)) = html.closest(".row-scroll") {
                    if let Ok(p) = parent.dyn_into::<web_sys::HtmlElement>() {
                        let already = containers.iter().any(|c| c.is_same_node(Some(p.as_ref())));
                        if !already {
                            let style = p.style();
                            let _ = style.set_property("overflow-x", "visible");
                            let _ = style.set_property("overflow-y", "visible");
                            containers.push(p);
                        }
                    }
                }

                // Critical for Hand<->Deck moves:
                // the element exists in the *destination* container when animating.
                // If that destination panel has a lower z-index than the source panel,
                // the card can start its FLIP "behind" the other panel and look like it disappears.
                // Temporarily lift the nearest hand/deck panel above siblings during the animation.
                let bar_parent = html
                    .closest(".handbar")
                    .ok()
                    .flatten()
                    .or_else(|| html.closest(".deckbar").ok().flatten());
                if let Some(bar) = bar_parent {
                    if let Ok(bar) = bar.dyn_into::<web_sys::HtmlElement>() {
                        let already = bars.iter().any(|b| b.is_same_node(Some(bar.as_ref())));
                        if !already {
                            let style = bar.style();
                            let _ = style.set_property("position", "relative");
                            let _ = style.set_property("z-index", "9995");
                            bars.push(bar);
                        }
                    }
                }

                let style = html.style();
                let _ = style.set_property("transition", "none");
                let _ = style.set_property("position", "relative");
                let _ = style.set_property("z-index", "9999");
                let _ = style.set_property("transform", &format!("translate({dx}px, {dy}px)"));
                animated.push(html);
            }

            // Step 2: next frame, animate back to identity
            let Some(window2) = web_sys::window() else {
                return;
            };
            let cb2 = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                for html in animated.iter() {
                    let style = html.style();
                    let _ = style.set_property(
                        "transition",
                        &format!("transform {}ms cubic-bezier(0.16, 1, 0.3, 1)", duration_ms),
                    );
                    let _ = style.set_property("transform", "translate(0px, 0px)");
                }

                // Step 3: after animation, restore styles.
                if let Some(window3) = web_sys::window() {
                    let animated2 = animated.clone();
                    let containers2 = containers.clone();
                    let bars2 = bars.clone();
                    let cb3 = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                        for html in animated2.iter() {
                            let style = html.style();
                            // Clear any temporary inline styles so CSS can own layout/stacking.
                            let _ = style.set_property("transition", "");
                            let _ = style.set_property("transform", "");
                            let _ = style.set_property("z-index", "");
                            let _ = style.set_property("position", "");
                        }
                        for c in containers2.iter() {
                            let style = c.style();
                            // Restore whatever CSS defines (e.g. overflow-x: auto).
                            let _ = style.set_property("overflow-x", "");
                            let _ = style.set_property("overflow-y", "");
                        }
                        for b in bars2.iter() {
                            let style = b.style();
                            let _ = style.set_property("z-index", "");
                            let _ = style.set_property("position", "");
                        }
                    })
                        as Box<dyn FnMut()>);

                    let _ = window3.set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb3.as_ref().unchecked_ref(),
                        duration_ms.max(0.0) as i32,
                    );
                    cb3.forget();
                }
            }) as Box<dyn FnMut()>);
            let _ = window2.request_animation_frame(cb2.as_ref().unchecked_ref());
            cb2.forget();
        }) as Box<dyn FnMut()>);

        let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
        cb.forget();
    }

    pub fn scroll_card_into_view(card_dom_id: &str) {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(doc) = window.document() else {
            return;
        };

        let id = card_dom_id.to_string();
        let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let Some(el) = doc.get_element_by_id(&id) else {
                return;
            };
            let Ok(Some(parent)) = el.closest(".row-scroll") else {
                return;
            };
            let Ok(container) = parent.dyn_into::<web_sys::HtmlElement>() else {
                return;
            };
            let Ok(card) = el.dyn_into::<web_sys::HtmlElement>() else {
                return;
            };

            // Use layout offsets (not transformed rects) for stable keyboard scrolling.
            let card_left = card.offset_left() as f64;
            let card_width = card.offset_width() as f64;
            let viewport_width = container.client_width() as f64;
            let max_scroll = (container.scroll_width() - container.client_width()).max(0) as f64;

            // Center selected card in row viewport.
            let mut target = card_left + card_width / 2.0 - viewport_width / 2.0;
            if !target.is_finite() {
                return;
            }
            target = target.clamp(0.0, max_scroll);
            container.set_scroll_left(target.round() as i32);
        }) as Box<dyn FnMut()>);

        let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
        cb.forget();
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum HitZone {
        Hand,
        Deck,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct HitTest {
        pub card_id: Option<u64>,
        /// Relative X within the hit card (0..1). Only set when `card_id` is Some.
        pub rel_x: Option<f64>,
        /// The zone under the pointer (hand/deck), if any.
        pub zone: Option<HitZone>,
        /// If the pointer is over a drop sliver, the insertion index (0..=len).
        pub drop_index: Option<usize>,
    }

    pub fn hit_test(client_x: f64, client_y: f64) -> HitTest {
        let Some(window) = web_sys::window() else {
            return HitTest {
                card_id: None,
                rel_x: None,
                zone: None,
                drop_index: None,
            };
        };
        let Some(doc) = window.document() else {
            return HitTest {
                card_id: None,
                rel_x: None,
                zone: None,
                drop_index: None,
            };
        };

        let el = doc.element_from_point(client_x as f32, client_y as f32);

        let mut zone: Option<HitZone> = None;
        if let Some(el) = el.as_ref() {
            if el
                .closest(r#"[data-testid="hand-zone"]"#)
                .ok()
                .flatten()
                .is_some()
            {
                zone = Some(HitZone::Hand);
            } else if el
                .closest(r#"[data-testid="deck-zone"]"#)
                .ok()
                .flatten()
                .is_some()
            {
                zone = Some(HitZone::Deck);
            }
        }

        let Some(el) = el else {
            return HitTest {
                card_id: None,
                rel_x: None,
                zone,
                drop_index: None,
            };
        };

        // Drop slivers (between cards): prefer these over card hit-testing.
        if let Ok(Some(slot)) = el.closest(r#"[data-drop-zone][data-drop-index]"#) {
            let z = slot
                .get_attribute("data-drop-zone")
                .as_deref()
                .and_then(|s| match s {
                    "hand" => Some(HitZone::Hand),
                    "deck" => Some(HitZone::Deck),
                    _ => None,
                });
            let idx = slot
                .get_attribute("data-drop-index")
                .and_then(|s| s.parse::<usize>().ok());
            if let (Some(z), Some(idx)) = (z, idx) {
                return HitTest {
                    card_id: None,
                    rel_x: None,
                    zone: Some(z),
                    drop_index: Some(idx),
                };
            }
        }

        // Find the nearest card element by id="card-<u64>"
        let card_el = el.closest(r#"[id^="card-"]"#).ok().flatten();

        let Some(card_el) = card_el else {
            return HitTest {
                card_id: None,
                rel_x: None,
                zone,
                drop_index: None,
            };
        };

        let id_str = card_el.id();
        let id = id_str.trim_start_matches("card-").parse::<u64>().ok();
        let rect = card_el.get_bounding_client_rect();
        let w = rect.width().max(1.0);
        let rel = ((client_x - rect.left()) / w).clamp(0.0, 1.0);

        HitTest {
            card_id: id,
            rel_x: Some(rel),
            zone,
            drop_index: None,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy)]
    pub struct Rect {
        pub left: f64,
        pub top: f64,
        pub width: f64,
        pub height: f64,
    }

    pub fn visible_card_ids(
        deck: &[kardinality::game::CardInstance],
        hand: &[kardinality::game::CardInstance],
    ) -> Vec<String> {
        // Desktop: basic impl for animations to run without errors.
        let mut ids = Vec::with_capacity(deck.len() + hand.len());
        for c in deck {
            ids.push(format!("card-{}", c.id));
        }
        for c in hand {
            ids.push(format!("card-{}", c.id));
        }
        ids
    }

    pub fn capture_rects(_ids: &[String]) -> HashMap<String, Rect> {
        // Desktop: no-op; Dioxus desktop doesn't expose getBoundingClientRect, so skip FLIP.
        HashMap::new()
    }

    pub fn play_flip(_before: HashMap<String, Rect>, _duration_ms: f64) {
        // Desktop: no-op; no FLIP animations without DOM rect access.
    }

    pub fn scroll_card_into_view(_card_dom_id: &str) {
        // Desktop: no-op; Dioxus desktop manages scrolling internally.
    }

    pub fn rect_for_testid(_testid: &str) -> Option<Rect> {
        // Desktop: no-op; used for playback projectile targeting, gracefully skips.
        None
    }

    pub fn rect_for_id(_id: &str) -> Option<Rect> {
        // Desktop: no-op; used for drag overlay positioning, gracefully skips.
        None
    }

    pub fn set_opacity_for_id(_id: &str, _opacity: f64) {
        // Desktop: no-op; opacity changes are CSS-only, no DOM manipulation needed.
    }

    pub async fn sleep_ms(ms: i32) {
        // Desktop: real async sleep for playback timing (using std::thread for simplicity).
        use std::time::Duration;
        async_std::task::sleep(Duration::from_millis(ms as u64)).await;
    }

    pub fn query_param(_key: &str) -> Option<String> {
        // Desktop: no query params in native builds.
        None
    }

    pub fn add_temp_class_for_id(_id: &str, _class: &str, _ms: i32) {
        // Desktop: no-op; temporary class manipulation for "pop-in" effect, CSS-only fallback.
    }

    pub fn start_focus_halo_loop() {
        // Desktop: no-op.
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum HitZone {
        Hand,
        Deck,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct HitTest {
        pub card_id: Option<u64>,
        pub rel_x: Option<f64>,
        pub zone: Option<HitZone>,
        pub drop_index: Option<usize>,
    }

    pub fn hit_test(_client_x: f64, _client_y: f64) -> HitTest {
        HitTest {
            card_id: None,
            rel_x: None,
            zone: None,
            drop_index: None,
        }
    }
}

pub use imp::*;
