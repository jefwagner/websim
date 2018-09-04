extern crate websim;

use std::rc::Rc;

use websim::container::Container;
use websim::output::{
	Canvas,
	TextArea
};
use websim::control::Range;
use websim::extra::spring;

use websim::simple_vec::Vec2 as Point;

fn main() {
    let app = Container::new("app");
    app.add_to_body();
    let mut canvas = Canvas::new("canvas");
    app.add( &canvas);
    let sldr0 = Range::new("p0x","p0.x : ", 0.0, 7.0, 0.01, 0.1);
    let sldr1 = Range::new("p0y","p0.y : ", 0.0, 7.0, 0.01, 0.1);
    let sldr2 = Range::new("p1x","p1.x : ", 1.0, 8.0, 0.01, 0.1);
    let sldr3 = Range::new("p1y","p1.y : ", 1.0, 8.0, 0.01, 0.1);
    let sldr4 = Range::new("l","default length : ", 1.0, 5.0, 0.01, 0.1);
    let sldr5 = Range::new("n","number of loops : ", 1.0, 8.0, 1.0, 1.0);
    app.add( &sldr0);
    app.add( &sldr1);
    app.add( &sldr2);
    app.add( &sldr3);
    app.add( &sldr4);
    app.add( &sldr5);
    let textarea = TextArea::new("text");
    app.add( &textarea);

    canvas.set_width_height(500,500);
    canvas.set_window(((0.0,0.0),(8.0,8.0)));

    let my_spring = {
    	let canvas = canvas.clone();
    	let sldr0 = sldr0.clone();
    	let sldr1 = sldr1.clone();
    	let sldr2 = sldr2.clone();
    	let sldr3 = sldr3.clone();
    	let sldr4 = sldr4.clone();
    	let sldr5 = sldr5.clone();
    	move | _:f64 | {
    		let p0 = Point{x: sldr0.query(), y: sldr1.query()};
    	 	let p1 = Point{x: sldr2.query(), y: sldr3.query()};
    		let l = sldr4.query();
    		let n = sldr5.query() as u32;
    		let spring = spring::spring(n,l,p0,p1);
    		canvas.clear();
    		spring.draw( &canvas);
    	}
    };
    let my_spring = Rc::new(my_spring);

    sldr0.add_continuous_range_function({
    	let my_spring = my_spring.clone();
    	move |x| my_spring(x)
    });
    sldr1.add_continuous_range_function({
    	let my_spring = my_spring.clone();
    	move |x| my_spring(x)
    });
    sldr2.add_continuous_range_function({
    	let my_spring = my_spring.clone();
    	move |x| my_spring(x)
    });
    sldr3.add_continuous_range_function({
    	let my_spring = my_spring.clone();
    	move |x| my_spring(x)
    });
    sldr4.add_continuous_range_function({
    	let my_spring = my_spring.clone();
    	move |x| my_spring(x)
    });
    sldr5.add_continuous_range_function({
    	let my_spring = my_spring.clone();
    	move |x| my_spring(x)
    });

	textarea.writeln("Hello world!");
}