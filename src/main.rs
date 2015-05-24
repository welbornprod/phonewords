#![feature(exit_status)]

/// Finds possible words that can be made from a phone number.
/// Generates all possible letter combos, and searches the `words` file
/// for combos that contain words.
/// -Christopher Welborn 5-19-15

/// Copyright (C) 2015 Christopher Welborn
///
/// This program is free software; you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation; either version 2 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License along
/// with this program; if not, write to the Free Software Foundation, Inc.,
/// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

#[macro_use(iproduct)]
extern crate itertools;

use std::collections::{HashMap};
use std::{env, error, fmt};
use std::fs::{read_link, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

const VERSION: &'static str = "PhoneWords v. 0.0.7-3";
const HELP: &'static str = "Usage:
        phonewords -h | -v
        phonewords <number> [-q]
    Options:
        number        : Phone number to check (7 digits).
        -h,--help     : Show this message.
        -q,--quiet    : Only print results.
        -v,--version  : Show version and exit.
    Exit status is 1 on error, 2 if no matches were found, and 0 on success.
    ";

fn main() {
    let args: Vec<String> = env::args().collect();

    let arglen = args.len();
    let mut quiet = false;

    if arglen < 2 {
        print_usage("No arguments!");
        return;
    } else if arglen == 3 {
        if args[2] == "-q" || args[2] == "--quiet" {
            quiet = true;
        } else {
            print_usage("Expecting -q as the second argument.");
            return;
        }
    } else if arglen > 3 {
        print_usage("Invalid number of arguments.");
        return;
    }

    if args[1] == "-h" || args[1] == "--help" {
        println!("{}\n{}", VERSION, HELP);
        return;
    } else if args[1] == "-v" || args[1] == "--version" {
        println!("{}", VERSION);
        return;
    }

    let wordpath = match get_wordfile_path() {
        Err(e) => {
            fail_msg(&format!("Unable to get word file path: {}", e));
            return;
        },
        Ok(p) => p
    };

    match check_number(&args[1], &wordpath, quiet) {
        Err(err) => {
            fail_msg(&err.to_string());
            return;
        },
        Ok(()) => {},
    }
}

/// Creates and initializes a new hashmap.
macro_rules! hashmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

/// Check a number for matches, print optional status and matches as they
/// are found.
fn check_number(number: &str, wordfile: &Path, quiet: bool) -> Result<(), Error> {

    // Optional status printer.
    let status = |msg: &String| {
        if !quiet {
            println!("{}", msg);
        }
    };

    // Ensure that the number is exactly 7 digits,
    // ..truncate or pad with 0's if needed.
    let mut usenumber = format!("{:0>7}", number);
    // format! returns character in the ascii range so every byte is a valid char boundary.
    usenumber.truncate(7);

    status(&format!("\n Checking: {}", usenumber));

    // Generate all possible letter combinations for this number.
    let combos = try!(get_combos(&usenumber));
    status(&format!("   Combos: {}", combos.len()));

    // Load word file for iteration, save filename for display purposes.
    let wordreader = BufReader::new(
        try!(File::open(wordfile).map_err(|err| Error::Io(Some(wordfile.into()), err))));
    status(&format!("Word File: {}\n", wordfile.display()));

    // In the future, variable-length numbers may be used.
    // It would be wasteful to check all combos for a word that is longer.
    let numberlen = usenumber.len();
    // Keep track of the actual attempts/counts.
    let mut matchcnt = 0u32;
    let mut wordcnt = 0usize;
    let mut trycnt = 0usize;

    for tryword in wordreader.lines() {
        let word = try!(tryword);
        if word.len() > numberlen {
            // A combo will never contain a word that is longer than itself.
            continue;
        }

        wordcnt += 1;
        for c in combos.iter() {
            trycnt += 1;
            if c.contains(&word) {
                matchcnt += 1;
                println!("{}: {}", c, word);
                break;
            }
        }
    }

    // Format final status messages.
    let pluralmatches = if matchcnt == 1 {"match"} else {"matches"};
    let pluralwords = if wordcnt == 1  {"word"} else {"words"};

    status(&format!(
        "\nFound {mcnt} {mplural} against {wcnt} {wplural} in {tcnt} tries.",
        mcnt=matchcnt,
        mplural=pluralmatches,
        wcnt=wordcnt,
        wplural=pluralwords,
        tcnt=trycnt
    ));

    // Exit status 2 if no matches were found (otherwise successful).
    env::set_exit_status(if matchcnt == 0 {2} else {0});
    Ok(())
}

/// Print a failure message and set the exit code to 1.
fn fail_msg(msg: &str) {
    println!("\n{}\n", msg);
    env::set_exit_status(1);
}

/// Print usage string, with an reason for printing it.
fn print_usage(reason: &str) {
    println!("\n{}\n\n{}\n{}", reason, VERSION, HELP);
    env::set_exit_status(1);
}

/// Get a list of all possible letter combinations from a phone number.
/// Assumes a 7 digit number, and returns an Error on non-numeric characters.
fn get_combos(number: &str) -> Result<Vec<String>, Error> {

    let numbermap: HashMap<char, Vec<char>> = hashmap!{
        '0' => vec!['0'],
        '1' => vec!['1'],
        '2' => vec!['a', 'b', 'c'],
        '3' => vec!['d', 'e', 'f'],
        '4' => vec!['g', 'h', 'i'],
        '5' => vec!['j', 'k', 'l'],
        '6' => vec!['m', 'n', 'o'],
        '7' => vec!['p', 'q', 'r', 's'],
        '8' => vec!['t', 'u', 'v'],
        '9' => vec!['w', 'x', 'y', 'z']
    };


    let chars: Vec<char> = number.chars().collect();
    let mut combos = Vec::new();

    // Iterate over the product of all possible letters.
    for combo in iproduct!(
        match numbermap.get(&chars[0]) {
            Some(c) => c,
            None => {
                return Err(Error::NotANumber(chars[0]));
            }
        },
        match numbermap.get(&chars[1]) {
            Some(c) => c,
            None => {
                return Err(Error::NotANumber(chars[1]));
            }
        },
        match numbermap.get(&chars[2]) {
            Some(c) => c,
            None => {
                return Err(Error::NotANumber(chars[2]));
            }
        },
        match numbermap.get(&chars[3]) {
            Some(c) => c,
            None => {
                return Err(Error::NotANumber(chars[3]));
            }
        },
        match numbermap.get(&chars[4]) {
            Some(c) => c,
            None => {
                return Err(Error::NotANumber(chars[4]));
            }
        },
        match numbermap.get(&chars[5]) {
            Some(c) => c,
            None => {
                return Err(Error::NotANumber(chars[5]));
            }
        },
        match numbermap.get(&chars[6]) {
            Some(c) => c,
            None => {
                return Err(Error::NotANumber(chars[6]));
            }
        }
    ) {
        // Create a new string from these letters, to be stored in `combos`.
        let combov = vec![
            combo.0.clone(),
            combo.1.clone(),
            combo.2.clone(),
            combo.3.clone(),
            combo.4.clone(),
            combo.5.clone(),
            combo.6.clone()];
        combos.push(combov.into_iter().collect::<String>());
    }

    Ok(combos)
}

/// Return this executable's parent directory.
fn get_exe_parent() -> io::Result<PathBuf> {
    let mut exepath = try!(env::current_exe());
    exepath.pop();
    Ok(exepath)
}

/// Return the word-file path. The file must be named 'words', and it must
/// be located next to the `phonewords` executable.
/// A symlink can be used to point to some other file, but the symlink must
/// follow the same rules (named 'words', located next to the exe).
fn get_wordfile_path() -> io::Result<PathBuf> {
    let mut exeparent = try!(get_exe_parent());
    exeparent.push("words");
    // Try to read as a symlink first, but fallback to the normal path.
    read_link(&exeparent).or(Ok(exeparent))
}

#[derive(Debug)]
enum Error {
    Io(Option<PathBuf>, io::Error),
    NotANumber(char),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(..) => "IO error",
            Error::NotANumber(_) => "Not a number",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref file, ref err) => {
                match *file {
                    Some(ref file) => write!(fmt, "Io error: {}, file: {}", err, file.display()),
                    None => write!(fmt, "Io error: {}", err)
                }
            },
            Error::NotANumber(num) => {
                write!(fmt, "Error while generating letter combos, not a number: {}", num)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(None, err)
    }
}
