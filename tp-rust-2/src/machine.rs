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
            Machine {mem : [0; MEMORY_SIZE], reg : [0; NREGS]}
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
        self.set_reg(IP, ip + 1);
        let nip : usize = ip.try_into().unwrap();
        let instr = &self.mem[nip..nip+3];
        let op1 = *instr.get(1).unwrap();
        let op2 = *instr.get(2).unwrap();
        let op3 = *instr.get(3).unwrap();
        match *instr.get(0).unwrap() {
            0 => self.moveif(op1, op2, op3),
            1 => self.store(op1, op2),
            2 => self.load(op1, op2),
            3 => self.loadimm(op1, op2, op3),
            4 => self.sub(op1, op2, op3),
            5 => Ok(println!("{}", op2)),
            6 => return Ok(true),
            7 => Ok(println!("{}", op2)),
        }
        Ok(false)
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

    pub fn moveif(&mut self, reg1 : u8, reg2 : u8, cond : u8) -> Result<(), MachineError> {
        Ok(())
    }

    pub fn store(&mut self, reg1 : u8, reg2 : u8) -> Result<(), MachineError> {
        unimplemented!()
    }

    pub fn load(&mut self, reg1 : u8, reg2 : u8) -> Result<(), MachineError> {
        unimplemented!()
    }

    pub fn loadimm(&mut self, reg1 : u8, l : u8, h : u8) -> Result<(), MachineError> {
        unimplemented!()
    }

    pub fn sub(&mut self, dest : u8, op1 : u8, op2 : u8) -> Result<(), MachineError> {
        unimplemented!()
    }
}
