# Voom

A simple virtual architecture.

## Memory

Memory is byte-addressable with 16-bit addresses.

## Registers

There are two 16-bit registers:

- Accumulator (_A_)
- Program counter (_P_)

## Status flags

- Halt (_h_)
- Test (_t_)
- Carry (_c_)
- Signed overflow (_o_)

## Instructions

Instructions are made up of a 1-byte opcode, and zero or one 2-byte operands. An
instruction that has no operand is known as **implicit**. All instructions with
operands (except `set`) have two modes:

- **Immediate:** the intruction operand is the relevant value
- **Direct:** the instruction operand points to the relevant value in memory

The mnemonic for the direct version of an instruction is append with `m` (e.g.
immediate `add` and direct `addm`). In the descriptions of instructions below,
the relevant value is referred to as _X_.

The `set` instruction only has an immediate mode since a `setm` would be
equivalent to `load`.

Instructions cause the program counter to increment counter unless they set the
program counter, by 1 if they're implicit and by 3 if they're immediate or
direct.


### Control

#### Implicit

| Mnemonic | Description        |
|:---------|:-------------------|
| `nop`    | Do nothing         |
| `halt`   | Set _h_            |
| `tc`     | Test if _c_ is set |
| `to`     | Test if _o_ is set |
| `nott`   | Invert _t_         |


#### Immediate/direct

| Mnemonic | Description                  |
|:---------|:-----------------------------|
| `jump`   | Set _P_ to _X_               |
| `branch` | Set _P_ to _X_ if _t_ is set |

### Memory interaction

#### Immediate

| Mnemonic | Description    |
|:---------|:---------------|
| `set`    | Set _A_ to _X_ |

#### Immediate/direct

| Mnemonic | Description                            |
|:---------|:---------------------------------------|
| `load`   | Load the given memory address into _A_ |
| `store`  | Store _A_ at the given address         |

### Arithmatic and logic

### Implicit

| Mnemonic | Description                                      |
|:---------|:-------------------------------------------------|
| `inc`    | Increment _A_                                    |
| `dec`    | Decrement _A_                                    |
| `not`    | Bitwise NOT _A_                                  |
| `neg`    | Two's complement negate _A_                      |
| `shl`    | Left shift _A_                                   |
| `shlc`   | Left shift _A_, using carry                      |
| `shru`   | Unsigned (logical) right shift _A_               |
| `shruc`  | Unsigned (logical) right shift _A_, using carry  |
| `shrs`   | Signed (arithmetic) right shift _A_              |
| `shrsc`  | Signed (arithmetic) right shift _A_, using carry |

### Immediate/direct

| Mnemonic | Description                                |
|:---------|:-------------------------------------------|
| `add`    | Add _X_ to _A_                             |
| `addc`   | Add _X_ to _A_, using carry                |
| `sub`    | Subtract _X_ from _A_                      |
| `subc`   | Subtract _X_ from _A_, using carry         |
| `and`    | Bitwise AND _X_ with _A_                   |
| `or`     | Bitwise OR _X_ with _A_                    |
| `xor`    | Bitwise XOR _X_ with _A_                   |
| `teq`    | Test if _A_ is equal to _X_                |
| `tgtu`   | Test if _A_ is greater than _X_ (unsigned) |
| `tgts`   | Test if _A_ is greater than _X_ (signed)   |
| `tltu`   | Test if _A_ is less than _X_ (unsigned)    |
| `tlts`   | Test if _A_ is less than _X_ (signed)      |
