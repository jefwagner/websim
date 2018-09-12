/// Xoroshift implemented in rust
///
///	Algoritm Written in 2016-2018 by David Blackman and 
/// Sebastiano Vigna (vigna@acm.org)
///
/// http://vigna.di.unimi.it/xorshift/xoroshiro128plus.c
///
/// This implementation written in 2018 by Jef Wagner
/// (wagnerj@union.edu)
///
/// The uniform random floats is implemented as described
/// by Andy Gainey in 2016:
///
/// https://experilous.com/1/blog/post/perfect-fast-random-floating-point-numbers#cast-method


use std::f64;
use std::f64::consts::PI;

/// These are "random" seeds taken from random.org
/// lol! constant "random" numbers!
const S0 : u64 = 0x2475136db02a2834_u64;
const S1 : u64 = 0x9417ed9540b8fe6e_u64;

/// The random number generator has 128 bits of internal
/// state, stored at two `u64`s.
#[derive(Debug,Clone,Copy)]
pub struct Rng{
	s0: u64,
	s1: u64,
}

impl Rng {
	/// A new random number generator. Warning!!! The
	/// random number genertor is a started with a 
	/// constant seed. You have to manually set the 
	/// seed by hand if you want a different sequence
	/// each time you run
    pub fn new() -> Rng {
        Rng{s0: S0, s1: S1}
    }

    /// Sets the interal state of the random number
   	/// generator.
	pub fn seed( &mut self, s0: u64, s1: u64) {
		self.s0 = s0.wrapping_add(S0);
		self.s1 = s1.wrapping_add(S1); 
	}

	/// Gives a random `u64` between 0 and 2^(64)-1
	pub fn next(&mut self) -> u64 {
		let s0 = self.s0;
		let mut s1 = self.s1;
		let result = s0.wrapping_add(s1);

		s1 ^= s0;
		self.s0 = s0.rotate_left(24) ^ s1 ^ (s1 << 16);
		self.s1 = s1.rotate_left(37);
		result
	}
	
	/// Gives a random `f64` uniformly distributed in
	/// the range (0,1]: exclusive zero, inclusive one.
	pub fn uniform(&mut self) -> f64 {
		let rand_int = self.next();
		let bits = 0x3FF0000000000000_u64 | (rand_int >> 12);
		2.0_f64 - f64::from_bits(bits)
	}
}

/// This is a wrapper around the random number genertor that
/// will generte random reals from a normal distribution with
/// a given mean (mu) and standard deviation (sigma).
///
/// The implimentation used the Box-Muller method, which 
/// uses a pair of random numbers in a uniform distribution
/// to generate a pair of random numbers from a standard 
/// normal distribution. Since the numbers can be used one
/// at a time, the second number of the pair is stored and
/// used the next time the genertor is called.
#[derive(Debug,Copy,Clone)]
pub struct NormalDist {
    rng: Rng,
    stored: Option<f64>,
    pub mu: f64,
    pub sigma: f64,
}

impl NormalDist {
	/// Creates a new normal distrubtion with mean `mu` and
	/// standard deviation `sigma`. Warning!!! The
	/// random number genertor is a started with a 
	/// constant seed. You have to manually set the 
	/// seed by hand if you want a different sequence
	/// each time you run
    pub fn new(mu: f64, sigma: f64) -> NormalDist {
        let rng = Rng::new();
        let stored = None;
        NormalDist{rng,stored,mu,sigma}
    }
    
    /// Reset the state of the random number generator
    pub fn seed(&mut self, s0: u64, s1:u64) {
        self.rng.seed(s0,s1);
    }
    
    /// Gives a random `f64` drawn from the normal 
    /// distribution: N(mu, sigma^2).
    pub fn next(&mut self) -> f64 {
        match self.stored {
            Some(val) => {
                self.stored.take();
                val
            },
            None => {
                let u0 = self.rng.uniform();
                let u1 = self.rng.uniform();
                let r = self.sigma*(-2.0*u0.ln()).sqrt();
                let (s,c) = (2.0*PI*u1).sin_cos();
                self.stored = Some(r*s+self.mu);
                r*c+self.mu
            }
        }
    }
}

// blahbahExample
// fn main() {
//     println!("Hello rust");
//     let mut rng = Rng::new();
//     for _ in 0..10 {
//         println!("Random 64bit integer: {:x}",rng.next());
//     }
//     for _ in 0..10 {
//         println!("Random 64bit float between (0,1]: {}",rng.uniform());
//     }
    
//     let mut normal = NormalDist::new(2.0, 5.0);
//     let mut rand_array = [0.0_f64; 1000];
//     for val in rand_array.iter_mut() {
//         *val = normal.next();
//     }
//     println!("Random 64bit floats from a normal distribution N(2.0, 5.0)");
//     for val in rand_array.iter() {
//         println!("{},",val);
//     }
// }

