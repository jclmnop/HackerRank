extern crate core;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pangrams' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

const ALPHABET_SIZE: usize = 26;

fn pangrams(s: &str) -> String {
    let mut letters: HashMap<char, bool> = HashMap::new();
    let s = s.to_lowercase();
    for c in s.chars() {
        if c != ' ' {
            letters.insert(c, true);
        }
    }

    if letters.keys().len() >= ALPHABET_SIZE {
        String::from("pangram")
    } else {
        String::from("not pangram")
    }

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = pangrams(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
