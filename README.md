phonewords
==========

Rust program to find words that can be spelled with a phone number.


Command line help:
------------------

```
Usage:
    phonewords -h | -v
    phonewords <number> [-q]

Options:
    number        : Phone number to check (7 digits).
    -h,--help     : Show this message.
    -q,--quiet    : Only print results.
    -v,--version  : Show version and exit.

Exit status is 1 on error, 2 if no matches were found, and 0 on success.
```

Core:
-----

* rustc 1.10.0-nightly (8da2bcac5 2016-04-28)
* cargo 0.11.0-nightly (afac7fc 2016-04-27)

Dependencies:
-------------

- [clippy](https://github.com/Manishearth/rust-clippy) - Lints code at compile time.
    - Compiler plugins are unstable at the moment,
      so this means you must build using a *nightly* rust version.

Running `make` is enough to grab all of the dependencies.

Build Instructions:
-------------------

Clone this repo:
```
git clone https://github.com/welbornprod/phonewords.git
```

Build with make:
```
make
```

This will ensure the words file is symlinked in the build directory, and
build the project using `cargo`.

Running the executable:
-----------------------

Once the project is built, there are two ways to run this.
One is using `cargo`, which will build the project if it is not already built.
```
cargo run --release -- [PHONEWORDS_ARGS...]
```

Using `--release` is important, because `cargo` builds a debug version by
default and it's much slower.

You can also just run the executables that `cargo` builds, located in
`./target/release` for release builds.
```
./target/release/phonewords --help
```

The debug build will be located in `./target/debug`.
```
./target/debug/phonewords --help
```

#### Words File:

The `phonewords` executable expects to find a words file in the same
directory.
This file is included in the repo, but needs to be symlinked to the
debug/release build directories.

If you ran `make` then this was already done for you.

```
# Release builds (recommended)
ln -s "$PWD/words" "$PWD/target/release/words"
# Debug builds
ln -s "$PWD/words" "$PWD/target/debug/words"
```

Note:
-----

I wrote this to learn more about rust.
Any helpful advice about the code would be welcome.
