use solana_sdk::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(module = "/js/date-picker.js")]
extern "C" {
    #[wasm_bindgen(js_name = initDatePicker)]
    fn init_date_picker(buf: &str) -> JsValue;
}

use yew::prelude::*;

#[function_component(DatePicker)]
pub fn date_picker() -> Html {
    use_effect_with((), |_| {
        init_date_picker("#datepicker");
    });

    html! {
        <div>
            <div id="datepicker" />
        </div>
    }
}
