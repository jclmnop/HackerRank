use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::ops::BitXor;

/*
 * Complete the 'flippingBits' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER n as parameter.
 */

fn flippingBits(n: i64) -> i64 {
    // Problem spec says it's giving us a u32 and that it
    // expects a u32 to be returned, yet for some reason
    // it's parsed to i64 before being given to this function
    // and the function is also expected to return as i64
    (n as u32).bitxor(u32::MAX) as i64
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i64>().unwrap();

        let result = flippingBits(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
