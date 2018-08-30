/// Testing the Graphics objects
/// ----------------------------
extern crate websim;

use websim::container::Container;
use websim::output::{
	TextArea,
	Canvas,
};

use websim::simple_vec::Vec2 as Point;
use websim::simple_vec::Transform2D as Transform;
use websim::simple_color::Color::{Rgb,Rgba};
use websim::gfx::Graphic;

const PI : f64 = 3.14159265358979323846264338327950288f64;

fn main() {
    let app = Container::new("app");
    app.add_to_body();
    let canvas = Canvas::new("canvas");
    app.add(&canvas);
    let textarea = TextArea::new("txt");
    app.add(&textarea);

    // Draw a Circle
    let center = Point{x:0.7, y:0.7};
    let radius = 0.15;
    let mut circ = Graphic::circle( center, radius);
    circ.set_color( Rgb{r:255,g:0,b:0});
    circ.draw(&canvas);

    // Draw a line with 5 points
    let points = vec!(
    	Point{x:0.2,y:0.7},
    	Point{x:0.15,y:0.75},
    	Point{x:0.25,y:0.8},
    	Point{x:0.35,y:0.75},
    	Point{x:0.3,y:0.7},
    );
    let mut line = Graphic::line( &points);
    line.set_color( Rgb{r:255,g:255,b:0})
    	.set_line_width(0.002)	
    	.draw( &canvas);

    // Draw a triangle
    let points = vec!(
    	Point{x:0.8,y:0.1},
    	Point{x:0.9,y:0.1},
    	Point{x:0.85,y:0.2},
    );
    let mut tri = Graphic::polygon( &points);
   	tri.set_color( Rgb{r:102,g:0,b:255});
    tri.draw( &canvas);

    // Show the bounding box for the circle
    let (ll,wh) = circ.bounding_box();
    let outline = Graphic::rect_outline(ll,wh);
    outline.draw( &canvas);

    // rotate the triangle, and test bounding box
    tri.rotate(PI/4.0)
    	.set_color( Rgba{r:0,g:255,b:255,a:128})
    	.draw( &canvas);
    let (ll,wh) = tri.bounding_box();
    let outline = Graphic::rect_outline(ll,wh);
    outline.draw( &canvas);

    // Try scaling a circle into an ellipse
    let center = Point{x:0.2, y:0.2};
    let radius = 0.1;
    let mut elipse0 = Graphic::circle(center, radius);
    elipse0.scale_xy(0.5,1.0)
    	.set_color( Rgb{r:255,g:0,b:0})
    	.draw(&canvas);
    // rotate the new ellipse
    elipse0.rotate(PI/4.0)
    	.translate(0.2, 0.2)
    	.set_color( Rgba{r:255,g:255,b:0,a:128})
    	.draw(&canvas);
	// test the bounding box    	
    let (ll,wh) = elipse0.bounding_box();
    let outline = Graphic::rect_outline(ll,wh);
    outline.draw( &canvas);    

    // test the collection method
    let circ0 = Graphic::circle(
    	Point{x:0.3, y:0.1}, 0.1);
    let circ1 = Graphic::circle(
    	Point{x:0.4, y:0.1}, 0.1);
    let mut circ2 = Graphic::circle(
    	Point{x:0.6, y:0.1}, 0.1);
    circ2.translate(-0.1,0.0);
	let mut catapillar = Graphic::collection(
    	&vec!(circ0, circ1, circ2));
    catapillar.draw( &canvas);
    // and the bounding box for the collection
    let (ll,wh) = catapillar.bounding_box();
    let outline = Graphic::rect_outline(ll,wh);
    outline.draw( &canvas);

    // move and rotate the collection
    catapillar.translate(0.4,0.3)
    	.rotate(PI/4.0)
    	.draw( &canvas);
    // and test the bounding box
    let (ll,wh) = catapillar.bounding_box();
    let outline = Graphic::rect_outline(ll,wh);
    outline.draw( &canvas);
}