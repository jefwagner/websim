extern crate websim;

use websim::container::Container;
use websim::output::TextArea;

fn main() {
    let app = Container::new("app");
    app.add_to_body();
    let textarea = TextArea::new("txt");
    app.add(&textarea);

    textarea.writeln("Hello world!");
}
