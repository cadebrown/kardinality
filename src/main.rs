#[cfg(target_arch = "wasm32")]
mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Kardinality runs in the browser (WASM). Install `dioxus-cli` and run `dx serve`.");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(ui::App);
}
