extern crate websim;
extern crate smallvec;

use smallvec::SmallVec;

use websim::container::Container;
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

impl State {
	/// Create an initial arrangement of solvent
	/// molecules. The particles are put on a grid a
	/// distance `spacing` appart, and any that overlap
	/// the central large particle are rejected. The 
	/// initial velocites are taken from a maxwell
	/// distribution.
	fn init(spacing: f64, p: Params) -> State {
		let t = 0.0;
		let Params{rsol:rad_sol, rpar: rad_par, temp, msol, ..} = p;
		let rpar = Point{x:SIZE/2.0, y:SIZE/2.0};
		let vpar = Point{x:0.0, y:0.0};
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

	fn clear(&mut self) {
		for bin in self.bins.iter_mut() {
			bin.clear();
		}
	}
}

/// Force calculation pieces

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
		24.0*ep*rij/r2*(2.0*ir12-ir6)
	}
}

/// Stockmeyer force between the two points
fn stkmyr(ri: Point, rj: Point, p: Params) -> Point {
	let Params{rpar, rsol, ep, ..} = p;
	let sigma = 2.0*rsol;
	let shift = rpar-rsol; // Maybe
	let rij = ri-rj;
	let r = rij.norm();
	if r > shift+sigma*CBRT2.sqrt() {
		Point{x:0.0, y:0.0}
	} else {
		let ir1 = sigma/(r-shift);
		let ir6 = ir1.powi(6);
		let ir12 = ir6*ir6;
		24.0*ep*rij/(r*(r-shift))*(2.0*ir12-ir6)
	}
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
							let rj = rsol[idx_j];
							let f = lj(ri, rj, p);
							fsol[idx_i] = fsol[idx_i] + f;
							fsol[idx_j] = fsol[idx_j] - f;
						}
					}
					// calc_big_particle_force;
					let f = stkmyr(rpar, ri, p);
					fpar = fpar + f;
					fsol[idx_i] = fsol[idx_i] - f;
				},
				_ => { // calc forces between particles in same bin
					for k in 0..bin.len()-1 {
						for l in k+1..bin.len() {
							let rk = rsol[bin[k]];
							let rl = rsol[bin[l]];
							let f = lj(rk,rl,p);
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
								let rj = rsol[idx_j];
								let f = lj(ri, rj, p);
								fsol[idx_i] = fsol[idx_i] + f;
								fsol[idx_j] = fsol[idx_j] - f;
							}
						}
						// calc forces on the big particle
						let f = stkmyr(rpar, ri, p);
						fpar = fpar + f;
						fsol[idx_i] = fsol[idx_i] - f;
					}
				}
			}
		}
	}
	(fpar, fsol)
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
	for i in 0..2 {
		let p1 = Point{x:0.0, y:0.3}+(i as f64)*dx;
		let f = lj(p0, p1, p);
		textarea.writeln(&format!("{{{}, {}}},",p1.y, f.y));
	}
	textarea.writeln("");
	let (fpar, fsol) = force_calc(p, state.rpar, &state.rsol);
	for force in &fsol {
	 	textarea.writeln(&format!("f = {:?}",force));
	}
	// textarea.writeln("");
	// textarea.writeln(&format!("number of solvent molecules: {}", state.rsol.len()));

}
