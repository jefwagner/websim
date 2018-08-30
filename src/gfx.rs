/// Graphics Primatives
/// ===================

use std::f64::consts::PI;

use stdweb::traits::*;
use stdweb::web::CanvasRenderingContext2d;
use stdweb::web::FillRule::NonZero;

use ::output::Canvas;

use ::simple_vec::Vec2 as Point;
use ::simple_vec::Transform2D as Transform;
use ::simple_color::Color;
use ::simple_color::Color::Rgb;

const DEFAULT_SHAPE_COLOR : Color = Rgb{r:153,g:153,b:153};
const DEFAULT_LINE_COLOR : Color = Rgb{r:255,g:255,b:255};
const DEFAULT_RELATIVE_LINE_WIDTH : f64 = 0.004;

#[derive(Debug,Clone)]
enum Geometry {
	Circle{center: Point, radius: f64},
	Polygon{points: Vec<Point>},
	Line{points: Vec<Point>, rel_width: f64},
	Bezier{
		start: Point,
		others: Vec<(Point,Point,Point)>,
		rel_width: f64,
	},
}

#[derive(Debug,Clone)]
enum GfxObj {
	Primative{ geometry: Geometry,	color: Color},
	Collection{ elements: Vec<Graphic>},
}

#[derive(Debug,Clone)]
pub struct Graphic {
	object: GfxObj,
	transform: Transform,
	bounding_box: Option<(Point,Point)>,
}

/// This function finds the bounding box of a set of points
/// The return is pair of points giving the lower left corner
/// and the width and height.
fn point_set_bounding_box( points: &Vec<Point>, t: Transform) -> (Point, Point) {
	let mut point_iter = points.iter().map(|p|  p.transform(t));
	let p0 = point_iter.next().unwrap();
	let mut xmin = p0.x;
	let mut xmax = p0.x;
	let mut ymin = p0.y;
	let mut ymax = p0.y; 
	for point in point_iter {
			xmin = xmin.min(point.x);
			xmax = xmax.max(point.x);
			ymin = ymin.min(point.y);
			ymax = ymax.max(point.y);
	}
	let ll = Point{x:xmin, y:ymin};
	let wh = Point{x:xmax-xmin, y:ymax-ymin};
	(ll,wh)
}

/// This function find the real roots of a general quadradic:
///            a*x*x+b*x+c == 0
/// The return is a quadroot enum that contains the roots
#[derive(Debug,Clone)]
enum QuadRoots{
	Zero,
	One(f64),
	Two(f64,f64),
}

fn quad(a: f64, b: f64, c: f64) -> QuadRoots {
    if a == 0.0 {
        if b == 0.0 {
            QuadRoots::Zero
        } else {
        	QuadRoots::One(-c/b)
        }
    } else {
        let disc = b*b-4.0*a*c;
        if disc < 0.0 {
            QuadRoots::Zero
        } else if disc == 0.0 {
        	QuadRoots::One(-0.5*b/a)
        } else {
            let r1 = -0.5*(b+disc.sqrt())/a;
            let r2 = -0.5*(b-disc.sqrt())/a;
            QuadRoots::Two(r1,r2)
        }
    }
}

/// Evaluates a bezier curve at a point t
fn eval_bezier(
	t:f64, bezier_points:(Point,Point,Point,Point)
	) -> Point {
	let u = 1.0-t;
	let (p0, c0, c1, p1) = bezier_points;
	u*u*u*p0 + 3.0*u*u*t*c0 + 3.0*u*t*t*c1 + t*t*t*p1
}

/// Calculates the bounding box for a bezier curve
fn bezier_bounding_box( 
	start: Point, 
	others: &Vec<(Point, Point, Point)>,
	t: Transform,
	) -> (Point,Point) {

	// Get the x and y positions of the starting point
	let mut p0 = start.transform(t);
	let mut xmin = p0.x;
	let mut xmax = p0.x;
	let mut ymin = p0.y;
	let mut ymax = p0.y;
	// Loop over all the bezier segments
	for (c0_temp, c1_temp, p1_temp) in others.iter() {
		let c0 = c0_temp.transform(t);
		let c1 = c1_temp.transform(t);
		let p1 = p1_temp.transform(t);

		// Check the other endpoint
		xmin = xmin.min(p1.x);
		xmax = xmax.max(p1.x);
		ymin = ymin.min(p1.y);
		ymax = ymax.max(p1.y);

		// Get the coefficients for the quadratic formula
		let a = p1 - 3.0*c1 + 3.0*c0 - p0;
		let b = 2.0*(p0 - 2.0*c0 + c1);
		let c = c0 - p0;
		// Check for the x components
		match quad(a.x, b.x, c.x) {
			QuadRoots::Two(r0, r1) => {
				if r0 > 0.0 && r0 < 1.0 {
					let e = eval_bezier( r0, (p0, c0, c1, p1));
					xmin = xmin.min(e.x);
					xmax = xmax.max(e.x);
				};
				if r1 > 0.1 && r1 < 1.0 {
					let e = eval_bezier( r1, (p0, c0, c1, p1));
					xmin = xmin.min(e.x);
					xmax = xmax.max(e.x);
				}
			},
			QuadRoots::One(r) => {
				if r > 0.0 && r < 1.0 {
					let e = eval_bezier( r, (p0, c0, c1, p1));
					xmin = xmin.min(e.x);
					xmax = xmax.max(e.x);
				};
			},
			QuadRoots::Zero => {},
		};
		// Check for the y components
		match quad(a.y, b.y, c.y) {
			QuadRoots::Two(r0, r1) => {
				if r0 > 0.0 && r0 < 1.0 {
					let e = eval_bezier( r0, (p0, c0, c1, p1));
					ymin = ymin.min(e.y);
					ymax = ymax.max(e.y);
				};
				if r1 > 0.1 && r1 < 1.0 {
					let e = eval_bezier( r1, (p0, c0, c1, p1));
					ymin = ymin.min(e.y);
					ymax = ymax.max(e.y);
				}
			},
			QuadRoots::One(r) => {
				if r > 0.0 && r < 1.0 {
					let e = eval_bezier( r, (p0, c0, c1, p1));
					ymin = ymin.min(e.y);
					ymax = ymax.max(e.y);
				};
			},
			QuadRoots::Zero => {},
		};
		// Set the next starting point to the current endpoint
		p0 = p1;
	};
	// Return the bounding box
	(Point{x:xmin,y:ymin},Point{x:xmax-xmin,y:ymax-ymin})
}

impl Geometry {
	/// Define the bounding box function for all geometries
	pub fn bounding_box(&self, t: Transform) -> (Point, Point) {
		match self {
			// If we have a Circle
			&Geometry::Circle{center, radius} => {
				let Transform{a,b,c,d,..} = t;
				let tc = center.transform(t);
				let srx = radius*a.hypot(b);
				let sry = radius*c.hypot(d);
				let ll = tc - Point{x:srx, y:sry};
				let wh = Point{x:2.0*srx, y:2.0*sry};
				(ll,wh)				
			},
			// If we have a Polygon
			Geometry::Polygon{points} => {
				point_set_bounding_box(points, t)
			},
			// If we have a Line
			Geometry::Line{points,..} => {
				point_set_bounding_box(points, t)
			},
			Geometry::Bezier{start, others, ..} => {
				bezier_bounding_box(*start, others, t)
			}
		}
	}

	pub fn draw(&self, color: Color, t: Transform, canvas: &Canvas) {
		let context = &canvas.context;
		context.save();
		let Transform{a, b, c, d, dx, dy} = t;
		context.transform(a, c, b, d, dx, dy);
		match self {
			&Geometry::Circle{center, radius} => {
				context.set_fill_style_color( &color.to_string());
				context.begin_path();
				context.arc( center.x, center.y, radius, 0.0, 2.0*PI, false);
				context.close_path();
				context.fill( NonZero);
			},
			Geometry::Polygon{points} => {
				context.set_fill_style_color( &color.to_string());
				context.begin_path();
				let mut point_iter = points.iter();
				let p0 = point_iter.next().unwrap();
				context.move_to(p0.x, p0.y);
				for point in point_iter {
					context.line_to(point.x, point.y);
				}				
				context.close_path();
				context.fill( NonZero);
			}
			Geometry::Line{points, rel_width} => {
				context.set_stroke_style_color( &color.to_string());
				let (_,(w,h)) = canvas.window();
				let line_width = rel_width*w.max(h);
				context.set_line_width( line_width);
				context.begin_path();
				let mut point_iter = points.iter();
				let p0 = point_iter.next().unwrap();
				context.move_to(p0.x, p0.y);
				for point in point_iter {
					context.line_to(point.x, point.y);
				}
				context.stroke();
			},
			&Geometry::Bezier{start, ref others, rel_width} => {
				context.set_stroke_style_color( &color.to_string());
				let (_,(w,h)) = canvas.window();
				let line_width = rel_width*w.max(h);
				context.set_line_width( line_width);
				context.begin_path();
				context.move_to(start.x, start.y);
				let mut control_iter = others.iter();
				for (c0, c1, p) in control_iter {
					context.bezier_curve_to( c0.x, c0.y, c1.x, c1.y, p.x, p.y);
				}
				context.stroke();
			}
		}
		context.restore();
	}
}

/// Sort a list of points into a start, then list of sets of three points
fn sort_bezier_points( points: &[Point]) -> (Point,Vec<(Point,Point,Point)>) {
	let mut point_iter = points.iter();
	let start = *point_iter.next().unwrap();
	let mut others = Vec::new();
	let mut count = 0;
	let (x,y) = (0.0, 0.0);
	let mut three_set = (Point{x,y},Point{x,y},Point{x,y});
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

impl Graphic {
	pub fn circle( center: Point, radius: f64 ) -> Graphic {
		Graphic{
			object: GfxObj::Primative{
				geometry: Geometry::Circle{ center, radius },
				color: DEFAULT_SHAPE_COLOR,
			},
			transform: Transform::eye(),
			bounding_box: None,
		}
 	}

 	pub fn polygon( points: &[Point]) -> Graphic {
 		let points : Vec<_> = points.iter().cloned().collect();
 		Graphic{
 			object: GfxObj::Primative{
 				geometry: Geometry::Polygon{ points},
 				color: DEFAULT_SHAPE_COLOR,
 			},
 			transform: Transform::eye(),
 			bounding_box: None,
 		}
 	}

 	pub fn triangle( p0: Point, p1: Point, p2: Point) -> Graphic {
 		let points = vec!(p0, p1, p2);
 		Graphic{
 			object: GfxObj::Primative{
 				geometry: Geometry::Polygon{ points},
 				color: DEFAULT_SHAPE_COLOR,
 			},
 			transform: Transform::eye(),
 			bounding_box: None,
 		}
 	}

 	pub fn rect( ll: Point, wh: Point) -> Graphic {
  		let Point{x:w,y:h} = wh;
 		let lr = ll+Point{x:w,y:0.0};
 		let ur = ll+wh;
 		let ul = ll+Point{x:0.0,y:h};
 		let points = vec!(ll,lr,ur,ul);
		Graphic{
 			object: GfxObj::Primative{
 				geometry: Geometry::Polygon{ points},
 				color: DEFAULT_SHAPE_COLOR,
 			},
 			transform: Transform::eye(),
 			bounding_box: None,
 		}
 	}

 	pub fn line( points: &[Point]) -> Graphic {
 		let points : Vec<_> = points.iter().cloned().collect();
 		let rel_width = DEFAULT_RELATIVE_LINE_WIDTH;
 		Graphic{
 			object: GfxObj::Primative{
 				geometry: Geometry::Line{ points, rel_width},
 				color: DEFAULT_LINE_COLOR,
 			},
 			transform: Transform::eye(),
 			bounding_box: None,
 		}
 	}

 	pub fn bezier( points: &[Point]) -> Graphic {
 		let (start, others) = sort_bezier_points( &points);
 		let rel_width = DEFAULT_RELATIVE_LINE_WIDTH;
 		Graphic{
 			object: GfxObj::Primative{
 				geometry: Geometry::Bezier{start, others, rel_width},
 				color: DEFAULT_LINE_COLOR,
 			},
 			transform: Transform::eye(),
 			bounding_box: None,
 		}
 	}

 	pub fn circle_outline( center: Point, radius: f64) -> Graphic {
 		let c = 0.551915024494;

 		let mut points = [
 			Point{x:0.0,y:1.0},
    		Point{x:c,y:1.0},
    		Point{x:1.0,y:c},
    		Point{x:1.0,y:0.0},
    		Point{x:1.0,y:-c},
    		Point{x:c,y:-1.0},
    		Point{x:0.0,y:-1.0},
    		Point{x:-c,y:-1.0},
    		Point{x:-1.0,y:-c},
    		Point{x:-1.0,y:0.0},
    		Point{x:-1.0,y:c},
    		Point{x:-c,y:1.0},
    		Point{x:0.0,y:1.0},
 		];
 		for mut point in &mut points {
 			point.x = point.x*radius + center.x;
 			point.y = point.y*radius + center.y;
 		}

 	 	let (start, others) = sort_bezier_points( &points);
 		let rel_width = DEFAULT_RELATIVE_LINE_WIDTH;
 		Graphic{
 			object: GfxObj::Primative{
 				geometry: Geometry::Bezier{start, others, rel_width},
 				color: DEFAULT_LINE_COLOR,
 			},
 			transform: Transform::eye(),
 			bounding_box: None,
 		}	
 	}

 	pub fn polygon_outline( points: &[Point]) -> Graphic {
 		let first = *points.first().unwrap();
 		let mut points : Vec<_> = points.iter().cloned().collect();
 		points.push(first);
 		Graphic{
 			object: GfxObj::Primative{
 				geometry: Geometry::Polygon{ points},
 				color: DEFAULT_SHAPE_COLOR,
 			},
 			transform: Transform::eye(),
 			bounding_box: None,
 		}
 	}

 	pub fn rect_outline( ll: Point, wh: Point ) -> Graphic {
 		let Point{x:w,y:h} = wh;
 		let lr = ll+Point{x:w,y:0.0};
 		let ur = ll+wh;
 		let ul = ll+Point{x:0.0,y:h};
 		let points = vec!(ll,lr,ur,ul,ll);
 		let rel_width = DEFAULT_RELATIVE_LINE_WIDTH;
 		Graphic{
 			object: GfxObj::Primative{
 				geometry: Geometry::Line{ points, rel_width},
 				color: DEFAULT_LINE_COLOR,
 			},
 			transform: Transform::eye(),
 			bounding_box: None,
 		}
  	}

  	pub fn collection( elements: &[Graphic]) -> Graphic {
  		let elements : Vec<_> = elements.iter().cloned().collect();
 		Graphic{
 			object: GfxObj::Collection{ elements },
 			transform: Transform::eye(),
 			bounding_box: None,
		}  		
  	}

  	pub fn add<'a>( &'a mut self,  graphic: Graphic ) -> &'a mut Self {
  		match &mut self.object {
  			GfxObj::Primative{..} => {},
  			GfxObj::Collection{ref mut elements} => {
  				elements.push(graphic)
  			},
  		}
  		self
  	}

 	pub fn set_line_width<'a>(&'a mut self, width: f64) -> &'a mut Self {
 		match &mut self.object {
 			&mut GfxObj::Primative{ref mut geometry,..} => {
 				match geometry {
 					Geometry::Line{ref mut rel_width,..} => {
 						*rel_width = width;
 					},
	 				_ => {},
	 			}
 			},
 			GfxObj::Collection{..} => {},
 		};
 		self
 	}

 	pub fn set_color<'a>(&'a mut self, color: Color) -> &'a mut Self {
 		match &mut self.object {
 			&mut GfxObj::Primative{color: ref mut obj_color, ..} => {
 				*obj_color = color;
 			},
 			GfxObj::Collection{..} => {}
 		};
 		self
 	}

 	fn calc_bb(&self, transform: Transform) -> (Point, Point) {
 	 	let t = self.transform.combine_left(transform);
 		match &self.object {
 			GfxObj::Primative{geometry,..} => {
 				geometry.bounding_box(t)
 			},
 			GfxObj::Collection{ref elements} => {
 				let mut element_iter = elements.iter();
 				let graphic = element_iter.next().unwrap();
 				let (ll,wh) = graphic.calc_bb(t);
 				let mut xmin = ll.x;
 				let mut ymin = ll.y;
 				let mut xmax = ll.x+wh.x;
 				let mut ymax = ll.y+wh.y;
 				for graphic in element_iter {
 					let (ll,wh) = graphic.calc_bb(t);
 					xmin = xmin.min(ll.x);
 					ymin = ymin.min(ll.y);
 					xmax = xmax.max(ll.x+wh.x);
 					ymax = ymax.max(ll.y+wh.y);
 				}
 				(Point{x:xmin,y:ymin},Point{x:xmax-xmin,y:ymax-ymin})
 			},
 		}
 	}

 	pub fn bounding_box(&mut self) -> (Point, Point) {
 		match self.bounding_box {
 			Some(bounding_box) => bounding_box,
 			None => {
 				let bb = self.calc_bb( Transform::eye());
 				self.bounding_box = Some(bb);
 				bb
 			},
 		}
 	}

 	fn center(&mut self) -> Point {
 		let (ll,wh) = self.bounding_box();
 		ll+0.5*wh
 	}

 	fn add_transform<'a>(&'a mut self, transform: Transform) -> &'a mut Self {
 		self.transform = self.transform.combine_left(transform);
 		self.bounding_box = None;
 		self
 	}

 	pub fn rotate<'a>(&'a mut self, angle: f64) -> &'a mut Self {
 		let center = self.center();
 		self.add_transform( Transform::rotate(angle, center))
 	}

 	pub fn scale<'a>(&'a mut self, scale: f64) -> &'a mut Self {
 		let center = self.center();
 		self.add_transform( Transform::scale(scale, center))
 	}

 	pub fn scale_xy<'a>(&'a mut self, sx: f64, sy: f64) -> &'a mut Self {
 		let center = self.center();
 		self.add_transform( Transform::scale_xy(sx, sy, center))
 	}

 	pub fn translate<'a>(&'a mut self, dx: f64, dy: f64) -> &'a mut Self {
 		self.add_transform( Transform::translate(dx, dy))
 	}

 	fn tr_draw( &self, transform: Transform, canvas: &Canvas) {
		let t = self.transform.combine_left(transform);
		match &self.object {
			GfxObj::Primative{geometry, color} => {
				geometry.draw( *color, t, &canvas);
			},
			GfxObj::Collection{elements} => {
				let mut element_iter = elements.iter();
				for graphic in element_iter {
					graphic.tr_draw(t, &canvas);
				}
			}
		}
 	}

 	pub fn draw( &self, canvas: &Canvas) {
 		self.tr_draw(Transform::eye(), &canvas)
 	}
}

