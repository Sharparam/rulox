use std::io::{self, Write};

use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum VmError {
    #[error("Compilation error")]
    Compilation,

    #[error("Runtime error: {}", .0)]
    Runtime(#[from] RuntimeError),
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Invalid opcode: {}", .0)]
    InvalidOpCode(u8),

    #[error("Input/Output failure")]
    Io(#[from] io::Error),
}

pub type InterpretResult = Result<(), VmError>;

impl From<io::Error> for VmError {
    fn from(error: io::Error) -> Self {
        Self::Runtime(RuntimeError::Io(error))
    }
}

struct IP<'a> {
    chunk: &'a Chunk,
    offset: usize,
}

impl<'a> IP<'a> {
    fn new(chunk: &'a Chunk, offset: usize) -> Self {
        Self { chunk, offset }
    }

    fn read(&mut self) -> u8 {
        let value = self.chunk.read(self.offset);
        self.offset += 1;
        value
    }

    fn read_constant(&mut self, long: bool) -> Value {
        let index = if long {
            let high = self.read() as usize;
            let mid = self.read() as usize;
            let low = self.read() as usize;
            (high << 16) | (mid << 8) | low
        } else {
            self.read() as usize
        };

        self.chunk.constants[index]
    }
}

pub struct VM<'a, O: Write, E: Write> {
    out: &'a mut O,
    err: &'a mut E,
}

impl<'a, O: Write, E: Write> VM<'a, O, E> {
    pub fn new(out: &'a mut O, err: &'a mut E) -> Self {
        Self { out, err }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.run(chunk)
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        let mut ip = IP::new(chunk, 0);
        loop {
            let instruction = ip.read();
            let opcode = OpCode::try_from(instruction);
            match opcode {
                Ok(OpCode::Return) => {
                    return Ok(());
                }

                Ok(OpCode::Constant) => {
                    let value = ip.read_constant(false);
                    writeln!(self.out, "CONSTANT: {:?}", value)?;
                }

                Ok(OpCode::ConstantLong) => {
                    let value = ip.read_constant(true);
                    writeln!(self.out, "CONSTANT (LONG): {:?}", value)?;
                }

                Err(_) => {
                    error!("Invalid opcode: {:?}", opcode);
                    return Err(VmError::Runtime(RuntimeError::InvalidOpCode(instruction)));
                }
            }
        }
    }
}
