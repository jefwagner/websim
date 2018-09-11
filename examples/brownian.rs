extern crate websim;
extern crate rand;

use rand::{thread_rng,Rng};
use rand::distributions::{Normal};

use websim::container::Container;
use websim::output::{
	Canvas,
	TextArea,
};
use websim::gfx::Graphic;

use websim::simple_vec::Vec2 as Point;
use websim::simple_color::Color::{Rgb};

const CBRT2 : f64 = 1.259921049894873164767210607278228350570251464701507980081_f64;
#[allow(non_upper_case_globals)]
const kB : f64 = 0.0138064838709677419355_f64;

/// Structure that holds a set of parameters
#[derive(Debug,Clone,Copy,PartialEq)]
struct Params{
	force: f64, // Force on large particle
	temp: f64, // Temperature
	ep: f64, // Lennard Jones potential depth
	rsol: f64, // solvent particle radius
	rpar: f64, // large particle radius
	msol: f64, // solvent particle mass
	mpar: f64, // large particle mass
}

impl Params {
	/// Create a default set of parameters
	fn init() -> Params{
		Params{
			force: 0.0, // units: pN
			temp: 310.0, // units: Kelvin
			ep: 5.0, // units: pN nm
			rsol: 0.15, // units: nm
			rpar: 1.5, // units: nm
			msol: 30.0, // units: 10^-27 kg
			mpar: 30000.0, // units: 10^-27 kg
		}
	}
}

/// Lennard Jones force between two solvent molecules
fn lj(ri: Point, rj: Point, p: Params) -> Point {
	let Params{rsol, ep, ..} = p;
	let sigma = 2.0*rsol;
	let rij = ri-rj;
	let r2 = rij.norm_squared();
	if r2 >= sigma*sigma*CBRT2 {
		Point{x:0.0, y:0.0}
	} else {
		let ir2 = sigma*sigma/r2;
		let ir6 = ir2*ir2*ir2;
		let ir12 = ir6*ir6;
		24.0*ep*rij/sigma/sigma*ir2*(2.0*ir12-ir6)
	}
}

/// Structure that holds the state variables
#[derive(Debug,Clone)]
struct State {
	t: f64, // time
	// Particle position and velocity
	rpar: Point,
	vpar: Point,
	// Solvent particle position and velocities
	rsol: Vec<Point>,
	vsol: Vec<Point>,
}

/// Size of screen
const SIZE : f64 = 30.0;

impl State {
	/// Create an initial arrangement of solvent
	/// molecules. The particles are put on a grid 
	fn init(spacing: f64, p: Params) -> State {
		let t = 0.0;
		let Params{rsol:rad_sol, rpar: rad_par, temp, msol, ..} = p;
		let rpar = Point{x:SIZE/2.0, y:SIZE/2.0};
		let vpar = Point{x:0.0, y:0.0};
		let mut rsol = Vec::new();
		let mut vsol = Vec::new();

		let mut rng = thread_rng();
		// let maxwell = Normal::new(0.0, (kB*temp/msol).sqrt());

		let mut r = Point{x:spacing/2.0, y:spacing/2.0};
		while r.y < SIZE-spacing/2.0 {
			while r.x < SIZE-spacing/2.0 {
				let dist = (r-rpar).norm();
				if dist > rad_par+rad_sol {
					rsol.push(r);
					// let vx = maxwell.sample(&mut rng);
					// let vy = maxwell.sample(&mut rng);
					// vsol.push(Point{x:vx, y:vy});
				}
				r.x += spacing;
			}
			r.x = spacing/2.0;
			r.y += spacing;
		}
		State{
			t,
			rpar,
			vpar,
			rsol,
			vsol,
		}
	}

	fn draw(&self, p: Params, canvas: &Canvas) {
		let Params{rpar, rsol, ..} = p;
		canvas.clear();
		let mut par = Graphic::circle(self.rpar, rpar);
		par.set_color(Rgb{r:255,g:0,b:0});
		canvas.draw(&par);
		for pos in self.rsol.iter().cloned() {
			let sol = Graphic::circle(pos, rsol);
			canvas.draw(&sol);
		}
	}
}

fn main() {
	let app = Container::new("app");
	app.add_to_body();
	let mut canvas = Canvas::new("canvas");
	app.add( &canvas);
	let textarea = TextArea::new("txt");
	app.add( &textarea);

	canvas.set_window(((0.0, 0.0), (SIZE,SIZE)));

	let p = Params::init();
	let state = State::init(0.6,p);
	state.draw(p, &canvas);

	let p0 = Point{x:0.0, y:0.0};
	let dx = Point{x:0.0, y:0.0004};
	let p = Params::init();
	for i in 0..2 {
		let p1 = Point{x:0.0, y:0.3}+(i as f64)*dx;
		let f = lj(p0, p1, p);
		textarea.writeln(&format!("{{{}, {}}},",p1.y, f.y));
	}
	textarea.writeln("");	
	for vel in &state.vsol {
		textarea.writeln(&format!("v = {:?}",vel));
	}
	textarea.writeln("");
	textarea.writeln(&format!("number of solvent molecules: {}", state.rsol.len()));

}
