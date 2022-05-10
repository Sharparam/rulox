pub struct Chunk {
    code: Vec<u8>,
}

#[repr(u8)]
pub enum OpCode {
    Return = 0,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            _ => panic!("Unknown opcode {}", value),
        }
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_opcode(&mut self, opcode: OpCode) {
        self.write(opcode as u8);
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

        let instruction = self.code[offset];
        let opcode: OpCode = instruction.into();
        match opcode {
            Some(OpCode::Return) => {}

            None => {
                println!("Unknown opcode {}", instruction);
                return offset + 1;
            }
        }
    }
}
