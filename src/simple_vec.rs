/// 2D euclidean vector of floats
/// =============================
///
/// This module defines a two component vector
/// and defines a set of useful methods for the
/// vectors. It overloads the simple arithmatic
/// operators `-`, `+`, `*`, and `/` to implement
/// vector addition, vector subtraction and
/// negations, and scalar multiplication and 
/// division.
use std::ops::{
	Neg,
	Add,
	Sub,
	Mul,
	Div,
};

/// Two components, x and y;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
	pub x: f64,
	pub y: f64,
}

/// Vector Negation
/// ---------------
/// negates (multiplies by -1) each component 
/// of the vector
/// 
/// # Examples
/// ```
/// let v = Vec2{x: 1.0, y: 2.0};
/// assert_eq!(-v, Vec2{x: -1.0, y: -2.0});
/// ```
impl Neg for Vec2 {
	type Output = Vec2;

	fn neg(self) -> Vec2 {
		Vec2{ x: -self.x, y: -self.y}
	}
}

/// Vector Addition
/// ---------------
/// componentwise addition of two vectors.
///
/// # Examples
/// ```
/// let va = Vec2{x: 1.0, y: 1.0};
/// let vb = Vec2{x: 1.0, y: -1.0};
/// assert_eq!(va+vb, Vec2{x: 2.0, y: 1.0});
/// ```
impl Add for Vec2 {
	type Output = Vec2;

	fn add(self, rhs: Vec2) -> Vec2 {
		Vec2{ x: self.x + rhs.x, y: self.y + rhs.y}
	}
}

impl Sub for Vec2 {
	type Output = Vec2;

	fn sub(self, rhs: Vec2) -> Vec2 {
		Vec2{ x: self.x - rhs.x, y: self.y - rhs.y}
	}
}

impl Mul<f64> for Vec2 {
	type Output = Vec2;

	fn mul(self, rhs: f64) -> Vec2 {
		Vec2{ x: self.x*rhs, y: self.y*rhs}
	}
}

impl Mul<Vec2> for f64 {
	type Output = Vec2;

	fn mul(self, rhs: Vec2) -> Vec2 {
		Vec2{ x: self*rhs.x, y: self*rhs.y}
	}
}

impl Div<f64> for Vec2 {
	type Output = Vec2;

	fn div(self, rhs: f64) -> Vec2 {
		Vec2{ x: self.x/rhs, y: self.y/rhs}
	}
}

impl Vec2 {

	pub fn zero() -> Vec2 { 
		Vec2{ x: 0_f64, y: 0_f64}
	}

	pub fn dot(self, other: Vec2) -> f64 {
		self.x*other.x + self.y*other.y
	}

	pub fn norm(self) -> f64 {
		self.x.hypot(self.y)
	}

	pub fn norm_squared(self) -> f64 {
		self.x*self.x + self.y*self.y
	}

	pub fn cross(self, other: Vec2) -> f64 {
		self.x*other.y - self.y*other.x
	}

	pub fn transform(self, transform: Transform2D) -> Vec2 {
		Vec2{
			x: self.x*transform.a + self.y*transform.b + transform.dx,
			y: self.x*transform.c + self.y*transform.d + transform.dy,
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform2D {
	pub a: f64,
	pub b: f64,
	pub dx: f64,
	pub c: f64,
	pub d: f64,
	pub dy: f64,
}

impl Transform2D {

	pub fn new( a: f64, b: f64, dx: f64, c: f64, d: f64, dy: f64) -> Transform2D {
		Transform2D{ a, b, dx, c, d, dy}
	}

	pub fn eye() -> Transform2D {
		Transform2D{ 
			// two by two transformation matrix
			a: 1_f64, b: 0_f64, 
			c: 0_f64, d: 1_f64, 
			// translation 
			dx: 0_f64, dy: 0_f64 }
	}

	pub fn rotate( angle: f64, point: Vec2) -> Transform2D {
		let (s, c) = angle.sin_cos();
		Transform2D{ 
			// two by two transformation matrix
			a: c, b: -s, 
			c: s, d: c, 
			// translation 
			dx: (1_f64-c)*point.x + s*point.y,
			dy: (1_f64-c)*point.y - s*point.x,
		}
	}

	pub fn scale_xy( sx: f64, sy: f64, point: Vec2) -> Transform2D {
		Transform2D{ 
			// two by two transformation matrix
			a: sx, b: 0_f64, 
			c: 0_f64, d: sy,
			// translation 
			dx: (1_f64-sx)*point.x,
			dy: (1_f64-sy)*point.y,
		}
	}

	pub fn scale( s: f64, point: Vec2) -> Transform2D {
		Transform2D::scale_xy( s, s, point)
	}

	pub fn translate( dx: f64, dy: f64) -> Transform2D {
		Transform2D{ 
			// two by two transformation matrix
			a: 1_f64, b: 0_f64, 
			c: 0_f64, d: 1_f64, 
			// translation 
			dx, dy }
	}

	pub fn inv( self) -> Transform2D {
		let inv_det = 1_f64/(self.a*self.d - self.b*self.c);
		Transform2D{
			// two by two transformation matrix
			a: self.d*inv_det, b: -self.b*inv_det,
			c: -self.c*inv_det, d: self.a*inv_det,
			// translation 
			dx: (self.b*self.dy - self.d*self.dx)*inv_det,
			dy: (self.c*self.dx - self.a*self.dy)*inv_det,
		}
	}

	pub fn combine_left(self, lhs: Transform2D) -> Transform2D {
		Transform2D{
			// two by two transformation matrix
			a: lhs.a*self.a + lhs.b*self.c,
			b: lhs.a*self.b + lhs.b*self.d,
			c: lhs.c*self.a + lhs.d*self.c,
			d: lhs.c*self.b + lhs.d*self.d,
			// translation 
			dx: lhs.a*self.dx + lhs.b*self.dy + lhs.dx,
			dy: lhs.c*self.dx + lhs.d*self.dy + lhs.dy,
		}
	}

	pub fn combine_right(self, rhs: Transform2D) -> Transform2D {
		Transform2D{
			// two by two transformation matrix
			a: self.a*rhs.a + self.b*rhs.c,
			b: self.a*rhs.b + self.b*rhs.d,
			c: self.c*rhs.a + self.d*rhs.c,
			d: self.c*rhs.b + self.d*rhs.d,
			// translation 
			dx: self.a*rhs.dx + self.b*rhs.dy + self.dx,
			dy: self.c*rhs.dx + self.d*rhs.dy + self.dy,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Test out the overloaded arithmetic operators
	#[test]
	fn test_vec_arithmatic() {
		let va = Vec2{ x: 1.0, y: 1.0};
		let vb = Vec2{ x: 1.0, y: -1.0};

		assert_eq!(-va, Vec2{x: -1.0, y: -1.0});
		assert_eq!(va+vb, Vec2{x: 2.0, y: 0.0});
		assert_eq!(va-vb, Vec2{x: 0.0, y: 2.0});
		assert_eq!(2.0*va, Vec2{x: 2.0, y: 2.0});
		assert_eq!(va*2.0, Vec2{x: 2.0, y: 2.0});
		assert_eq!(va/2.0, Vec2{x: 0.5, y: 0.5});
	}

	/// Test out the dot, cross, and norm methods
	#[test]
	fn test_vec_methods() {
		let va = Vec2{ x: 1.0, y: 1.0};
		let vb = Vec2{ x: 1.0, y: -1.0};
		
		assert_eq!(Vec2::zero(), Vec2{x:0_f64, y:0_f64});
		assert_eq!(va.dot(va), 2.0);
		assert_eq!(va.dot(vb), 0.0);
		assert_eq!(va.cross(va), 0.0);
		assert_eq!(va.cross(vb), -2.0);
		assert_eq!(vb.cross(va), 2.0);
		assert_eq!(va.norm(), 2_f64.sqrt());
		assert_eq!(va.norm_squared(), 2.0);
	}

	/// Define an "almost equal" function
	const EPSILON : f64 = 0.0000000001f64;
	const PI : f64 = 3.14159265358979323846264338327950288f64;

	fn almost_eq( v0: Vec2, v1: Vec2) -> bool {
		2.0*(v0-v1).norm()/(v0.norm()+v1.norm()) < EPSILON
	}

	/// Test out the vector transformations
	#[test]
	fn test_vec_transforms() {
		let point = Vec2{x: 1.0, y: 2.0};
		let rot_center = Vec2{x: 1.0, y: 1.0};
		let scale_center = Vec2{x: 0.0, y: 1.0};

		assert_eq!(
			point.transform( Transform2D::eye() ), 
			point);
		assert!(almost_eq(
			point.transform( Transform2D::rotate(PI/2.0, rot_center) ),
			Vec2{x:0.0, y:1.0}));
		assert!(almost_eq(
			point.transform( Transform2D::rotate(-PI/2.0, rot_center) ),
			Vec2{x:2.0, y:1.0}));
		assert!(almost_eq(
			point.transform( Transform2D::scale_xy(2.0, 3.0, scale_center) ),
			Vec2{x:2.0, y: 4.0}));
		assert!(almost_eq(
			point.transform( Transform2D::scale(2.0, scale_center) ),
			Vec2{x:2.0, y: 3.0}));
		assert!(almost_eq(
			point.transform( Transform2D::translate(1.0, -1.0) ),
			Vec2{x:2.0, y: 1.0}));
	}

	/// Define an norm and almost equal for the transform matricies
	fn transform_norm( t: Transform2D) -> f64 {
		let Transform2D{a,b,c,d,dx,dy} = t;
		(a*a+b*b+c*c+d*d+dx*dx+dy*dy).sqrt()
	}

	fn transform_almost_eq( t0: Transform2D, t1: Transform2D) -> bool {
		let Transform2D{a:a0,b:b0,c:c0,d:d0,dx:dx0,dy:dy0} = t0;
		let Transform2D{a:a1,b:b1,c:c1,d:d1,dx:dx1,dy:dy1} = t1;
		let dif = transform_norm( Transform2D::new(a0-a1, b0-b1, dx0-dx1, c0-c1, d0-d1, dy0-dy1));
		2.0*dif/(transform_norm(t0)+transform_norm(t1)) < EPSILON	
	}

	/// Test out left and right matrix multiplication and inversion
	#[test]
	fn test_transform_methods() {
		let origin = Vec2::zero();
		let point = Vec2{x:1.0, y:1.0};
		let eye = Transform2D::eye();
		let rot0 = Transform2D::rotate(PI/2.0, origin);
		let rot1 = Transform2D::rotate(PI/2.0, point);
		let trans_neg = Transform2D::translate(-1.0, -1.0);
		let trans_pos = Transform2D::translate(1.0, 1.0);

		assert!(transform_almost_eq(
			rot0.combine_left(rot0.inv()), eye));
		assert!(transform_almost_eq(
			rot1.combine_right(rot1.inv()), eye));
		let stepped_rot = trans_neg.combine_left(rot0).combine_left(trans_pos);
		assert!(transform_almost_eq(
			stepped_rot, rot1));
	}
}