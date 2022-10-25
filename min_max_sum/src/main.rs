use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut v: Vec<i64> = arr.iter().map(|x|{*x as i64}).collect();
    v.sort();

    let min_sum: i64 = v[..v.len()-1].iter().sum();
    let max_sum: i64 = v[1..v.len()].iter().sum();

    println!("{min_sum} {max_sum}");


}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
