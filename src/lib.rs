mod about;
mod content;
mod handle;
mod js;
mod main_component;
mod posts;
mod site_info;
mod theme;
mod theme_switcher;
use crate::main_component::Main;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
extern crate rust_fel;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let main = Main::create();
    let app = rust_fel::App::new(main);
    app.mount("root");

    Ok(())
}
