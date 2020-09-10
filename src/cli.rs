use clap::{App as CApp, ArgMatches, load_yaml};

pub struct App;

impl App {
    pub fn parse_from_cli() -> App {
        let config = load_yaml!("clidef.yml");
        let matches = CApp::from_yaml(config).get_matches();
        App::from_matches(&matches)
    }

    fn from_matches(_matches: &ArgMatches) -> App {
        App
    }
}
