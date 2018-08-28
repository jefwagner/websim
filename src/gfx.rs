/// Graphics Primatives
/// ===================

//use stdweb::web::html_element::CanvasElement;

use ::simple_vec::Vec2 as Point;
use ::simple_vec::Transform2D as Transform;

#[derive(Debug,Clone)]
enum Geometry {
	Circle{center: Point, radius: f64},
//	Arc{center: Point, radius: f64, theta0: f64, theta1: f64},
	Polygon{points: Vec<Point>},
	Line{points: Vec<Point>},
}

fn point_set_bounding_box( 
		points: &Vec<Point>, 
		t: Option<Transform>
	) -> (Point, Point) {
	match t {
		None => { 
			let mut point_iter = points.iter(); 
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
		},
		Some(t) => {
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
	}
}

impl Geometry {
	fn bounding_box(&self, t: Option<Transform>) -> (Point, Point) {
		match self {
			// If we have a Circle
			&Geometry::Circle{center, radius} => {
				match t {
					None => {
						let ll = center - Point{x:radius, y:radius};
						let wh = Point{x:2.0*radius, y:2.0*radius};
						(ll,wh)
					},
					Some(t) => {
						let Transform{a,b,c,d,dx,dy} = t;
						let tc = center + Point{x:dx, y:dy};
						let srx = radius*a.hypot(b);
						let sry = radius*c.hypot(d);
						let ll = tc - Point{x:srx, y:sry};
						let wh = Point{x:2.0*srx, y:2.0*sry};
						(ll,wh)
					}
				}
			},
			// If we have a Polygon
			Geometry::Polygon{points} => {
				point_set_bounding_box(points, t)
			},
			// If we have a Line
			Geometry::Line{points} => {
				point_set_bounding_box(points, t)
			}
		}
	}
}

// pub trait Drawable{
// 	fn draw(&self, canvas: CanvasElement);
// 	fn bounding_box(&self) -> (Point, Point);
// }

#[derive(Debug,Clone)]
pub enum Graphic {
	Primative{
		geometry: Geometry,
		color: Color,
		transform: Option<Transform>,
		bounding_box: Option<(Point,Point)>
	},
	Collection{
		elements: Vec<Graphic>,
		transform: Option<Transform>,
		bounding_box: Option<(Point,Point)>
	}
}

impl Graphic {
	pub fn circle( center: Point, radius: f64 ){
		Graphics::Primative{
			geometry: Geometry::Circle{ center, radius},
			color: DEFAULT_SHAPE_COLOR,
			transform: None,
			bounding_box: None,
		}
	}

	pub fn triangle( p0: Point, p1: Point, p2: Point){
		let points = vec!(p0, p1, p2);
		Graphics::Primative{
			geometry: Geometry::Polygon{ points},
			color: DEFAULT_SHAPE_COLOR,
			transform: None,
			bounding_box: None,
		}
	}

	pub fn rect( ll: Point, wh: Point ){
		let Point{x:w, y:h} = wh;
		let lr = ll+Point{x:w, y:0.0};
		let ur = ll+wh;
		let ul = ll+Point{x:0.0, y:h};
		let points = vec!(ll, lr, ur, ul);
		Graphics::Primative{
			geometry: Geometry::Polygon{ points},
			color: DEFAULT_SHAPE_COLOR,
			transform: None,
			bounding_box: None,
		}
	}

	pub fn polygon( points: &[Point]){
		let points : Vec<Point> = points.iter().cloned().collect();
		Graphics::Primative{
			geometry: Geometry::Polygon{ points},
			color: DEFAULT_SHAPE_COLOR,
			transform: None,
			bounding_box: None,
		}		
	}

	pub fn polygon( points: &[Point]){
		let points : Vec<Point> = points.iter().cloned().collect();
		Graphics::Primative{
			geometry: Geometry::Polygon{ points},
			color: DEFAULT_SHAPE_COLOR,
			transform: None,
			bounding_box: None,
		}		
	}

}

// impl Drawable for Graphic {


// }

#[cfg(test)]
mod tests {
	use super::*;

	const PI : f64 = 3.14159265358979323846264338327950288f64;

	/// Test out the overloaded arithmetic operators
	#[test]
	fn test_bounding_box() {
		let c = Point{x: 3.0, y: 4.0};
		let r = 2.0;

		let ps = vec!(
			Point{x: 2.0, y: 4.0},
			Point{x: 3.0, y: 2.0},
			Point{x: 1.0, y: 4.0},
			Point{x: 2.0, y: 5.0},
			Point{x: 3.0, y: 3.0},
			Point{x: 2.0, y: 3.0},
		);

		let a = Geometry::Circle{center:c, radius:r};
		assert_eq!(
			a.bounding_box(None), 
			(Point{x:1.0, y:2.0}, Point{x:4.0, y:4.0}));

		let b = Geometry::Polygon{points:ps};
		assert_eq!(
			b.bounding_box(None),
			(Point{x:1.0, y:2.0}, Point{x:2.0, y:3.0}));

		// let rot1 = Transform::rotate(PI/2.0, Point{x:0.0, y:0.0});
		// assert_eq!(
		// 	b.bounding_box(Some(rot1)),
		// 	(Point{x:-5.0, y:1.0}, Point{x:3.0, y:2.0}));
	}
}

