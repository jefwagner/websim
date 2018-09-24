/// Brownian Dynamics Polymer Simulation

extern crate websim;

use websim::container::Container;
use websim::control::{
	Button,
	Toggle,
	Range,
};
use websim::output::{
	Canvas,
	TextArea,
};
use websim::gfx::Graphic;

use websim::simple_vec::Vec2 as Point;
use websim::simple_color::Color::{Rgb,Rgba};
use websim::simple_rng::{NormalDist};

use websim::simulation::get_time;
use websim::simulation::{
	SimStep,
	Simloop,
};

use std::iter;

#[allow(non_upper_case_globals)]
const kB : f64 = 0.0138064838709677419355_f64;
const DIFF : f64 = 1.0e+13_f64; // nm^2/ns
const RADIUS : f64 = 0.75; // nm
const SPRINGK : f64 = 3.0; // pN/nm
const NN : usize = 512;
const MAX_SEP : f64 = 768.0; // nm

fn rand_vel( rng: &mut NormalDist, dt: f64 ) -> Point {
	let scale = (2.0*DIFF*dt).sqrt();
	let fx = scale*rng.next();
	let fy = scale*rng.next();
	Point{x: fx, y:fy}
}

fn bond_force( ri: Point, rj: Point, temp: f64 ) -> Point {
	let k = 3.0*kB*temp/(2.0*RADIUS).powi(2);
	let rij = ri-rj;
	let r = rij.norm();
	let f = -k*(r-2.0*RADIUS);
	rij/r*f
}

#[derive(Debug,Clone)]
struct Polymer {
	temp: f64, // Temperature
	sep: f64, // Separation of endpoints
	monomers: Vec<Point>, // Position of monomers
	force: Option<Point>, // Force on the endpoint
}

impl Polymer {
	fn init(temp: f64, sep: f64) -> Polymer {
		let dx = sep/(NN-1) as f64;
		let mut x = -sep/2.0;
		let mut monomers = Vec::new();
		for _ in 0..NN {
			monomers.push(Point{x,y:0.0});
			x += dx;
		}
		Polymer{
			temp,
			sep,
			monomers,
			force: None,
		}
	}

	fn draw(&self, canvas: &Canvas) {
		canvas.clear();
		let mut line = Graphic::line(&self.monomers);
		line.set_line_width(0.002);
		canvas.draw(&line);
		for &point in self.monomers.iter() {
			let mut circ = Graphic::circle(point, 4.0*RADIUS);
			circ.set_color(Rgba{r:255,g:0,b:0,a:128});
			canvas.draw(&circ);
		}
	}

	fn step(&mut self, rng: &mut NormalDist, dt: f64) {
		let mut vel : Vec<Point> = Vec::new();
		vel.extend(iter::repeat(Point{x:0.0, y:0.0}).take(NN));
		let mu = DIFF/(kB*self.temp);
		for i in 1..NN {
			let ri = self.monomers[i];
			let rj = self.monomers[i-1];
			let f = bond_force(ri, rj, self.temp);
			vel[i] = vel[i] + mu*f + rand_vel(rng, dt);
			vel[i-1] = vel[i-1] - mu*f;
		}
		for i in 1..NN-1 {
			self.monomers[i] = self.monomers[i] + vel[i]*dt;
		}
		self.force = Some(vel[NN-1]/mu);
	}

	fn scale(&mut self, new_sep: f64) {
		let scale = new_sep/self.sep;
		for point in self.monomers.iter_mut() {
			point.x *= scale;
		}
		self.sep = new_sep;
	}
}

#[derive(Debug,Clone)]
struct FullSim{
	writing: bool,
	step_count: u32,
	poly: Polymer,
	rng: NormalDist,
	canvas: Canvas,
	textarea: TextArea,
}

impl FullSim{
	fn draw(&self) {
		self.poly.draw(&self.canvas);
	}

	fn writeln(&self) {
		if let Some(Point{x,y}) = self.poly.force {
			self.textarea.write(&format!(",\n {{ {}, {} }}",x,y));
		}
	}
}

impl SimStep for FullSim {
	fn step(&mut self, dt: f64) {
		self.poly.step(&mut self.rng, dt);
	}
}

fn main() {
	let app = Container::new("app");
	app.add_to_body();

	let vis = Container::new("vis");
	let mut canvas = Canvas::new("canvas");
	vis.add( &canvas);
	let sim_control = Toggle::new("start_stop", "Start", "Stop");
	vis.add( &sim_control);
	let sim_reset = Button::new("reset", "Reset");
	vis.add( &sim_reset);
	app.add( &vis);

	canvas.set_window(((-MAX_SEP/2.0,-MAX_SEP/2.0),(MAX_SEP,MAX_SEP)));

	let controls = Container::new("controls");
	let sep_slider = Range::new("sep_slider", "R (nm) : ", 0.0, MAX_SEP, 1.0, 16.0);
	controls.add( &sep_slider);
	sep_slider.set(MAX_SEP/2.0);
	let temp_slider = Range::new("temp_slider", "T (K) : ", 200.0, 500.0, 1.0, 20.0);
	controls.add( &temp_slider);
	temp_slider.set(310.0);
	app.add( &controls);

	let output = Container::new("output");
	let output_control = Toggle::new("write", "Write Data", "Stop Data");
	output.add( &output_control);
	let output_clear = Button::new("clear", "Clear");
	output.add( &output_clear);
	let mut textarea = TextArea::new("txt");
	textarea.set_cols(110);
	textarea.set_rows(30);
	output.add( &textarea);
	app.add( &output);

	textarea.writeln("hello world");

	let mut poly = Polymer::init(temp_slider.query(), sep_slider.query());
	poly.draw( &canvas);

	let s0 = get_time();
	let s1 = 0_u64;
	let mut rng = NormalDist::new(0.0, 1.0);
	rng.seed(s0,s1);

	let fullsim = FullSim{
		writing: false,
		step_count: 0,
		poly,
		rng,
		canvas,
		textarea,
	};

	let ref_sim = Simloop::new_ref(fullsim);

	sim_reset.add_button_function({
		let ref_sim = ref_sim.clone();
		move | _:bool | {
			let sim = &mut ref_sim.borrow_mut().state;
			sim.step(0.0001);
			sim.draw();
		}
	});

}
