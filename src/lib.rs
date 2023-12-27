#[derive(Debug, Clone)]
pub enum Token {
    INC,
    DEC,
    FWD,
    BWD,
    INP,
    OUT,
    LPS,
    LPE,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    INC,
    DEC,
    FWD,
    BWD,
    INP,
    OUT,
    LOOP(Vec<Instruction>),
}

pub fn lexer(src: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    for c in src.chars() {
        match c {
            '+' => tokens.push(Token::INC),
            '-' => tokens.push(Token::DEC),
            '>' => tokens.push(Token::FWD),
            '<' => tokens.push(Token::BWD),
            ',' => tokens.push(Token::INP),
            '.' => tokens.push(Token::OUT),
            '[' => tokens.push(Token::LPS),
            ']' => tokens.push(Token::LPE),
            _ => (),
        }
    }
    tokens
}

pub fn parser(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, token) in tokens.iter().enumerate() {
        if loop_stack == 0 {
            let ins = match token {
                Token::INC => Some(Instruction::INC),
                Token::DEC => Some(Instruction::DEC),
                Token::FWD => Some(Instruction::FWD),
                Token::BWD => Some(Instruction::BWD),
                Token::INP => Some(Instruction::INP),
                Token::OUT => Some(Instruction::OUT),

                Token::LPS => {
                    loop_stack += 1;
                    loop_start = i;
                    None
                }

                Token::LPE => panic!("Unmatched loop end at {}", i),
            };

            match ins {
                Some(ins) => instructions.push(ins),
                None => (),
            }
        } else {
            match token {
                Token::LPS => loop_stack += 1,
                Token::LPE => {
                    loop_stack -= 1;
                    if loop_stack == 0 {
                        instructions.push(Instruction::LOOP(parser(
                            tokens[loop_start + 1..i].to_vec(),
                        )));
                    }
                }
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        panic!("Unmatched loop start at {}", loop_start);
    }

    instructions
}

pub fn executor(ins: &Vec<Instruction>, tape: &mut Vec<i32>, ptr: &mut usize) {
    for i in ins {
        match i {
            Instruction::INC => tape[*ptr] += 1,
            Instruction::DEC => tape[*ptr] -= 1,
            Instruction::FWD => *ptr += 1,
            Instruction::BWD => *ptr -= 1,
            Instruction::INP => {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                tape[*ptr] = buf.parse::<i32>().unwrap();
            }
            Instruction::OUT => print!("{}", tape[*ptr] as u8 as char),
            Instruction::LOOP(ins) => {
                while tape[*ptr] != 0 {
                    executor(ins, tape, ptr);
                }
            }
        }
    }
}
