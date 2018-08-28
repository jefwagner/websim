/// Defines containers, to which you can add divs and other stuff
extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
	HtmlElement,
	document,
};
use stdweb::web::html_element::{
	InputElement,
};
use stdweb::web::event::{
 	ClickEvent,
};

pub trait UiElement {
	fn elem( &self) -> &HtmlElement; 
}

/// A basic container consisting of a div, to which we can add UiElements
#[derive(Debug, Clone)]
pub struct Container {
	elem: HtmlElement,
}

impl Container {
	pub fn new( name: &str) -> Container {
		let elem : HtmlElement = document().create_element( "div").unwrap().try_into().unwrap();
		elem.class_list().add("container").unwrap();
		elem.set_attribute("id",name).unwrap();
		Container{ elem: elem }		
	}

	pub fn get( name: &str) -> Container {
		let elem : HtmlElement = document().query_selector( &format!("#{}",name)).unwrap().unwrap().try_into().unwrap();
		Container{ elem: elem }
	}

	pub fn add<T: UiElement>( &self, object: &T) {
		self.elem.append_child( object.elem());
	}

	pub fn add_to_body( &self){
		let body = document().body().unwrap();
		body.append_child( self.elem());
	}
}

//impl UiElement for Container {}
impl UiElement for Container {
	fn elem( &self) -> &HtmlElement{ &self.elem }
}

// A hideable container with text `text` and a button to hide it's contents
#[derive(Debug, Clone)]
pub struct HideableContainer {
	elem: HtmlElement,
	inner: HtmlElement,
}

 impl HideableContainer {
	pub fn new( name: &str, text: &str) -> HideableContainer {
		let elem : HtmlElement = document().create_element( "div").unwrap().try_into().unwrap();
		elem.class_list().add("container").unwrap();
		elem.set_attribute("id",name).unwrap();
		let text = document().create_text_node(text);
		elem.append_child( &text);
		let hide_button : InputElement = document().create_element("input").unwrap().try_into().unwrap();
		hide_button.set_attribute("id", &format!("{}_button",name)).unwrap();
		hide_button.set_attribute("type", "button").unwrap();
		hide_button.set_attribute("value", "Hide").unwrap();
		hide_button.set_attribute("data-toggle-state", "off").unwrap();
		elem.append_child( &hide_button);
		let inner : HtmlElement = document().create_element( "div").unwrap().try_into().unwrap();
		inner.class_list().add("container_inner").unwrap();
		inner.set_attribute("id", &format!("{}_inner", name)).unwrap();
		elem.append_child( &inner);
		hide_button.add_event_listener({
			// let name = String::from(name);
			let button = hide_button.clone();
			let inner = inner.clone();
			move | _:ClickEvent | {
				// let button : InputElement = document().query_selector( &format!("#{}_button",name)).unwrap().unwrap().try_into().unwrap();
				// let inner : HtmlElement = document().query_selector( &format!("#{}_inner",name)).unwrap().unwrap().try_into().unwrap();
				let test = button.get_attribute("data-toggle-state").unwrap() == "off";
				if test {
					button.set_attribute("value", "Unhide").unwrap();
					button.set_attribute("data-toggle-state", "on").unwrap();
					inner.set_attribute("style", "display: none;").unwrap();
				} else {
					button.set_attribute("value", "Hide").unwrap();
					button.set_attribute("data-toggle-state", "off").unwrap();
					inner.set_attribute("style", "display: block;").unwrap();
				};
			}
		});
		HideableContainer{ elem: elem, inner: inner}		
	}

	pub fn add<T: UiElement>( &self, object: &T) {
		self.inner.append_child( object.elem());
	}
}

// impl UiElement for HideableContainer {}
impl UiElement for HideableContainer {
	fn elem( &self) -> &HtmlElement { &self.elem }
}
