websim
======

This library provides a web-based simulation library. This is built upon stdweb

To do:
------
[ ] Finish simple harmonic oscillator
	[ ] Get reset working correctly
	[ ] Get header & start/stop of data working
[ ] Create einstein relation simulation
[ ] Create brownian polymer simulation
[ ] Comment simple_vec
[ ] Add to simple_color
	[ ] Add sRGB blending
	[ ] Add HSV/HSB to RGB
	[ ] Add named colors
[ ] Comment simple_color
[ ] Comment container
[ ] Comment control
[ ] Comment output
[ ] Comment gfx
[ ] Comment extra
[ ] Comment examples
[ ] Finish README

For the web-based widgets, the library provides:
	* Outputs
		- Canvas
		- TextArea
	* Controls
		- Checkboxes
		- Buttons
		- Dropdown selectors
		- Sliders

Simulation:
	object holds data
	object evolves in steps

Library:
	provides a wrapper for the object 
	uses request-animation-frame to call the step.
