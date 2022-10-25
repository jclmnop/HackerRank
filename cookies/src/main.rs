use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'cookies' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY A
 */

// Rust has a BinaryHeap struct in the standard library,
// but I wanted to implement my own to get some experience
// with building data structures in Rust

struct MinHeap<T: Ord + Copy> {
    heap: Vec<T>
}

impl<T: Ord + Copy> MinHeap<T> {
    fn from_slice(input: &[T]) -> Self {
        // TODO: figure out why in-place heap building isn't working
        // let mut heap: Vec<T> = Vec::new();
        // heap.extend_from_slice(input);
        // let mut min_heap = MinHeap{ heap: Vec::new() };
        // let start_index = if min_heap.size() >= 2 {
        //     min_heap.size() / 2 - 1
        // } else {0};

        // for i in (start_index..=0).rev() {
        //     min_heap.heapify(i)
        // }

        // min_heap

        let mut heap: MinHeap<T> = MinHeap { heap: Vec::new() };
        for v in input {
            heap.push(v.clone());
        }
        heap
    }

    fn heapify(&mut self, root: usize) -> () {
        let n = self.size();
        let mut min = root;
        let left = self.left_child(root);
        let right = self.right_child(root);

        if left < n && self.heap[left] < self.heap[min] {
            min = left;
        }
        if right < n && self.heap[right] < self.heap[min] {
            min = right;
        }

        if min != root {
            self.swap_elements(root, min);
            self.heapify(min)
        }

    }

    fn pop(&mut self) -> T {
        let v = self.heap.swap_remove(0);
        self.heapify(0);
        v
    }

    fn push(&mut self, v: T) {
        self.heap.push(v);
        let mut i = self.size() - 1;
        let mut parent = self.parent(i);
        if i == 0 { return }

        while self.heap[i] < self.heap[parent] {
            self.swap_elements(i, parent);
            i = parent;
            parent = self.parent(i);
        }
    }

    fn size(&self) -> usize {
        self.heap.len()
    }

    fn swap_elements(&mut self, i: usize, k: usize) {
        (self.heap[i], self.heap[k]) = (self.heap[k], self.heap[i])
    }

    fn is_leaf(&self, i: usize) -> bool {
        self.left_child(i) > self.size()
    }

    fn parent(&self, i: usize) -> usize {
        return if i >= 2 {
            (i - 1) / 2
        } else {
            0
        }
    }

    fn left_child(&self, i: usize) -> usize {
        i * 2 + 1
    }

    fn right_child(&self, i: usize) -> usize {
        i * 2 + 2
    }
}

fn cookies(k: i32, A: &[i32]) -> i32 {
    fn sweet_enough(cookies: &MinHeap<i32>, k: i32) -> bool {
        cookies.heap[0] >= k
    }

    fn mix_cookies(cookie_1: i32, cookie_2: i32) -> i32 {
        cookie_1 + (2 * cookie_2)
    }

    if A.is_empty() { return -1 }

    let mut iterations = 0;
    let mut cookies = MinHeap::from_slice(A);
    while !sweet_enough(&cookies, k) {
        if cookies.size() <= 1 { return -1 }
        iterations += 1;
        let new_cookie = mix_cookies(
            cookies.pop(),
            cookies.pop()
        );
        cookies.push(new_cookie);
    }

    iterations


}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

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

    let result = cookies(k, &A);

    writeln!(&mut fptr, "{}", result).ok();
}
