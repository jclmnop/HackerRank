use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let diagonal_sums = sum_diagonals(arr);
    i32::abs(diagonal_sums.0 - diagonal_sums.1)
}

fn sum_diagonals(arr: &[Vec<i32>]) -> (i32, i32) {
    let left_diagonal: i32 = arr.iter().enumerate().fold(
        0,
        |acc, (i, a)| { acc + a[i] }
    );
    let right_diagonal: i32 = arr.iter().enumerate().fold(
        0,
        |acc, (i, a)| { acc + a[arr.len()-i-1] }
    );
    (left_diagonal, right_diagonal)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
