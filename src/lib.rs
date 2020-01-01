#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::iter::FromIterator;
use std::path::Path;

static WORDS_PATH: &str = "src/assets/words.txt";

static OPT_NAME_FILES: &str = "FILES";

lazy_static! {
    static ref LETTERS: Vec<String> = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ]
    .into_iter()
    .map(String::from)
    .collect();
}

pub fn run() {
    let opt_files = Arg::with_name(OPT_NAME_FILES)
        .help("Files to analyze")
        .required(true)
        .multiple(true)
        .validator_os(exists_on_filesystem)
        .index(1);

    let matches = App::new("stava")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(opt_files)
        .get_matches();

    if let Some(files) = matches.values_of(OPT_NAME_FILES) {
        let paths: Vec<&Path> = files.map(|file| Path::new(file)).collect::<Vec<&Path>>();
        println!("Got: {:?}", paths);
    }
}

fn get_dictionary() -> String {
    fs::read_to_string(WORDS_PATH)
        .expect("Something went wrong reading the file")
        .to_lowercase()
        .replace("\\-", " ")
        .replace("\n", " ")
        .replace("\t", " ")
}

fn strip_special_chars(string: &mut String) {
    string.retain(|c| !r#"!\"\"\\\/(),".;:"[]*#?_+="#.contains(c));
}

fn exists_on_filesystem(path: &OsStr) -> Result<(), OsString> {
    match path.to_str() {
        None => Err(OsString::from("Could not convert input file path -> str")),
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

fn splits(word: String) -> Vec<(String, String)> {
    let mut result = vec![];
    let range = 0..(word.len() + 1);
    for i in range {
        let left = String::from(&word[..i]);
        let right = String::from(&word[i..]);
        let tuple = (left, right);
        result.push(tuple);
    }
    result
}

fn deletes(words: Vec<(String, String)>) -> Vec<String> {
    let mut result = vec![];
    words.iter().for_each(|word| {
        let (left, right) = word;
        if !right.is_empty() {
            result.push(String::from(left) + &right[1..]);
        }
    });
    result
}

fn transposes(words: Vec<(String, String)>) -> Vec<String> {
    let mut result = vec![];
    words.iter().for_each(|word| {
        let (left, right) = word;
        if right.len() > 1 {
            let right1 = right.chars().nth(1).unwrap();
            let right2 = right.chars().nth(0).unwrap();
            let right3 = &right[2..];
            result.push(
                String::from(left)
                    + right1.to_string().as_str()
                    + right2.to_string().as_str()
                    + right3,
            );
        }
    });
    result
}

fn replaces(words: Vec<(String, String)>) -> Vec<String> {
    let mut result = vec![];
    words.iter().for_each(|word| {
        let (left, right) = word;
        if !right.is_empty() {
            LETTERS.iter().for_each(|letter| {
                let right1 = &right[1..];
                result.push(String::from(left) + letter + right1);
            })
        }
    });
    result
}

fn inserts(words: Vec<(String, String)>) -> Vec<String> {
    let mut result = vec![];
    words.iter().for_each(|word| {
        let (left, right) = word;
        LETTERS.iter().for_each(|letter| {
            result.push(String::from(left) + letter + right);
        })
    });
    result
}

fn edits1(word: String) -> HashSet<String> {
    let splits = splits(word);
    let deletes = deletes(splits.clone());
    let transposes = transposes(splits.clone());
    let replaces = replaces(splits.clone());
    let inserts = inserts(splits.clone());
    let all_words: Vec<String> = [deletes, transposes, replaces, inserts].concat();

    HashSet::from_iter(all_words.iter().cloned())
}

fn edits2(word: String) -> HashSet<String> {
    let mut result = HashSet::new();
    let e1 = edits1(word);
    e1.iter().for_each(|word| {
        result.extend(edits1(String::from(word)));
    });

    result
}

fn find_corrected_word(word: String) -> String {
    let mut dict_str = get_dictionary();

    strip_special_chars(&mut dict_str);

    let dictionary: HashSet<&str> = dict_str.split_whitespace().collect();
    let known_word = dictionary.get(word.as_str());

    match known_word {
        Some(word) => word.to_string(),
        None => {
            let e1_set = edits1(word.clone());
            let e1_word = e1_set
                .iter()
                .find(|word| dictionary.get(word.as_str()).is_some());

            match e1_word {
                None => edits2(word.clone())
                    .iter()
                    .find(|word| dictionary.get(word.as_str()).is_some())
                    .unwrap_or(&word.clone())
                    .to_string(),
                Some(word) => word.to_owned(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splits() {
        let actual = splits(String::from("monkey"));
        let expected: Vec<(String, String)> = vec![
            (String::from(""), String::from("monkey")),
            (String::from("m"), String::from("onkey")),
            (String::from("mo"), String::from("nkey")),
            (String::from("mon"), String::from("key")),
            (String::from("monk"), String::from("ey")),
            (String::from("monke"), String::from("y")),
            (String::from("monkey"), String::from("")),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deletes() {
        let splits: Vec<(String, String)> = vec![
            (String::from(""), String::from("monkey")),
            (String::from("m"), String::from("onkey")),
            (String::from("mo"), String::from("nkey")),
            (String::from("mon"), String::from("key")),
            (String::from("monk"), String::from("ey")),
            (String::from("monke"), String::from("y")),
            (String::from("monkey"), String::from("")),
        ];
        let actual = deletes(splits);
        let expected: Vec<String> = vec![
            String::from("onkey"),
            String::from("mnkey"),
            String::from("mokey"),
            String::from("money"),
            String::from("monky"),
            String::from("monke"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_transposes() {
        let splits: Vec<(String, String)> = vec![
            (String::from(""), String::from("monkey")),
            (String::from("m"), String::from("onkey")),
            (String::from("mo"), String::from("nkey")),
            (String::from("mon"), String::from("key")),
            (String::from("monk"), String::from("ey")),
            (String::from("monke"), String::from("y")),
            (String::from("monkey"), String::from("")),
        ];
        let actual = transposes(splits);
        let expected: Vec<String> = vec!["omnkey", "mnokey", "mokney", "moneky", "monkye"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_replaces() {
        let splits: Vec<(String, String)> = vec![
            (String::from(""), String::from("monkey")),
            (String::from("m"), String::from("onkey")),
            (String::from("mo"), String::from("nkey")),
            (String::from("mon"), String::from("key")),
            (String::from("monk"), String::from("ey")),
            (String::from("monke"), String::from("y")),
            (String::from("monkey"), String::from("")),
        ];
        let actual = replaces(splits);
        let expected: Vec<String> = vec![
            "aonkey", "bonkey", "conkey", "donkey", "eonkey", "fonkey", "gonkey", "honkey",
            "ionkey", "jonkey", "konkey", "lonkey", "monkey", "nonkey", "oonkey", "ponkey",
            "qonkey", "ronkey", "sonkey", "tonkey", "uonkey", "vonkey", "wonkey", "xonkey",
            "yonkey", "zonkey", "mankey", "mbnkey", "mcnkey", "mdnkey", "menkey", "mfnkey",
            "mgnkey", "mhnkey", "minkey", "mjnkey", "mknkey", "mlnkey", "mmnkey", "mnnkey",
            "monkey", "mpnkey", "mqnkey", "mrnkey", "msnkey", "mtnkey", "munkey", "mvnkey",
            "mwnkey", "mxnkey", "mynkey", "mznkey", "moakey", "mobkey", "mockey", "modkey",
            "moekey", "mofkey", "mogkey", "mohkey", "moikey", "mojkey", "mokkey", "molkey",
            "momkey", "monkey", "mookey", "mopkey", "moqkey", "morkey", "moskey", "motkey",
            "moukey", "movkey", "mowkey", "moxkey", "moykey", "mozkey", "monaey", "monbey",
            "moncey", "mondey", "moneey", "monfey", "mongey", "monhey", "moniey", "monjey",
            "monkey", "monley", "monmey", "monney", "monoey", "monpey", "monqey", "monrey",
            "monsey", "montey", "monuey", "monvey", "monwey", "monxey", "monyey", "monzey",
            "monkay", "monkby", "monkcy", "monkdy", "monkey", "monkfy", "monkgy", "monkhy",
            "monkiy", "monkjy", "monkky", "monkly", "monkmy", "monkny", "monkoy", "monkpy",
            "monkqy", "monkry", "monksy", "monkty", "monkuy", "monkvy", "monkwy", "monkxy",
            "monkyy", "monkzy", "monkea", "monkeb", "monkec", "monked", "monkee", "monkef",
            "monkeg", "monkeh", "monkei", "monkej", "monkek", "monkel", "monkem", "monken",
            "monkeo", "monkep", "monkeq", "monker", "monkes", "monket", "monkeu", "monkev",
            "monkew", "monkex", "monkey", "monkez",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_inserts() {
        let splits: Vec<(String, String)> = vec![
            (String::from(""), String::from("monkey")),
            (String::from("m"), String::from("onkey")),
            (String::from("mo"), String::from("nkey")),
            (String::from("mon"), String::from("key")),
            (String::from("monk"), String::from("ey")),
            (String::from("monke"), String::from("y")),
            (String::from("monkey"), String::from("")),
        ];
        let actual = inserts(splits);
        let expected: Vec<String> = vec![
            "amonkey", "bmonkey", "cmonkey", "dmonkey", "emonkey", "fmonkey", "gmonkey", "hmonkey",
            "imonkey", "jmonkey", "kmonkey", "lmonkey", "mmonkey", "nmonkey", "omonkey", "pmonkey",
            "qmonkey", "rmonkey", "smonkey", "tmonkey", "umonkey", "vmonkey", "wmonkey", "xmonkey",
            "ymonkey", "zmonkey", "maonkey", "mbonkey", "mconkey", "mdonkey", "meonkey", "mfonkey",
            "mgonkey", "mhonkey", "mionkey", "mjonkey", "mkonkey", "mlonkey", "mmonkey", "mnonkey",
            "moonkey", "mponkey", "mqonkey", "mronkey", "msonkey", "mtonkey", "muonkey", "mvonkey",
            "mwonkey", "mxonkey", "myonkey", "mzonkey", "moankey", "mobnkey", "mocnkey", "modnkey",
            "moenkey", "mofnkey", "mognkey", "mohnkey", "moinkey", "mojnkey", "moknkey", "molnkey",
            "momnkey", "monnkey", "moonkey", "mopnkey", "moqnkey", "mornkey", "mosnkey", "motnkey",
            "mounkey", "movnkey", "mownkey", "moxnkey", "moynkey", "moznkey", "monakey", "monbkey",
            "monckey", "mondkey", "monekey", "monfkey", "mongkey", "monhkey", "monikey", "monjkey",
            "monkkey", "monlkey", "monmkey", "monnkey", "monokey", "monpkey", "monqkey", "monrkey",
            "monskey", "montkey", "monukey", "monvkey", "monwkey", "monxkey", "monykey", "monzkey",
            "monkaey", "monkbey", "monkcey", "monkdey", "monkeey", "monkfey", "monkgey", "monkhey",
            "monkiey", "monkjey", "monkkey", "monkley", "monkmey", "monkney", "monkoey", "monkpey",
            "monkqey", "monkrey", "monksey", "monktey", "monkuey", "monkvey", "monkwey", "monkxey",
            "monkyey", "monkzey", "monkeay", "monkeby", "monkecy", "monkedy", "monkeey", "monkefy",
            "monkegy", "monkehy", "monkeiy", "monkejy", "monkeky", "monkely", "monkemy", "monkeny",
            "monkeoy", "monkepy", "monkeqy", "monkery", "monkesy", "monkety", "monkeuy", "monkevy",
            "monkewy", "monkexy", "monkeyy", "monkezy", "monkeya", "monkeyb", "monkeyc", "monkeyd",
            "monkeye", "monkeyf", "monkeyg", "monkeyh", "monkeyi", "monkeyj", "monkeyk", "monkeyl",
            "monkeym", "monkeyn", "monkeyo", "monkeyp", "monkeyq", "monkeyr", "monkeys", "monkeyt",
            "monkeyu", "monkeyv", "monkeyw", "monkeyx", "monkeyy", "monkeyz",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_corrected_word() {
        //        let word = String::from("inconvient");
        //        let actual = find_corrected_word(word);
        //        let expected = String::from("inconvenient");
        //        assert_eq!(actual, expected);

        let word = String::from("arrainged");
        let actual = find_corrected_word(word);
        let expected = String::from("arranged");
        assert_eq!(actual, expected);

        let word = String::from("speling");
        let actual = find_corrected_word(word);
        let expected = String::from("spelling");
        assert_eq!(actual, expected);

        let word = String::from("korrectud");
        let actual = find_corrected_word(word);
        let expected = String::from("corrected");
        assert_eq!(actual, expected);

        let word = String::from("peotry");
        let actual = find_corrected_word(word);
        let expected = String::from("poetry");
        assert_eq!(actual, expected);

        let word = String::from("peotryy");
        let actual = find_corrected_word(word);
        let expected = String::from("poetry");
        assert_eq!(actual, expected);

        let word = String::from("word");
        let actual = find_corrected_word(word);
        let expected = String::from("word");
        assert_eq!(actual, expected);

        let word = String::from("quintessential");
        let actual = find_corrected_word(word);
        let expected = String::from("quintessential");
        assert_eq!(actual, expected);
    }
}
