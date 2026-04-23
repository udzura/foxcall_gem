use std::cell::Cell;

use crate::foxcall::Insn;

const TAPE_SIZE: usize = 30000;

pub struct Machine {
    tape: [u8; TAPE_SIZE],
    data_ptr: usize,
    insns: Vec<Cell<Insn>>,
    insn_ptr: usize,
    input: Vec<u8>,
    input_pos: usize,
    output: Vec<u8>,
}

impl Machine {
    pub fn new(insns: Vec<Cell<Insn>>, input: Vec<u8>) -> Self {
        Self {
            tape: [0; TAPE_SIZE],
            data_ptr: 0,
            insns,
            insn_ptr: 0,
            input,
            input_pos: 0,
            output: Vec::new(),
        }
    }

    pub fn step(&mut self) -> bool {
        if self.insn_ptr >= self.insns.len() {
            return false;
        }
        let insn = self.insns[self.insn_ptr].get();
        match insn {
            Insn::IncrPrt => self.data_ptr = (self.data_ptr + 1) % TAPE_SIZE,
            Insn::DecrPrt => self.data_ptr = (self.data_ptr + TAPE_SIZE - 1) % TAPE_SIZE,
            Insn::IncrVal => self.tape[self.data_ptr] = self.tape[self.data_ptr].wrapping_add(1),
            Insn::DecrVal => self.tape[self.data_ptr] = self.tape[self.data_ptr].wrapping_sub(1),
            Insn::Print => self.output.push(self.tape[self.data_ptr]),
            Insn::Scan => {
                if self.input_pos < self.input.len() {
                    self.tape[self.data_ptr] = self.input[self.input_pos];
                    self.input_pos += 1;
                } else {
                    eprintln!("Warning: input exhausted");
                    return false; // EOF
                }
            }
            Insn::JumpFwd(jump_to) => {
                if self.tape[self.data_ptr] == 0 {
                    self.insn_ptr = jump_to;
                    return true;
                }
            }
            Insn::JumpBwd(jump_to) => {
                if self.tape[self.data_ptr] != 0 {
                    self.insn_ptr = jump_to;
                    return true;
                }
            }
        }
        self.insn_ptr += 1;
        true
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn into_output(self) -> Vec<u8> {
        self.output
    }
}
