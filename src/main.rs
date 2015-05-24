#![feature(exit_status)]
#[macro_use]

/// Finds possible words that can be made from a phone number.
/// Generates all possible letter combos, and searches the `words` file
/// for combos that contain words.
/// -Christopher Welborn 5-19-15

extern crate itertools;

use std::collections::{HashMap};
use std::env;
use std::fs::{read_link, File};
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

static VERSIONSTR: &'static str = "PhoneWords v. 0.0.7-2";


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

fn main() {
    let args: Vec<String> = env::args().collect();

    let arglen = args.len();
    let mut quiet:bool = false;

    if arglen < 2 {
        print_usage(Some("No arguments!"));
        return;
    } else if arglen == 3 {
        if args[2] == "-q" || args[2] == "--quiet" {
            quiet = true;
        } else {
            print_usage(Some("Expecting -q as the second argument."));
            return;
        }

    } else if arglen > 3 {
        print_usage(Some("Invalid number of arguments."));
        return;
    }

    if args[1] == "-h" || args[1] == "--help" {
        print_usage(None);
        return;
    } else if args[1] == "-v" || args[1] == "--version" {
        println!("{}", VERSIONSTR);
        return;
    }

    let wordpath = match get_wordfile_path() {
        Err(e) => {
            fail_msg(&format!("Unable to get word file path: {}", e));
            return;
        },
        Ok(p) => p
    };

    check_number(&args[1], &wordpath, quiet);
}

/// Check a number for matches, print optional status and matches as they
/// are found.
fn check_number(number: &String, wordfile: &Path, quiet: bool) {

    // Optional status printer.
    let status = |msg: &String| {
        if !quiet {
            println!("{}", msg);
        }
    };

    // Ensure that the number is exactly 7 digits,
    // ..truncate or pad with 0's if needed.
    let mut usenumber: String = number.clone();
    while usenumber.len() < 7 {
        usenumber.insert(0, '0');
    }
    while usenumber.len() > 7 {
        usenumber.pop();
    }

    status(&format!("\n Checking: {}", usenumber));

    let combos = match get_combos(&usenumber) {
        Ok(c) => c,
        Err(e) => {
            fail_msg(
                &format!("Error while generating letter combos:\n{}", e));
            return;
        }
    };
    status(&format!("   Combos: {}", combos.len()));

    let filename = wordfile.to_str().unwrap();
    let wordfile = BufReader::new(match File::open(wordfile) {
        Err(e) => {
            fail_msg(&format!("Cannot open file: {}\n{}", filename, e));
            return;
        },
        Ok(f) => f
    });
    status(&format!("Word File: {}\n", filename));

    // In the future, variable-length numbers may be used.
    // It would be wasteful to check all combos for a word that is longer.
    let numberlen = usenumber.len();
    // Keep track of the actual attempts/counts.
    let mut matchcnt = 0u32;
    let mut wordcnt = 0usize;
    let mut trycnt = 0usize;

    for tryword in wordfile.lines() {
        let word = match tryword {
            Err(e) => {
                fail_msg(&format!("Error while reading word: {}", e));
                return;
            },
            Ok(w) => w
        };
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

    status(
        &format!(
            "\nFound {mcnt} {mplur} against {wcnt} {wplur} in {tcnt} tries.",
            mcnt=matchcnt,
            mplur=pluralmatches,
            wcnt=wordcnt,
            wplur=pluralwords,
            tcnt=trycnt
        )
    );

    // Exit status 2 if no matches were found (otherwise successful).
    env::set_exit_status(if matchcnt == 0 {2} else {0});
}

/// Print a failure message and set the exit code to 1.
fn fail_msg(msg: &String) {
    println!("{}", msg);
    env::set_exit_status(1);
}

/// Get a list of all possible letter combinations from a phone number.
/// Assumes a 7 digit number, and returns an Error on non-numeric characters.
fn get_combos(number: &str) -> Result<Box<Vec<String>>> {

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
    let mut combos: Vec<String> = Vec::new();

    let invalid_num_err = |c: char| {
        Error::new(ErrorKind::Other, format!("Not a number: {}", c))
    };

    // Iterate over the product of all possible letters.
    for combo in iproduct!(
        match numbermap.get(&chars[0]) {
            Some(c) => c,
            None => {
                return Err(invalid_num_err(chars[0]));
            }
        },
        match numbermap.get(&chars[1]) {
            Some(c) => c,
            None => {
                return Err(invalid_num_err(chars[1]));
            }
        },
        match numbermap.get(&chars[2]) {
            Some(c) => c,
            None => {
                return Err(invalid_num_err(chars[2]));
            }
        },
        match numbermap.get(&chars[3]) {
            Some(c) => c,
            None => {
                return Err(invalid_num_err(chars[3]));
            }
        },
        match numbermap.get(&chars[4]) {
            Some(c) => c,
            None => {
                return Err(invalid_num_err(chars[4]));
            }
        },
        match numbermap.get(&chars[5]) {
            Some(c) => c,
            None => {
                return Err(invalid_num_err(chars[5]));
            }
        },
        match numbermap.get(&chars[6]) {
            Some(c) => c,
            None => {
                return Err(invalid_num_err(chars[6]));
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

    Ok(Box::new(combos))
}

/// Return this executable's parent directory.
fn get_exe_parent() -> Result<PathBuf> {
    let exepath = try!(env::current_exe());
    let mut exeparent = exepath.clone();
    exeparent.pop();
    Ok(exeparent)
}

/// Return the word-file path. The file must be named 'words', and it must
/// be located next to the `phonewords` executable.
/// A symlink can be used to point to some other file, but the symlink must
/// follow the same rules (named 'words', located next to the exe).
fn get_wordfile_path() -> Result<PathBuf> {
    let mut exeparent = try!(get_exe_parent());
    exeparent.push("words");
    // Try to read as a symlink first, but fallback to the normal path.
    match read_link(&exeparent) {
        Err(_) => Ok(exeparent),
        Ok(realpath) => Ok(realpath)
    }
}

/// Print usage string, with an optional reason for printing it.
/// If a `reason` is set, the exit status is set to 1.
fn print_usage(reason: Option<&str>) {
    if let Some(msg) = reason {
        // A 'reason' means the message is printed because of user error.
        println!("\n{}\n", msg);
        env::set_exit_status(1);
    }

    println!("{}

    Usage:
        phonewords -h | -v
        phonewords <number> [-q]

    Options:
        number        : Phone number to check (7 digits).
        -h,--help     : Show this message.
        -q,--quiet    : Only print results.
        -v,--version  : Show version and exit.

    Exit status is 1 on error, 2 if no matches were found, and 0 on success.
    ", VERSIONSTR);
}
