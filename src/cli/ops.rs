use anyhow::Context;
use anyhow::Result;
use clap::ArgMatches;
use std::env::current_dir;
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
        (
            folder,
            if folder {
                Self::get_required_path(matches, "dir")
            } else {
                Self::get_required_path(matches, "file")
            },
        )
    }

    fn get_required_path(matches: &ArgMatches, path: &str) -> PathBuf {
        let path_value = matches
            .value_of(path)
            .expect("Value is required in configuration present at interface.yml");
        let path = PathBuf::from(path_value);

        if !path.is_absolute() {
            let mut curr_dir = current_dir().expect("Unable to retrieve the current directory");
            curr_dir.push(path);
            curr_dir
        } else {
            path
        }
    }

    pub fn run(&self) -> Result<()> {
        if self.folder {
            let mut runner = FolderRunner::new(&self.path, &self.target);
            runner.run()
        } else {
            let mut parser = BitcoinParser::default();
            let mut writer = JsonWriter::new(&self.target);
            let mut runner = FileRunnerRef::new(&self.path, &mut parser, &mut writer);
            runner.run()
        }
    }

    fn get_file_save_path<P: AsRef<Path>>(file: P) -> PathBuf {
        let filename = file.as_ref().file_stem().expect("file name must exist");
        PathBuf::from(format!(
            "{}.json",
            filename.to_str().expect("unable to convert to string")
        ))
    }
}

struct FolderRunner<'a> {
    path: &'a Path,
    target: &'a Path,
}

impl<'a> FolderRunner<'a> {
    fn new(path: &'a Path, target: &'a Path) -> Self {
        Self { path, target }
    }

    fn run(&mut self) -> Result<()> {
        let mut parser = BitcoinParser::default();
        let mut writer = JsonWriter::new(&self.target);
        fs::read_dir(&self.path)
            .context("Unable to read the given folder")?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| {
                let path = entry.path();
                let mut runner = FileRunnerRef::new(&path, &mut parser, &mut writer);
                runner.run()
            })
            .collect::<Result<()>>()?;
        Ok(())
    }
}

struct FileRunnerRef<'b, 'a: 'b, 'c: 'b> {
    path: &'a Path,
    parser: &'b mut BitcoinParser,
    writer: &'b mut JsonWriter<'c>,
}

impl<'b, 'a, 'c> FileRunnerRef<'b, 'a, 'c> {
    fn new(path: &'a Path, parser: &'b mut BitcoinParser, writer: &'b mut JsonWriter<'c>) -> Self {
        Self { path, parser, writer }
    }

    fn run(&mut self) -> Result<()> {
        let path = Dump::get_file_save_path(&self.path);
        let blockchain = self
            .parser
            .parse(&self.path)
            .context("Unable to parse the blk file contents")?;
        self.writer
            .save(blockchain, path)
            .context("Unable to save parsed contents")?;
        Ok(())
    }
}
