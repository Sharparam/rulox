use crate::chunk::Chunk;
use anyhow::Result;

pub enum VmError {
    Compilation,

    Runtime,
}

struct VM<'a> {
    chunk: &'a Chunk,

    ip: usize,
}

impl<'a> VM<'a> {
    pub fn interpret(&mut self, chunk: &'a Chunk) -> Result<(), VmError> {
        self.chunk = chunk;
        self.ip = 0;

        Ok(())

        //self.run()
    }

    //pub fn run(&self) -> Result<(), VmError> {}
}
