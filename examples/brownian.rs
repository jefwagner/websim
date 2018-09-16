extern crate smallvec;
use smallvec::SmallVec;

extern crate websim;

use websim::container::Container;
use websim::control::{
	Button,
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

use std::rc::Rc;
use std::cell::RefCell;

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
			rad_par: 1.5, // units: nm
			msol: 30.0, // units: 10^-27 kg
			mpar: 30000.0, // units: 10^-27 kg
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
	// Solvent particle position and velocities
	rsol: Vec<Point>,
	vsol: Vec<Point>,
}

/// Size of screen
const SIZE : f64 = 30.0;
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
		let mut rsol = Vec::new();
		let mut vsol = Vec::new();

		// let s0 = get_time();
		// let s1 = 0_u64;
		let vstd = 1000.0*(kB*temp/msol).sqrt(); // ns/nm
		let mut maxwell_dist = NormalDist::new(0.0, vstd);
 		// maxwell_dist.seed(s0,s1);

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
			rsol,
			vsol,
		}
	}

	fn step(&mut self, dt: f64, p: Params) {
		let (fpar0, fsol0) = force_calc(p, self.rpar, &self.rsol);

		let apar0 = fpar0/p.mpar*1.0E+6_f64;
		self.rpar = self.rpar + self.vpar*dt + 0.5*apar0*dt*dt;
		wrap(&mut self.rpar); 

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
			// if a1.norm() > 40.0E+6_f64 {
			// 	println!("vel={:?}",vel);
			// }
		}
	}
	// 	let (fpar0, fsol0) = force_calc(p, self.rpar, &self.rsol);

	// 	let apar0 = fpar0/p.mpar*1.0E+6_f64;
	// 	self.rpar = self.rpar + self.vpar*dt + 0.5*apar0*dt*dt;

	// 	{
	// 	let pos_vel_force = self.rsol.iter_mut()
	// 		.zip( self.vsol.iter())
	// 		.zip( fsol0.iter())
	// 		.map( |((r,v),f)| (r,v,f) );

	// 	for (pos,&vel,&force) in pos_vel_force {
	// 		let a = force/p.msol*1.0E+6_f64;
	// 		*pos = *pos + vel*dt + 0.5*a*dt*dt;
	// 		if pos.x > SIZE {
	// 			pos.x -= SIZE;
	// 		}else if pos.x < 0.0 {
	// 			pos.x += SIZE;
	// 		}
	// 		if pos.y > SIZE {
	// 			pos.y -= SIZE;
	// 		} else if pos.y < 0.0 {
	// 			pos.y += SIZE;
	// 		}
	// 	}
	// 	}

	// 	let (fpar1, fsol1) = force_calc(p, self.rpar, &self.rsol);

	// 	let apar1 = fpar1/p.mpar*1.0E+6_f64;
	// 	self.vpar = self.vpar + 0.5*(apar0 + apar1)*dt;

	// 	let vel_force0_force1 = self.vsol.iter_mut()
	// 		.zip( fsol0.iter())
	// 		.zip( fsol1.iter())
	// 		.map( |((v,f0),f1)| (v,f0,f1) );

	// 	for (vel, &force0, &force1) in vel_force0_force1 {
	// 		let a0 = force0/p.msol*1.0E+6_f64;
	// 		let a1 = force1/p.msol*1.0E+6_f64;
	// 		*vel = *vel + 0.5*(a0 + a1)*dt;
	// 	}

	// }

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
type Bin = SmallVec<[usize;4]>;
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
				bins.push(SmallVec::<[usize;4]>::new());
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
	if r >= r0 {
		0.0
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

#[derive(Debug,Clone)]
struct FullSim{
	writing: bool,
	step_count: u32,
	state: State,
	p: Params,
	canvas: Canvas,
	textarea: TextArea,
}

impl SimStep for FullSim {
	fn step( &mut self, _dt: f64) {
		let dt = 0.00002;
		self.state.step(dt, self.p);
		self.state.draw(self.p, &self.canvas);
	}
}

fn main() {
	let app = Container::new("app");
	app.add_to_body();
	let mut canvas = Canvas::new("canvas");
	app.add( &canvas);
	let step_button = Button::new("step","Step");
	app.add( &step_button);
	let textarea = TextArea::new("txt");
	app.add( &textarea);

	canvas.set_window(((0.0, 0.0), (SIZE,SIZE)));

	let p = Params::init();
	let state = State::init(0.6,p);

	let sim = FullSim{
		writing: false,
		step_count: 0,
		p,
		state,
		canvas: canvas.clone(),
		textarea: textarea.clone(),
	};

	let ref_sim = Simloop::new_ref(sim);
	Simloop::start_loop(ref_sim.clone());

	// let ref_state = Rc::new(RefCell::new(state));
	// ref_state.borrow_mut().draw(p, &canvas);

	// step_button.add_button_function({
	// 	let ref_state = ref_state.clone();
	// 	let canvas = canvas.clone();
	// 	let textarea = textarea.clone();
	// 	move | _:bool | {
	// 		let mut state = ref_state.borrow_mut();
	// 		state.step(0.00002, p);
	// 		state.draw(p, &canvas);

	// 		textarea.clear();
	// 		let (fpar, _fsol) = force_calc(p, state.rpar, &state.rsol);
	// 		textarea.writeln(&format!("f = {:?}", fpar));
	// 		// for force in &fsol {
	// 		// 	let Point{x, y} = *force;
	// 		// 	if x != 0.0 && y != 0.0 {				
	// 	 // 			textarea.writeln(&format!("f = {:?}",force));
	// 	 // 		}
	// 		// } 
	// 	}
	// })

	// let (fpar, fsol) = force_calc(p, state.rpar, &state.rsol);
	// for force in &fsol {
	//  	textarea.writeln(&format!("f = {:?}",force));
	// } 
	// textarea.writeln("");
	// textarea.writeln(&format!("number of solvent molecules: {}", state.rsol.len()));

}
