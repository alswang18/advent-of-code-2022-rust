use std::{env, fs};
use std::collections::HashSet;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("this is the answer: {}", priority_sum_2(&input));
}

fn priority_sum(input:&str)->usize{
    let split = input.split("\n");
    let mut res = 0;
    for s in split{
        let first = &s[..s.len()/2];
        let second = &s[s.len()/2..];   
        let hs_a: HashSet<char> = first.chars().collect();
        let hs_b: HashSet<char> = second.chars().collect();
        let common_char = hs_a.intersection(&hs_b).next().unwrap();
        res += char_to_priority(common_char);
    }
    res
}

// part 2
fn priority_sum_2(input:&str)->usize{
    let mut split = input.split("\n");
    let mut res = 0;
    loop{
        let first = match split.next(){
            Some(a) => a,
            None =>break
        };
        let second = match split.next(){
            Some(a) => a,
            None =>break
        };
        let third = match split.next(){
            Some(a) => a,
            None =>break
        };

        let hs_a: HashSet<char> = first.chars().collect();
        let hs_b: HashSet<char> = second.chars().collect();
        let hs_c: HashSet<char> = third.chars().collect();
        let tmp_hs: HashSet<char> = hs_a.intersection(&hs_b).cloned().collect();
        let common_char = tmp_hs.intersection(&hs_c).next().unwrap();
        res += char_to_priority(common_char);
    }
    res
}

fn char_to_priority(key: &char)->usize{
    let ascii_val = *key as u32;
    if ascii_val > 90{
        return (ascii_val - 96) as usize;
    }else{
        return (ascii_val - 38) as usize;
    }
}