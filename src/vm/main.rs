extern crate voom;

use voom::machine::{Machine, Memory};

const ROM: [u8; 4] = [
    0x80, 0x03, 0x15,   // set 0x0315 ; 789
    0x3F                // halt
];

pub fn main() {
    // Make memory and write ROM into it
    let mut mem = Memory::empty();
    for (n, byte) in ROM.iter().enumerate() {
        mem.write_byte(n as u16, *byte);
    }

    let mut vm = Machine::new(mem);
    vm.run();
    println!("{:?}", vm);
}
