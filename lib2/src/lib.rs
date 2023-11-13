use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Bar)]
pub struct Bar {}

#[wasm_bindgen(js_class = Bar)]
impl Bar {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Bar {
        tracing::info!("new bar");
        // panic!("bar new");
        Bar {}
    }
}

#[wasm_bindgen(start)]
pub fn setup_logging() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
}
