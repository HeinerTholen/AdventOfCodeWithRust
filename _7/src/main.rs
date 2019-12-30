mod input;
mod intcode_cmp;

use intcode_cmp::IntCodeComputer;

fn calc_single_sequence(program: &[i32], sequence: &[i32]) -> i32 {
    let mut input_signal = 0;
    for phase_setting in sequence {
        let mut icc = IntCodeComputer::new(&program.to_vec());
        icc.input.send(*phase_setting).unwrap();
        icc.input.send(input_signal).unwrap();
        icc.compute();
        input_signal = icc.output.recv().unwrap();
    }
    input_signal
}

fn calc_single_sequence_feedback(program: &[i32], sequence: &[i32]) -> i32 {
    // initialize computers
    let mut iccs: Vec<IntCodeComputer> = sequence
        .iter()
        .map(|init_code| {
            let mut icc = IntCodeComputer::new(&program.to_vec());
            icc.set_return_control(true);
            icc.input.send(*init_code).unwrap();
            icc
        })
        .collect();

    // run the program with feedback loop
    let mut input_signal = 0;
    loop {
        for icc in &mut iccs {
            icc.input.send(input_signal).unwrap();
            icc.compute();
            input_signal = icc.output.try_recv().unwrap();
        }
        if iccs[4].finished() {
            break;
        }
    }
    input_signal
}

fn calc_max_signal<F>(program: &[i32], phases: &mut [i32], func: F) -> i32
where
    F: Fn(&[i32], &[i32]) -> i32,
{
    let mut output_signals = Vec::new();
    permutohedron::heap_recursive(phases, |permutation| {
        output_signals.push(func(program, &permutation.to_vec()))
    });
    *output_signals.iter().max().unwrap()
}

fn main() {
    let result = calc_max_signal(
        &input::PROGRAM_INPUT,
        &mut [0, 1, 2, 3, 4],
        calc_single_sequence,
    );
    println!("Result for task 1: {:?}", result);

    let result = calc_max_signal(
        &input::PROGRAM_INPUT,
        &mut [9, 8, 7, 6, 5],
        calc_single_sequence_feedback,
    );
    println!("Result for task 2: {:?}", result);
}

#[cfg(test)]
mod tests {
    use crate::calc_max_signal;
    use crate::calc_single_sequence;
    use crate::calc_single_sequence_feedback;

    #[test]
    fn sequence_43210() {
        let result = calc_single_sequence(
            &[
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            &[4, 3, 2, 1, 0],
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
            &[0, 1, 2, 3, 4],
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
            &[1, 0, 4, 3, 2],
        );
        assert_eq!(65210, result);
    }

    #[test]
    fn sequence_43210_max() {
        let result = calc_max_signal(
            &[
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            &mut [0, 1, 2, 3, 4],
            calc_single_sequence,
        );
        assert_eq!(43210, result);
    }

    #[test]
    fn sequence_01234_max() {
        let result = calc_max_signal(
            &[
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            &mut [0, 1, 2, 3, 4],
            calc_single_sequence,
        );
        assert_eq!(54321, result);
    }

    #[test]
    fn sequence_10432_max() {
        let result = calc_max_signal(
            &[
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ],
            &mut [0, 1, 2, 3, 4],
            calc_single_sequence,
        );
        assert_eq!(65210, result);
    }

    #[test]
    fn sequence_feedback_98765() {
        let result = calc_single_sequence_feedback(
            &[
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5,
            ],
            &[9, 8, 7, 6, 5],
        );
        assert_eq!(139629729, result)
    }

    #[test]
    fn sequence_feedback_97856() {
        let result = calc_single_sequence_feedback(
            &[
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
            ],
            &[9, 7, 8, 5, 6],
        );
        assert_eq!(18216, result)
    }

    #[test]
    fn sequence_feedback_98765_max() {
        let result = calc_max_signal(
            &[
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5,
            ],
            &mut [9, 8, 7, 6, 5],
            calc_single_sequence_feedback
        );
        assert_eq!(139629729, result)
    }

    #[test]
    fn sequence_feedback_97856_max() {
        let result = calc_max_signal(
            &[
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
            ],
            &mut [9, 7, 8, 5, 6],
            calc_single_sequence_feedback
        );
        assert_eq!(18216, result)
    }
}
