extern crate websim;

use websim::container::Container;
use websim::output::TextArea;
use websim::simple_color::Color::{Rgb,Rgba};

fn main() {
    let app = Container::new("app");
    app.add_to_body();
    let textarea = TextArea::new("txt");
    app.add(&textarea);

    let color = Rgb{r:255,g:255,b:255};
    textarea.writeln( &color.to_string());
    let color = Rgba{r:255,g:255,b:255,a:255};
    textarea.writeln( &color.to_string());
}