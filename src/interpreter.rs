use core::result::Result::{self, Ok, Err};
use arduino::serial;

pub struct Environment {
    instructions: [u8; 512],
    instruction_ptr: usize,
    memory: [u8; 256],
    ptr: usize
}

impl Environment {

    pub fn new() -> Environment {
        Environment {
            instructions: [0; 512],
            instruction_ptr: 0,
            memory: [0; 256],
            ptr: 0
        }
    }

    pub fn run(&mut self, instructions: [u8; 512]) -> Result<(), &'static str> {

        self.instructions = instructions;
        self.instruction_ptr = 0;
        self.memory = [0; 256];
        self.ptr = 0;

        while self.instruction_ptr < 511 {

            if self.instructions[self.instruction_ptr] == 0 {
                break;
            }

            let first = self.instructions[self.instruction_ptr] >> 4u8;
            let second = self.instructions[self.instruction_ptr] & 0x0Fu8;

            for &instruction in &[first, second] {
                match self.eval(instruction) {
                    Ok(_) => {}
                    Err(msg) => return Err(msg)
                }
            }

            self.instruction_ptr += 1;
        }

        Ok(())
    }

    fn eval(&mut self, instruction: u8) -> Result<(), &'static str> {
        match instruction {
            1 => { // +
                if self.memory[self.ptr] < 255 {
                    self.memory[self.ptr] += 1;
                } else {
                    self.memory[self.ptr] = 0;
                }
            },

            2 => { // -
                if self.memory[self.ptr] > 1 {
                    self.memory[self.ptr] -= 1
                } else {
                    self.memory[self.ptr] = 0;
                }
            },

            3 => { // >
                if self.ptr > 254 {
                    return Err("Error: pointer out of bounds\n")
                }

                self.ptr += 1;
            },

            4 => { // <
                if self.ptr < 1 {
                    return Err("Error: pointer out of bounds\n")
                }

                self.ptr -= 1;
            },

            5 => { // [

            },

            6 => { // ]

            },

            7 => { // ,
                self.memory[self.ptr] = serial::receive();
            },

            8 => { // .
                serial::transmit(self.memory[self.ptr])
            },

            _ => {}
        };

        Ok(())
    }
}
