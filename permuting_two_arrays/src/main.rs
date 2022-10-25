use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'twoArrays' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY A
 *  3. INTEGER_ARRAY B
 */

fn twoArrays(k: i32, A: &[i32], B: &[i32]) -> String {
    // TODO: just sort A, then sort B in reverse.
    //       check that [A[i]+B[i]..].min() < k
    let mut a: Vec<i32> = Vec::from(A);
    let mut b: Vec<i32> = Vec::from(B);
    a.sort();
    b.sort_by(|a, b| { b.cmp(a)});

    for (i, num) in a.iter_mut().enumerate() {
        *num += b[i];
    }

    if a.iter().min().unwrap() >= &k {
        String::from("YES")
    } else {
        String::from("NO")
    }

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let A: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let B: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = twoArrays(k, &A, &B);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
