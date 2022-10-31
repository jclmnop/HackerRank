use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'icecreamParlor' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER m
 *  2. INTEGER_ARRAY arr
 */

fn icecreamParlor(m: i32, arr: &[i32]) -> Vec<i32> {
    let mut previous: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();

    for (i, cost) in arr.iter().enumerate() {
        let diff = m - cost;
        if let Some(k) = previous.get(&diff) {
            result.push(*k as i32 + 1);
            result.push(i as i32 + 1);
            break;
        } else {
            previous.insert(*cost, i);
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = icecreamParlor(m, &arr);

        for i in 0..result.len() {
            write!(&mut fptr, "{}", result[i]).ok();

            if i != result.len() - 1 {
                write!(&mut fptr, " ").ok();
            }
        }

        writeln!(&mut fptr).ok();
    }
}
