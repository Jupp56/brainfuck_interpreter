use crate::program::*;

const ALLOWED_SYMBOLS: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];

#[derive(Debug)]
pub enum ParseError {
    BracketError(usize),
}

/// # Overview
/// Parses the program into a Vec of Instructions.
/// It strips all ignored characters and optimizes the parsed code to use less instructions.
/// # Behaviour
/// ## Validity
/// This parser does only check for bracket validity. It does not determine in any way (even if possible) if there are i.e. underruns in cells.
/// ## Optimizations
/// The parser optimizes into the given instructions in `program.rs`.
/// It detects:
/// - multiple succeeding operators (like +++ or <<<) get optimized into IncCell(3) or DecPtr(3)
/// - [-] gets optimized into the ZeroCell operator, which zeroes a cell instantly instead of incremantally
/// - structures like [>>>>] get transformed into FindZeroRight(4)
/// # Example
/// ```
/// use brainfuck_interpreter::*;
/// use brainfuck_interpreter::program::Instruction;
/// let content = "><<+-.,[][-]";
/// let program = parse_program(&mut content.to_string()).unwrap();
/// let reference_result = vec![
///     Instruction::IncPtr(1),
///     Instruction::DecPtr(2),
///     Instruction::IncCell(1),
///     Instruction::DecCell(1),
///     Instruction::Output,
///     Instruction::Input,
///     Instruction::JumpForward(7),
///     Instruction::JumpBackward(6),
///     Instruction::ZeroCell,
/// ];
/// for i in 0..9 {
///     println!("{:?}", program[i]);
///     assert_eq!(program[i], reference_result[i]);
/// }
/// ```
pub fn parse_program(mut raw_input: &mut String) -> Result<Program, ParseError> {
    strip_program(&mut raw_input);
    let pre_processed_program = pre_process_program(&mut raw_input);

    let mut instructions: Program = Vec::new(); //list of all instructions found
    let mut markers_forward: Vec<usize> = Vec::new();
    let chars: Vec<char> = pre_processed_program.chars().collect();
    let input_len = chars.len();

    for c in chars {
        match c {
            '+' => {
                let found_inc_cell: u32;
                match instructions.last() {
                    Some(last) => match last {
                        Instruction::IncCell(x) => {
                            found_inc_cell = *x;
                        }
                        _ => found_inc_cell = 0,
                    },
                    None => found_inc_cell = 0,
                };
                if found_inc_cell == 0 {
                    instructions.push(Instruction::IncCell(1));
                } else {
                    instructions.pop();
                    instructions.push(Instruction::IncCell(found_inc_cell + 1));
                }
            }
            '-' => {
                let prev_len: u32;
                let instr = instructions.last();
                match instr {
                    Some(last) => match last {
                        Instruction::DecCell(x) => {
                            prev_len = *x;
                        }
                        _ => prev_len = 0,
                    },
                    None => prev_len = 0,
                }
                if prev_len == 0 {
                    instructions.push(Instruction::DecCell(1))
                } else {
                    instructions.pop();
                    instructions.push(Instruction::DecCell(prev_len + 1));
                }
            }
            '>' => {
                let prev_len: usize;
                let instr = instructions.last();
                match instr {
                    Some(last) => match last {
                        Instruction::IncPtr(x) => {
                            prev_len = *x;
                        }
                        _ => prev_len = 0,
                    },
                    None => prev_len = 0,
                }
                if prev_len == 0 {
                    instructions.push(Instruction::IncPtr(1))
                } else {
                    instructions.pop();
                    instructions.push(Instruction::IncPtr(prev_len + 1));
                }
            }
            '<' => {
                let prev_len: usize;
                let instr = instructions.last();
                match instr {
                    Some(last) => match last {
                        Instruction::DecPtr(x) => {
                            prev_len = *x;
                        }
                        _ => prev_len = 0,
                    },
                    None => prev_len = 0,
                }
                if prev_len == 0 {
                    instructions.push(Instruction::DecPtr(1))
                } else {
                    instructions.pop();
                    instructions.push(Instruction::DecPtr(prev_len + 1));
                }
            }
            ',' => instructions.push(Instruction::Input),
            '.' => instructions.push(Instruction::Output),
            '[' => {
                instructions.push(Instruction::JumpForward(0));
                markers_forward.push(instructions.len() - 1);
            }
            ']' => {
                let opening_bracket_index = markers_forward.pop();
                let current_instruction_index = instructions.len();
                match opening_bracket_index {
                    Some(opening_bracket_index) => {
                        if !look_for_find_zero(&mut instructions, current_instruction_index)? {
                            instructions.push(Instruction::JumpBackward(opening_bracket_index));
                            instructions[opening_bracket_index] =
                                Instruction::JumpForward(current_instruction_index);
                        }
                    }
                    None => return Err(ParseError::BracketError(current_instruction_index)),
                }
            }
            '0' => instructions.push(Instruction::ZeroCell),
            _ => (), //everything else but the aforementioned symbols is a comment
        }
    }

    //if any opening brackets were not closed, throw error
    if !markers_forward.is_empty() {
        return Err(ParseError::BracketError(markers_forward[0]));
    }
    println!(
        "Parsing finished, found {} instructions, optimized into {} internal instructions.",
        input_len,
        instructions.len()
    );
    Ok(instructions)
}

/// removes all but the allowed characters (speed optimization for later match clause)
fn strip_program(code: &mut String) {
    code.retain(|c| ALLOWED_SYMBOLS.contains(&c));
}
/// optimizes everything that can be easily expressed by a regex.
fn pre_process_program(code: &String) -> String {
    // set zero character, easy and almost elegant
    code.replace("[-]", "0")
}

/// If a closing bracket was found, this checks for the condition necessary for the find_zero instruction.
/// If the condition is met, the instructions get updated accordingly.
fn look_for_find_zero(
    instructions: &mut Program,
    current_instruction_index: usize,
) -> Result<bool, ParseError> {
    if instructions.len() > 2 {
        match instructions.last() {
            Some(ins) => match ins {
                Instruction::DecPtr(x) => {
                    let jump_length: usize = *x;
                    match instructions.get(instructions.len() - 2) {
                        Some(ins2) => match ins2 {
                            Instruction::JumpForward(_) => {
                                instructions.pop();
                                instructions.pop();
                                instructions.push(Instruction::FindZeroLeft(jump_length));
                                Ok(true)
                            }
                            _ => Ok(false),
                        },
                        None => Ok(false),
                    }
                }
                Instruction::IncPtr(x) => {
                    let jump_length: usize = *x;
                    match instructions.get(instructions.len() - 2) {
                        Some(ins2) => match ins2 {
                            Instruction::JumpForward(_x) => {
                                instructions.pop();
                                instructions.pop();
                                instructions.push(Instruction::FindZeroRight(jump_length));
                                Ok(true)
                            }
                            _ => Ok(false),
                        },
                        None => Ok(false),
                    }
                }
                _ => Ok(false),
            },
            None => return Err(ParseError::BracketError(current_instruction_index)),
        }
    } else {
        Ok(false)
    }
}

#[test]
fn test_parse_simple_program() {
    use crate::program::*;
    let content = "><+-.,[][-]";
    let program = parse_program(&mut content.to_string()).unwrap();
    let reference_result = vec![
        Instruction::IncPtr(1),
        Instruction::DecPtr(1),
        Instruction::IncCell(1),
        Instruction::DecCell(1),
        Instruction::Output,
        Instruction::Input,
        Instruction::JumpForward(7),
        Instruction::JumpBackward(6),
        Instruction::ZeroCell
    ];
    for i in 0..9 {
        println!("{:?}", program[i]);
        assert_eq!(program[i], reference_result[i]);
    }
}
