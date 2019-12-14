use std::collections::HashSet;

const RANGE: (i32, i32) = (172930, 683082);

fn crit_1(num: i32) -> bool {
    // dirty way to go via strings
    let num_chars = num.to_string().into_bytes();
    let mut last_char = num_chars[0];
    for i in 1..6 {
        if num_chars[i] == last_char {
            return true;
        }
        last_char = num_chars[i];
    }
    false
}

fn crit_2(num: i32) -> bool {
    // dirty way to go via strings... again!!
    let num_str = num.to_string();
    let num_chars = num_str.chars();
    let num_ints: Vec<u32> = num_chars.map(|x| x.to_digit(10).unwrap()).collect();
    for i in 0..5 {
        let a = num_ints[i];
        let b = num_ints[i + 1];
        if b < a {
            return false;
        }
    }
    true
}

fn crit_3(num: i32) -> bool {
    // dirty way to go via strings
    let num_chars = num.to_string().into_bytes();

    let mut last_val = num_chars[0];
    let mut n_same = 1;
    let mut counts = HashSet::new();

    for i in 1..6 {
        if last_val == num_chars[i] {
            n_same += 1;
        } else {
            counts.insert(n_same);
            n_same = 1;
            last_val = num_chars[i];
        }
    }
    counts.insert(n_same);
    counts.contains(&2)
}

fn main() {
    let range = RANGE.0..RANGE.1 + 1;
    let range2 = range.clone();

    let result: i32 = range.map(|x| (crit_1(x) && crit_2(x)) as i32).sum();
    println!("Only crit 1 and 2: {:?}", result);

    let result: i32 = range2
        .map(|x| (crit_3(x) && crit_2(x) && crit_1(x)) as i32)
        .sum();
    println!("Only crit 2 and 3: {:?}", result);
}
