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

Dependencies:
-------------

- [clippy](https://github.com/Manishearth/rust-clippy) - Lints code at compile time.
    - Compiler plugins are unstable at the moment,
      so this means you must build using a *nightly* rust version.

Note:
-----

I wrote this to learn more about rust.
Any helpful advice about the code would be welcome.

