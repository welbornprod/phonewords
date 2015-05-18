#[macro_use]
extern crate itertools;

use std::collections::{HashMap};

/// Creates and initializes a hashmap.
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
    let combos = get_combos("2759372");
    for c in combos.iter() {
        println!("{}", c);
    }
}


fn get_combos(number: &str) -> Box<Vec<String>> {
    #![allow(dead_code)]
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

    if (number.len()) != 7 {
        panic!("Invalid length for the number: {}", number);
    }

    let chars: Vec<char> = number.chars().collect();

    let mut total = 0isize;

    let mut combos: Vec<String> = Vec::new();
    for combo in iproduct!(
        numbermap.get(&chars[0]).unwrap(),
        numbermap.get(&chars[1]).unwrap(),
        numbermap.get(&chars[2]).unwrap(),
        numbermap.get(&chars[3]).unwrap(),
        numbermap.get(&chars[4]).unwrap(),
        numbermap.get(&chars[5]).unwrap(),
        numbermap.get(&chars[6]).unwrap()
    ) {
        let combov = vec![
            combo.0.clone(),
            combo.1.clone(),
            combo.2.clone(),
            combo.3.clone(),
            combo.4.clone(),
            combo.5.clone(),
            combo.6.clone()];
        combos.push(combov.into_iter().collect::<String>());
        total += 1;
    }
    println!("\nFound {} combos.", total);
    Box::new(combos)
}
