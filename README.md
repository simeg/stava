# stava [![Crate Status](https://img.shields.io/crates/v/stava.svg)](https://crates.io/crates/stava)  [![Build Status](https://travis-ci.com/simeg/stava.svg?branch=master)](https://travis-ci.com/simeg/stava)

CLI tool to perform spell checking.

Rust implementation of [Peter Norvig's Spell Corrector](http://norvig.com/spell-correct.html).

```bash
USAGE:
    stava <WORD> [FILES]...

ARGS:
    <WORD>        Word to correct
    <FILES>...    Files to learn words from
```

* The default file contains ~30k unique words and is included in the crate
* If needed you can pass in one or many of your own files
* The files doesn't require any certain formatting (except whitespace separated words) and special
characters are allowed, `stava` knows how to ignore them

Currently `stava` only supports the English alphabet.

## Installation
```bash
$ cargo install stava
```

## Usage
**Use the default word file**
```bash
$ stava bycycle
bicycle
```

**Use your own files**
```bash
$ echo "bicycle" > words.txt
$ echo "some other words" > words2.txt
$ stava bycycle words.txt words2.txt
bicycle
```

**Use your own files and the default set of words**
```bash
$ echo "bicycle" > words.txt
$ echo "some other words" > words2.txt
$ stava --default mankey words.txt words2.txt
monkey
```

**With exit code**
```bash
$ stava --exit-code bycycle  # Word is corrected so exit code = 1
bicycle
$ echo $?
1
```

```bash
$ stava --exit-code-only bycycle  # Word is corrected so exit code = 1
$ echo $?
1
```

* If multiple candidates are found, the one occurring the most in the provided files are returned
* If no candidate is found the input word is returned
