extern crate ishido;
extern crate cursive;

use cursive::Cursive;
use cursive::views::{Dialog, TextView};

use ishido::start_game;

fn main() {

    let mut siv = Cursive::new();
    siv.add_global_callback('q', |s| s.quit());
    let start = Dialog::around(TextView::new("Welcome to Ishido!"))
                       .title("Ishido")
                       .button("Start", |s| start_game(s))
                       .button("Help", |_| unimplemented!())
                       .button("Quit", |siv| siv.quit());
    siv.add_layer(start);

    siv.run();
}
