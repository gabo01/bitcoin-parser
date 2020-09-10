use clap::ArgMatches;
use std::env::current_dir;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Dump {
    file: PathBuf,
    target: PathBuf,
}

impl Dump {
    pub fn from_matches(matches: &ArgMatches) -> Self {
        Self {
            file: Self::get_required_path(matches, "file"),
            target: Self::get_required_path(matches, "target"),
        }
    }

    fn get_required_path(matches: &ArgMatches, path: &str) -> PathBuf {
        let path = PathBuf::from(
            matches
                .value_of(path)
                .expect("Value is required in configuration present at interface.yml"),
        );

        if !path.is_absolute() {
            let mut curr_dir = current_dir().expect("Unable to retrieve the current directory");
            curr_dir.push(path);
            curr_dir
        } else {
            path
        }
    }

    pub fn run(&self) {
        unimplemented!()
    }
}
