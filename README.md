# stava [![Crate Status](https://img.shields.io/crates/v/stava.svg)](https://crates.io/crates/stava)  [![Build Status](https://travis-ci.com/simeg/stava.svg?branch=master)](https://travis-ci.com/simeg/stava)

CLI tool to perform spell checking.

Rust implementation of [Peter Norvig's Spell Checker](http://norvig.com/spell-correct.html).

```bash
USAGE:
    stava <WORD> [FILES]...

ARGS:
    <WORD>        Word to correct
    <FILES>...    Files to learn words from [default: src/assets/words.txt]
```

The default file contains ~30k unique words. If needed you can pass in one or many of your own files.
