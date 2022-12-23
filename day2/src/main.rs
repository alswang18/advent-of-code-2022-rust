use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("this is the answer: {}", max_score(&input));
}

fn max_score(input: &str) -> usize {
    let input_arr = input.split("\n");
    let mut res = 0;
    for s in input_arr {
        let mut choices = s.split_whitespace();
        let enemy_raw = choices.nth(0).unwrap();
        let me_raw = choices.nth(0).unwrap();
        let enemy = letter_to_string(enemy_raw);
        // let me = letter_to_string(me_raw);
        let me = letter_enemy_to_string(enemy_raw, me_raw);
        res += result(enemy, me);
        res += string_to_num(me);
    }
    res
}

fn result(a: &str, b: &str) -> usize {
    if a == b {
        return 3;
    } else if (a == "ROCK" && b == "PAPER")
        || (a == "PAPER" && b == "SCISSORS")
        || (a == "SCISSORS" && b == "ROCK")
    {
        return 6;
    } else {
        return 0;
    }
}

fn string_to_num(a: &str) -> usize {
    if a == "ROCK" {
        return 1;
    } else if a == "PAPER" {
        return 2;
    } else {
        return 3;
    }
}
// for day 1
fn letter_to_string(a: &str) -> &str {
    if a == "A" || a == "X" {
        return "ROCK";
    } else if a == "B" || a == "Y" {
        return "PAPER";
    } else {
        return "SCISSORS";
    }
}

// for day 2
fn letter_enemy_to_string<'a>(a: &'a str, b: &'a str) -> &'a str {
    if (a == "A" && b == "X") || (a == "C" && b == "Y") || (a == "B" && b == "Z") {
        return "SCISSORS";
    } else if (a == "B" && b == "X") || (a == "A" && b == "Y") || (a == "C" && b == "Z") {
        return "ROCK";
    } else {
        return "PAPER";
    }
}
