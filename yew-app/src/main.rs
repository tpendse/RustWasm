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
                <CappedInputComponent min_value={0} max_value={10} />
                <CappedInputComponent min_value={0} max_value={100} />
                <CappedInputComponent min_value={0} max_value={1000} />
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("App is starting!");
    yew::start_app::<App>();
}