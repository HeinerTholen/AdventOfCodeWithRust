mod input;


fn compute(noun: i32, verb: i32) -> i32 {
    let mut instr = input::PROGRAM_INPUT;
    instr[1] = noun;
    instr[2] = verb;

    let mut i_pointer = 0;
    while instr[i_pointer] != 99 {
        let a = instr[i_pointer+1] as usize;
        let b = instr[i_pointer+2] as usize;
        let target = instr[i_pointer+3] as usize;
        if instr[i_pointer] == 1 {
            instr[target] = instr[a] + instr[b];
        } else if instr[i_pointer] == 2 {
            instr[target] = instr[a] * instr[b];
        } else {
            println!("WRONG! {}", instr[i_pointer]);
        }
        i_pointer += 4;
    }
    instr[0]
}

fn main() {
    println!("Task 1 result: {}", compute(12, 02));

    for noun in 0..100 {
        for verb in 0..100 {
            let result = compute(noun, verb);
            if result == 19690720 {
                println!("noun: {}, verb: {}", noun, verb);
                return;
            }
        }
    }
}
