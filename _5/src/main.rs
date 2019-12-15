mod input;

fn fetch_input_a(instr: &[i32], i_pointer: usize) -> i32 {
    if instr[i_pointer] % 1000 / 100 == 1 {
        // use int division
        instr[i_pointer + 1] // immediate mode
    } else {
        instr[instr[i_pointer + 1] as usize] // position mode
    }
}

fn fetch_input_b(instr: &[i32], i_pointer: usize) -> i32 {
    if instr[i_pointer] % 10000 / 1000 == 1 {
        // use int division
        instr[i_pointer + 2] // immediate mode
    } else {
        instr[instr[i_pointer + 2] as usize] // position mode
    }
}

fn fetch_input(instr: &[i32], i_pointer: usize) -> (i32, i32, usize) {
    let a = fetch_input_a(&instr, i_pointer);
    let b = fetch_input_b(&instr, i_pointer);
    let target = instr[i_pointer + 3] as usize;

    (a, b, target)
}

fn compute(instr: &mut [i32]) {
    let mut i_pointer = 0;

    while instr[i_pointer] != 99 {
        if instr[i_pointer] % 10 == 1 {
            let (a, b, target) = fetch_input(&instr, i_pointer);
            instr[target] = a + b;
            i_pointer += 4;
        } else if instr[i_pointer] % 10 == 2 {
            let (a, b, target) = fetch_input(&instr, i_pointer);
            instr[target] = a * b;
            i_pointer += 4;
        } else if instr[i_pointer] == 3 {
            println!("Please enter your input:");
            let mut input_str = String::new();
            std::io::stdin().read_line(&mut input_str).unwrap();
            input_str = input_str.replace("\n", "");
            let input_int: i32 = input_str.parse().unwrap();
            let target = instr[i_pointer + 1] as usize;
            instr[target] = input_int;
            i_pointer += 2;
        } else if instr[i_pointer] % 10 == 4 {
            let a = fetch_input_a(&instr, i_pointer);
            println!("Output (at {}): {}", i_pointer, a);
            i_pointer += 2;
        } else if instr[i_pointer] % 10 == 5 {
            let a = fetch_input_a(&instr, i_pointer);
            let b = fetch_input_b(&instr, i_pointer) as usize;
            if a != 0 {
                i_pointer = b;
            } else {
                i_pointer += 3;
            }
        } else if instr[i_pointer] % 10 == 6 {
            let a = fetch_input_a(&instr, i_pointer);
            let b = fetch_input_b(&instr, i_pointer) as usize;
            if a == 0 {
                i_pointer = b;
            } else {
                i_pointer += 3;
            }
        } else if instr[i_pointer] % 10 == 7 {
            let (a, b, target) = fetch_input(&instr, i_pointer);
            instr[target] = (a < b) as i32;
            i_pointer += 4;
        } else if instr[i_pointer] % 10 == 8 {
            let (a, b, target) = fetch_input(&instr, i_pointer);
            instr[target] = (a == b) as i32;
            i_pointer += 4;
        } else {
            println!("WRONG INSTRUCTION AT {}: {}", i_pointer, instr[i_pointer]);
            break;
        }
    }
}

fn main() {
    println!("Task 1:");
    compute(&mut input::PROGRAM_INPUT);
}
