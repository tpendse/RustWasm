use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Hello, World!"}</h1>
            <h3>{"This is a message from yew WASM"}</h3>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}