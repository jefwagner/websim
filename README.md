websim
======

This library provides a web-based simulation library. This is built upon stdweb

Simulation with object oriented programming:
* object holds data
* object evolves in steps

Simple example
```rust
///My first simulation of a damped simple harmonic osciallator

/// Constant parametes
const SPRINGK : f64 = 1.0; // N/m
const GAMMA : f64 = 0.1; // N/(m/s)
const MASS : f64 = 0.2; // kg
/// This struct holds the data about 
#[derive(Debug, Clone)]
struct SpringState {
  x: f64, //position
  v: f64, //velocity
}

impl SpringState {
  // Simple damped spring force
  fn force(&self) -> f64 {
    -SPRINGK*self.x - GAMMA*self.v
  }

  // Evolve the positions and velocities one small time step forward
  fn step(&mut self, dt: f64) {
    self.x += self.v*dt;
    self.v += self.force()/MASS*dt;
  }
}

/// Run the simulation and output in the main function
const DT : f64 = 0.001; // 1ms
const TMAX : f64 = 60.0; // 1min

fn main() {
  let x0 = 1.0; // m
  let v0 = 0.0; // m/s

  let mut spring = SpringState{x:x0, v:v0};

  let mut t = 0.0;
  println!("time(s) position(m) velocity(m/s)");
  println!("{} {} {}",t,spring.x,spring.v);
  while t < TMAX {
    t += DT;
    spring.step(DT);
    println!("{} {} {}",t,spring.x,spring.v);
  } 
}
```

Library:
  provides a wrapper for the object 
    uses request-animation-frame to call the step

For the web-based widgets, the library provides:
* Outputs
  - Canvas
  - TextArea
* Controls
  - Checkboxes
  - Buttons
  - Dropdown selectors
  - Sliders


To do:
------
- [ ] Finish simple harmonic oscillator
-   [ ] Get reset working correctly
-   [ ] Get header & start/stop of data working
- [ ] Create einstein relation simulation
- [ ] Create brownian polymer simulation
- [ ] Comment simple_vec
- [ ] Add to simple_color
-   [ ] Add sRGB blending
-   [ ] Add HSV/HSB to RGB
-   [ ] Add named colors
- [ ] Comment simple_color
- [ ] Comment container
- [ ] Comment control
- [ ] Comment output
- [ ] Comment gfx
- [ ] Comment extra
- [ ] Comment examples
- [ ] Finish README

