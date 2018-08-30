extern crate websim;

use websim::container::Container;
use websim::output::{
	TextArea,
	Canvas,
};

use websim::simple_vec::Vec2 as Point;
use websim::simple_color::Color::Rgb;
use websim::gfx::Graphic;

fn tp(x:f64, y:f64) -> Point {
	Point{x,y}
}

fn sort_bezier_points( points: &[Point]) -> (Point,Vec<(Point,Point,Point)>) {
	let mut point_iter = points.iter();
	let start = *point_iter.next().unwrap();
	let mut others = Vec::new();
	let mut count = 0;
	let mut three_set = (tp(0.0,0.0),tp(0.0,0.0),tp(0.0,0.0));
	for point in point_iter {
		match count {
			0 => {three_set.0 = *point; count += 1;},
			1 => {three_set.1 = *point; count += 1;},
			2 => {
				three_set.2 = *point;
				count = 0;
				others.push(three_set);
			},
			_ => {}
		};
	};
	(start, others)
}

fn main() {
    let app = Container::new("app");
    app.add_to_body();
    let mut canvas = Canvas::new("canvas");
    canvas.set_window(((-4.0,-4.0),(8.0,8.0)));
    app.add(&canvas);
    let textarea = TextArea::new("txt");
    app.add(&textarea);

    let c = 0.551915024494;

    let pts = vec!(
    	tp(0.0,1.0),
    	tp(c,1.0),
    	tp(1.0,c),
    	tp(1.0,0.0),
    	tp(1.0,-c),
    	tp(c,-1.0),
    	tp(0.0,-1.0),
    	tp(-c,-1.0),
    	tp(-1.0,-c),
    	tp(-1.0,0.0),
    	tp(-1.0,c),
    	tp(-c,1.0),
    	tp(0.0,1.0),
    );

    let mut offset = 0.0;
    let dx = 0.15;
    let mut new_pts = Vec::new();
    for point in pts.iter().cloned() {
    	new_pts.push(point+tp(offset,0.0));
    	offset += dx;
    }
    offset -= dx;

    let mut bezier = Graphic::bezier(&new_pts);
    bezier.translate(-offset/2.0,0.0).draw( &canvas);
    bezier.translate(offset,0.0).draw( &canvas);
    textarea.writeln("Hello World");

    let center = Point{x:-2.0,y:2.0};
    let mut circ = Graphic::circle_outline(center, 1.0);
    circ.set_color(Rgb{r:255,g:0,b:0})
    	.draw( &canvas);

}