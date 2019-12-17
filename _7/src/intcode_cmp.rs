use std::sync::mpsc;

#[derive(Debug)]
pub struct IntCodeComputer {
    pub program: Vec<i32>,
    pub tx: mpsc::Sender<i32>,
    pub rx: mpsc::Receiver<i32>,
}

impl IntCodeComputer {
    fn fetch_arg_0(&self, i_pointer: usize) -> i32 {
        // use int division to cut out digit
        if self.program[i_pointer] % 1000 / 100 == 1 {
            self.program[i_pointer + 1] // immediate mode
        } else {
            self.program[self.program[i_pointer + 1] as usize] // position mode
        }
    }

    fn fetch_arg_1(&self, i_pointer: usize) -> i32 {
        // use int division to cut out digit
        if self.program[i_pointer] % 10000 / 1000 == 1 {
            self.program[i_pointer + 2] // immediate mode
        } else {
            self.program[self.program[i_pointer + 2] as usize] // position mode
        }
    }

    fn fetch_arg_0_1(&self, i_pointer: usize) -> (i32, i32, usize) {
        let a = self.fetch_arg_0(i_pointer);
        let b = self.fetch_arg_1(i_pointer);
        let target = self.program[i_pointer + 3] as usize;

        (a, b, target)
    }

    pub fn compute(&mut self) {
        let mut i_pointer = 0;

        while self.program[i_pointer] != 99 {
            if self.program[i_pointer] % 10 == 1 {
                let (a, b, target) = self.fetch_arg_0_1(i_pointer);
                self.program[target] = a + b;
                i_pointer += 4;
            } else if self.program[i_pointer] % 10 == 2 {
                let (a, b, target) = self.fetch_arg_0_1(i_pointer);
                self.program[target] = a * b;
                i_pointer += 4;
            } else if self.program[i_pointer] == 3 {
                let input_int = self.rx.recv().unwrap();
                let target = self.program[i_pointer + 1] as usize;
                self.program[target] = input_int;
                i_pointer += 2;
            } else if self.program[i_pointer] % 10 == 4 {
                let a = self.fetch_arg_0(i_pointer);
                self.tx.send(a).unwrap();
                i_pointer += 2;
            } else if self.program[i_pointer] % 10 == 5 {
                let a = self.fetch_arg_0(i_pointer);
                let b = self.fetch_arg_1(i_pointer) as usize;
                if a != 0 {
                    i_pointer = b;
                } else {
                    i_pointer += 3;
                }
            } else if self.program[i_pointer] % 10 == 6 {
                let a = self.fetch_arg_0(i_pointer);
                let b = self.fetch_arg_1(i_pointer) as usize;
                if a == 0 {
                    i_pointer = b;
                } else {
                    i_pointer += 3;
                }
            } else if self.program[i_pointer] % 10 == 7 {
                let (a, b, target) = self.fetch_arg_0_1(i_pointer);
                self.program[target] = (a < b) as i32;
                i_pointer += 4;
            } else if self.program[i_pointer] % 10 == 8 {
                let (a, b, target) = self.fetch_arg_0_1(i_pointer);
                self.program[target] = (a == b) as i32;
                i_pointer += 4;
            } else {
                println!(
                    "WRONG INSTRUCTION AT {}: {}",
                    i_pointer, self.program[i_pointer]
                );
                break;
            }
        }
    }
}
