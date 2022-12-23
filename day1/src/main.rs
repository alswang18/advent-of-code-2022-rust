use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let top_n = &args[2]
        .parse::<usize>()
        .expect("input number is not parsable");
    println!("In file {}", file_path);

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!(
        "{} is the {} most calories!",
        most_calories(&input, *top_n),
        *top_n
    );
}
fn most_calories(input: &String, top_n: usize) -> usize {
    let split = input.split("\n");
    let mut cur_cal = 0;
    let mut heap = BinaryHeap::new();

    for s in split {
        if s == "" {
            heap.push(Reverse(cur_cal));

            while heap.len() > top_n {
                heap.pop();
            }

            cur_cal = 0;
        } else {
            cur_cal += s
                .parse::<i32>()
                .expect("string cannot be parsed into number");
        }
    }
    heap.push(Reverse(cur_cal));

    while heap.len() > top_n {
        heap.pop();
    }

    let mut res = 0;
    for _ in 0..top_n {
        res += heap.pop().unwrap().0;
    }
    res as usize
}
