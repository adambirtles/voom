pub enum ImplicitOp {
    NoOp,
    TestCarry,
    TestOverflow,
    InvertTest,
    BitwiseNot,
    Negate,
    Halt,
}

impl ImplicitOp {
    pub fn from_opcode(opcode: u8) -> Option<ImplicitOp> {
        use ImplicitOp::*;
        match opcode {
            0x00 => Some(NoOp),
            0x01 => Some(TestCarry),
            0x02 => Some(TestOverflow),
            0x03 => Some(InvertTest),
            0x04 => Some(BitwiseNot),
            0x05 => Some(Negate),
            0x3F => Some(Halt),
            _ => None,
        }
    }
}

pub enum CompactOp {
    Increment,
    Decrement,
    Skip,
    CondSkip,
    LeftShift,
    LeftShiftC,
    RightShiftUnsigned,
    RightShiftUnsignedC,
    RightShiftSigned,
}

impl CompactOp {
    pub fn from_opcode(opcode: u8) -> Option<CompactOp> {
        use CompactOp::*;
        match opcode & 0b1111_1100 {
            0x40 => Some(Increment),
            0x44 => Some(Decrement),
            0x48 => Some(Skip),
            0x4C => Some(CondSkip),
            0x6C => Some(LeftShift),
            0x70 => Some(LeftShiftC),
            0x74 => Some(RightShiftUnsigned),
            0x78 => Some(RightShiftUnsignedC),
            0x7C => Some(RightShiftSigned),
            _ => None,
        }
    }
}

pub enum OperandedOp {
    Set,
    Load,
    Store,
    Jump,
    CondJump,
    Add(bool),
    Subtract(bool),
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    TestGtUnsigned,
    TestGtSigned,
    TestLtUnsigned,
    TestLtSigned,
    TestEqual,
}

impl OperandedOp {
    pub fn from_opcode(opcode: u8) -> Option<OperandedOp> {
        use OperandedOp::*;

        if opcode == 0x81 {
            // There is no `setm`
            return None;
        }

        match opcode & 0b1111_1110 {
            0x80 => Some(Set),
            0x82 => Some(Load),
            0x84 => Some(Store),
            0x86 => Some(Jump),
            0x88 => Some(CondJump),
            0x90 => Some(Add(false)),
            0x92 => Some(Add(true)),
            0x94 => Some(Subtract(false)),
            0x96 => Some(Subtract(true)),
            0x98 => Some(BitwiseAnd),
            0x9A => Some(BitwiseOr),
            0x9C => Some(BitwiseXor),
            0xA0 => Some(TestGtUnsigned),
            0xA2 => Some(TestGtSigned),
            0xA4 => Some(TestLtUnsigned),
            0xA6 => Some(TestLtSigned),
            0xA8 => Some(TestEqual),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum OperandMode {
    Immediate,
    Direct,
}

pub enum Opcode {
    Implicit(ImplicitOp),
    Compact(CompactOp, u8),
    Operanded(OperandedOp, OperandMode),
}

impl Opcode {
    pub fn decode(opcode: u8) -> Option<Opcode> {
        use Opcode::*;

        Some(if opcode & 0b1100_0000 == 0b0000_0000 {
            Implicit(ImplicitOp::from_opcode(opcode)?)
        } else if opcode & 0b1100_0000 == 0b0100_0000 {
            Compact(CompactOp::from_opcode(opcode)?, opcode & 0b0000_0011)
        } else {
            let mode = if opcode & 0b0000_0001 == 0 {
                OperandMode::Immediate
            } else {
                OperandMode::Direct
            };

            Operanded(OperandedOp::from_opcode(opcode)?, mode)
        })
    }
}
