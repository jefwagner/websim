extern crate websim2;

use websim2::container::Container;
use websim2::output::TextArea;

fn main() {
    let app = Container::new("app");
    app.add_to_body();
    let textarea = TextArea::new("txt");

    app.add(&textarea);
    textarea.writeln("Hello world!");
}
