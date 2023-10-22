mod app;

use app::App;

mod solana;
pub mod types;
mod util;

#[macro_use]
extern crate num_derive;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
