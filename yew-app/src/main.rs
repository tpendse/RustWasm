mod capped_input_component;

use yew::prelude::*;
use capped_input_component::CappedInputComponent;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div id="maincontent">
            <h1>{"Hello, World!"}</h1>
            <h3>{"This is output from yew WASM"}</h3>
            <div>
                <CappedInputComponent />
                <CappedInputComponent />
                <CappedInputComponent />
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("App is starting!");
    yew::start_app::<App>();
}