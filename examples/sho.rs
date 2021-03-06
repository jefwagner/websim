///Simple Harmonic Osciallator

extern crate websim;

use websim::container::Container;
use websim::output::{
	Canvas,
	TextArea,
};
use websim::control::{
	Range,
	Toggle,
	Button,
};

use websim::extra;
use websim::gfx::Graphic;

use websim::simulation::{
	SimStep,
	Simloop,
};

use websim::simple_vec::Vec2 as Point;

#[derive(Debug,Clone)]
struct Screen {
	/// Parameter Controls
	mass_slider: Range,
	spring_slider: Range,
	gravity_slider: Range,
	amp_slider: Range,
	angle_slider: Range,

	/// Simulation Controls
	sim_control: Toggle,
	sim_reset: Button,

	/// Outputs
	canvas: Canvas,
	textarea: TextArea,

	/// Output Controls
	output_control: Toggle,
	output_clear: Button,
}

impl Screen {
	fn new() -> Screen {
		let app = Container::new("app");
		let vis = Container::new("vis");
		let controls = Container::new("controls");
		let output = Container::new("output");

		app.add_to_body();
		app.add( &vis);
		app.add( &controls);
		app.add( &output);

		let mut canvas = Canvas::new("canvas");
		canvas.set_window(((-0.7,-1.3), (1.4, 1.4)));
		let sim_control = Toggle::new("sim_control", "Start", "Stop");
		let sim_reset = Button::new("sim_reset", "Reset");
		vis.add( &canvas);
		vis.add( &sim_control);
		vis.add( &sim_reset);

		let mass_slider = Range::new("m", "m (g) : ", 50.0, 500.0, 1.0, 10.0);
		mass_slider.set(250.0);
		let spring_slider = Range::new("k", "k (N/m) : ", 1.0, 20.0, 0.1, 1.0);
		spring_slider.set(10.0);
		let gravity_slider = Range::new("g", "g (m/s²) : ", 1.0, 20.0, 0.1, 1.0);
		gravity_slider.set(9.8);
		let amp_slider = Range::new("a_0", "y₀ (cm) : ", 0.0, 20.0, 0.1, 1.0);
		amp_slider.set(0.0);
		let angle_slider = Range::new("th_0", "θ₀ (deg) : ", -25.0, 25.0, 1.0, 5.0);
		angle_slider.set(0.0);
		controls.add( &mass_slider);
		controls.add( &spring_slider);
		controls.add( &gravity_slider);
		controls.add( &amp_slider);
		controls.add( &angle_slider);

		let output_control = Toggle::new("output_control", "Write Data", "Stop Data");
		let output_clear = Button::new("output_clear", "Clear");
		let mut textarea = TextArea::new("textarea");
		textarea.set_cols(110);
		textarea.set_rows(30);
		output_clear.add_button_function({
			let textarea = textarea.clone();
			move | _:bool | {
				textarea.clear();
			}
		});
		output.add( &output_control);
		output.add( &output_clear);
		output.add( &textarea);

		Screen{
			canvas,
			sim_control,
			sim_reset,

			mass_slider,
			spring_slider,
			gravity_slider,
			amp_slider,
			angle_slider,

			textarea,
			output_control,
			output_clear,
		}
	}
}


/// Holds the simulation state
#[derive(Debug,Clone)]
struct ShoState {
	/// Simulation varaibles
	t: f64, // time
	r: Point, // position
	v: Point, // velocity

	/// Parameters
	m: f64, // Mass
	k: f64, // Spring constant
	g: f64, // Acceleration of gravity
	a_0: f64, // initial amplitude
	th_0: f64, // initial angle
}

/// Spring force
const L0 : f64 = 0.20; // 20 cm
fn spring_force( k: f64, r: Point) -> Point {
	let fhat = -r/r.norm();
	let fmag = k*(r.norm()-L0);
	fhat*fmag
}

/// Velocity Verlet Step
impl ShoState{
	fn new(screen: &Screen) -> ShoState {
		let t = 0.0;
		let a_0 = L0 + screen.amp_slider.query()/100.0;
		let th_0 = screen.angle_slider.query().to_radians();
		let (s, c) = th_0.sin_cos();
		let r = Point{x: a_0*s, y: -a_0*c};
		let v = Point{x: 0.0, y:0.0};

		let m = screen.mass_slider.query()/1000.0;
		let k = screen.spring_slider.query();
		let g = screen.gravity_slider.query();

		ShoState{t, r, v, m, k, g, a_0, th_0}
	}

	fn reset(&mut self) {
		self.t = 0.0;
		let (s, c) = self.th_0.sin_cos();
		self.r = Point{x: self.a_0*s, y: -self.a_0*c};
		self.v = Point{x: 0.0, y:0.0};
	}

	fn force(&self, r: Point) -> Point {
		spring_force( self.k, r) + self.m*Point{x:0.0, y:-self.g}
	}

	fn vv_step(&mut self, dt: f64) {
		let a_0 = self.force(self.r)/self.m;
		self.r = self.r + self.v*dt + a_0*0.5*dt*dt;
		let a_1 = self.force(self.r)/self.m;
		self.v = self.v + (a_0 + a_1)*0.5*dt;
	}

	fn draw(&self, canvas : &Canvas) {
		let spring = extra::spring(
			5, // Number of loops
			L0, // Default spring length
			Point{x:0.0, y:0.0},
			self.r);
		let dot = Graphic::circle(Point{x:0.0, y:0.0}, 0.01 );
		let rhat = self.r/self.r.norm();
		let thhat = Point{x: rhat.y, y: -rhat.x};
		let w = self.m.sqrt()*0.16;
		let h = self.m.sqrt()*0.2;
		let ul = self.r+thhat*0.5*w;
		let ll = self.r+rhat*h+thhat*0.5*w;
		let lr = self.r+rhat*h-thhat*0.5*w;
		let ur = self.r-thhat*0.5*w;
		let mass = Graphic::polygon(&[ul,ll,lr,ur]);

		canvas.clear();
		canvas.draw(&spring);
		canvas.draw(&dot);
		canvas.draw(&mass);
	}

	fn write_header(&self, textarea: &TextArea) {
		textarea.writeln("time (s), x pos (cm), y pos (cm), x vel (m/s), y vel (m/s)");
	}

	fn write(&self, textarea: &TextArea) {
		textarea.write(&format!(
			"{{ {}, {}, {}, {}, {} }}", 
			self.t, self.r.x*100.0, (self.r.y-L0)*100.0, self.v.x, self.v.y
		));
	}
}

#[derive(Debug,Clone)]
struct FullSim{
	active: bool,
	writing: bool,
	step_count: u32,
	screen: Screen,
	sho_state: ShoState,
}

const STEPS_PER_STEP : i32 = 10;

impl FullSim {
	fn draw(&self) {
		self.sho_state.draw( &self.screen.canvas);
	}
}

impl SimStep for FullSim {
	fn step(&mut self, dt: f64) {
		let dt = 0.01666666666666666666666666666666666666666666666666_f64; // convert to seconds
		self.sho_state.t += dt;

		let dt = dt/(STEPS_PER_STEP as f64);
		for _ in 0..STEPS_PER_STEP {
			self.sho_state.vv_step(dt);
		}

		if self.writing {
			if self.step_count == 5 {
				self.screen.textarea.writeln(",");
				self.sho_state.write( &self.screen.textarea);
				self.step_count = 0;
			} else {
				self.step_count += 1;
			}
		}

		self.draw();
	}
}

fn main() {
	let active = false;
	let writing = false;
	let screen = Screen::new();
	let mut sho_state = ShoState::new(&screen);
	let mut sim = FullSim{
		active,
		writing,
		step_count: 0, 
		screen: screen.clone(), 
		sho_state};
	sim.draw();
	let state = Simloop::new_ref( sim);
	screen.sim_control.add_toggle_function({
		let state = state.clone();
		move | status:bool | {
			if status {
				{
					let mut ref_state = state.borrow_mut();
					if ref_state.state.writing {
						ref_state.state.sho_state.write_header( &ref_state.state.screen.textarea);
						ref_state.state.screen.textarea.write("{");
						ref_state.state.sho_state.write( &ref_state.state.screen.textarea);
						ref_state.state.step_count = 0;
					}
					ref_state.state.active = true;
				}
				Simloop::start_loop(state.clone());
			} else {
				let mut ref_state = state.borrow_mut();
				if ref_state.state.writing {
					ref_state.state.screen.textarea.writeln("}\n");
				}
				ref_state.stop_loop();
			}
		}
	});
	screen.sim_reset.add_button_function({
		let sim_control = screen.sim_control.clone();
		let output_control = screen.output_control.clone();
		let state = state.clone();
		move | _:bool | {
			let mut ref_state = state.borrow_mut();
			if sim_control.query() {
				ref_state.stop_loop();
				sim_control.set(false);
				if ref_state.state.writing {
					ref_state.state.screen.textarea.writeln("}\n");
					ref_state.state.writing = false;
					output_control.set(false);					
				}
			}
			ref_state.state.active = false;
			ref_state.state.sho_state.reset();
			ref_state.state.draw();
		}
	});
	screen.output_control.add_toggle_function({
		let state = state.clone();
		move | status:bool | {
			let mut ref_state = state.borrow_mut();
			ref_state.state.writing = status;
			if ref_state.state.screen.sim_control.query() {
				if status {			
					ref_state.state.sho_state.write_header( &ref_state.state.screen.textarea);
					ref_state.state.screen.textarea.write("{");
					ref_state.state.sho_state.write( &ref_state.state.screen.textarea);
					ref_state.state.step_count = 0;
				} else {
					ref_state.state.screen.textarea.writeln("}\n");					
				}
			}
		}
	});
	screen.output_clear.add_button_function({
		let state = state.clone();
		move | _:bool| {
			let mut ref_state = state.borrow_mut();
			let running = ref_state.state.screen.sim_control.query();
			if running && ref_state.state.writing {
					ref_state.state.sho_state.write_header( &ref_state.state.screen.textarea);
					ref_state.state.screen.textarea.write("{");
					ref_state.state.sho_state.write( &ref_state.state.screen.textarea);
					ref_state.state.step_count = 0;
			}
		}
	});


	screen.amp_slider.add_continuous_range_function({
		let sim_control = screen.sim_control.clone();
		let state = state.clone();
		move | val:f64 | {
			let mut ref_state = state.borrow_mut();
			ref_state.state.sho_state.a_0 = L0 + val/100.0;
			if !ref_state.state.active {
				ref_state.state.sho_state.reset();
				ref_state.state.draw();
			}
		}
	});

	screen.angle_slider.add_continuous_range_function({
		let sim_control = screen.sim_control.clone();
		let state = state.clone();
		move | val:f64 | {
			let mut ref_state = state.borrow_mut();
			ref_state.state.sho_state.th_0 = val.to_radians();
			if !ref_state.state.active {
				ref_state.state.sho_state.reset();
				ref_state.state.draw();
			}
		}
	});

	screen.mass_slider.add_continuous_range_function({
		let state = state.clone();
		move | val:f64 | {
			let mut ref_state = state.borrow_mut();
			ref_state.state.sho_state.m = val/1000.0;
			ref_state.state.draw();
		}
	});

	screen.spring_slider.add_continuous_range_function({
		let state = state.clone();
		move | val:f64 | {
			state.borrow_mut().state.sho_state.k = val;
		}
	});

	screen.gravity_slider.add_continuous_range_function({
		let state = state.clone();
		move | val:f64 | {
			state.borrow_mut().state.sho_state.g = val;
		}
	});

} 	