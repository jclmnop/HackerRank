use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let n: f32 = arr.len() as f32;
    let n_positives: f32 = arr.iter().filter(|x| { **x > 0 }).count() as f32;
    let n_negatives: f32 = arr.iter().filter(|x| { **x < 0 }).count() as f32;
    let n_zeroes: f32 = arr.iter().filter(|x| { **x == 0 }).count() as f32;

    let positive_ratio = n_positives / n;
    let negative_ratio = n_negatives / n;
    let zero_ratio = n_zeroes / n;

    println!("{positive_ratio:.6}");
    println!("{negative_ratio:.6}");
    println!("{zero_ratio:.6}");
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
