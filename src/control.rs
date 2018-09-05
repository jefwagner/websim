/// Defines functions to create HTML form elements for the simulation
///
extern crate stdweb;

use std::rc::Rc;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
	HtmlElement,
	document,
};
use stdweb::web::html_element::{
	InputElement,
	SelectElement,
	OptionElement,
};
use stdweb::web::event::{
 	ChangeEvent,
 	ClickEvent,
 	InputEvent,
};

use ::container::UiElement;
use ::output::TextArea;

/// A labeled checkbox
#[derive(Debug, Clone)]
pub struct Checkbox{
	elem: HtmlElement,
	checkbox: InputElement,
}

// impl Clone for Checkbox {
// 	fn clone(&self) -> Checkbox {
// 		Checkbox{
// 			elem: self.elem.clone(),
// 			checkbox: self.checkbox.clone(),
// 		}
// 	}
// }

impl Checkbox {
/// no default action attached to checking the box
	pub fn new( name: &str, text: &str) -> Checkbox {
		let elem : HtmlElement = document().create_element( "div").unwrap().try_into().unwrap();
		elem.class_list().add("control").unwrap();
		elem.set_attribute("data-param-type", "checkbox").unwrap();
		elem.set_attribute("id",name).unwrap();
		// add the text
		let text = document().create_text_node( text);
		elem.append_child( &text);
		// add select node
		let checkbox : InputElement = document().create_element( "input").unwrap().try_into().unwrap();
		checkbox.set_attribute("type", "checkbox").unwrap();
		checkbox.set_attribute("id",&format!("{}_checkbox",name)).unwrap();
		elem.append_child( &checkbox);
		Checkbox{ elem: elem, checkbox: checkbox}
	}

	pub fn query( &self) -> bool {
		let checked: bool = js!( return @{&self.checkbox}.checked; ).try_into().unwrap();
		checked
	}

	pub fn add_check_function<F>( &self, func:F ) 
		where F: Fn(bool) + 'static {
		self.checkbox.add_event_listener({
			let checkbox = self.clone();
			move | _:ClickEvent | {
				let checked = checkbox.query();
				func(checked);
			}
		});
	}
}

impl UiElement for Checkbox {
	fn elem( &self) -> &HtmlElement { &self.elem }
}

pub fn query_checkbox( name: &str) -> bool {
	let checkbox : InputElement = document().query_selector( &format!("#{}_checkbox",name)).unwrap().unwrap().try_into().unwrap();
	let checked: bool = js!( return @{&checkbox}.checked; ).try_into().unwrap();
	checked
}

/// A button showning text
#[derive(Debug, Clone)]
pub struct Button{ 
	elem: HtmlElement,
	button: InputElement,
}

impl Button {
	pub fn new( name: &str, text: &str) -> Button {
		let elem : HtmlElement = document().create_element("span").unwrap().try_into().unwrap();
		elem.class_list().add("control").unwrap();
		elem.set_attribute("id", name).unwrap();
		elem.set_attribute("data-param-type", "button").unwrap();
		let button : InputElement = document().create_element( "input").unwrap().try_into().unwrap();
		button.class_list().add("control_button").unwrap();
		button.set_attribute("type","button").unwrap();
		button.set_attribute("id", &format!("{}_button", name)).unwrap();
		button.set_attribute("value", text).unwrap();
		button.set_attribute("data-control-pressed", "false").unwrap();
		button.add_event_listener({
			let button = button.clone();
			move | _:ClickEvent | {
				button.set_attribute("data-control-pressed", "true").unwrap();
			}
		});
		elem.append_child( &button);
		Button{
			elem: elem,
			button: button,
		}
	}

	pub fn create_from( elem: HtmlElement, button: InputElement) -> Button {
		Button{ elem: elem, button: button }
	}

	pub fn query( &self) -> bool {
		let pressed = self.button.get_attribute("data-control-pressed").unwrap() == "true";
		if pressed {
			self.button.set_attribute("data-control-pressed", "false").unwrap();
		};
		pressed		
	}

	pub fn add_button_function<F>( &self, func:F)
		where F: Fn(bool) + 'static {
		self.button.add_event_listener({
			let button = self.clone();
			move | _:ClickEvent | {
				let pressed = button.query();
				func(pressed);
			}
		});
	}
}

impl UiElement for Button {
	fn elem( &self) -> &HtmlElement { &self.elem }
}

fn query_button( name: &str) -> bool {
	let elem : HtmlElement = document().query_selector( &format!("#{}",name)).unwrap().unwrap().try_into().unwrap();	
	let button : InputElement = elem.last_child().unwrap().try_into().unwrap();
	let pressed = button.get_attribute("data-control-pressed").unwrap() == "true";
	if pressed {
		button.set_attribute("data-control-pressed", "false").unwrap();
	};
	pressed
}

/// A toggle-able button with two possible text to show.
#[derive(Debug, Clone)]
pub struct Toggle{
	elem: HtmlElement,
	button: InputElement,
	text_on: String,
	text_off: String,
}

impl Toggle {
	pub fn new( name: &str, text_off: &str, text_on: &str) -> Toggle {
		let elem : HtmlElement = document().create_element("span").unwrap().try_into().unwrap();
		elem.class_list().add("control").unwrap();
		elem.set_attribute("id", name).unwrap();
		elem.set_attribute("data-param-type", "toggle").unwrap();
		let button : InputElement = document().create_element( "input").unwrap().try_into().unwrap();
		button.class_list().add("control_toggle").unwrap();
		button.set_attribute("type","button").unwrap();
		button.set_attribute("id", &format!("{}_button",name)).unwrap();
		button.set_attribute("value", text_off).unwrap();
		button.set_attribute("data-toggle-state", "off").unwrap();
		button.add_event_listener({
			let button = button.clone();
			let text_on = String::from(text_on);
			let text_off = String::from(text_off);
			move | _:ClickEvent | {
				let test = button.get_attribute("data-toggle-state").unwrap() == "off";
				if test {
					button.set_raw_value( &text_on);
					button.set_attribute("data-toggle-state", "on").unwrap();
				} else {
					button.set_raw_value( &text_off);
					button.set_attribute("data-toggle-state", "off").unwrap();
				};
			}
		});
		elem.append_child( &button);
		Toggle{ 
			elem: elem, 
			button: button, 
			text_on: String::from(text_on),
			text_off: String::from(text_off),
		}
	}

	pub fn query( &self) -> bool {
		self.button.get_attribute("data-toggle-state").unwrap() == "on"
	}

	pub fn set( &self, new_state:bool) {
		let current_state = self.query();
		if current_state && !new_state {
			self.button.set_raw_value( &self.text_off);
			self.button.set_attribute( "data-toggle-state", "off").unwrap();
		} else if !current_state && new_state {
			self.button.set_raw_value( &self.text_on);
			self.button.set_attribute("data-toggle-state", "on").unwrap();
		}
	}

	pub fn add_toggle_function<F>( &self, func:F)
		where F: Fn(bool) + 'static {
		self.button.add_event_listener({
			let toggle = self.clone();
			move | _:ClickEvent | {
				let current_state = toggle.query();
				func(current_state);
			}
		});
	}
}

pub fn query_toggle( name: &str) -> bool {
	let elem : HtmlElement = document().query_selector( &format!("#{}",name)).unwrap().unwrap().try_into().unwrap();	
	let button : InputElement = elem.last_child().unwrap().try_into().unwrap();
	button.get_attribute("data-toggle-state").unwrap() == "on"
}

// impl UiElement for Toggle {}
impl UiElement for Toggle {
	fn elem( &self) -> &HtmlElement { &self.elem }
}

/// A labeled dropdown menu.
/// no deault action attached.
#[derive(Debug, Clone)]
pub struct Dropdown {
	elem: HtmlElement,
	select: SelectElement,
	options: Vec<OptionElement>,
}

impl Dropdown{
	pub fn new( name: &str, text: &str) -> Dropdown {
		let elem : HtmlElement = document().create_element( "div").unwrap().try_into().unwrap();
		elem.class_list().add("control").unwrap();
		elem.set_attribute("data-param-type", "dropdown").unwrap();
		elem.set_attribute("id",name).unwrap();
		// add the text
		let text = document().create_text_node( text);
		elem.append_child( &text);
		// add select node
		let select : SelectElement = document().create_element("select").unwrap().try_into().unwrap();
		select.set_attribute( "name", &format!("{}_name",name)).unwrap();
		select.set_attribute( "id", &format!("{}_select",name)).unwrap();
		elem.append_child(&select);
		let options : Vec<OptionElement> = Vec::new();
		Dropdown{ 
			elem: elem, 
			select: select, 
			options: options,
		}
	}

	pub fn add_option<'a>( &'a mut self, value: &'static str, text: &'static str) -> &'a Dropdown {
		let option : OptionElement = document().create_element("option").unwrap().try_into().unwrap();
		option.set_attribute("value", value).unwrap();
		let text = document().create_text_node(text);
		option.append_child(&text);
		self.select.append_child(&option);
		self.options.push(option);
		self
	}

	pub fn add_multiple_options<'a>( &'a mut self, options: &[(&'static str, &'static str)]) ->&'a Dropdown {
		for (value, text) in options {
			self.add_option(value, text);
 		}
 		self
	}

	pub fn query( &self) -> String {
		let index: usize = js!( return @{&self.select}.selectedIndex; ).try_into().unwrap();
		self.options[index].get_attribute("value").unwrap()
	}

	pub fn add_dropdown_function<F>( &self, func:F)
		where F: Fn(String) + 'static {
		self.select.add_event_listener({
			let dropdown = self.clone();
			move | _:ChangeEvent | {
				let selected = dropdown.query();
				func(selected);
			}
		});
	}

}

// impl UiElement for Dropdown {}
impl UiElement for Dropdown {
	fn elem( &self) -> &HtmlElement { &self.elem }
}

// impl<'a> UiElement for &'a Dropdown {}
impl<'a> UiElement for &'a Dropdown {
	fn elem( &self) -> &HtmlElement { &self.elem }
}

/// A label range control
/// there is a textbox that can be changed
/// "-" and "+" buttons that can be clicked
/// and a slider than can be dragged
/// any action (changing the textbox, dragging the slider, or 
/// clicking the buttons) will change the value shown in the
/// textbox and position of the slider
#[derive(Debug,Clone)]
pub struct Range{
	elem : HtmlElement,
	input : InputElement,
	plus_button : InputElement,
	minus_button: InputElement,
	slider : InputElement,
}

impl Range {
	pub fn new( 
		name: &str, text: &str,
		min: f64, max: f64,
		slider_step: f64, button_step: f64
		) -> Range {
		// create the wrapper
		let elem : HtmlElement = document().create_element( "div").unwrap().try_into().unwrap();
		elem.class_list().add("control").unwrap();
		elem.set_attribute("id",name).unwrap();
		elem.set_attribute("data-param-type","range").unwrap();

		// create the text
		let text = document().create_text_node( text);
		// create textarea
		let input : InputElement = document().create_element( "input").unwrap().try_into().unwrap();
		input.set_attribute("type", "text").unwrap();
		input.set_attribute("id", &format!("{}_text",name)).unwrap();
		input.set_raw_value(&format!("{}",min));
		// create buttons
		let minus_button : InputElement = document().create_element( "input").unwrap().try_into().unwrap();
		minus_button.class_list().add("pm_button").unwrap();
		minus_button.set_attribute("type","button").unwrap();
		minus_button.set_attribute("id", &format!("{}_-", name)).unwrap();
		minus_button.set_attribute("value", "-").unwrap();
		let plus_button : InputElement = document().create_element( "input").unwrap().try_into().unwrap();
		plus_button.class_list().add("pm_button").unwrap();
		plus_button.set_attribute("type","button").unwrap();
		plus_button.set_attribute("id", &format!("{}_+", name)).unwrap();
		plus_button.set_attribute("value", "+").unwrap();
		// create the slider
		let slider : InputElement = document().create_element( "input").unwrap().try_into().unwrap();
		slider.class_list().add("control_slider").unwrap();
		slider.set_attribute("type", "range").unwrap();
		slider.set_attribute("id",&format!("{}_slider",name)).unwrap();
		slider.set_attribute("min",&min.to_string()).unwrap();
		slider.set_attribute("max",&max.to_string()).unwrap();
		slider.set_attribute("step",&slider_step.to_string()).unwrap();
		slider.set_raw_value(&min.to_string());

		// Add functions for changing the text input.
		// upon changing, update the slider
		input.add_event_listener({
			let input = input.clone();
			let slider = slider.clone();
			let min = min.clone();
			let max = max.clone();
			move | _:ChangeEvent | {
				let old_value : f64 = slider.raw_value().parse().unwrap();
				let new_value : f64 = input.raw_value().parse().unwrap_or(old_value);
				let new_value = max.min(new_value);
				let new_value = min.max(new_value);
				slider.set_raw_value( &new_value.to_string());
				input.set_raw_value( &slider.raw_value());
			}
		});

		// Add functions for pressing the buttons.
		// Changes the value in the input and slider by amount `button_step`
		minus_button.add_event_listener({
			let input = input.clone();
			let slider = slider.clone();
			let button_step = button_step.clone();
			let min = min.clone();
			move | _:ClickEvent | {
				let value : f64 = slider.raw_value().parse().unwrap();
				let new_value = min.max(value-button_step);
				slider.set_raw_value( &new_value.to_string());
				input.set_raw_value( &slider.raw_value());
			}
		});
		plus_button.add_event_listener({
			let input = input.clone();
			let slider = slider.clone();
			let button_step = button_step.clone();
			let max = max.clone();
			move | _:ClickEvent | {
				let value : f64 = slider.raw_value().parse().unwrap();
				let new_value = max.min(value+button_step);
				slider.set_raw_value( &new_value.to_string());
				input.set_raw_value( &slider.raw_value());
			}
		});

		// Add function for changing the slider.
		// Changes the value shown in the text-input.
		slider.add_event_listener( {
			let input = input.clone();
			let slider = slider.clone();
			move | _:InputEvent | {
				let value = slider.raw_value();
				input.set_raw_value(&value);		
			}
		});

		elem.append_child( &text);
		elem.append_child( &input);
		elem.append_child( &minus_button);
		elem.append_child( &plus_button);
		let br = document().create_element("br").unwrap();
		elem.append_child( &br);
		elem.append_child( &slider);
		// Output the Range control
		Range{ 
			elem: elem, 
			input: input,
			minus_button: minus_button,
			plus_button: plus_button, 
			slider: slider,
		}
	}

	pub fn query( &self) -> f64 {
		let value: f64 = self.slider.raw_value().parse().unwrap();
		value
	}

	pub fn set( &self, value: f64) {
		self.slider.set_raw_value( &value.to_string());
		self.input.set_raw_value( &self.slider.raw_value());
	}

	pub fn add_range_function<F>( &self, func:F)
		where F: Fn(f64) + 'static {
		let func = Rc::new(func);
		self.input.add_event_listener({
			let func = func.clone();
			let range = self.clone();
			move | _:ChangeEvent | {
				let val = range.query();
				func(val);
			}
		});
		self.minus_button.add_event_listener({
			let func = func.clone();
			let range = self.clone();
			move | _:ClickEvent | {
				let val = range.query();
				func(val);
			}
		});
		self.plus_button.add_event_listener({
			let func = func.clone();
			let range = self.clone();
			move | _:ClickEvent | {
				let val = range.query();
				func(val);
			}
		});
		self.slider.add_event_listener({
			let func = func.clone();
			let range = self.clone();
			move | _:ChangeEvent | {
				let val = range.query();
				func(val);
			}
		});
	}

	pub fn add_continuous_range_function<F>( &self, func:F)
		where F: Fn(f64) + 'static {
		let func = Rc::new(func);
		self.input.add_event_listener({
			let func = func.clone();
			let range = self.clone();
			move | _:ChangeEvent | {
				let val = range.query();
				func(val);
			}
		});
		self.minus_button.add_event_listener({
			let func = func.clone();
			let range = self.clone();
			move | _:ClickEvent | {
				let val = range.query();
				func(val);
			}
		});
		self.plus_button.add_event_listener({
			let func = func.clone();
			let range = self.clone();
			move | _:ClickEvent | {
				let val = range.query();
				func(val);
			}
		});
		self.slider.add_event_listener({
			let func = func.clone();
			let range = self.clone();
			move | _:InputEvent | {
				let val = range.query();
				func(val);
			}
		});
	}
}

// impl UiElement for Range {}
impl UiElement for Range {
	fn elem( &self) -> &HtmlElement { &self.elem }
}

