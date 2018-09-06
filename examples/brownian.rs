extern crate websim;

use websim::simple_vec::Vec2 as Point;

#[derive(Debug)]
struct Name {
	// Particle position and velocity
	ppos: Point,
	pvel: Point,
	// Solvent particle position and velocities
	psol: Vec<Point>,
	vsol: Vec<Point>,
}

fn lj_f(r:f64) -> f64 {
	inv_r = 1.0/r;
	
}
