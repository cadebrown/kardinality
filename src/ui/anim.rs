// FLIP animations are only meaningful in the browser build.
// Provide no-op stubs for native builds so `cargo test` remains pleasant.

#[cfg(target_arch = "wasm32")]
mod imp {
    use std::collections::HashMap;

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
            let _ = html.style().set_property("opacity", &format!("{opacity:.3}"));
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
            let Some(window) = web_sys::window() else { return };
            let Some(doc) = window.document() else { return };
            let Some(el) = doc.get_element_by_id(&id) else { return };
            let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() else { return };
            let cur = html.class_name();
            let next = cur
                .split_whitespace()
                .filter(|c| *c != class)
                .collect::<Vec<_>>()
                .join(" ");
            html.set_class_name(&next);
        }) as Box<dyn FnMut()>);

        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            ms,
        );
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

                // Avoid clipping during movement: temporarily allow overflow in the nearest scroll row.
                if let Ok(Some(parent)) = el.closest(".row-scroll") {
                    if let Ok(p) = parent.dyn_into::<web_sys::HtmlElement>() {
                        let style = p.style();
                        let _ = style.set_property("overflow-x", "visible");
                        let _ = style.set_property("overflow-y", "visible");
                        containers.push(p);
                    }
                }

                let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() else {
                    continue;
                };

                let style = html.style();
                let _ = style.set_property("transition", "none");
                let _ = style.set_property("position", "relative");
                let _ = style.set_property("z-index", "9999");
                let _ = style.set_property("transform", &format!("translate({dx}px, {dy}px)"));
                animated.push(html);
            }

            // Step 2: next frame, animate back to identity
            let Some(window2) = web_sys::window() else { return };
            let cb2 = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                for html in animated.iter() {
                    let style = html.style();
                    let _ = style.set_property(
                        "transition",
                        &format!(
                            "transform {}ms cubic-bezier(0.16, 1, 0.3, 1)",
                            duration_ms
                        ),
                    );
                    let _ = style.set_property("transform", "translate(0px, 0px)");
                }

                // Step 3: after animation, restore stacking order.
                if let Some(window3) = web_sys::window() {
                    let animated2 = animated.clone();
                    let containers2 = containers.clone();
                    let cb3 = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                        for html in animated2.iter() {
                            let style = html.style();
                            let _ = style.set_property("z-index", "1");
                        }
                        for c in containers2.iter() {
                            let style = c.style();
                            let _ = style.set_property("overflow-x", "scroll");
                            let _ = style.set_property("overflow-y", "visible");
                        }
                    }) as Box<dyn FnMut()>);

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

            let rect = el.get_bounding_client_rect();
            let crect = container.get_bounding_client_rect();

            // Center the element in the scroll container.
            let cur = container.scroll_left() as f64;
            let target =
                (rect.left() - crect.left()) + cur - (crect.width() / 2.0 - rect.width() / 2.0);
            container.set_scroll_left(target.round() as i32);
        }) as Box<dyn FnMut()>);

        let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
        cb.forget();
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
        _deck: &[kardinality::game::CardInstance],
        _hand: &[kardinality::game::CardInstance],
    ) -> Vec<String> {
        Vec::new()
    }

    pub fn capture_rects(_ids: &[String]) -> HashMap<String, Rect> {
        HashMap::new()
    }

    pub fn play_flip(_before: HashMap<String, Rect>, _duration_ms: f64) {}

    pub fn scroll_card_into_view(_card_dom_id: &str) {}

    pub fn rect_for_testid(_testid: &str) -> Option<Rect> {
        None
    }

    pub fn rect_for_id(_id: &str) -> Option<Rect> {
        None
    }

    pub fn set_opacity_for_id(_id: &str, _opacity: f64) {}

    pub async fn sleep_ms(_ms: i32) {}

    pub fn query_param(_key: &str) -> Option<String> {
        None
    }

    pub fn add_temp_class_for_id(_id: &str, _class: &str, _ms: i32) {}
}

pub use imp::*;


