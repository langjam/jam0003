use rand::rngs::SmallRng;
use rand::Rng;
use strum_macros::{Display, EnumIter};

use crate::direction::Direction;
use crate::memory::Memory;
use crate::processor::Processor;

#[derive(EnumIter, Debug, PartialEq, Display, FromPrimitive, ToPrimitive)]
pub enum Instruction {
    // Noop
    NOOP = 0,
    // Numbers
    N1 = 1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    RND, // Random number

    // stack operators
    DUP = 20,
    DUP2,
    DROP,
    SWAP,
    OVER,
    ROT,

    // Arithmetic
    ADD = 30,
    SUB,
    MUL,
    DIV,
    MOD,

    // Comparison
    EQ = 40,
    GT,
    LT,

    // Logic
    NOT = 50,
    AND,
    OR,

    // control
    JMP = 60, // also serves as return
    JMPIF,    // jump if boolean true,
    CALL,     // put return address on stack before jumping,
    CALLIF,   // call if boolean true

    // memory
    ADDR = 70,
    READ,
    WRITE,

    // PRINT0,
    // PRINT1,
    // PRINT2,

    // processors
    START = 80, // start a new processor given a starting point (only 1 can started in execution block)
    END,        // end this processor's existence

    // resources
    EAT = 90,
    GROW,

    // split and merge
    SPLIT = 100,
    MERGE,
}

impl Instruction {
    pub fn decode(value: u8) -> Option<Instruction> {
        num::FromPrimitive::from_u8(value)
    }

    pub fn execute(&self, processor: &mut Processor, memory: &mut Memory, rng: &mut SmallRng) {
        match self {
            // Instruction::PRINT0 => {
            //     println!("P0 {:?}", processor.current_stack());
            // }
            // Instruction::PRINT1 => {
            //     println!("P1 {:?}", processor.current_stack());
            // }
            // Instruction::PRINT2 => {
            //     println!("P2 {:?}", processor.current_stack());
            // }
            Instruction::NOOP => {
                // nothing
            }
            // Numbers
            Instruction::N1 => {
                processor.push(1);
            }
            Instruction::N2 => {
                processor.push(2);
            }
            Instruction::N3 => {
                processor.push(3);
            }
            Instruction::N4 => {
                processor.push(4);
            }
            Instruction::N5 => {
                processor.push(5);
            }
            Instruction::N6 => {
                processor.push(6);
            }
            Instruction::N7 => {
                processor.push(7);
            }
            Instruction::N8 => {
                processor.push(8);
            }
            Instruction::RND => {
                processor.push(rng.gen::<u8>() as u64);
            }

            // Stack manipulation
            Instruction::DUP => {
                processor.dup();
            }
            Instruction::DUP2 => {
                processor.dup2();
            }
            Instruction::DROP => {
                processor.drop();
            }
            Instruction::SWAP => {
                processor.swap();
            }
            Instruction::OVER => {
                processor.over();
            }
            Instruction::ROT => {
                processor.rot();
            }

            // Arithmetic
            Instruction::ADD => {
                let a = processor.pop();
                let b = processor.pop();
                processor.push(b.wrapping_add(a));
            }
            Instruction::SUB => {
                let a = processor.pop();
                let b = processor.pop();
                processor.push(b.wrapping_sub(a));
            }
            Instruction::MUL => {
                let a = processor.pop();
                let b = processor.pop();
                processor.push(b.wrapping_mul(a));
            }
            Instruction::DIV => {
                let a = processor.pop();
                let b = processor.pop();
                if a == 0 {
                    processor.push(0);
                    return;
                }
                processor.push(b.wrapping_div(a));
            }
            Instruction::MOD => {
                let a = processor.pop();
                let b = processor.pop();
                if a == 0 {
                    processor.push(0);
                    return;
                }
                processor.push(b.wrapping_rem(a));
            }

            // Comparison
            Instruction::EQ => {
                let a = processor.pop();
                let b = processor.pop();
                if a == b {
                    processor.push(1);
                } else {
                    processor.push(0);
                }
            }
            Instruction::GT => {
                let a = processor.pop();
                let b = processor.pop();
                if b > a {
                    processor.push(1);
                } else {
                    processor.push(0);
                }
            }
            Instruction::LT => {
                let a = processor.pop();
                let b = processor.pop();
                if b < a {
                    processor.push(1);
                } else {
                    processor.push(0);
                }
            }

            // Logic
            Instruction::NOT => {
                let a = processor.pop();
                if a > 0 {
                    processor.push(0);
                } else {
                    processor.push(1);
                }
            }
            Instruction::AND => {
                let a = processor.pop();
                let b = processor.pop();
                if a > 0 && b > 0 {
                    processor.push(1);
                } else {
                    processor.push(0);
                }
            }
            Instruction::OR => {
                let a = processor.pop();
                let b = processor.pop();
                if a > 0 || b > 0 {
                    processor.push(1);
                } else {
                    processor.push(0);
                }
            }

            // Control
            Instruction::JMP => {
                let popped = processor.pop_address(memory);
                if let Some(address) = popped {
                    processor.jump(address);
                }
            }
            Instruction::JMPIF => {
                let condition = processor.pop();
                let popped = processor.pop_address(memory);
                if condition == 0 {
                    return;
                }
                if let Some(address) = popped {
                    processor.jump(address);
                }
            }
            Instruction::CALL => {
                let popped = processor.pop_address(memory);
                if let Some(address) = popped {
                    processor.call(address);
                }
            }
            Instruction::CALLIF => {
                let condition = processor.pop();
                let popped = processor.pop_address(memory);
                if condition == 0 {
                    return;
                }
                if let Some(address) = popped {
                    processor.call(address);
                }
            }

            // Memory
            Instruction::ADDR => {
                processor.push(processor.address());
            }
            Instruction::READ => {
                let popped = processor.pop_address(memory);
                let value = match popped {
                    Some(address) => memory.values[address],
                    // out of bounds address
                    None => u8::MAX,
                };
                processor.push(value as u64);
            }
            Instruction::WRITE => {
                let value = processor.pop();
                let popped = processor.pop_address(memory);
                match popped {
                    Some(address) => {
                        let constrained_value = if value >= u8::MAX as u64 {
                            u8::MAX
                        } else {
                            // truncate
                            value as u8
                        };
                        memory.values[address] = constrained_value;
                    }
                    None => {
                        // no write out of bounds
                    }
                }
            }

            // Processors
            Instruction::START => {
                let popped = processor.pop_address(memory);
                if let Some(address) = popped {
                    processor.start(address);
                }
            }

            Instruction::END => {
                processor.end();
            }

            // resources
            Instruction::EAT => {
                processor.want_eat = true;
            }
            Instruction::GROW => {
                processor.want_grow = true;
            }

            // split and merge
            Instruction::SPLIT => {
                let direction = processor.pop();
                let popped = processor.pop_address(memory);
                if let Some(address) = popped {
                    let direction = if let Some(direction) =
                        num::FromPrimitive::from_u8((direction % 4) as u8)
                    {
                        direction
                    } else {
                        // XXX random instead. but shouldn't happen...
                        Direction::North
                    };
                    processor.want_split = Some((direction, address));
                }
            }

            Instruction::MERGE => {
                let direction = processor.pop();
                let direction =
                    if let Some(direction) = num::FromPrimitive::from_u8((direction % 4) as u8) {
                        direction
                    } else {
                        // XXX random instead. but shouldn't happen...
                        Direction::North
                    };
                processor.want_merge = Some(direction);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::assembler::text_to_words;
    use crate::testutil::{execute, execute_lines};

    #[test]
    fn test_decode_success() {
        assert_eq!(Instruction::decode(0), Some(Instruction::NOOP));
    }

    #[test]
    fn test_decode_failure() {
        assert_eq!(Instruction::decode(u8::MAX), None);
    }

    #[test]
    fn test_rnd() {
        let exec = execute("RND RND");

        assert_eq!(exec.processor.current_stack(), [97, 61]);
    }

    #[test]
    fn test_add() {
        let exec = execute("N2 N1 ADD");

        assert_eq!(exec.processor.current_stack(), [3]);
    }

    #[test]
    fn test_sub() {
        let exec = execute("N4 N2 SUB");

        assert_eq!(exec.processor.current_stack(), [2]);
    }

    #[test]
    fn test_mul() {
        let exec = execute("N4 N2 MUL");

        assert_eq!(exec.processor.current_stack(), [8]);
    }

    #[test]
    fn test_div() {
        let exec = execute("N8 N2 DIV");

        assert_eq!(exec.processor.current_stack(), [4]);
    }

    #[test]
    fn test_div_by_zero() {
        let exec = execute("N8 N2 N2 SUB DIV");
        assert_eq!(exec.processor.current_stack(), [0]);
    }

    #[test]
    fn test_mod() {
        let exec = execute("N8 N2 N1 ADD MOD");
        assert_eq!(exec.processor.current_stack(), [2]);
    }

    #[test]
    fn test_mod_by_zero() {
        let exec = execute("N8 N2 N2 SUB MOD");
        assert_eq!(exec.processor.current_stack(), [0]);
    }

    #[test]
    fn test_not_positive() {
        let exec = execute("N2 NOT");
        assert_eq!(exec.processor.current_stack(), [0]);
    }

    #[test]
    fn test_not_zero() {
        let exec = execute("N2 N2 SUB NOT");
        assert_eq!(exec.processor.current_stack(), [1]);
    }

    #[test]
    fn test_eq_equal() {
        let exec = execute("N2 N2 EQ");

        assert_eq!(exec.processor.current_stack(), [1]);
    }

    #[test]
    fn test_eq_not_equal() {
        let exec = execute("N1 N2 EQ");
        assert_eq!(exec.processor.current_stack(), [0]);
    }

    #[test]
    fn test_addr() {
        let exec = execute("ADDR");
        assert_eq!(exec.processor.current_stack(), [0]);
    }

    #[test]
    fn test_addr_further() {
        let exec = execute("N1 N2 N4 ADDR");
        assert_eq!(exec.processor.current_stack(), [1, 2, 4, 3]);
    }

    #[test]
    fn test_jmp() {
        let exec = execute("ADDR JMP");
        assert_eq!(exec.processor.current_stack(), []);
        assert_eq!(exec.processor.address(), 0);
    }

    #[test]
    fn test_jump_further() {
        let exec = execute("N2 JMP NOOP NOOP N1 N2");
        assert_eq!(exec.processor.current_stack(), [1, 2]);
    }

    #[test]
    fn test_jmpif_true() {
        let exec = execute("ADDR N1 JMPIF");
        assert_eq!(exec.processor.current_stack(), []);
        assert_eq!(exec.processor.address(), 0);
    }

    #[test]
    fn test_jmpif_false() {
        let exec = execute("ADDR N1 N1 SUB JMPIF");
        assert_eq!(exec.processor.current_stack(), []);
        assert_eq!(exec.processor.address(), 5);
    }

    #[test]
    fn test_die_if_out_of_bounds() {
        let mut exec = execute("N1 N2");
        assert_eq!(exec.processor.current_stack(), [1, 2]);
        // execute two more
        exec.processor
            .execute_amount(&mut exec.memory, &mut exec.small_rng, 1002);
        assert_eq!(exec.processor.current_stack(), [1, 2]);
        assert_eq!(exec.processor.alive, false);
    }

    #[test]
    fn test_copy_self() {
        let text = "
            ADDR  # c
            ADDR  # c loop
            SWAP  # loop c
            DUP   # loop c c
            READ  # loop c inst
            SWAP  # loop inst c
            DUP   # loop inst c c
            N8
            N8
            MUL
            ADD   # loop inst c c+64
            ROT   # loop c c+64 inst
            WRITE # loop c
            N1
            ADD   # loop c+1
            SWAP  # c+1 loop
            JMP";

        let mut exec = execute_lines(text);
        let words = text_to_words(text);
        let words_amount = words.len();

        exec.processor.execute_amount(
            &mut exec.memory,
            &mut exec.small_rng,
            (words_amount - 1) * words_amount,
        );

        assert_eq!(
            exec.assembler
                .disassemble_to_words(&exec.memory.values[64..64 + words_amount]),
            words
        );
    }
}
