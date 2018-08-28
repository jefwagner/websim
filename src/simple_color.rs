#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Color {
	Rgb{r: u8, g: u8, b:u8},
	Rgba{r: u8, g: u8, b: u8, a: u8}
}

impl Color {
	fn to_string(&self) -> String {
		match self {
			Color::Rgb{r,g,b} => {
				format("rgb( {}, {}, {})",r,g,b)},
			Color::Rgba{r,g,b,a} => {
				format("rgba( {}, {}, {}, {})",r,g,b,a as f64/255.0)}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_string() {
		let rgb_white = Color::Rgb{r:255,b:255,c:255};
		let rgba_opaque_white = Color::Rgb{r:255,g:255,b:255,a:255};
		let white_string_1 = rgb_white.to_string();
		let white_string_2 = rgba_opaque_white.to_string();

		assert!(white_string_1 == "rgb( 255, 255, 255)");
		assert!(white_string_2 == "rgba( 255, 255, 255, 1.0)");
	}
}