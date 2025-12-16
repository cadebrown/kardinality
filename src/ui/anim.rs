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
                    },
                );
            }
        }
        out
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
                    let cb3 = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                        for html in animated2.iter() {
                            let style = html.style();
                            let _ = style.set_property("z-index", "1");
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
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy)]
    pub struct Rect {
        pub left: f64,
        pub top: f64,
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
}

pub use imp::*;


