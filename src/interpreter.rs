use std::io;
use std::io::prelude::*;
use crate::program::*;

const STACK_LIMIT: usize = 30000;

/// Error types that can happen during runtime
pub enum RuntimeError {
    InputError(String),
    RegisterLimitExceededError
}

/// executes a program
pub fn run_program(program: &Program) -> Result<(), RuntimeError> {
    let mut registers: [u32; STACK_LIMIT] = [0; STACK_LIMIT];
    let mut current_instruction_index: usize = 0;
    let mut current_register_index: usize = 0;

    loop {
        let inst = program.get(current_instruction_index);
        match inst {
            Some(instr) => {
                //println!("Current inst nr: {}, inst: {:?}, current_register: {}, current_register content: {}", current_instruction_index, instr, current_register_index, registers[current_register_index]);

                match instr {
                    Instruction::IncPtr(x) => current_register_index += x,
                    Instruction::DecPtr(x) => current_register_index -= x,
                    Instruction::IncCell(x) => registers[current_register_index] += x,
                    Instruction::DecCell(x) => registers[current_register_index] -= x,
                    Instruction::Output => {
                        let ch = std::char::from_u32(registers[current_register_index]);
                        let ch_disp: String = match ch {
                            Some(c) => c.to_string(),
                            None => registers[current_register_index].to_string(),
                        };
                        print!("{}", ch_disp);
                    }
                    Instruction::Input => {
                        let mut buff: [u8; 1] = [0; 1];
                        match io::stdin().read_exact(&mut buff) {
                            Err(e) => {
                                return Err(RuntimeError::InputError(format!(
                                    "Error while reading from console! Error: {}",
                                    e.to_string()
                                )))
                            }
                            Ok(_res) => (),
                        }
                        registers[current_register_index] = buff[0] as u32;
                    }
                    Instruction::JumpForward(jump_index) => {
                        if registers[current_register_index] == 0 {
                            current_instruction_index = *jump_index;
                        }
                    }
                    Instruction::JumpBackward(jump_index) => {
                        if registers[current_register_index] != 0 {
                            current_instruction_index = *jump_index;
                        }
                    }
                    Instruction::ZeroCell => registers[current_register_index] = 0,
                    Instruction::FindZeroRight(jump_distance) => {
                        if registers[current_register_index] != 0 {
                            loop {
                                current_register_index += jump_distance;
                                if registers[current_register_index] == 0 {
                                    break;
                                }
                            }
                        }
                    }
                    Instruction::FindZeroLeft(jump_distance) => {
                        if registers[current_register_index] != 0 {
                            loop {
                                current_register_index -= jump_distance;
                                if registers[current_register_index] == 0 {
                                    break;
                                }
                            }
                        }
                    }
                }
                current_instruction_index += 1;
            }
            None => {
                break;
            }
        }
        if current_register_index >= STACK_LIMIT {
            return Err(RuntimeError::RegisterLimitExceededError)
        }
    }

    Ok(())
}