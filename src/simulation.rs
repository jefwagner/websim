/// A container that runs the simulation loop.

use std::boxed::Box;
use std::rc::Rc;
use std::cell::RefCell;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
	window,
	// document,
	// HtmlElement,
	RequestAnimationFrameHandle,
 };
// use stdweb::web::html_element::InputElement;
// use stdweb::web::event::ClickEvent;

// use ::control::Toggle;

const MAX_TIME_STEP : f64 = 16.66666666666666666666666666666666666666666666666666666666;

pub trait SimStep {
	fn step(&mut self, dt: f64);
}

#[derive(Debug)]
pub struct Simloop<T: SimStep + 'static> {
	time_old: f64,
	handle: Option<RequestAnimationFrameHandle>,
	pub state: Box<T>,
}


impl<T: SimStep + 'static> Simloop<T> {
	fn step_frame(&mut self, time: f64, refstate: Rc<RefCell<Self>>) {
		let dt = if time > 0_f64 { 
			MAX_TIME_STEP.min(time-self.time_old)
		} else {
			MAX_TIME_STEP
		};
		self.time_old = time;
		self.state.step(dt);
		self.handle = Some(
			window().request_animation_frame( move | time | {
				refstate.borrow_mut().step_frame(time, refstate.clone());
			})
		);
	}

	pub fn new( state: T) -> Simloop<T> {
		Simloop{
			time_old: 0_f64,
			handle: None,
			state: Box::new(state),
		}
	}

	pub fn new_ref( state: T) -> Rc<RefCell<Simloop<T>>> {
		Rc::new(RefCell::new(Simloop::new(state)))
	}

	pub fn stop_loop(&mut self) {
		let maybe_handle = self.handle.take();
		match maybe_handle {
			Some(handle) => {
				handle.cancel();
			},
			None => {},
		};
	}

	pub fn start_loop( refstate: Rc<RefCell<Simloop<T>>>) {
		refstate.borrow_mut().step_frame(0_f64 , refstate.clone());
	}

	// pub fn create_toggle( refstate: Rc<RefCell<Simloop<T>>> ) -> Toggle {
	// 	let elem : HtmlElement = document().create_element("span").unwrap().try_into().unwrap();
	// 	elem.class_list().add("control").unwrap();
	// 	elem.set_attribute("data-param-type", "toggle").unwrap();
	// 	let button : InputElement = document().create_element( "input").unwrap().try_into().unwrap();
	// 	button.class_list().add("control_toggle").unwrap();
	// 	button.set_attribute("type","button").unwrap();
	// 	button.set_attribute("value", "Start").unwrap();
	// 	button.set_attribute("data-toggle-state", "off").unwrap();
	// 	button.add_event_listener({
	// 		let button = button.clone();
	// 		let refstate = refstate.clone();//?
	// 		move | _:ClickEvent | {
	// 			let test = button.get_attribute("data-toggle-state").unwrap() == "off";
	// 			if test {
	// 				button.set_attribute("data-toggle-state", "on").unwrap();
	// 				Simloop::start_loop( refstate.clone());
	// 				button.set_raw_value( "Stop");
	// 			} else {
	// 				button.set_attribute("data-toggle-state", "off").unwrap();
	// 				refstate.borrow_mut().stop_loop();
	// 				button.set_raw_value( "Start");
	// 			};
	// 		}
	// 	});
	// 	elem.append_child( &button);
	// 	Toggle::create_from(elem, button)
	// }
}
