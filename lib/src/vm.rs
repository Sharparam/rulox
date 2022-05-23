use std::io::{self, Write};

use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};
use thiserror::Error;
use tracing::error;

/// Errors that can occur when the VM executes.
#[derive(Error, Debug)]
pub enum VmError {
    #[error("Compilation error")]
    Compilation,

    #[error("Runtime error: {}", .0)]
    Runtime(#[from] RuntimeError),
}

/// Errors that can occur during runtime.
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Invalid opcode: {}", .0)]
    InvalidOpCode(u8),

    #[error("Attempt to pop value from empty stack")]
    PoppedEmptyStack,

    #[error("Type error")]
    TypeError,

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

    fn offset(&self) -> usize {
        self.offset
    }
}

pub struct VM<'a, O: Write, E: Write> {
    stack: Vec<Value>,
    out: &'a mut O,
    err: &'a mut E,
}

impl<'a, O: Write, E: Write> VM<'a, O, E> {
    pub fn new(out: &'a mut O, err: &'a mut E) -> Self {
        Self {
            stack: Vec::new(),
            out,
            err,
        }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.run(chunk)
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        macro_rules! binary_op {
            ($op:tt) => { {
                let right: f64 = self.pop_stack()?.try_into()?;
                let left: f64 = self.pop_stack()?.try_into()?;
                let result = left $op right;
                self.stack.push(result.into());
            } };
        }

        let mut ip = IP::new(chunk, 0);
        loop {
            #[cfg(feature = "trace")]
            {
                self.print_stack()?;
                chunk.disassemble_instruction(ip.offset());
            }

            let instruction = ip.read();
            let opcode = OpCode::try_from(instruction);
            match opcode {
                Ok(OpCode::Return) => {
                    let value = self.pop_stack()?;
                    writeln!(self.out, "{}", value)?;
                    return Ok(());
                }

                Ok(OpCode::Constant) => {
                    let value = ip.read_constant(false);
                    self.stack.push(value);
                }

                Ok(OpCode::ConstantLong) => {
                    let value = ip.read_constant(true);
                    self.stack.push(value);
                }

                Ok(OpCode::Add) => binary_op!(+),
                Ok(OpCode::Subtract) => binary_op!(-),
                Ok(OpCode::Multiply) => binary_op!(*),
                Ok(OpCode::Divide) => binary_op!(/),

                Ok(OpCode::Negate) => {
                    let value: f64 = self.pop_stack()?.try_into()?;
                    self.stack.push((-value).into());
                }

                Err(_) => {
                    error!("Invalid opcode: {:?}", opcode);
                    return Err(VmError::Runtime(RuntimeError::InvalidOpCode(instruction)));
                }
            }
        }
    }

    fn pop_stack(&mut self) -> Result<Value, VmError> {
        if let Some(value) = self.stack.pop() {
            Ok(value)
        } else {
            Err(VmError::Runtime(RuntimeError::PoppedEmptyStack))
        }
    }

    fn print_stack(&mut self) -> Result<(), io::Error> {
        write!(self.err, "          ")?;
        for value in &self.stack {
            write!(self.err, "[ {value} ]")?;
        }

        writeln!(self.err)?;

        Ok(())
    }
}
