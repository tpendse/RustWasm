use web_sys::{HtmlInputElement};
use yew::{Component, html, Html, Properties, InputEvent, TargetCast, NodeRef};

pub enum Msg {
	SetInput(u32)
}

pub struct CappedInputComponent {
	input_ref: NodeRef
}

#[derive(Properties, PartialEq)]
pub struct CappedInputProps {
    pub min_value: u32,
    pub max_value: u32
}

impl Component for CappedInputComponent {
	type Message = Msg;
	type Properties = CappedInputProps;

	fn create(_ctx: &yew::Context<Self>) -> Self {
		Self { input_ref: NodeRef::default() }
	}

	fn view(&self, ctx: &yew::Context<Self>) -> Html {
		let min_val = ctx.props().min_value;
		let max_val = ctx.props().max_value;

		let on_input = ctx.link().callback(move |e: InputEvent| {
			let input_el: HtmlInputElement = e.target_unchecked_into();
			let mut val: u32 = match input_el.value().parse() {
				Ok(val) => {
					log::debug!("On input!");
					val
				},
				Err(err) => {
					log::error!("error ocurred parsing value {}", err);
					0
				}
			};
			log::debug!("Input value: {}", val);

			if val > max_val {
				val = max_val;
				log::debug!("CLAMP to min: {}", val);
			}
			else if val < min_val {
				val = min_val;
				log::debug!("CLAMP to max: {}", val);
			}

			Msg::SetInput(val)
		});

		let input_ref = self.input_ref.clone();
		html! {
			<div>
				<label>{format!("Input value. Min: {}, Max: {}", min_val, max_val)}</label>
				<input 
					ref={input_ref} type="number" placeholder="input a number" 
					min={min_val.to_string()} max={max_val.to_string()}
					oninput={on_input} /> 
			</div>
		}
	}

	fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::SetInput(val) => {
				log::debug!("Setting input value to: {}", val);
				let input_element = self.input_ref.clone().cast::<HtmlInputElement>().unwrap();
				input_element.set_value(&val.to_string());
				true
			}
		}
	}
}