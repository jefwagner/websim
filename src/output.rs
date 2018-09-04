extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
	HtmlElement,
	document,
	CanvasRenderingContext2d
};
use stdweb::web::html_element::{
	CanvasElement,
	TextAreaElement,
	InputElement,
};
use stdweb::web::event::{
 	ChangeEvent,
 	ClickEvent,
 	InputEvent,
};

// use ::control::Button;
use ::container::UiElement;
// use ::gfx::Drawable;
use ::gfx::Graphic;

const DEFAULT_CANVAS_WIDTH : u32 = 500;
const DEFAULT_CANVAS_HEIGHT : u32 = 500;
const DEFAULT_CANVAS_WINDOW : ((f64,f64),(f64,f64)) = ((0_f64,0f64),(1_f64,1_f64));

#[derive(Debug, Clone)]
pub struct Canvas{
	elem: HtmlElement,
	pub canvas: CanvasElement,
	pub context: CanvasRenderingContext2d,
	window: ((f64, f64), (f64,f64))
}

impl Canvas {
	pub fn new( name: &str) -> Canvas {
		// Create the div wrapper and add the attributes
		let elem : HtmlElement = document().create_element( "div").unwrap().try_into().unwrap();
		elem.class_list().add("output").unwrap();
		elem.set_attribute("data-output-type", "canvas").unwrap();
		elem.set_attribute("id",name).unwrap();
		// Create the canvas wrapper and add the attributes and add to wrapper
		let canvas : CanvasElement = document().create_element("canvas").unwrap().try_into().unwrap();
		canvas.class_list().add("output_canvas").unwrap();
		canvas.set_attribute("id",&format!("{}_canvas",name)).unwrap();
		elem.append_child( &canvas);
		// Grab the context
		let context : CanvasRenderingContext2d = canvas.get_context().unwrap();
		// Create the `Canvas` struct with empty window
		let mut canvas = Canvas{ 
				elem: elem, 
				canvas: canvas, 
				context: context, 
				window: DEFAULT_CANVAS_WINDOW,
			};
		// set default width and height
		canvas.set_width_height(DEFAULT_CANVAS_WIDTH, DEFAULT_CANVAS_HEIGHT);
		// set default window
		canvas.clear();
		canvas
	}

	pub fn get( name: &str) -> Canvas {
		let elem : HtmlElement = document().query_selector( &format!("#{}",name)).unwrap().unwrap().try_into().unwrap();
		let canvas : CanvasElement = elem.last_child().unwrap().try_into().unwrap();
		let context : CanvasRenderingContext2d = canvas.get_context().unwrap();
		Canvas{ 
			elem: elem, 
			canvas: canvas, 
			context: context, 
			window: DEFAULT_CANVAS_WINDOW,
		}
	}

	fn set_transform<'a>(&'a mut self) -> &'a Canvas {
		let ((xlower_left, ylower_left), (width,height)) = self.window;
		let pixel_width: f64 = self.canvas.width() as f64;
		let pixel_height: f64 = self.canvas.height() as f64;
		let scale_x = pixel_width/width;
		let scale_y = -pixel_height/height;
		let move_x = -pixel_width/width*xlower_left;
		let move_y = pixel_height+pixel_height/height*ylower_left;
		self.context.set_transform(scale_x, 0_f64, 0_f64, scale_y, move_x, move_y);
		self
	}

	pub fn set_width_height<'a>( &'a mut self, width: u32, height: u32) -> &'a Canvas {
		self.canvas.set_width( width);
		self.canvas.set_height( height);
		self.set_transform()		
	}

	pub fn width(&self) -> u32 { self.canvas.width() }

	pub fn set_width<'a>( &'a mut self, width: u32) -> &'a Canvas {
		self.canvas.set_width( width);
		self.set_transform()		
	}

	pub fn height(&self) -> u32 { self.canvas.height() }

	pub fn set_height<'a>( &'a mut self, height: u32) -> &'a Canvas {
		self.canvas.set_height( height);
		self.set_transform()		
	}

	pub fn window(&self) -> ((f64,f64),(f64,f64)) { self.window }

	pub fn set_window<'a>( &'a mut self, window: ((f64,f64),(f64,f64)) ) -> &'a Canvas {
		self.window = window;
		self.set_transform()		
	}

	pub fn clear<'a>( &'a self) -> &'a Canvas {
		let ((x,y),(w,h)) = self.window();
		self.context.clear_rect( x, y, w, h);
		self.context.fill_rect( x, y, w, h);
		self
	}

	pub fn draw<'a>( &'a self, object: &Graphic) -> &'a Canvas {
		object.draw( &self);
		self
	}
}

// impl UiElement for Canvas {}
impl UiElement for Canvas {
	fn elem( &self) -> &HtmlElement { &self.elem }
}

// impl<'a> UiElement for &'a Canvas {}
impl<'a> UiElement for &'a Canvas {
	fn elem( &self) -> &HtmlElement { &self.elem }
}


#[derive(Debug, Clone)]
pub struct TextArea {
	elem: HtmlElement,
	text_area: TextAreaElement,
}

impl TextArea {
	pub fn new( name: &str) -> TextArea {
		let elem : HtmlElement = document().create_element( "div").unwrap().try_into().unwrap();
		elem.class_list().add("output").unwrap();
		elem.set_attribute("data-output-type", "textarea").unwrap();
		elem.set_attribute("id",name).unwrap();
		let text_area : TextAreaElement = document().create_element("textarea").unwrap().try_into().unwrap();
		text_area.set_attribute("id", &format!("{}_text_area",name)).unwrap();
		text_area.set_attribute("cols", "72").unwrap();
		text_area.set_attribute("rows", "10").unwrap();
		text_area.set_attribute("wrap", "off").unwrap();
		elem.append_child( &text_area);
		TextArea{ elem: elem, text_area: text_area,}
	}

	pub fn get( name: &str) -> TextArea {
		let elem : HtmlElement = document().query_selector( &format!("#{}",name)).unwrap().unwrap().try_into().unwrap();
		let text_area : TextAreaElement = elem.last_child().unwrap().try_into().unwrap();
		TextArea{ elem: elem, text_area: text_area, }
	}

	pub fn set_cols<'a>( &'a self, cols: u32) -> &'a TextArea {
		self.text_area.set_attribute("cols", &cols.to_string()).unwrap();
		self
	}

	pub fn set_rows<'a>( &'a self, rows: u32) -> &'a TextArea {
		self.text_area.set_attribute("rows", &rows.to_string()).unwrap();
		self
	}

	// pub fn create_clear_button(&self) -> Button {
	// 	let elem : HtmlElement = document().create_element("span").unwrap().try_into().unwrap();
	// 	elem.class_list().add("control").unwrap();
	// 	elem.set_attribute("data-param-type", "button").unwrap();
	// 	let button : InputElement = document().create_element( "input").unwrap().try_into().unwrap();
	// 	button.class_list().add("control_button").unwrap();
	// 	button.set_attribute("type","button").unwrap();
	// 	button.set_attribute("value", "Clear").unwrap();
	// 	button.add_event_listener({
	// 		let text_area = self.clone();
	// 		move | _:ClickEvent | {
	// 			text_area.clear();
	// 		}
	// 	});
	// 	elem.append_child( &button);
	// 	Button::create_from(elem, button)
	// }

	pub fn clear<'a>( &'a self) -> &'a TextArea {
		self.text_area.set_value( "");
		self
	}

	pub fn write<'a>( &'a self, new_text: &str) -> &'a TextArea {
		let mut text = self.text_area.value();
		text.push_str( new_text);
		self.text_area.set_value( &text);
		self
	}

	pub fn writeln<'a>( &'a self, new_text: &str) -> &'a TextArea {
		let mut text = self.text_area.value();
		text.push_str( new_text);
		text.push('\n');
		self.text_area.set_value( &text);
		self
	}
}

impl UiElement for TextArea {
	fn elem( &self) -> &HtmlElement { &self.elem }
}

impl<'a> UiElement for &'a TextArea {
	fn elem( &self) -> &HtmlElement { &self.elem }
}
