use std::sync::mpsc;


/// Intcode computer implementation from AdventOfCode 2019 puzzles.
///
/// A detailed description is found in the puzzle descriptions:
/// https://adventofcode.com/2019/day/2
/// https://adventofcode.com/2019/day/5
/// https://adventofcode.com/2019/day/7
///
/// ```
/// use intcode_cmp::IntCodeComputer;
///
/// let program = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
/// let (mut icc, tx_in, rx_out) = IntCodeComputer::new();
///
/// tx_in.send(0).unwrap();
/// icc.compute(program.to_vec());
/// let result = rx_out.recv().unwrap();
///
/// assert_eq!(0, result);
/// ```
#[derive(Debug)]
pub struct IntCodeComputer {
    program: Vec<i32>,
    i_pointer: usize,
    tx: mpsc::Sender<i32>,
    rx: mpsc::Receiver<i32>,
}

impl IntCodeComputer {
    /// Create a new Intcode computer.
    ///
    /// Returns a tuple with the computer itself, an input- and an output channel.
    ///
    /// TODO find a more idiomatic way of providing the in- and output channels.
    pub fn new() -> (IntCodeComputer, mpsc::Sender<i32>, mpsc::Receiver<i32>) {
        let (tx_in, rx_in) = mpsc::channel();
        let (tx_out, rx_out) = mpsc::channel();
        let icc = IntCodeComputer {
            program: Vec::new(),
            i_pointer: 0,
            tx: tx_out,
            rx: rx_in,
        };
        (icc, tx_in, rx_out)
    }

    fn fetch_arg(&self, number: usize) -> i32 {
        let mod_ = 1000 * 10_i32.pow(number as u32);
        let div_ = 100 * 10_i32.pow(number as u32);

        // use int division to cut out digit
        match self.program[self.i_pointer] % mod_ / div_ {
            // position mode
            0 => self.program[self.program[self.i_pointer + 1 + number] as usize],

            // immediate mode
            1 => self.program[self.i_pointer + 1 + number],

            _ => {
                panic!(
                    "INVALID INSTRUCTION AT {}: {}",
                    self.i_pointer, self.program[self.i_pointer]
                );
            }
        }
    }

    fn fetch_arg_0_1_t(&mut self) -> (i32, i32, usize) {
        let a = self.fetch_arg(0);
        let b = self.fetch_arg(1);
        let target = self.program[self.i_pointer + 3] as usize;

        (a, b, target)
    }

    fn handle_math_instr<F>(&mut self, func: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        let (a, b, target) = self.fetch_arg_0_1_t();
        self.program[target] = func(a, b);
        self.i_pointer += 4;
    }

    fn fetch_input(&mut self) {
        let input_int = self.rx.recv().unwrap();
        let target = self.program[self.i_pointer + 1] as usize;
        self.program[target] = input_int;
        self.i_pointer += 2;
    }

    fn send_output(&mut self) {
        let a = self.fetch_arg(0);
        self.tx.send(a).unwrap();
        self.i_pointer += 2;
    }

    fn jump_if(&mut self, what: bool) {
        let a = self.fetch_arg(0);
        let b = self.fetch_arg(1) as usize;
        if (a != 0) == what {
            self.i_pointer = b;
        } else {
            self.i_pointer += 3;
        }
    }

    /// Run a program on the computer.
    ///
    /// If the program expects an input, it must be supplied through the input channel. If running
    /// on only one thread, the channel must be filled before `compute` is called, as otherwise the
    /// computer will wait for input indefinitely.


    pub fn compute(&mut self, program: Vec<i32>) {
        self.program = program;
        self.i_pointer = 0;

        loop {
            match self.program[self.i_pointer] % 100 {
                1 => self.handle_math_instr(|a, b| a + b),
                2 => self.handle_math_instr(|a, b| a * b),
                3 => self.fetch_input(),
                4 => self.send_output(),
                5 => self.jump_if(true),
                6 => self.jump_if(false),
                7 => self.handle_math_instr(|a, b| (a < b) as i32),
                8 => self.handle_math_instr(|a, b| (a == b) as i32),
                99 => break,
                _ => {
                    panic!(
                        "UNKNOWN INSTRUCTION AT {}: {}",
                        self.i_pointer, self.program[self.i_pointer]
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::intcode_cmp::IntCodeComputer;

    // input as taken from https://adventofcode.com/2019/day/5
    pub const INTCODE_TEST_PROGRAM: [i32; 678] = [
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1002, 114, 19, 224, 1001, 224, -646, 224,
        4, 224, 102, 8, 223, 223, 1001, 224, 7, 224, 1, 223, 224, 223, 1101, 40, 62, 225, 1101, 60,
        38, 225, 1101, 30, 29, 225, 2, 195, 148, 224, 1001, 224, -40, 224, 4, 224, 1002, 223, 8,
        223, 101, 2, 224, 224, 1, 224, 223, 223, 1001, 143, 40, 224, 101, -125, 224, 224, 4, 224,
        1002, 223, 8, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 101, 29, 139, 224, 1001, 224, -99,
        224, 4, 224, 1002, 223, 8, 223, 1001, 224, 2, 224, 1, 224, 223, 223, 1101, 14, 34, 225,
        102, 57, 39, 224, 101, -3420, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 7, 224, 1,
        223, 224, 223, 1101, 70, 40, 225, 1102, 85, 69, 225, 1102, 94, 5, 225, 1, 36, 43, 224, 101,
        -92, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 1, 224, 224, 1, 224, 223, 223, 1102, 94, 24,
        224, 1001, 224, -2256, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 1, 224, 1, 223, 224, 223,
        1102, 8, 13, 225, 1101, 36, 65, 224, 1001, 224, -101, 224, 4, 224, 102, 8, 223, 223, 101,
        3, 224, 224, 1, 223, 224, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1,
        99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 8, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 329, 1001, 223, 1, 223,
        1108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 344, 101, 1, 223, 223, 1108, 226, 677,
        224, 1002, 223, 2, 223, 1006, 224, 359, 101, 1, 223, 223, 107, 226, 226, 224, 1002, 223, 2,
        223, 1005, 224, 374, 101, 1, 223, 223, 1107, 226, 226, 224, 1002, 223, 2, 223, 1005, 224,
        389, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 404, 101, 1, 223,
        223, 1008, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 419, 101, 1, 223, 223, 108, 677,
        226, 224, 1002, 223, 2, 223, 1006, 224, 434, 101, 1, 223, 223, 1108, 677, 226, 224, 102, 2,
        223, 223, 1005, 224, 449, 101, 1, 223, 223, 1008, 677, 226, 224, 102, 2, 223, 223, 1006,
        224, 464, 1001, 223, 1, 223, 108, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 479, 101, 1,
        223, 223, 7, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 494, 1001, 223, 1, 223, 8, 226,
        677, 224, 102, 2, 223, 223, 1006, 224, 509, 101, 1, 223, 223, 107, 677, 226, 224, 1002,
        223, 2, 223, 1005, 224, 524, 1001, 223, 1, 223, 7, 677, 226, 224, 1002, 223, 2, 223, 1005,
        224, 539, 1001, 223, 1, 223, 1007, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 554, 1001,
        223, 1, 223, 8, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 569, 101, 1, 223, 223, 7, 226,
        677, 224, 102, 2, 223, 223, 1006, 224, 584, 1001, 223, 1, 223, 1008, 677, 677, 224, 102, 2,
        223, 223, 1005, 224, 599, 101, 1, 223, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1006,
        224, 614, 101, 1, 223, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 629, 101, 1,
        223, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 644, 101, 1, 223, 223, 1007,
        226, 226, 224, 102, 2, 223, 223, 1005, 224, 659, 1001, 223, 1, 223, 108, 226, 226, 224,
        102, 2, 223, 223, 1006, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226,
    ];

    #[test]
    fn instruction_codes_01_02_03_04() {
        let (mut icc, tx_in, rx_out) = IntCodeComputer::new();

        // Functionality of the computer is being tested by the test program. Every successful run
        // emits a zero at the end. The last output is the result for the first task in the
        // challenge.
        // https://adventofcode.com/2019/day/5
        tx_in.send(1).unwrap();
        icc.compute(INTCODE_TEST_PROGRAM.to_vec());
        assert_eq!(0, rx_out.recv().unwrap());
        assert_eq!(0, rx_out.recv().unwrap());
        assert_eq!(0, rx_out.recv().unwrap());
        assert_eq!(0, rx_out.recv().unwrap());
        assert_eq!(0, rx_out.recv().unwrap());
        assert_eq!(0, rx_out.recv().unwrap());
        assert_eq!(0, rx_out.recv().unwrap());
        assert_eq!(0, rx_out.recv().unwrap());
        assert_eq!(0, rx_out.recv().unwrap());
        assert_eq!(15314507, rx_out.recv().unwrap());
    }

    #[test]
    fn instruction_code_05() {
        // test program taken from https://adventofcode.com/2019/day/5
        let program = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let (mut icc, tx_in, rx_out) = IntCodeComputer::new();

        tx_in.send(0).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(0, rx_out.recv().unwrap());

        tx_in.send(-4).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(1, rx_out.recv().unwrap());

        tx_in.send(120).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(1, rx_out.recv().unwrap());
    }

    #[test]
    fn instruction_code_06() {
        // test program taken from https://adventofcode.com/2019/day/5
        let program = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let (mut icc, tx_in, rx_out) = IntCodeComputer::new();

        tx_in.send(0).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(0, rx_out.recv().unwrap());

        tx_in.send(-4).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(1, rx_out.recv().unwrap());

        tx_in.send(120).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(1, rx_out.recv().unwrap());
    }

    #[test]
    fn instruction_code_07() {
        // test program taken from https://adventofcode.com/2019/day/5
        let program = [3,9,7,9,10,9,4,9,99,-1,8];
        let (mut icc, tx_in, rx_out) = IntCodeComputer::new();

        tx_in.send(0).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(1, rx_out.recv().unwrap());

        tx_in.send(-4).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(1, rx_out.recv().unwrap());

        tx_in.send(120).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(0, rx_out.recv().unwrap());
    }

    #[test]
    fn instruction_code_08() {
        // test program taken from https://adventofcode.com/2019/day/5
        let program = [3,3,1108,-1,8,3,4,3,99];
        let (mut icc, tx_in, rx_out) = IntCodeComputer::new();

        tx_in.send(0).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(0, rx_out.recv().unwrap());

        tx_in.send(8).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(1, rx_out.recv().unwrap());

        tx_in.send(120).unwrap();
        icc.compute(program.to_vec());
        assert_eq!(0, rx_out.recv().unwrap());
    }
}
