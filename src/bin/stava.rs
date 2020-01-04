#[macro_use]
extern crate clap;
extern crate stava;

use clap::{App, Arg};
use stava::Stava;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;

static OPT_NAME_WORD: &str = "WORD";
static OPT_NAME_FILES: &str = "FILES";

static DEFAULT_WORDS_PATH: &str = "src/assets/words.txt";

fn main() {
    let opt_word = Arg::with_name(OPT_NAME_WORD)
        .help("Word to correct")
        .required(true)
        .index(1);

    let opt_files = Arg::with_name(OPT_NAME_FILES)
        .help("Files to learn words from")
        .multiple(true)
        .required(false)
        .validator_os(exists_on_filesystem)
        .default_value(DEFAULT_WORDS_PATH)
        .index(2);

    let matches = App::new("stava")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(opt_word)
        .arg(opt_files)
        .get_matches();

    if let Some(files) = matches.values_of(OPT_NAME_FILES) {
        let paths: Vec<&Path> = files.map(Path::new).collect::<Vec<&Path>>();

        let mut stava = Stava {
            words_w_count: HashMap::new(),
        };

        for file in paths {
            let words = fs::read_to_string(file).unwrap_or_else(|_| {
                panic!("Something went wrong reading the file: {}", file.display())
            });
            stava.learn(words.as_str());
        }

        let word = matches.value_of(OPT_NAME_WORD).unwrap();
        let corrected_word = stava.correct(word);

        println!("{}", corrected_word);
    }
}

fn exists_on_filesystem(path: &OsStr) -> Result<(), OsString> {
    match path.to_str() {
        None => Err(OsString::from("Could not convert input file path -> &str")),
        Some(p) => {
            if Path::new(p).exists() {
                return Ok(());
            }
            Err(OsString::from(format!(
                "File not found [{}]",
                path.to_str().unwrap()
            )))
        }
    }
}
