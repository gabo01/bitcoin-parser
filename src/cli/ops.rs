use clap::ArgMatches;
use serde_json as json;
use std::env::current_dir;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use btlib::blkparser::BitcoinParser;
use btlib::blkparser::SerialBlock;
use btlib::blockchain::BlockChain;
use btlib::parser::Parser;

pub const BLK_BUFFER: usize = 400 * 1024 * 1024;

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
        let parser = BitcoinParser::new(&self.file);
        let blockchain = parser.parse();
        match blockchain {
            Ok(data) => self.write(data),
            Err(err) => {
                println!("The following error was raised during the parsing: {}", err);
                let mut cause = err.source();
                while let Some(source) = cause {
                    println!("Error source: {}", source);
                    cause = source.source();
                }
            }
        }
    }

    fn write(&self, blockchain: BlockChain<SerialBlock>) {
        let mut buffer = Vec::with_capacity(BLK_BUFFER);
        json::to_writer(&mut buffer, &blockchain).expect("Write to in memory buffer cannot fail");
        self.save(&buffer);
    }

    fn save(&self, data: &[u8]) {
        let filename = self.file.file_stem().expect("file name must exist");
        let mut filepath = PathBuf::from(&self.target);
        filepath.push(format!(
            "{}.json",
            filename.to_str().expect("unable to convert to string")
        ));
        File::create(filepath)
            .map(|mut file| file.write(data))
            .expect("unable to create file")
            .expect("unable to write into file");
    }
}
