extern crate smallvec;
use smallvec::SmallVec;

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
use websim::simple_color::Color::{Rgb};

use websim::simulation::get_time;
use websim::simple_rng::NormalDist;

use std::ops::{
	Index,
	IndexMut,
};

// use std::rc::Rc;
// use std::cell::RefCell;

use websim::simulation::{
	SimStep,
	Simloop,
};

#[allow(non_upper_case_globals)]
const kB : f64 = 0.0138064838709677419355_f64;

/// Structure that holds a set of parameters
#[derive(Debug,Clone,Copy,PartialEq)]
struct Params{
	force: f64, // Force on large particle
	temp: f64, // Temperature
	ep: f64, // Lennard Jones potential depth
	rad_sol: f64, // solvent particle radius
	rad_par: f64, // large particle radius
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
			rad_sol: 0.15, // units: nm
			rad_par: 0.75, // units: nm
			msol: 30.0, // units: 10^-27 kg
			mpar: 1500.0, // units: 10^-27 kg
		}
	}
}


/// Structure that holds the state variables
#[derive(Debug,Clone)]
struct State {
	t: f64, // time
	// Particle position and velocity
	rpar: Point,
	vpar: Point,
	// an offset for the position if it screenwraps
	offset: Point,
	// Solvent particle position and velocities
	rsol: Vec<Point>,
	vsol: Vec<Point>,
}

/// Size of screen
const SIZE : f64 = 15.0;
/// Maximum number of bins across
const BIMAX : usize = 15;

#[inline]
fn wrap( r: &mut Point) {
	if r.x > SIZE {
		r.x -= SIZE;
	} else if r.x < 0.0 {
		r.x += SIZE;
	}
	if r.y > SIZE {
		r.y -= SIZE;
	} else if r.y < 0.0 {
		r.y += SIZE;
	}
}

impl State {
	/// Create an initial arrangement of solvent
	/// molecules. The particles are put on a grid a
	/// distance `spacing` appart, and any that overlap
	/// the central large particle are rejected. The 
	/// initial velocites are taken from a maxwell
	/// distribution.
	fn init(spacing: f64, p: Params) -> State {
		let t = 0.0;
		let Params{rad_sol, rad_par, temp, msol, ..} = p;
		let rpar = Point{x:SIZE/2.0, y:SIZE/2.0};
		let vpar = Point{x:0.0, y:0.0};
		let offset = Point{x:0.0, y:0.0};
		let mut rsol = Vec::new();
		let mut vsol = Vec::new();

		let s0 = get_time();
		let s1 = 0_u64;
		let vstd = 1000.0*(kB*temp/msol).sqrt(); // ns/nm
		let mut maxwell_dist = NormalDist::new(0.0, vstd);
 		maxwell_dist.seed(s0,s1);

		let mut r = Point{x:spacing/2.0, y:spacing/2.0};
		while r.y < SIZE-spacing/2.0 {
			while r.x < SIZE-spacing/2.0 {
				let dist = (r-rpar).norm();
				if dist > rad_par+rad_sol {
					rsol.push(r);
					let vx = maxwell_dist.next();
					let vy = maxwell_dist.next();
					vsol.push(Point{x:vx, y:vy});
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
			offset,
			rsol,
			vsol,
		}
	}

	fn step(&mut self, dt: f64, p: Params) {
		self.t += dt;

		let (fpar0, fsol0) = force_calc(p, self.rpar, &self.rsol);
		let fpar0 = fpar0 + Point{x:p.force, y:0.0};

		let apar0 = fpar0/p.mpar*1.0E+6_f64;
		self.rpar = self.rpar + self.vpar*dt + 0.5*apar0*dt*dt;
		if self.rpar.x >= SIZE {
			self.rpar.x -= SIZE;
			self.offset.x += SIZE;
		} else if self.rpar.x < 0.0 {
			self.rpar.x += SIZE;
			self.offset.x -= SIZE;
		}
		if self.rpar.y >= SIZE {
			self.rpar.y -= SIZE;
			self.offset.y += SIZE;
		} else if self.rpar.y < 0.0 {
			self.rpar.y += SIZE;
			self.offset.y -= SIZE;
		}

		{
			let pos_vel_force = self.rsol.iter_mut()
				.zip( self.vsol.iter())
				.zip( fsol0.iter())
				.map( |((r,v),f)| (r,v,f) );

			for (pos,&vel,&force) in pos_vel_force {
				let a = force/p.msol*1.0E+6_f64;
				*pos = *pos + vel*dt + 0.5*a*dt*dt;
				wrap(pos);
			}
		}

		let (fpar1, fsol1) = force_calc(p, self.rpar, &self.rsol);
		let fpar1 = fpar1 + Point{x:p.force, y:0.0};

		let apar1 = fpar1/p.mpar*1.0E+6_f64;
		self.vpar = self.vpar + 0.5*(apar0 + apar1)*dt;

		let vel_force0_force1 = self.vsol.iter_mut()
			.zip( fsol0.iter())
			.zip( fsol1.iter())
			.map( |((v,f0),f1)| (v,f0,f1) );

		for (vel, &force0, &force1) in vel_force0_force1 {
			let a0 = force0/p.msol*1.0E+6_f64;
			let a1 = force1/p.msol*1.0E+6_f64;
			*vel = *vel + 0.5*(a0 + a1)*dt;
		}
	}

	fn draw(&self, p: Params, canvas: &Canvas) {
 		let Params{rad_par, rad_sol, ..} = p;
		canvas.clear();
		let mut par = Graphic::circle(self.rpar, rad_par);
		par.set_color(Rgb{r:255,g:0,b:0});
		canvas.draw(&par);
		for pos in self.rsol.iter().cloned() {
			let sol = Graphic::circle(pos, rad_sol);
			canvas.draw(&sol);
		}
	}
}

/// Structure the divides the space into square regions
/// called 'bins'. Each bin contains a smallvec (vector
/// on the stack - provided with the smallvec crate) that
/// holds the solvent particles in that set. Currently
/// it holds the indices.
type Bin = SmallVec<[usize;10]>;
struct BinList {
	bins: Vec<Bin>,
}

impl Index<[usize;2]> for BinList{
	type Output = Bin;

	fn index<'a>(&'a self, idx: [usize;2]) -> &'a Bin {
		&self.bins[idx[0]*BIMAX+idx[1]]
	}
}

impl IndexMut<[usize;2]> for BinList{
	fn index_mut<'a>(&'a mut self, idx: [usize;2]) -> &'a mut Bin {
		&mut self.bins[idx[0]*BIMAX+idx[1]]
	}
}

impl BinList{
	fn new() -> BinList {
		let mut bins : Vec<Bin> = Vec::with_capacity(BIMAX*BIMAX);
		for _i in 0..BIMAX {
			for _j in 0..BIMAX{
				bins.push(SmallVec::<[usize;10]>::new());
			}
		}
		BinList{bins}
	}

	// fn clear(&mut self) {
	// 	for bin in self.bins.iter_mut() {
	// 		bin.clear();
	// 	}
	// }
}

/// Force calculation pieces

const LJN : i32 = 2;
/// Scalar Lennard-Jones force
fn lj(r: f64, p: Params) -> f64 {
	let Params{rad_sol, ep, .. } = p;
	let r0 = 2.0*rad_sol;
	let r1 = 0.6*r0;
	if r >= r0 {
		0.0
	} else if r < r1 {
		2.0*(LJN as f64)*ep/r1*((r0/r1).powi(2*LJN)-(r0/r1).powi(LJN))
	} else {
		2.0*(LJN as f64)*ep/r*((r0/r).powi(2*LJN)-(r0/r).powi(LJN))
	}
}

/// force between two solvent particles with positions ri and rj
fn forceij(ri: Point, rj: Point, p: Params) -> Point {
	let rij = ri-rj;
	let r = rij.norm();
	rij/r*lj(r, p)
}

/// force between the large particle and a solvent particle.
fn forcepar(rpar: Point, rsol: Point, p: Params) -> Point {
	let Params{rad_par, rad_sol, ..} = p;
	let shift = rad_par-rad_sol;
	let mut rij = rpar-rsol;
	// This checks if we need to wrap around our screen
	if rij.x < -SIZE+rad_par+rad_sol {
		rij.x += SIZE;
	} else if rij.x > SIZE-rad_par-rad_sol {
		rij.x -= SIZE;
	}
	if rij.y < -SIZE+rad_par+rad_sol {
		rij.y += SIZE;
	} else if rij.y > SIZE-rad_par-rad_sol {
		rij.y -= SIZE;
	}
	let r = rij.norm();
	let rprime = r-shift;
	rij/r*lj(rprime, p)
}

use std::iter;
fn force_calc( p: Params, rpar: Point, rsol: &[Point]) -> (Point, Vec<Point>) {
	// Initialize the forces
	let mut fpar = Point{x:0.0, y:0.0};
	let mut fsol : Vec<Point> = Vec::new();
	let len = rsol.iter().count();
	fsol.extend(iter::repeat(Point{x:0.0, y:0.0}).take(len));

	// Loop over all particle and stick them into bins
	let size = SIZE/BIMAX as f64;
	let mut binlist = BinList::new();
	for (idx,pos) in rsol.iter().enumerate() {
		let i = (pos.y/size) as usize;
		let j = (pos.x/size) as usize;
		binlist[[i,j]].push(idx);
	}
// 	// Loop over the bins and calculate the forces
	for i in 0..BIMAX {
		for j in 0..BIMAX {
			let bin = &binlist[[i,j]];
			match bin.len() {
				0 => {}, // Nothing here to see!
				1 => { // calc forces in neighboring cells
					let idx_i = bin[0];
					let ri = rsol[idx_i];
					let im = if i != 0 { i-1 } else { BIMAX-1 };
					let jm = if j != 0 { j-1 } else { BIMAX-1 };
					let jp = if j != BIMAX-1 { j+1 } else { 0 };
					let neighbors = [ (im, jm), (im, j), (im, jp), (i, jm) ];
					for &(ii,jj) in neighbors.iter() {
						let other_bin = &binlist[[ii,jj]];
						for &idx_j in other_bin.iter() {
							let mut rj = rsol[idx_j];

							if im == BIMAX-1 && ii == im {
								rj.y -= SIZE;
							}
							if jm == BIMAX-1 && jj == jm {
								rj.x -= SIZE;	
							}
							if jp == 0 && jj == jp {
								rj.x += SIZE;
							}

							let f = forceij(ri, rj, p);
							fsol[idx_i] = fsol[idx_i] + f;
							fsol[idx_j] = fsol[idx_j] - f;
						}
					}
					// calc_big_particle_force;
					let f = forcepar(rpar, ri, p);
					fpar = fpar + f;
					fsol[idx_i] = fsol[idx_i] - f;
				},
				_ => { // calc forces between particles in same bin
					for k in 0..bin.len()-1 {
						for l in k+1..bin.len() {
							let rk = rsol[bin[k]];
							let rl = rsol[bin[l]];
							let f = forceij(rk,rl,p);
							fsol[bin[k]] = fsol[bin[k]] + f;
							fsol[bin[l]] = fsol[bin[l]] - f;
						}
					}
					for &idx_i in bin.iter() {
						// calc forces in neighboring cells
						let ri = rsol[idx_i];
						let im = if i != 0 { i-1 } else { BIMAX-1 };
						let jm = if j != 0 { j-1 } else { BIMAX-1 };
						let jp = if j != BIMAX-1 { j+1 } else { 0 };
						let neighbors = [ (im, jm), (im, j), (im, jp), (i, jm) ];
						for &(ii,jj) in neighbors.iter() {
							let other_bin = &binlist[[ii,jj]];
							for &idx_j in other_bin.iter() {
								let mut rj = rsol[idx_j];

								if im == BIMAX-1 && ii == im {
									rj.y -= SIZE;
								}
								if jm == BIMAX-1 && jj == jm {
									rj.x -= SIZE;	
								}
								if jp == 0 && jj == jp {
									rj.x += SIZE;
								}

								let f = forceij(ri, rj, p);
								fsol[idx_i] = fsol[idx_i] + f;
								fsol[idx_j] = fsol[idx_j] - f;
							}
						}
						// calc forces on the big particle
						let f = forcepar(rpar, ri, p);
						fpar = fpar + f;
						fsol[idx_i] = fsol[idx_i] - f;
					}
				}
			}
		}
	}
	(fpar, fsol)
}

fn reset_temp( vsol: &mut [Point], p: Params) {
	let Params{msol, temp, ..} = p;
	let mut v2avg = 0.0;
	for vel in vsol.iter() {
		v2avg += vel.norm_squared();
	}
	v2avg /= vsol.len() as f64;
	let temp_current = msol/2.0/kB*v2avg*1.0e-6_f64;
	let alpha = (temp/temp_current).sqrt();
	for vel in vsol.iter_mut() {
		*vel = *vel*alpha;
	}
}

#[derive(Debug,Clone)]
struct FullSim{
	writing: bool,
	step_count: u32,
	state: State,
	p: Params,
	canvas: Canvas,
	textarea: TextArea,
}

impl FullSim {
	fn writeln(&self) {
		let t = self.state.t;
		let Point{x,y} = self.state.rpar + self.state.offset;
		self.textarea.write(&format!(",\n{{ {}, {}, {} }}", t, x, y));
	}

	fn writehead(&self) {
		self.textarea.writeln( "t(ns), x(nm), y(nm)");
		let t = self.state.t;
		let Point{x,y} = self.state.rpar + self.state.offset;
		self.textarea.write(&format!("{{ {{ {}, {}, {} }}", t, x, y));
	}

	fn writefoot(&self) {
		self.textarea.writeln("}\n");
	}
}

impl SimStep for FullSim {
	fn step( &mut self, _dt: f64) {
		if self.step_count == 19 {
			reset_temp(&mut self.state.vsol, self.p);
			if self.writing { 
				self.writeln(); 
			}
			self.step_count = 0;
		} else {
			self.step_count += 1;
		}
		let dt = 0.000025;
		self.state.step(dt, self.p);
		self.state.step(dt, self.p);
		self.state.step(dt, self.p);
		self.state.step(dt, self.p);
		self.state.draw(self.p, &self.canvas);
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

	let controls = Container::new("controls");
	let force_slider = Range::new("f_slider", "Force (pN) : ", 0.0, 100.0, 1.0, 10.0);
	controls.add( &force_slider);
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

	canvas.set_window(((0.0, 0.0), (SIZE,SIZE)));

	let p = Params::init();
	let state = State::init(0.6,p);
	state.draw( p, &canvas);

	let sim = FullSim{
		writing: false,
		step_count: 0,
		p,
		state,
		canvas: canvas.clone(),
		textarea: textarea.clone(),
	};

	let ref_sim = Simloop::new_ref(sim);
	// Simloop::start_loop(ref_sim.clone());

	sim_control.add_toggle_function({
		let ref_sim = ref_sim.clone();
		move | status:bool | {
			if status {
				Simloop::start_loop(ref_sim.clone());
				let sim = &mut ref_sim.borrow_mut().state;
				if sim.writing {
					sim.writehead();
				}
			} else {
				ref_sim.borrow_mut().stop_loop();
				let sim = &mut ref_sim.borrow_mut().state;
				if sim.writing {
					sim.writefoot();
				}
			}
		}
	});

	sim_reset.add_button_function({
		let ref_sim = ref_sim.clone();
		let sim_control = sim_control.clone();
		let force_slider = force_slider.clone();
		move | _:bool | {
			if sim_control.query() {
				ref_sim.borrow_mut().stop_loop();
				let sim = &mut ref_sim.borrow_mut().state;
				sim_control.set(false);
				sim.writefoot();
			}
			let sim = &mut ref_sim.borrow_mut().state;
			sim.state = State::init(0.6, sim.p);
			sim.p.force = force_slider.query();
			sim.state.draw( sim.p, &sim.canvas);
		}
	});

	force_slider.add_continuous_range_function({
		let ref_sim = ref_sim.clone();
		move | val:f64 | {
			let sim = &mut ref_sim.borrow_mut().state;
			sim.p.force = val;
		}
	});

	output_control.add_toggle_function({
		let sim_control = sim_control.clone();
		let ref_sim = ref_sim.clone();
		move | status:bool | {
			let sim = &mut ref_sim.borrow_mut().state;
			sim.writing = status;
			sim.step_count = 0;
			if sim_control.query() {
				if status {
					sim.writehead();
				} else {
					sim.writefoot();
				}
			}
		} 
	});

	output_clear.add_button_function({
		let ref_sim = ref_sim.clone();
		let sim_control = sim_control.clone();
		let textarea = textarea.clone();
		move | _:bool | {
			textarea.clear();
			let sim = &mut ref_sim.borrow_mut().state;
			if sim_control.query() && sim.writing {
				sim.writehead();
			}
		}
	});
}
