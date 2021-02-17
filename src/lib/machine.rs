use std::fmt;

use crate::instruction::{CompactOp, ImplicitOp, Opcode, OperandMode, OperandedOp};

pub struct Memory([u8; Memory::SIZE]);

impl Memory {
    const SIZE: usize = u16::MAX as usize;

    pub fn empty() -> Memory {
        Memory([0; Memory::SIZE])
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.0[addr as usize] = value;
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        u16::from_be_bytes([self.0[addr as usize], self.0[addr as usize + 1]])
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        let bytes = value.to_be_bytes();
        self.0[addr as usize] = bytes[0];
        self.0[addr as usize + 1] = bytes[1];
    }
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Memory([...])")
    }
}

#[derive(Debug)]
pub struct Machine {
    memory: Memory,
    accumulator: u16,
    program_counter: u16,
    flag_halt: bool,
    flag_test: bool,
    flag_carry: bool,
    flag_overflow: bool,
}

impl Machine {
    pub fn new(memory: Memory) -> Machine {
        Machine {
            memory,
            accumulator: 0,
            program_counter: 0,
            flag_halt: false,
            flag_test: false,
            flag_carry: false,
            flag_overflow: false,
        }
    }

    pub fn run(&mut self) {
        while !self.flag_halt {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        let opcode_byte = self.memory.read_byte(self.program_counter);
        let opcode = Opcode::decode(opcode_byte).expect("encounted undefined opcode");

        self.program_counter += 1;

        match opcode {
            Opcode::Implicit(op) => self.do_implicit(op),
            Opcode::Compact(op, operand) => self.do_compact(op, operand),
            Opcode::Operanded(op, mode) => self.do_operanded(op, mode),
        }
    }

    fn do_implicit(&mut self, op: ImplicitOp) {
        use ImplicitOp::*;
        match op {
            NoOp => {}
            TestCarry => self.flag_test = self.flag_carry,
            TestOverflow => self.flag_test = self.flag_overflow,
            InvertTest => self.flag_test = !self.flag_test,
            BitwiseNot => self.accumulator = !self.accumulator,
            Negate => self.accumulator = -(self.accumulator as i16) as u16,
            Halt => self.flag_halt = true,
        }
    }

    fn do_compact(&mut self, op: CompactOp, operand: u8) {
        use CompactOp::*;

        let x = operand as u16 + 1;
        match op {
            Increment => self.accumulator += x,
            Decrement => self.accumulator -= x,
            Skip => self.program_counter += x,
            CondSkip => {
                if self.flag_test {
                    self.program_counter += x;
                    self.flag_test = false;
                }
            }
            LeftShift => self.accumulator <<= x,
            LeftShiftC => {
                self.accumulator <<= 1;
                self.accumulator |= if self.flag_carry { 1 } else { 0 };
                self.accumulator <<= x - 1;
            }
            RightShiftUnsigned => self.accumulator >>= x,
            RightShiftUnsignedC => {
                self.accumulator >>= 1;
                self.accumulator |= if self.flag_carry { 1 << 15 } else { 0 };
                self.accumulator >>= x - 1;
            }
            RightShiftSigned => {
                self.accumulator = (self.accumulator as i16 >> x) as u16;
            }
        }
    }

    fn do_operanded(&mut self, op: OperandedOp, mode: OperandMode) {
        use OperandedOp::*;

        let operand = self.memory.read_word(self.program_counter);
        self.program_counter += 2;

        let x = match mode {
            OperandMode::Immediate => operand,
            OperandMode::Direct => self.memory.read_word(operand),
        };

        match op {
            Set => self.accumulator = x,
            Load => self.accumulator = self.memory.read_word(x),
            Store => self.memory.write_word(x, self.accumulator),
            Jump => self.program_counter = x,
            CondJump => {
                if self.flag_test {
                    self.program_counter = x;
                    self.flag_test = false;
                }
            }
            Add(use_carry) => {
                let carry = if use_carry && self.flag_carry { 1 } else { 0 };

                let (result, carried) = self.accumulator.overflowing_add(x + carry);

                // TODO: work this out better
                let (_, overflowed) =
                    (self.accumulator as i16).overflowing_add(x as i16 + carry as i16);

                self.accumulator = result;
                self.flag_carry = carried;
                self.flag_overflow = overflowed;
            }
            Subtract(use_carry) => {
                let carry = if use_carry && self.flag_carry { 1 } else { 0 };

                let (result, carried) = self.accumulator.overflowing_sub(x - carry);

                // TODO: work this out better
                let (_, overflowed) =
                    (self.accumulator as i16).overflowing_sub(x as i16 - carry as i16);

                self.accumulator = result;
                self.flag_carry = carried;
                self.flag_overflow = overflowed;
            }
            BitwiseAnd => self.accumulator &= x,
            BitwiseOr => self.accumulator |= x,
            BitwiseXor => self.accumulator ^= x,
            TestGtUnsigned => self.flag_test = self.accumulator > x,
            TestGtSigned => self.flag_test = (self.accumulator as i16) > (x as i16),
            TestLtUnsigned => self.flag_test = self.accumulator < x,
            TestLtSigned => self.flag_test = (self.accumulator as i16) < (x as i16),
            TestEqual => self.flag_test = self.accumulator == x,
        }
    }
}
