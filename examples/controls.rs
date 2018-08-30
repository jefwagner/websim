extern crate websim;

use websim::container::Container;
use websim::output::TextArea;
use websim::control::{
	Checkbox,
	Button,
	Toggle,
	Dropdown,
	Range,
};

fn main() {
    let app = Container::new("app");
    app.add_to_body();
    let textarea = TextArea::new("text");
    app.add( &textarea);

    let checkbox = Checkbox::new("chbx","My Checkbox : ");
    app.add( &checkbox);

    textarea.writeln("Hello world!");

    checkbox.add_check_function({
    	let textarea = textarea.clone();
    	move | checked:bool | {
    		textarea.writeln(&format!("Checkbox : {}",checked));
    	}
    });

    let button = Button::new("bt0", "Press Me!");
    app.add( &button);

    button.add_button_function({
    	let textarea = textarea.clone();
    	move | pressed:bool | {
    		textarea.writeln(&format!("Button Pressed : {}",pressed));
    	}
    });

    let toggle = Toggle::new("tg0", "Start", "Stop");
    app.add( &toggle);
    toggle.add_toggle_function({
    	let textarea = textarea.clone();
    	move | is_on:bool | {
    		let text = if is_on { 
    			String::from("On") 
    		} else {
    			String::from("Off")
    		};
    		textarea.writeln(&format!("Toggle Switch State : {}",text));
    	}    	
    });

    let clear = Button::new("bt1", "Clear");
    app.add( &clear);

    clear.add_button_function({
    	let textarea = textarea.clone();
    	move | _:bool | {
    		textarea.clear();
    	}
    });

    let mut opts = Dropdown::new("dd0", "Select One : ");
    opts.add_multiple_options(&[("one","One"),("two","Two"),("three","Three")]);
    app.add( &opts);

    opts.add_dropdown_function({
    	let textarea = textarea.clone();
    	move | opt : String | {
    		match &opt {
    			s if s == "one" => {
    				textarea.writeln(&format!("{}: Good Choice", s));
    			},
    			_ => {
    				textarea.writeln(&format!("{}: You were supposed to choose \"One\".",opt));
    			},
    		}
    	}
    });

    let slider = Range::new("range0", "New Value : ", 0.0, 100.0, 1.0, 5.0);
    app.add( &slider);

    slider.add_range_function({
    	let textarea = textarea.clone();
    	move | val:f64 | {
    		textarea.writeln(&format!("Your value changed to: {}", val));
    	}
    });

}