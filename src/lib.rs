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

impl Token {
    pub fn from_char(c: char) -> Option<Token> {
        match c {
            '+' => Some(Token::INC),
            '-' => Some(Token::DEC),
            '>' => Some(Token::FWD),
            '<' => Some(Token::BWD),
            ',' => Some(Token::INP),
            '.' => Some(Token::OUT),
            '[' => Some(Token::LPS),
            ']' => Some(Token::LPE),
            _ => None,
        }
    }
    
}