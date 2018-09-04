use ::gfx::Graphic;

use ::simple_vec::Vec2 as Point;
use ::simple_vec::Transform2D as Transform;

use ::container::UiElement;
use ::container::Container;
use ::control::Toggle;

fn gen_spring_points( n: u32) -> Vec<Point> {
   	let c = 0.551915024494;
	let mut pts = Vec::new();
	pts.push(Point{x:-1.0, y:0.0});
	pts.push(Point{x:-1.0, y:c});
	pts.push(Point{x:-c, y: 1.0});
	for _ in 0..n {
		pts.push(Point{x:0.0, y:1.0});
		pts.push(Point{x:c, y:1.0});
		pts.push(Point{x:1.0, y:c});
		pts.push(Point{x:1.0, y:0.0});
		pts.push(Point{x:1.0, y:-c});
		pts.push(Point{x:c, y:-1.0});
		pts.push(Point{x:0.0, y:-1.0});
		pts.push(Point{x:-c, y:-1.0});
		pts.push(Point{x:-1.0, y:-c});
		pts.push(Point{x:-1.0, y:0.0});
		pts.push(Point{x:-1.0, y:c});
		pts.push(Point{x:-c, y:1.0});
	}
	pts.push(Point{x:0.0, y:1.0});
	pts.push(Point{x:c, y:1.0});
	pts.push(Point{x:1.0, y:c});
	pts.push(Point{x:1.0, y:0.0});
	pts
}

pub fn spring(
	num_loops: u32,
	default_length: f64,
	p0: Point,
	p1: Point,
	) -> Graphic {
	let inv_unit_length = 1.0/(1.5+0.8*(num_loops as f64));
	let loop_height = default_length*inv_unit_length;
	let loop_width = 0.5*default_length*inv_unit_length;
	let leader = 0.5*default_length*inv_unit_length;
	let spring_length = (p0-p1).norm();
	let mut spring_points = gen_spring_points(num_loops);
	// First we scale the loop
	let scale = Transform::scale_xy(loop_width, loop_height, Point{x:0.0, y:0.0});
	for point in spring_points.iter_mut() {
		*point = point.transform(scale);
	}
	// Get the length of the loopy section
	let loop_length = spring_length - 2.0*loop_width - 2.0*leader;
	// Get the shift per point
	let dx = loop_length/( (spring_points.len() - 1) as f64);
	// Shift the points for the loopy section
	for (i, point) in spring_points.iter_mut().enumerate() {
		let shift = (i as f64)*dx + leader + loop_width;
		let shift = Transform::translate(shift,0.0);
		*point = point.transform(shift);
	}
	// Create the leader points
	let s0 = spring_points[0];
	let mut leader_points0 = [s0 - Point{x:leader, y:0.0}, s0];
	let s1 = spring_points[spring_points.len()-1];
	let mut leader_points1 = [s1, s1 + Point{x:leader, y:0.0}];
	// Rotate and translate everything to the appropriate points
	let c = (p1.x-p0.x)/spring_length;
	let s = (p1.y-p0.y)/spring_length;
	let transform = Transform{a:c, b: -s, c: s, d: c, dx:p0.x, dy:p0.y};
	for point in leader_points0.iter_mut() {
		*point = point.transform(transform);
	}
	for point in spring_points.iter_mut() {
		*point = point.transform(transform);
	}
	for point in leader_points1.iter_mut() {
		*point = point.transform(transform);
	}
	// Create the graphics
	let leader0 = Graphic::line(&leader_points0);
	let spring = Graphic::bezier(&spring_points);
	let leader1 = Graphic::line(&leader_points1);
	Graphic::collection(&[leader0, spring, leader1])
}

pub fn hiding_container( name: &str) -> Container {
	let outer = Container::new(name);
	let button = Toggle::new(&format!("{}_hide_button",name),"Hide","Show");
	let inner = Container::new(&format!("{}_inner",name));
	outer.add( &button);
	outer.add( &inner);
	button.add_toggle_function({
		let inner = inner.clone();
		move | hidden:bool | {
			if hidden {
				inner.hide();
			} else {
				inner.unhide();
			}
		}
	});
	outer
}

