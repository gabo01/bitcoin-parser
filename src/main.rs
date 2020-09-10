mod cli;
mod errors;

use cli::App;

fn main() {
    let _app = App::parse_from_cli();
}
