extern crate websim;

use std::rc::Rc;

use websim::container::Container;
use websim::output::{
	Canvas,
	TextArea
};
use websim::control::Range;
use websim::gfx::Graphic;

use websim::simple_vec::Vec2 as Point;
use websim::simple_vec::Transform2D as Transform;

fn p(x:f64,y:f64) -> Point {Point{x,y}}

fn sp_p( n: u32) -> Vec<Point> {
    let c = 0.551915024494;
	let mut pts = Vec::new();
	pts.push(p(-1.0,0.0));
	pts.push(p(-1.0,c));
	pts.push(p(-c, 1.0));
	for _ in 0..n {
		pts.push(p(0.0,1.0));
		pts.push(p(c,1.0));
		pts.push(p(1.0,c));
		pts.push(p(1.0,0.0));
		pts.push(p(1.0,-c));
		pts.push(p(c,-1.0));
		pts.push(p(0.0,-1.0));
		pts.push(p(-c,-1.0));
		pts.push(p(-1.0,-c));
		pts.push(p(-1.0,0.0));
		pts.push(p(-1.0,c));
		pts.push(p(-c,1.0));
	}
	pts.push(p(0.0,1.0));
	pts.push(p(c,1.0));
	pts.push(p(1.0,c));
	pts.push(p(1.0,0.0));
	pts
}


fn draw_spring(
	ln0: &[Point], 
	sp: &[Point], 
	ln1: &[Point], 
	canvas: &Canvas) {
	let ln0 = Graphic::line(ln0);
	let sp = Graphic::bezier(sp);
	let ln1 = Graphic::line(ln1);
    canvas.clear();
	ln0.draw(&canvas);
	sp.draw(&canvas);
	ln1.draw(&canvas);
}

#[derive(Debug)]
struct Spring {
	pub leader: f64,
	pub loops: u32,
	pub loop_height: f64,
	pub loop_width: f64,
	pub spring_length: f64,
}

fn make_spring( spring: &Spring, canvas: &Canvas) {
	let mut sp = sp_p(spring.loops);
	let scale = Transform::scale_xy(
		spring.loop_width, 
		spring.loop_height,
		p(0.0,0.0));
	for pt in sp.iter_mut() {
		*pt = pt.transform(scale);
	};
	let loop_length = spring.spring_length 
		- 2.0*spring.loop_width 
		- 2.0*spring.leader;
	let dx = loop_length/( (sp.len() - 1) as f64);
	for (i, pt) in sp.iter_mut().enumerate() {
		let shift = (i as f64)*dx + spring.leader + spring.loop_width;
		let shift = Transform::translate(shift,0.0);
		*pt = pt.transform(shift);
	}
	let p0 = sp[0];
	let ln0 = [p0 - Point{x:spring.leader, y:0.0}, p0];
	let p1 = sp[sp.len()-1];
	let ln1 = [p1, p1 + Point{x:spring.leader, y:0.0}];
	draw_spring( &ln0, &sp, &ln1, &canvas);		
}

fn main() {
    let app = Container::new("app");
    app.add_to_body();
    let mut canvas = Canvas::new("canvas");
    app.add( &canvas);
    let sldr0 = Range::new("sldr0", "leader : ", 0.0, 0.5, 0.01, 0.1);
    let sldr1 = Range::new("sldr1", "loops : ", 1.0, 10.0, 1.0, 1.0);
    let sldr2 = Range::new("sldr2", "loop_height : ", 0.05, 1.0, 0.01, 0.05);
    let sldr3 = Range::new("sldr3", "loop_width : ", 0.05, 1.0, 0.01, 0.05);
    let sldr4 = Range::new("sldr4", "spring_length : ", 1.0, 8.0, 0.01, 0.1);
    app.add( &sldr0);
    app.add( &sldr1);
    app.add( &sldr2);
    app.add( &sldr3);
    app.add( &sldr4);
    let textarea = TextArea::new("text");
    app.add( &textarea);

    canvas.set_width_height(500,250);
    canvas.set_window(((0.0,-2.0),(8.0,4.0)));

    let my_spring = {
    	let canvas = canvas.clone();
    	let sldr0 = sldr0.clone();
    	let sldr1 = sldr1.clone();
    	let sldr2 = sldr2.clone();
    	let sldr3 = sldr3.clone();
    	let sldr4 = sldr4.clone();
    	move | _:f64 | {
		    let spring = Spring{
	   	 		leader: sldr0.query(),
    			loops: (sldr1.query() as u32),
    			loop_height: sldr2.query(),
    			loop_width: sldr3.query(),
    			spring_length: sldr4.query(),
    		};
	    	make_spring( &spring, &canvas);    		
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
    let spring = Spring{
    	leader: 0.5,
    	loops: 3,
    	loop_height: 1.0,
    	loop_width: 0.5,
    	spring_length: 6.0,
    };
    make_spring( &spring, &canvas);

	textarea.writeln("Hello world!");
}