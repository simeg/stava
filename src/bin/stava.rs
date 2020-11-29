extern crate stava;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate include_dir;

use clap::{App, Arg};
use include_dir::Dir;
use stava::{Stava, StavaResult};

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;
use std::process::exit;

const OPT_NAME_WORD: &str = "WORD";
const OPT_NAME_FILES: &str = "FILES";
const FLAG_INC_DEFAULT_WORDS: &str = "flag_inc_default_words";
const FLAG_RETURN_EXIT_CODE: &str = "flag_return_exit_code";
const FLAG_ONLY_EXIT_CODE: &str = "flag_only_exit_code";

const ASSETS_DIR: Dir = include_dir!("src/assets");

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
        .index(2);

    let flag_inc_default_words = Arg::with_name(FLAG_INC_DEFAULT_WORDS)
        .help("Include default set of words (default: false)")
        .short("d")
        .long("default");

    let flag_return_exit_code = Arg::with_name(FLAG_RETURN_EXIT_CODE)
        .help("Exit with 1 if word is not spelled correctly, otherwise 0 (default: false)")
        .short("e")
        .long("exit-code");

    let flag_only_exit_code = Arg::with_name(FLAG_ONLY_EXIT_CODE)
        .help("Only return exit code and not corrected word (default: false)")
        .short("o")
        .long("exit-code-only");

    let matches = App::new("stava")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(opt_word)
        .arg(opt_files)
        .arg(flag_inc_default_words)
        .arg(flag_return_exit_code)
        .arg(flag_only_exit_code)
        .get_matches();

    let mut stava = Stava {
        words_w_count: HashMap::new(),
    };

    if let Some(files) = matches.values_of(OPT_NAME_FILES) {
        if matches.is_present(FLAG_INC_DEFAULT_WORDS) {
            stava.learn(get_default_words());
        }

        let paths: Vec<&Path> = files.map(Path::new).collect::<Vec<&Path>>();

        for file in paths {
            let words = fs::read_to_string(file)
                .unwrap_or_else(|_| panic!("Could not read the file: {}", file.display()));
            stava.learn(words.as_str());
        }
    } else {
        // No files provided by user - use default word file
        stava.learn(get_default_words());
    }

    let word = matches.value_of(OPT_NAME_WORD).unwrap();
    let result = stava.correct(word);

    if matches.is_present(FLAG_ONLY_EXIT_CODE) {
        exit_with_code(result)
    } else {
        println!("{}", result.word);

        if matches.is_present(FLAG_RETURN_EXIT_CODE) {
            exit_with_code(result)
        }
    }
}

fn exit_with_code(result: StavaResult) -> ! {
    if result.was_corrected {
        exit(1)
    }

    exit(0)
}

fn get_default_words() -> &'static str {
    ASSETS_DIR
        .get_file("words.txt")
        .and_then(|file| file.contents_utf8())
        .unwrap_or_else(|| panic!("Could not get default words"))
}

fn exists_on_filesystem(path: &OsStr) -> Result<(), OsString> {
    match Some(path).map(Path::new).map(Path::exists).unwrap_or(false) {
        true => Ok(()),
        false => Err(OsString::from(format!("File not found [{:?}]", path))),
    }
}
