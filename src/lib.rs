#[macro_use]
extern crate lazy_static;
extern crate regex;
#[cfg(test)]
#[macro_use]
extern crate include_dir;

use regex::Regex;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

lazy_static! {
    static ref ENG_ALPHABET: Vec<&'static str> = "abcdefghijklmnopqrstuvwxyz"
        .split("")
        .filter(|l| !l.is_empty())
        .collect();
}

pub struct Stava {
    // The words from the input with the frequency count for each word
    pub words_w_count: HashMap<String, u32>,
}

#[derive(Debug, PartialEq)]
pub struct StavaResult {
    // The corrected word. If it was not corrected then the input word is returned
    pub word: String,
    // If the word was corrected
    pub was_corrected: bool,
}

impl Stava {
    pub fn learn(&mut self, text: &str) {
        let re = Regex::new(r"[a-z]+").unwrap();
        for m in re.find_iter(&text.to_lowercase()) {
            let count = self
                .words_w_count
                .entry(m.as_str().to_string())
                .or_insert(0);
            *count += 1;
        }
    }

    pub fn correct(&mut self, word: &str) -> StavaResult {
        // Word is known so we return it
        if self.words_w_count.contains_key(word) {
            return StavaResult {
                word: word.to_string(),
                was_corrected: false,
            };
        }

        let mut candidates: HashMap<u32, String> = HashMap::new();
        let edits = self.get_edits(word);

        // Add edited words as candidates
        for edit in &edits {
            if let Some(count) = self.words_w_count.get(edit) {
                candidates.insert(*count, edit.to_string());
            }
        }

        // Return candidate if found in edits
        if let Some(candidate) = candidates.iter().max_by_key(|&entry| entry.0) {
            return StavaResult {
                word: candidate.1.to_string(),
                was_corrected: true,
            };
        }

        // Add additional edits based on first edited words
        for edit in &edits {
            for word in self.get_edits(edit) {
                if let Some(count) = self.words_w_count.get(&word) {
                    candidates.insert(*count, word);
                }
            }
        }

        // Return candidate if found in edits
        if let Some(candidate) = candidates.iter().max_by_key(|&entry| entry.0) {
            return StavaResult {
                word: candidate.1.to_string(),
                was_corrected: true,
            };
        }

        // No correction was found
        StavaResult {
            word: word.to_string(),
            was_corrected: false,
        }
    }

    fn get_edits(&self, word: &str) -> HashSet<String> {
        let splits = self.splits(word);
        HashSet::from_iter(
            [
                self.deletes(&splits),
                self.transposes(&splits),
                self.replaces(&splits),
                self.inserts(&splits),
            ]
            .concat(),
        )
    }

    fn splits<'a>(&self, word: &'a str) -> Vec<(&'a str, &'a str)> {
        let mut result: Vec<(&str, &str)> = Vec::with_capacity(word.len() + 1);
        let range = 0..result.capacity();
        for i in range {
            let left = &word[..i];
            let right = &word[i..];
            result.push((left, right));
        }
        result
    }

    fn deletes(&self, words: &[(&str, &str)]) -> Vec<String> {
        let mut result = Vec::with_capacity(words.len() - 1);
        for (left, right) in words {
            if !right.is_empty() {
                result.push([left, &right[1..]].concat());
            }
        }
        result
    }

    fn transposes(&self, words: &[(&str, &str)]) -> Vec<String> {
        let mut result = Vec::with_capacity(words.len() - 2);
        for (left, right) in words {
            if right.len() > 1 {
                result.push(
                    [
                        left.to_owned(),
                        right.chars().nth(1).unwrap().to_string().as_str(),
                        right.chars().next().unwrap().to_string().as_str(),
                        &right[2..],
                    ]
                    .concat(),
                );
            }
        }
        result
    }

    fn replaces(&self, words: &[(&str, &str)]) -> Vec<String> {
        let mut result = Vec::new();
        for (left, right) in words {
            if !right.is_empty() {
                for letter in ENG_ALPHABET.iter() {
                    result.push([left.to_owned(), (*letter), &right[1..]].concat());
                }
            }
        }
        result
    }

    fn inserts(&self, words: &[(&str, &str)]) -> Vec<String> {
        let mut result = Vec::new();
        for (left, right) in words {
            for letter in ENG_ALPHABET.iter() {
                result.push([left, (*letter), right].concat());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splits() {
        let stava = Stava {
            words_w_count: HashMap::new(),
        };
        let actual = stava.splits("monkey");
        let expected = vec![
            ("", "monkey"),
            ("m", "onkey"),
            ("mo", "nkey"),
            ("mon", "key"),
            ("monk", "ey"),
            ("monke", "y"),
            ("monkey", ""),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deletes() {
        let stava = Stava {
            words_w_count: HashMap::new(),
        };
        let splits = vec![
            ("", "monkey"),
            ("m", "onkey"),
            ("mo", "nkey"),
            ("mon", "key"),
            ("monk", "ey"),
            ("monke", "y"),
            ("monkey", ""),
        ];
        let actual = stava.deletes(&*splits);
        let expected = vec!["onkey", "mnkey", "mokey", "money", "monky", "monke"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_transposes() {
        let stava = Stava {
            words_w_count: HashMap::new(),
        };
        let splits = vec![
            ("", "monkey"),
            ("m", "onkey"),
            ("mo", "nkey"),
            ("mon", "key"),
            ("monk", "ey"),
            ("monke", "y"),
            ("monkey", ""),
        ];
        let actual = stava.transposes(&*splits);
        let expected = vec!["omnkey", "mnokey", "mokney", "moneky", "monkye"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_replaces() {
        let stava = Stava {
            words_w_count: HashMap::new(),
        };
        let splits = vec![
            ("", "monkey"),
            ("m", "onkey"),
            ("mo", "nkey"),
            ("mon", "key"),
            ("monk", "ey"),
            ("monke", "y"),
            ("monkey", ""),
        ];
        let actual = stava.replaces(&*splits);
        let expected = vec![
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
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_inserts() {
        let stava = Stava {
            words_w_count: HashMap::new(),
        };
        let splits = vec![
            ("", "monkey"),
            ("m", "onkey"),
            ("mo", "nkey"),
            ("mon", "key"),
            ("monk", "ey"),
            ("monke", "y"),
            ("monkey", ""),
        ];
        let actual = stava.inserts(&*splits);
        let expected = vec![
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
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_learn_word_freq() {
        let mut stava = Stava {
            words_w_count: HashMap::new(),
        };

        stava.learn("spelling spelling spelling bicycle");

        let actual = stava.words_w_count;
        let mut expected: HashMap<String, u32> = HashMap::new();
        expected.insert("spelling".to_string(), 3);
        expected.insert("bicycle".to_string(), 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_learn_text_parse() {
        let mut stava = Stava {
            words_w_count: HashMap::new(),
        };

        stava.learn("(spelling) spelling, spelling. spelling! spelling22 [spelling]-^spelling#");

        let actual = stava.words_w_count;
        let mut expected: HashMap<String, u32> = HashMap::new();
        expected.insert("spelling".to_string(), 7);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_correct() {
        let mut stava = Stava {
            words_w_count: HashMap::new(),
        };

        stava.learn("spelling inconvenient bicycle corrected arranged poetry word");

        // insert
        let word = "speling";
        let actual = stava.correct(word);
        let expected = StavaResult {
            word: "spelling".to_string(),
            was_corrected: true,
        };
        assert_eq!(actual, expected);

        // insert 2
        let word = "inconvient";
        let actual = stava.correct(word);
        let expected = StavaResult {
            word: "inconvenient".to_string(),
            was_corrected: true,
        };
        assert_eq!(actual, expected);

        // replace
        let word = "bycyle";
        let actual = stava.correct(word);
        let expected = StavaResult {
            word: "bicycle".to_string(),
            was_corrected: true,
        };
        assert_eq!(actual, expected);

        // replace 2
        let word = "korrectud";
        let actual = stava.correct(word);
        let expected = StavaResult {
            word: "corrected".to_string(),
            was_corrected: true,
        };
        assert_eq!(actual, expected);

        // delete
        let word = "arrainged";
        let actual = stava.correct(word);
        let expected = StavaResult {
            word: "arranged".to_string(),
            was_corrected: true,
        };
        assert_eq!(actual, expected);

        // transpose
        let word = "peotry";
        let actual = stava.correct(word);
        let expected = StavaResult {
            word: "poetry".to_string(),
            was_corrected: true,
        };
        assert_eq!(actual, expected);

        // transpose + delete
        let word = "peotryy";
        let actual = stava.correct(word);
        let expected = StavaResult {
            word: "poetry".to_string(),
            was_corrected: true,
        };
        assert_eq!(actual, expected);

        // known word
        let word = "word";
        let actual = stava.correct(word);
        let expected = StavaResult {
            word: "word".to_string(),
            was_corrected: false,
        };
        assert_eq!(actual, expected);

        // unknown word
        let word = "quintessential";
        let actual = stava.correct(word);
        let expected = StavaResult {
            word: "quintessential".to_string(),
            was_corrected: false,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_default_word_file_is_available() {
        use include_dir::Dir;

        let assets_dir: Dir = include_dir!("src/assets");
        let file_len = assets_dir
            .get_file("words.txt")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .len();
        assert!(assets_dir.contains("words.txt"));
        assert_eq!(file_len, 6479566);
    }
}
