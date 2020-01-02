# stava [![Crate Status](https://img.shields.io/crates/v/stava.svg)](https://crates.io/crates/stava)  [![Build Status](https://travis-ci.com/simeg/stava.svg?branch=master)](https://travis-ci.com/simeg/stava)

CLI tool to perform spell checking.

Rust implementation of [Peter Norvig's Spell Corrector](http://norvig.com/spell-correct.html).

```bash
USAGE:
    stava <WORD> [FILES]...

ARGS:
    <WORD>        Word to correct
    <FILES>...    Files to learn words from [default: src/assets/words.txt]
```

* The default file contains ~30k unique words and is included in the crate
* If needed you can pass in one or many of your own files
* The files doesn't require any special formatting and special characters are allowed, `stava`
knows how to ignore them

Currently `stava` only supports the English alphabet.

## Installation
```bash
$ cargo install stava
```

## Usage
```bash
$ stava bycyle
bicycle
```

* If multiple candidates are found, the one occurring the most in the provided files are returned
* If no candidate is found the input word is returned
