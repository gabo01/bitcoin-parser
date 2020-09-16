use clap::{load_yaml, App as CApp, ArgMatches};

use crate::errors::Error;

mod ops;

#[derive(Debug)]
pub struct App {
    trace: bool,
    operation: Operation,
}

impl App {
    pub fn parse_from_cli() -> Result<Self, Error> {
        let config = load_yaml!("interface.yml");
        let matches = CApp::from_yaml(config).get_matches();
        Ok(App::from_matches(&matches)?)
    }

    fn from_matches(matches: &ArgMatches) -> Result<Self, Error> {
        Ok(App {
            trace: matches.is_present("backtrace"),
            operation: Operation::from_matches(matches)?,
        })
    }

    pub fn run(&self) {
        self.operation.run();
    }
}

#[derive(Debug)]
enum Operation {
    Dump(ops::Dump),
}

impl Operation {
    fn from_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            ("dump", Some(sb_matches)) => Ok(Self::Dump(ops::Dump::from_matches(sb_matches))),
            _ => unreachable!(), // subcommand specification is required from a restricted subset in interface.yml
        }
    }

    fn run(&self) {
        match *self {
            Operation::Dump(ref op) => op.run(),
        }
    }
}
