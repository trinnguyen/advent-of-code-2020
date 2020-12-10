use std::collections::{HashMap, HashSet, LinkedList};
use std::{fmt, io::Read};

fn main() {
    let mut str = String::new();
    std::io::stdin().read_to_string(&mut str).unwrap();

    println!("result part-1: {:?}", part_1(&str));
}

fn part_1(str: &str) -> i32 {
    let mut program = Program::from(str);
    program.execute()
}

struct Program {
    // code data loaded from set of instructions
    code_data: Vec<Inst>,

    /// accumulator register, should stored in another struct called Machine
    /// this is a simple puzzle, keep it simmple
    reg_acc: i32,

    /// program counter
    pc: usize,

    // visitted instr index
    cached_inst: HashSet<usize>
}

impl Program {
    /// load program from textual instr
    fn from(str_data: &str) -> Self {
        // load code data
        let mut code_data: Vec<Inst> = Vec::new();
        let lines = str_data.lines();
        for line in lines {
            code_data.push(Inst::from(line));
        }

        Program {
            code_data,
            reg_acc: 0,
            pc: 0,
            cached_inst: HashSet::new()
        }
    }

    /// start the program and stop at second-called instruction
    /// return value of accumulator variable
    fn execute(&mut self) -> i32 {
        // reset program
        self.reg_acc = 0;
        self.pc = 0;
        self.cached_inst.clear();

        // continuously executing the instruction
        loop {
            if self.cached_inst.contains(&self.pc) {
                break
            }

            self.execute_instr();
        }

        // return value of acc
        self.reg_acc
    }

    /// execute current instruction and update pc
    fn execute_instr(&mut self) {

        let instr = self.code_data.get(self.pc);
        let mut move_pc: i32 = 1;
        match instr {
            Some(Inst::Jmp(val)) => move_pc = *val,
            Some(Inst::Acc(val)) => self.reg_acc = self.reg_acc + val,
            Some(Inst::Nop) => (),
            _ => panic!("unexpected error, pc is out of index")
        }
        self.cached_inst.insert(self.pc);
        self.update_pc_by(move_pc)
    }

    fn update_pc_by(&mut self, val: i32) {
        let res = (self.pc as i32) + val;
        if res < 0 || res >= self.code_data.len() as i32 {
            panic!("jump of out range")
        }
        self.pc = res as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Inst {
    Acc(i32),
    Jmp(i32),
    Nop,
}

impl Inst {
    /// parse instruction
    ///
    /// example:
    /// nop +0
    /// acc +1
    /// jmp +4
    fn from(str: &str) -> Inst {
        let mut iter = str.split(' ');
        match (iter.next(), iter.next()) {
            (Some(code), Some(s_val)) => {
                // parse num
                let bytes = s_val.as_bytes();
                let prefix: i32 = match bytes[0] {
                    b'+' => 1,
                    b'-' => -1,
                    _ => panic!("expect + or -"),
                };
                let u_num: u32 = s_val[1..s_val.len()].parse().unwrap();
                let num: i32 = (u_num as i32) * prefix;

                // parse code
                match code {
                    "nop" => Inst::Nop,
                    "acc" => Inst::Acc(num),
                    "jmp" => Inst::Jmp(num),
                    _ => panic!("expect code nop or acc or jmp"),
                }
            }
            _ => panic!("invalid line"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Inst;

    #[test]
    fn test_inst_from() {
        assert_eq!(Inst::Nop, Inst::from("nop +0"));
        assert_eq!(Inst::Acc(1), Inst::from("acc +1"));
        assert_eq!(Inst::Acc(-1), Inst::from("acc -1"));
        assert_eq!(Inst::Acc(0), Inst::from("acc +0"));
        assert_eq!(Inst::Jmp(3), Inst::from("jmp +3"));
        assert_eq!(Inst::Jmp(-3), Inst::from("jmp -3"));
        assert_eq!(Inst::Jmp(0), Inst::from("jmp +0"));
    }

    #[test]
    #[should_panic]
    fn test_inst_from_panic() {
        Inst::from("");

        Inst::from("acc");
        Inst::from("jmp");
        Inst::from("nop");

        Inst::from("acc 1");
        Inst::from("jmp 1");
        Inst::from("nop 1");

        Inst::from("acc a");
        Inst::from("jmp a");
        Inst::from("nop a");
    }
}
