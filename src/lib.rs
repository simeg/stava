#[macro_use]
extern crate lazy_static;
extern crate regex;

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

    pub fn correct(&mut self, word: &str) -> String {
        // Word is known so we return it
        if self.words_w_count.contains_key(word) {
            return word.to_string();
        }

        let mut candidates: HashMap<u32, String> = HashMap::new();
        let edits = get_edits(word.to_string());

        // Add edited words as candidates
        for edit in &edits {
            if let Some(count) = self.words_w_count.get(edit) {
                candidates.insert(*count, edit.to_string());
            }
        }

        // Return candidate if found in edits
        if let Some(candidate) = candidates.iter().max_by_key(|&entry| entry.0) {
            return candidate.1.to_string();
        }

        // Add additional edits based on first edited words
        for edit in &edits {
            for word in get_edits(edit.to_string()) {
                if let Some(count) = self.words_w_count.get(&word) {
                    candidates.insert(*count, word);
                }
            }
        }

        // Return candidate if found in edits
        if let Some(candidate) = candidates.iter().max_by_key(|&entry| entry.0) {
            return candidate.1.to_string();
        }

        // No correction was found - return input word
        word.to_string()
    }
}

fn get_edits(word: String) -> HashSet<String> {
    let splits = splits(word);
    HashSet::from_iter(
        [
            deletes(splits.clone()),
            transposes(splits.clone()),
            replaces(splits.clone()),
            inserts(splits),
        ]
        .concat(),
    )
}

fn splits(word: String) -> Vec<(String, String)> {
    let mut result = Vec::with_capacity(word.len() + 1);
    let range = 0..result.capacity();
    for i in range {
        let left = String::from(&word[..i]);
        let right = String::from(&word[i..]);
        result.push((left, right));
    }
    result
}

fn deletes(words: Vec<(String, String)>) -> Vec<String> {
    let mut result = Vec::with_capacity(words.len() - 1);
    for (left, right) in words {
        if !right.is_empty() {
            result.push([left.to_owned(), right[1..].to_string()].concat());
        }
    }
    result
}

fn transposes(words: Vec<(String, String)>) -> Vec<String> {
    let mut result = Vec::with_capacity(words.len() - 2);
    for (left, right) in words {
        if right.len() > 1 {
            result.push(
                [
                    left.to_owned(),
                    right.chars().nth(1).unwrap().to_string(),
                    right.chars().nth(0).unwrap().to_string(),
                    right[2..].to_string(),
                ]
                .concat(),
            );
        }
    }
    result
}

fn replaces(words: Vec<(String, String)>) -> Vec<String> {
    let mut result = Vec::new();
    for (left, right) in words {
        if !right.is_empty() {
            for letter in ENG_ALPHABET.iter() {
                result.push(
                    [
                        left.to_owned(),
                        (*letter).to_string(),
                        right[1..].to_string(),
                    ]
                    .concat(),
                );
            }
        }
    }
    result
}

fn inserts(words: Vec<(String, String)>) -> Vec<String> {
    let mut result = Vec::new();
    for (left, right) in words {
        for letter in ENG_ALPHABET.iter() {
            result.push([left.to_owned(), (*letter).to_string(), right.to_string()].concat());
        }
    }
    result
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
        let expected = "spelling";
        assert_eq!(actual, expected);

        // insert 2
        let word = "inconvient";
        let actual = stava.correct(word);
        let expected = "inconvenient";
        assert_eq!(actual, expected);

        // replace
        let word = "bycyle";
        let actual = stava.correct(word);
        let expected = "bicycle";
        assert_eq!(actual, expected);

        // replace 2
        let word = "korrectud";
        let actual = stava.correct(word);
        let expected = "corrected";
        assert_eq!(actual, expected);

        // delete
        let word = "arrainged";
        let actual = stava.correct(word);
        let expected = "arranged";
        assert_eq!(actual, expected);

        // transpose
        let word = "peotry";
        let actual = stava.correct(word);
        let expected = "poetry";
        assert_eq!(actual, expected);

        // transpose + delete
        let word = "peotryy";
        let actual = stava.correct(word);
        let expected = "poetry";
        assert_eq!(actual, expected);

        // known word
        let word = "word";
        let actual = stava.correct(word);
        let expected = "word";
        assert_eq!(actual, expected);

        // unknown word
        let word = "quintessential";
        let actual = stava.correct(word);
        let expected = "quintessential";
        assert_eq!(actual, expected);
    }
}
