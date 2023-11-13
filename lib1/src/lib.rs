use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Foo)]
pub struct Foo {}

#[wasm_bindgen(js_class = Foo)]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Foo {
        tracing::info!("new foo");
        Foo {}
    }
}

#[wasm_bindgen(start)]
pub fn setup_logging() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();
}
