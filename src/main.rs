use anyhow::Context;
use anyhow::Result;

mod cli;
mod errors;

use cli::App;

fn main() -> Result<()> {
    App::parse_from_cli().context("Unable to parse cli arguments")?.run()
}
