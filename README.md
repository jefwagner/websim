websim
======

This library provides a web-based simulation library. This is built upon stdweb, and is still in a very early state.

To do:
------
- [x] Finish simple harmonic oscillator
  - [x] Get reset working correctly
  - [x] Get header & start/stop of data working
- [ ] Create einstein relation simulation
- [ ] Create brownian polymer simulation
- [ ] Comment simple_vec
- [ ] Add to simple_color
  - [ ] Add sRGB blending
  - [ ] Add HSV/HSB to RGB
  - [ ] Add named colors
- [ ] Comment simple_color
- [ ] Comment container
- [ ] Comment control
- [ ] Comment output
- [ ] Comment gfx
- [ ] Comment extra
- [ ] Comment examples
- [ ] Finish README

Overview
--------
Paragraph describing numerical simulations.

Description of simple simuations

Simulation with object oriented programming:
* constant parameters
* object holds data
* object evolves in steps
* external loop controls the stepping proceedure
* often NO visualization, only outputs numeric data

Describing of shortcomings, addition of interactive simulations.

Interactive simulation in browser:
* non-constant parameters controlable with widgets
* object holds data
* object evolves in steps
* the browsers java-script event loops controls execution
* visualization is in real-time on the web page
* numerical output must be written to a text-box

This library provides:
* Simple widgets for parameter control including:
  - Checkboxes
  - Buttons
  - Dropdown selectors
  - Sliders
* Interface to web-based outputs including:
  - Canvas
  - TextArea (a multi-line textbox)
* Wrapper to attach the simulation object to interface 
  with the event loop through the "requestion animation 
  frame" event.

Example of using library to convert simple simulation into interactive simulations.

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
