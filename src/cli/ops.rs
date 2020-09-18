use clap::ArgMatches;
use std::env::current_dir;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use btlib::blkparser::BitcoinParser;
use btlib::disk::JsonWriter;
use btlib::disk::Writer;
use btlib::parser::ParallelParser;

#[derive(Debug)]
pub struct Dump {
    folder: bool,
    path: PathBuf,
    target: PathBuf,
}

impl Dump {
    pub fn from_matches(matches: &ArgMatches) -> Self {
        let (folder, path) = Self::get_path(matches);
        let target = Self::get_required_path(matches, "target");

        Self { folder, path, target }
    }

    fn get_path(matches: &ArgMatches) -> (bool, PathBuf) {
        let folder = matches.is_present("dir");
        let path;
        if folder {
            path = Self::get_required_path(matches, "dir");
        } else {
            path = Self::get_required_path(matches, "file");
        }
        (folder, path)
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
        if !self.folder {
            self.run_file();
            return;
        }
        let mut parser = BitcoinParser::default();
        let mut save = JsonWriter::new(&self.target);
        fs::read_dir(&self.path)
            .expect("Unable to read folder")
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .for_each(|entry| {
                let path = entry.path();
                let blockchain = parser.parse(&path);
                match blockchain {
                    Ok(data) => {
                        let filename = Self::get_file_save_path(&path);
                        if let Err(err) = save.save(data, filename) {
                            Self::report_error(err, path);
                        }
                    }
                    Err(err) => Self::report_error(err, path),
                }
            })
    }

    fn run_file(&self) {
        let mut parser = BitcoinParser::default();
        let path = Self::get_file_save_path(&self.path);
        let mut save = JsonWriter::new(&self.target);
        match parser.parse(&self.path) {
            Ok(blockchain) => save
                .save(blockchain, path)
                .expect("Unable to save parsed contents to disk"),
            Err(err) => Self::report_error(err, path),
        }
    }

    fn get_file_save_path<P: AsRef<Path>>(file: P) -> PathBuf {
        let filename = file.as_ref().file_stem().expect("file name must exist");
        PathBuf::from(format!(
            "{}.json",
            filename.to_str().expect("unable to convert to string")
        ))
    }

    fn report_error<E: Error + 'static, P: AsRef<Path>>(err: E, file: P) {
        println!(
            "Found error {} while parsing or saving the blockchain file {}",
            err,
            file.as_ref().display()
        );
        let mut cause = err.source();
        while let Some(source) = cause {
            println!("Previous error caused by {}", source);
            cause = source.source();
        }
    }
}
