<div align="center">

# Nedry 🦖 - A bundler for C

![movie clip](./images/nedry.webp)

> You know anybody who can network 8 connection machine and debug **2 MILLION** lines of code.

[![Crates.io License](https://img.shields.io/crates/l/nedry)](https://github.com/KAIYOHUGO/Nedry/blob/master/LICENSE) [![Crates.io Version](https://img.shields.io/crates/v/nedry)](https://crates.io/crates/nedry) ![GitHub top language](https://img.shields.io/github/languages/top/kaiyohugo/nedry)

</div>

## What is this

This is mainly use for competitive programming & CS class.
Most online judge system used in collage only accept C without dependency.
This tool will auto replace `#include` with actual code.

Support List

- `#include "file_here"`
- `#include "file_here.h"` will also try to include `file_here.c`
- Duplicate include will be ignore

Know Limitation

- Cannot resolve `#include` correctly in some nest macro (`#if`/`#ifdef`)

## Usage

To bundle `test.c` to `test.bundle.c` run 

```bash
nedry -i test.c bundle -o test.bundle.c
```


Command list

```bash
$ nedry
Usage: nedry --input <INPUT> <COMMAND>

Commands:
  bundle  Bundle all dependency to single file
  build   Build file to an executable file
  run     Build & Run file
  help    Print this message or the help of the given subcommand(s)

Options:
  -i, --input <INPUT>  The entry point file
  -h, --help           Print help (see more with '--help')
```
