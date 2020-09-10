mod cli;
mod errors;

use cli::App;

fn main() {
    match App::parse_from_cli() {
        Ok(app) => app.run(),
        Err(_) => eprintln!("An error happened during app execution"),
    }
}
