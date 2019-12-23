mod input;
mod intcode_cmp;

use intcode_cmp::IntCodeComputer;

fn calc_single_sequence(program: &[i32], sequence: Vec<i32>) -> i32 {
    let (mut icc, tx_in, rx_out) = IntCodeComputer::new();
    let mut input_signal = 0;
    for phase_setting in sequence {
        tx_in.send(phase_setting).unwrap();
        tx_in.send(input_signal).unwrap();
        icc.compute(program.to_vec());
        input_signal = rx_out.recv().unwrap();
    }
    input_signal
}

fn calc_max_signal(program: &[i32]) -> i32 {
    let mut phases = [0, 1, 2, 3, 4];
    let mut output_signals = Vec::new();
    permutohedron::heap_recursive(&mut phases, |permutation| {
        output_signals.push(calc_single_sequence(program, permutation.to_vec()))
    });
    *output_signals.iter().max().unwrap()
}

fn main() {
    let result = calc_max_signal(&input::PROGRAM_INPUT);
    println!("Result for task 1: {:?}", result);
}

#[cfg(test)]
mod tests {
    use crate::calc_max_signal;
    use crate::calc_single_sequence;

    #[test]
    fn sequence_43210() {
        let result = calc_single_sequence(
            &[
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            [4, 3, 2, 1, 0].to_vec(),
        );
        assert_eq!(43210, result);
    }

    #[test]
    fn sequence_01234() {
        let result = calc_single_sequence(
            &[
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            [0, 1, 2, 3, 4].to_vec(),
        );
        assert_eq!(54321, result);
    }

    #[test]
    fn sequence_10432() {
        let result = calc_single_sequence(
            &[
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ],
            [1, 0, 4, 3, 2].to_vec(),
        );
        assert_eq!(65210, result);
    }

    #[test]
    fn sequence_43210_max() {
        let result = calc_max_signal(&[
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        assert_eq!(43210, result);
    }

    #[test]
    fn sequence_01234_max() {
        let result = calc_max_signal(&[
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        assert_eq!(54321, result);
    }

    #[test]
    fn sequence_10432_max() {
        let result = calc_max_signal(&[
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]);
        assert_eq!(65210, result);
    }
}
