include!("flags6502.rs");

pub struct CPU{
    pub status: u8,
    pub register_a: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self{
        CPU {
            status: 0,
            register_a: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>){
        self.program_counter = 0;

        loop{
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.register_a = param;
                    SetFlag(&mut self.status, Flags6502::Z, self.register_a == 0);
                    SetFlag(&mut self.status, Flags6502::N, self.register_a & 0x80 != 0x00);
                }
                _ => todo!()
            }
        }
    }
}

