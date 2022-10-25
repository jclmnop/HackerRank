use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'lonelyinteger' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 * Given an array of integers, where all elements but one occur twice, find the unique element.
 * Example
 * a = [1, 2, 3, 4, 3, 2, 1]
 * The unique element is 4
 */

fn lonelyinteger(a: &[i32]) -> i32 {
    let mut v = Vec::from(a);
    v.sort();
    let mut i: usize = 0;
    let mut k: usize = 1;
    let mut res: i32 = v[v.len()-1]; // in case array is only size 1, or largest number is lonely
    while k < v.len() {
        if !v[i].eq(&v[k]) {
            res = v[i];
            break;
        } else {
            i += 2;
            k += 2;
        }
    }

    res

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = lonelyinteger(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
