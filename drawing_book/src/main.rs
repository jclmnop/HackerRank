use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pageCount' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER p
 */

fn pageCount(n: i32, p: i32) -> i32 {
    // type Page = (i32, i32);
    // let mut pages: Vec<Page> = Vec::with_capacity((n/2+1) as usize);
    // for n in (0..=n).step_by(2) {
    //     pages.push((n, n+1));
    // }
    std::cmp::min(
        p / 2,
        n / 2 + 1 - (p / 2 + 1)
    )
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = pageCount(n, p);

    writeln!(&mut fptr, "{}", result).ok();
}
