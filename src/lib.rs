use std::{self, ops::{Add, AddAssign, SubAssign}, io::{self, prelude::*}, convert::TryInto};

extern crate num;
use num::Zero;

type pos = usize;

#[derive(Debug)]
pub enum InvalidProgramErr {
    ExcessiveOpeningBracket(pos),
    UnexpectedClosingBracket(pos),
}

#[derive(Debug)]
pub enum EvalErr {
    InvalidProgramErr(InvalidProgramErr),
    IOErr(std::io::Error),
}

impl std::convert::From<InvalidProgramErr> for EvalErr {
    fn from(err: InvalidProgramErr) -> EvalErr {
        EvalErr::InvalidProgramErr(err)
    }
}

pub struct Buf<T> {
    buf: Vec<T>,
    ptr: usize,
}

impl<T> Buf<T>
    where T: Zero + Add + AddAssign + SubAssign {
        pub fn new(bufsize: usize) -> Self {
            let mut buf = Self {
                buf: Vec::with_capacity(bufsize),
                ptr: 0,
            };

            for _ in 0..bufsize {
                buf.buf.push(T::zero);
            };

            buf
        }

        pub fn clone(buf_org: &[T]) -> Buf<T> {
            let mut buf = Self {
                buf: Vec::with_capacity(buf_org.len()),
                ptr: 0,
            };

            for i in 0..buf_org.len() {
                buf.buf.push(buf_org[i])
            };

            buf
        }

        pub fn buf(&self) -> &[T] {
            &self.buf[..]
        }

        pub fn fwd(&mut self, offset: usize) {
            self.ptr += offset;
        }

        pub fn bwd(&mut self, offset: usize) {
            self.ptr -= offset;
        }

        pub fn inc(&mut self, offset: T) {
            self.buf[self.ptr] += offset;
        }

        pub fn dec(&mut self, offset: T) {
            self.buf[self.ptr] -= offset;
        }

        pub fn r(&self) -> T {
            self.buf[self.ptr]
        }

        pub fn w(&mut self, val: T) {
            self.buf[self.ptr] = val;
        }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    START,
    END,
    LOOPSTART(pos),
    LOOPEND(pos),
    INCVAL(pos),
    DECVAL(pos),
    MOVEFORWARD(pos),
    MOVEBACK(pos),
    INPUT(pos),
    OUTPUT(pos),
}

pub fn tokenize(code: &Vec<char>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    tokens.push(Token::START);

    for (pos, opcode) in code.iter().enumerate() {
        match opcode {
            '[' => tokens.push(Token::LOOPSTART(pos)),
            ']' => tokens.push(Token::LOOPEND(pos)),
            '+' => tokens.push(Token::INCVAL(pos)),
            '-' => tokens.push(Token::DECVAL(pos)),
            _ => (),
        }
    }

    tokens.push(Token::END);

    tokens
}