use web_sys::{HtmlInputElement};
use yew::{Component, html, Html, Properties, InputEvent, TargetCast};

pub struct CappedInputComponent {
	// TODO
}

#[derive(Properties, PartialEq)]
pub struct CappedInputProps {
    pub min_value: u32,
    pub max_value: u32
}

impl Component for CappedInputComponent {
	type Message = ();
	type Properties = CappedInputProps;

	fn create(_ctx: &yew::Context<Self>) -> Self {
		Self {}
	}

	fn view(&self, ctx: &yew::Context<Self>) -> Html {
		let min_val = ctx.props().min_value;
		let max_val = ctx.props().max_value;

		let on_input = ctx.link().callback(move |e: InputEvent| {
			let input_el: HtmlInputElement = e.target_unchecked_into();
			let val: u32 = match input_el.value().parse() {
				Ok(val) => val,
				Err(err) => {
					log::error!("error ocurred parsing value {}", err);
					0
				}
			};
			log::debug!("Input value: {}", val);
		});

		html! {
			<div>
				<label>{format!("Input value. Min: {}, Max: {}", min_val, max_val)}</label>
				<input 
					type="number" placeholder="input a number" 
					min={min_val.to_string()} max={max_val.to_string()}
					oninput={on_input} /> 
			</div>
		}
	}
}