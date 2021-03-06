# Voom

A simple virtual architecture.

## Memory

Memory is byte-addressable with 16-bit addresses.

## Registers

There are two 16-bit registers:

- Accumulator (<var>A</var>)
- Program counter (<var>P</var>)

## Status flags

- Halt (<var>h</var>)
- Test (<var>t</var>)
- Carry (<var>c</var>)
- Signed overflow (<var>o</var>)

## Instructions

Instructions are made up of a 1-byte opcode, and zero or one 2-byte operands.

| Opcode bits | Category                                        | Operands |
|:-----------:|:------------------------------------------------|:--------:|
| `00__ ____` | Implicit                                        | 0        |
| `01__ __xx` | Compact                                         | 0        |
| `1___ ___m` | Immediate (`m` = `0`) and direct (`m` = `1`)    | 1        |

Any unlisted opcodes are undefined and reserved for future use.

The key for symbols in the tables below:

| Symbol | Meaning      |
|:------:|:-------------|
| —      | Not affected |
| \*     | As described |

### Implicit

Unless it's affected by the instruction, <var>P</var> is increased by 1 after
each implicit instruction.

#### No-op

Mnemonic: `nop`<br>
Opcodes: `0x00`

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | — | — | — | — | — |

Does nothing.

#### Test carry

Mnemonic: `tc`<br>
Opcodes: `0x01`

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | — | — | c | — | — |

Checks if the carry flag is set.

#### Test signed overflow

Mnemonic: `to`<br>
Opcodes: `0x02`

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | — | — | o | — | — |

Checks if the signed overflow flag is set.

#### Invert test

Mnemonic: `inv`<br>
Opcodes: `0x03`

| A | P | h | t  | c | o |
|:-:|:-:|:-:|:--:|:-:|:-:|
| — | — | — | !t | — | — |

Inverts the test flag, working as a NOT operation on the previous test.

#### Bitwise NOT

Mnemonic: `not`<br>
Opcodes: `0x04`

| A  | P | h | t | c | o |
|:--:|:-:|:-:|:-:|:-:|:-:|
| !A | — | — | — | — | — |

Performs a bitwise NOT on the accumulator.

#### Negate

Mnemonic: `neg`<br>
Opcodes: `0x05`

| A  | P | h | t | c | o |
|:--:|:-:|:-:|:-:|:-:|:-:|
| −A | — | — | — | — | — |

Converts the accumulator to its two's complement negative.

#### Halt

Mnemonic: `halt`<br>
Opcodes: `0x3F`

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | — | 1 | — | — | — |

Set the halt flag, halting execution of the program.

### Compact

Compact instructions do not take a separate operand but stealthily hide an
"operand" in the last two bits of the opcode. This "operand" is generally
number of times that the action should be repeated (not including the first).

In the following descriptions <var>X</var> is the value of the last two bits
of the opcode plus 1. The `#` in the mnemonics refers to <var>X</var>, except
for when <var>X</var> 1, in which case it is blank (e.g. `inc` and `inc2`).

#### Increment

Mnemonic: `inc#`<br>
Opcodes: `0100 00xx` (`0x40`–`0x43`)

|   A   | P | h | t | c | o |
|:-----:|:-:|:-:|:-:|:-:|:-:|
| A + X | — | — | — | — | — |

Increments the accumulator by <var>X</var>.

#### Decrement

Mnemonic: `dec#`<br>
Opcodes: `0100 01xx` (`0x44`–`0x47`)

|   A   | P | h | t | c | o |
|:-----:|:-:|:-:|:-:|:-:|:-:|
| A − X | — | — | — | — | — |

Decrements the accumulator by <var>X</var>.

#### Skip

Mnemonic: `skip#`<br>
Opcodes: `0100 10xx` (`0x48`–`0x4B`)

| A |   P   | h | t | c | o |
|:-:|:-----:|:-:|:-:|:-:|:-:|
| — | P + X | — | — | — | — |

Increments the program counter by <var>X</var> + 1.

#### Conditional skip

Mnemonic: `cskip#`<br>
Opcodes: `0100 11xx` (`0x4C`–`0x4F`)

| A | P  | h | t | c | o |
|:-:|:--:|:-:|:-:|:-:|:-:|
| — | \* | — | 0 | — | — |

If the test flag is set, increments the program counter by <var>X</var> + 1
instructions and clears the test flag. Otherwise the program counter is
unaffected.

#### Bitwise shifts

Mnemonic: See description<br>
Opcodes: `011a aaxx` (`0x6C`–`0x7F`)

| A  | P | h | t | c  | o |
|:--:|:-:|:-:|:-:|:--:|:-:|
| \* | — | — | — | \* | — |

This is a family of instructions that perform a bitwise shift by <var>X</var>
on the accumulator.

| Mnemonic | Opcode | Description                                            |
|:--------:|:------:|:-------------------------------------------------------|
| `ls#`    | `0x6C` | Left shift, filling LSB with `0`                       |
| `lsc#`   | `0x70` | Left shift, filling LSB with carry flag                |
| `rsu#`   | `0x74` | Unsigned (logical) right shift, filling MSB with `0`   |
| `rsuc#`  | `0x78` | Unsigned right shfit, filling MSB with carry flag      |
| `rss#`   | `0x7C` | Signed (arithmetic) right shift, preserving MSB        |

The carry flag is set to the bit that gets shifted out of the accumulator.

### Immediate and Direct

In the following instructions, the value of <var>X</var> depends on its mode:

- Immediate: <var>X</var> is the two-byte operand following the instruction.
- Direct: The operand is a pointer to <var>X</var> in memory.

For instructions' direct mode, their mnemonics are appended with `m` (e.g.
immediate `add` and direct `addm`).

Unless it's affected by the instruction, <var>P</var> is increased by 3 after
each of the following instructions.

#### Set (immediate only)

Mnemonic: `set`<br>
Opcodes: `0x80`

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| X | — | — | — | — | — |

Sets the accumulator to the given value.

This instruction does not have a direct mode. The `load` instruction fulfils
that role.

#### Load

Mnemonic: `load`(`m`)<br>
Opcodes: `1000 001m` (`0x82`/`0x83`)

| A  | P | h | t | c | o |
|:--:|:-:|:-:|:-:|:-:|:-:|
| \* | — | — | — | — | — |

Loads the two bytes starting at the given address in memory into the
accumulator.

#### Store

Mnemonic: `store`(`m`)<br>
Opcodes: `1000 010m` (`0x84`/`0x85`)

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | — | — | — | — | — |

Stores the value in the accumulator in the two bytes starting at the given
address in memory.

#### Jump

Mnemonic: `jump`(`m`)<br>
Opcodes: `1000 011m` (`0x86`/`0x87`)

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | X | — | — | — | — |

Sets the program counter to the given value.

#### Conditional jump

Mnemonic: `cjump`(`m`)<br>
Opcodes: `1000 100m` (`0x88`/`0x89`)

| A | P  | h | t | c | o |
|:-:|:--:|:-:|:-:|:-:|:-:|
| — | \* | — | 0 | — | — |

If the test flag is set, jumps to the given location and clears the test
flag. Otherwise the program counter is unaffected.

#### Arithmetic operations

Mnemonic: See description<br>
Opcodes: `1001 0aam` (`0x90`–`0x97`)

| A  | P | h | t | c  | o  |
|:--:|:-:|:-:|:-:|:--:|:--:|
| \* | — | — | — | \* | \* |

This is a set of instructions that perform addition and subtraction on the
accumulator.

| Mnemonic    | Opcode | Description                                                |
|:-----------:|:------:|:-----------------------------------------------------------|
| `add`(`m`)  | `0x90` | <var>A</var> := <var>A</var> + <var>X</var>                |
| `addc`(`m`) | `0x92` | <var>A</var> := <var>A</var> + <var>X</var> + <var>c</var> |
| `sub`(`m`)  | `0x94` | <var>A</var> := <var>A</var> − <var>X</var>                |
| `subc`(`m`) | `0x96` | <var>A</var> := <var>A</var> − <var>X</var> − <var>c</var> |

In all cases, the carry flag is set if an unsigned overflow occurs while the
signed overflow flag is set if a signed overflow occurs.

#### Bitwise operations

Mnemonic: See description<br>
Opcodes: `1001 1aam` (`0x98`–`0x9D`)

| A  | P | h | t | c | o |
|:--:|:-:|:-:|:-:|:-:|:-:|
| \* | — | — | — | — | — |

This is a set of instructions that perform bitwise operations on the
accumulator.

| Mnemonic   | Opcode | Description                                    |
|:----------:|:------:|:-----------------------------------------------|
| `and`(`m`) | `0x98` | <var>A</var> := <var>A</var> AND <var>X</var>  |
| `or`(`m`)  | `0x9A` | <var>A</var> := <var>A</var> OR <var>X</var>   |
| `xor`(`m`) | `0x9C` | <var>A</var> := <var>A</var> XOR <var>X</var>  |

#### Comparisons

Mnemonic: See description<br>
Opcodes: `1010 aaam` (`0xA0`–`0xA9`)

| A | P | h | t  | c | o |
|:-:|:-:|:-:|:--:|:-:|:-:|
| — | — | — | \* | — | — |

This is a set of instructions that perform a comparison and set the test flag
if true.

| Mnemonic    | Opcode | Comparison                                 |
|:-----------:|:------:|:-------------------------------------------|
| `tgtu`(`m`) | `0xA0` | <var>A</var> > <var>X</var> (unsigned)     |
| `tgts`(`m`) | `0xA2` | <var>A</var> > <var>X</var> (signed)       |
| `tltu`(`m`) | `0xA4` | <var>A</var> \< <var>X</var> (unsigned)    |
| `tlts`(`m`) | `0xA6` | <var>A</var> \< <var>X</var> (signed)      |
| `teq`(`m`)  | `0xA8` | <var>A</var> = <var>X</var>                |
