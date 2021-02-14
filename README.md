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
| `1m__ ____` | Immediate (`m` = `0`) and direct (`m` = `1`)    | 1        |

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

**Mnemonic:** `nop`<br>
**Opcode:**

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | — | — | — | — | — |

Does nothing.

#### Halt

**Mnemonic:** `halt`<br>
**Opcode:**

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | — | 1 | — | — | — |

Set the halt flag, halting execution of the program.

#### Test carry

**Mnemonic:** `tc`<br>
**Opcode:**

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | — | — | c | — | — |

Checks if the carry flag is set.

#### Test signed overflow

**Mnemonic:** `to`<br>
**Opcode:**

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | — | — | o | — | — |

Checks if the signed overflow flag is set.

#### Invert test

**Mnemonic:** `inv`<br>
**Opcode:**

| A | P | h | t  | c | o |
|:-:|:-:|:-:|:--:|:-:|:-:|
| — | — | — | !t | — | — |

Inverts the test flag, working as a NOT operation on the previous test.

#### Increment

**Mnemonic:** `inc`<br>
**Opcode:**

|   A   | P | h | t | c | o |
|:-----:|:-:|:-:|:-:|:-:|:-:|
| A + 1 | — | — | — | — | — |

Increments the accumulator.

#### Decrement

**Mnemonic:** `dec`<br>
**Opcode:**

|   A   | P | h | t | c | o |
|:-----:|:-:|:-:|:-:|:-:|:-:|
| A − 1 | — | — | — | — | — |

Decrements the accumulator.

#### Bitwise NOT

**Mnemonic:** `not`<br>
**Opcode:**

| A  | P | h | t | c | o |
|:--:|:-:|:-:|:-:|:-:|:-:|
| !A | — | — | — | — | — |

Performs a bitwise NOT on the accumulator.

#### Negate

**Mnemonic:** `neg`<br>
**Opcode:**

| A  | P | h | t | c | o |
|:--:|:-:|:-:|:-:|:-:|:-:|
| −A | — | — | — | — | — |

Converts the accumulator to its two's complement negative.

#### Bitwise shifts

**Mnemonic:** See description<br>
**Opcode:** See description

| A  | P | h | t | c  | o |
|:--:|:-:|:-:|:-:|:--:|:-:|
| \* | — | — | — | \* | — |

This is a family of instructions that perform a bitwise shift on the
accumulator.

| Mnemonic | Opcode | Description                                            |
|:--------:|:------:|:-------------------------------------------------------|
| `ls`     |        | Left shift, filling LSB with `0`                       |
| `lsc`    |        | Left shift, filling LSB with carry flag                |
| `rsu`    |        | Unsigned (logical) right shift, filling MSB with `0`   |
| `rsuc`   |        | Unsigned right shfit, filling MSB with carry flag      |
| `rss`    |        | Signed (arithmetic) right shift, preserving MSB        |

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

**Mnemonic:** `set`<br>
**Opcode:**

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| X | — | — | — | — | — |

Sets the accumulator to the given value.

This instruction does not have a direct mode. The `load` instructions fulfils
that role.

#### Load

**Mnemonic:** `load`(`m`)<br>
**Opcode:**

| A  | P | h | t | c | o |
|:--:|:-:|:-:|:-:|:-:|:-:|
| \* | — | — | — | — | — |

Loads the two bytes starting at the given address in memory into the
accumulator.

#### Store

**Mnemonic:** `store`(`m`)<br>
**Opcode:**

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | — | — | — | — | — |

Stores the value in the accumulator in the two bytes starting at the given
address in memory.

#### Jump

**Mnemonic:** `jump`(`m`)<br>
**Opcode:**

| A | P | h | t | c | o |
|:-:|:-:|:-:|:-:|:-:|:-:|
| — | X | — | — | — | — |

Sets the program counter to the given value.

#### Conditional branch

**Mnemonic:** `branch`(`m`)<br>
**Opcode:**

| A | P  | h | t | c | o |
|:-:|:--:|:-:|:-:|:-:|:-:|
| — | \* | — | 0 | — | — |

If the test flag is set, jumps to the given location and clears the test
flag. Otherwise the program counter is unaffected.

#### Arithmetic operations

**Mnemonic:** See description<br>
**Opcode:** See description

| A  | P | h | t | c  | o  |
|:--:|:-:|:-:|:-:|:--:|:--:|
| \* | — | — | — | \* | \* |

This is a set of instructions that perform addition and subtraction on the
accumulator.

| Mnemonic    | Opcode | Description                                                |
|:-----------:|:------:|:-----------------------------------------------------------|
| `add`(`m`)  |        | <var>A</var> := <var>A</var> + <var>X</var>                |
| `addc`(`m`) |        | <var>A</var> := <var>A</var> + <var>X</var> + <var>c</var> |
| `sub`(`m`)  |        | <var>A</var> := <var>A</var> − <var>X</var>                |
| `subc`(`m`) |        | <var>A</var> := <var>A</var> − <var>X</var> − <var>c</var> |

In all cases, the carry flag is set if an unsigned overflow occurs while the
signed overflow flag is set if a signed overflow occurs.

#### Bitwise operations

**Mnemonic:** See description<br>
**Opcode:**

| A  | P | h | t | c | o |
|:--:|:-:|:-:|:-:|:-:|:-:|
| \* | — | — | — | — | — |

This is a set of instructions that perform bitwise operations on the
accumulator.

| Mnemonic   | Opcode | Description                                    |
|:----------:|:------:|:-----------------------------------------------|
| `and`(`m`) |        | <var>A</var> := <var>A</var> AND <var>X</var>  |
| `or`(`m`)  |        | <var>A</var> := <var>A</var> OR <var>X</var>   |
| `xor`(`m`) |        | <var>A</var> := <var>A</var> XOR <var>X</var>  |

#### Comparisons

**Mnemonic:** See description<br>
**Opcode:** See description

| A | P | h | t  | c | o |
|:-:|:-:|:-:|:--:|:-:|:-:|
| — | — | — | \* | — | — |

This is a set of instructions that perform a comparison and set the test flag
if true.

| Mnemonic    | Opcode | Comparison                                 |
|:-----------:|:------:|:-------------------------------------------|
| `teq`(`m`)  |        | <var>A</var> = <var>X</var>                |
| `tgtu`(`m`) |        | <var>A</var> > <var>X</var> (unsigned)     |
| `tgts`(`m`) |        | <var>A</var> > <var>X</var> (signed)       |
| `tltu`(`m`) |        | <var>A</var> \< <var>X</var> (unsigned)    |
| `tlts`(`m`) |        | <var>A</var> \< <var>X</var> (signed)      |
