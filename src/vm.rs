use crate::chunk::Chunk;
use anyhow::Result;

pub enum VmError {
    Compilation,

    Runtime,
}

pub type InterpretResult = Result<(), VmError>;

pub struct VM {
    // chunk: &'a Chunk,
    ip: usize,
}

impl VM {
    pub fn new() -> Self {
        Self { ip: 0 }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        // self.chunk = chunk;
        self.ip = 0;

        self.run(chunk)
    }

    fn run(&self, chunk: &Chunk) -> InterpretResult {
        Ok(())
    }
}
