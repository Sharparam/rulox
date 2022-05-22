use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

use crate::value::Value;

#[derive(Default)]
pub struct Chunk {
    code: Vec<u8>,

    lines: Vec<usize>,

    pub constants: Vec<Value>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    Constant,

    ConstantLong,

    Return,
}

#[derive(Error, Clone, Debug)]
pub enum CompileError {
    #[error("Too many constants")]
    TooManyConstants,
}

// 24 bits
const MAX_CONSTANTS: usize = 0xFFFFFF;

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn read(&self, idx: usize) -> u8 {
        self.code[idx]
    }

    pub fn write<T>(&mut self, data: T, line: usize)
    where
        T: Into<u8>,
    {
        self.code.push(data.into());
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value, line: usize) -> Result<(), CompileError> {
        let index = self.add_constant(value)?;
        let opcode = if index > u8::MAX as usize {
            OpCode::ConstantLong
        } else {
            OpCode::Constant
        };

        self.write(opcode, line);

        if opcode == OpCode::Constant {
            self.write(index as u8, line);
        } else {
            let low = (index & 0xFF) as u8;
            let mid = ((index >> 8) & 0xFF) as u8;
            let high = ((index >> 16) & 0xFF) as u8;
            self.write(high, line);
            self.write(mid, line);
            self.write(low, line);
        }

        Ok(())
    }

    fn add_constant(&mut self, value: Value) -> Result<usize, CompileError> {
        if self.constants.len() >= MAX_CONSTANTS {
            return Err(CompileError::TooManyConstants);
        }

        self.constants.push(value);
        Ok(self.constants.len() - 1)
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        let line = self.lines[offset];

        if offset > 0 && line == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", line);
        }

        let instruction = self.code[offset];
        let opcode = OpCode::try_from(instruction);
        match opcode {
            Ok(OpCode::Constant) => constant_instruction("OP_CONSTANT", self, offset),
            Ok(OpCode::ConstantLong) => constant_long_instruction("OP_CONSTANT_LONG", self, offset),

            Ok(OpCode::Return) => simple_instruction("OP_RETURN", offset),

            Err(_) => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);

    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    let value = chunk.constants[constant as usize];
    println!("{:16} {:4} '{}'", name, constant, value);

    offset + 2
}

fn constant_long_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let high = chunk.code[offset + 1] as usize;
    let mid = chunk.code[offset + 2] as usize;
    let low = chunk.code[offset + 3] as usize;
    let constant = high << 16 | mid << 8 | low;
    let value = chunk.constants[constant];
    println!("{:16} {:8} '{}'", name, constant, value);

    offset + 4
}
