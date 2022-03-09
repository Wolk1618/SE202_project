use std::io::{self, Write};

const MEMORY_SIZE: usize = 4096;
const NREGS: usize = 16;

const IP: usize = 0;

pub struct Machine {
    mem : [u8; MEMORY_SIZE],
    reg : [u32; NREGS],
}

#[derive(Debug)]
pub enum MachineError {
    TooBigSize,
    BadRegisterName,
}

impl Machine {
    /// Create a new machine in its reset state. The `memory` parameter will
    /// be copied at the beginning of the machine memory.
    ///
    /// # Panics
    /// This function panics when `memory` is larger than the machine memory.
    pub fn new(memory: &[u8]) -> Self {
        if memory.len() > MEMORY_SIZE {
            panic!("Memory given is too big !");
        } else {
            Machine {mem : memory.try_into().unwrap(), reg : [0; NREGS]}
        }
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on `fd`.
    pub fn run_on<T: Write>(&mut self, fd: &mut T) -> Result<(), MachineError> {
        while !self.step_on(fd)? {}
        Ok(())
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on standard output.
    pub fn run(&mut self) -> Result<(), MachineError> {
        self.run_on(&mut io::stdout().lock())
    }

    /// Execute the next instruction by doing the following steps:
    ///   - decode the instruction located at IP (register 0)
    ///   - increment the IP by the size of the instruction
    ///   - execute the decoded instruction
    ///
    /// If output instructions are run, they print on `fd`.
    /// If an error happens at either of those steps, an error is
    /// returned.
    ///
    /// In case of success, `true` is returned if the program is
    /// terminated (upon encountering an exit instruction), or
    /// `false` if the execution must continue.
    pub fn step_on<T: Write>(&mut self, fd: &mut T) -> Result<bool, MachineError> {
        let ip = self.reg[IP];
        self.set_reg(IP, ip + 1).unwrap();
        let nip : usize = ip.try_into().unwrap();
        let instr = &self.mem[nip..=nip+3];
        let op1 = *instr.get(1).unwrap();
        let op2 = *instr.get(2).unwrap();
        let op3 = *instr.get(3).unwrap();
        match *instr.get(0).unwrap() {
            0 => self.moveif(op1.try_into().unwrap(), op2.try_into().unwrap(), op3),
            1 => self.store(op1.try_into().unwrap(), op2.try_into().unwrap()),
            2 => self.load(op1.try_into().unwrap(), op2.try_into().unwrap()),
            3 => self.loadimm(op1, op2, op3),
            4 => self.sub(op1.try_into().unwrap(), op2.try_into().unwrap(), op3.try_into().unwrap()),
            5 => {
                write!(fd, "{}", op2).unwrap();
                return Ok(false);
            },
            6 => return Ok(true),
            7 => {
                format!("{}", op2);
                return Ok(false);
            },
            _ => panic!("Bad number"),
        }
    }

    /// Similar to [step_on](Machine::step_on).
    /// If output instructions are run, they print on standard output.
    pub fn step(&mut self) -> Result<bool, MachineError> {
        self.step_on(&mut io::stdout().lock())
    }

    /// Reference onto the machine current set of registers.
    pub fn regs(&self) -> &[u32] {
        &self.reg
    }

    /// Sets a register to the given value.
    pub fn set_reg(&mut self, reg: usize, value: u32) -> Result<(), MachineError> {
        if reg > NREGS {
            return Err(MachineError::TooBigSize);
        }
        self.reg[reg] = value;
        Ok(())
    }

    /// Reference onto the machine current memory.
    pub fn memory(&self) -> &[u8] {
        &self.mem
    }

    pub fn moveif(&mut self, reg1 : usize, reg2 : usize, cond : u8) -> Result<bool, MachineError> {
        if cond != 0 {
            if (reg1 < NREGS) & (reg2 < NREGS) {
                self.set_reg(reg1, self.reg[reg2]).unwrap();
                Ok(false)
            } else {
                Err(MachineError::BadRegisterName)
            }
        } else {
            Ok(false)
        }
    }

    pub fn store(&mut self, reg1 : usize, reg2 : usize) -> Result<bool, MachineError> {
        if (reg1 < NREGS) & (reg2 < NREGS) {
            let content : &[u8] = &self.reg[reg2].to_le_bytes();
            let adr : usize = self.reg[reg1].try_into().unwrap();
            self.mem[adr] = content[0];
            self.mem[adr+1] = content[1];
            self.mem[adr+2] = content[2];
            self.mem[adr+3] = content[3];
            Ok(false)
        } else {
            Err(MachineError::BadRegisterName)
        }
    }

    pub fn load(&mut self, reg1 : usize, reg2 : usize) -> Result<bool, MachineError> {
        if (reg1 < NREGS) & (reg2 < NREGS) {
            let adr : usize = self.reg[reg2].try_into().unwrap();
            let content = &self.mem[adr..=adr+3];
            //self.set_reg(reg2, content.try_into().unwrap());
            Ok(false)
        } else {
            Err(MachineError::BadRegisterName)
        }
    }

    pub fn loadimm(&mut self, reg1 : u8, l : u8, h : u8) -> Result<bool, MachineError> {
        unimplemented!()
    }

    pub fn sub(&mut self, dest : usize, op1 : usize, op2 : usize) -> Result<bool, MachineError> {
        if (op1 < NREGS) & (op2 < NREGS) & (dest < NREGS) {
            let result = self.reg[op1] - self.reg[op2];
            self.set_reg(dest, result).unwrap();
            Ok(false)
        } else {
            Err(MachineError::BadRegisterName)
        }
    }
}
