# MIPS Instruction Set

This document describes the MIPS instructions supported by the VMIPS Simulator.

## Instruction Format

MIPS instructions follow one of these formats:

- **R-type**: `opcode (6) | rs (5) | rt (5) | rd (5) | shamt (5) | funct (6)`
- **I-type**: `opcode (6) | rs (5) | rt (5) | immediate (16)`
- **J-type**: `opcode (6) | target (26)`

## Supported Instructions

### Arithmetic Operations

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `add` | `add rd, rs, rt` | Add | `add $1, $2, $3` |
| `addi` | `addi rt, rs, imm` | Add immediate | `addi $1, $2, 100` |
| `addiu` | `addiu rt, rs, imm` | Add immediate unsigned | `addiu $1, $2, 100` |
| `sub` | `sub rd, rs, rt` | Subtract | `sub $1, $2, $3` |
| `mult` | `mult rs, rt` | Multiply | `mult $2, $3` |
| `div` | `div rs, rt` | Divide | `div $2, $3` |
| `mfhi` | `mfhi rd` | Move from HI register | `mfhi $1` |
| `mflo` | `mflo rd` | Move from LO register | `mflo $1` |

### Logical Operations

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `and` | `and rd, rs, rt` | Bitwise AND | `and $1, $2, $3` |
| `andi` | `andi rt, rs, imm` | Bitwise AND immediate | `andi $1, $2, 0xFF` |
| `or` | `or rd, rs, rt` | Bitwise OR | `or $1, $2, $3` |
| `ori` | `ori rt, rs, imm` | Bitwise OR immediate | `ori $1, $2, 0xFF` |
| `xor` | `xor rd, rs, rt` | Bitwise XOR | `xor $1, $2, $3` |
| `xori` | `xori rt, rs, imm` | Bitwise XOR immediate | `xori $1, $2, 0xFF` |
| `nor` | `nor rd, rs, rt` | Bitwise NOR | `nor $1, $2, $3` |

### Shift Operations

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `sll` | `sll rd, rt, shamt` | Shift left logical | `sll $1, $2, 3` |
| `srl` | `srl rd, rt, shamt` | Shift right logical | `srl $1, $2, 3` |
| `sra` | `sra rd, rt, shamt` | Shift right arithmetic | `sra $1, $2, 3` |
| `sllv` | `sllv rd, rt, rs` | Shift left logical variable | `sllv $1, $2, $3` |
| `srlv` | `srlv rd, rt, rs` | Shift right logical variable | `srlv $1, $2, $3` |
| `srav` | `srav rd, rt, rs` | Shift right arithmetic variable | `srav $1, $2, $3` |

### Memory Operations

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `lw` | `lw rt, offset(base)` | Load word | `lw $1, 0($2)` |
| `lh` | `lh rt, offset(base)` | Load halfword | `lh $1, 0($2)` |
| `lb` | `lb rt, offset(base)` | Load byte | `lb $1, 0($2)` |
| `lbu` | `lbu rt, offset(base)` | Load byte unsigned | `lbu $1, 0($2)` |
| `lhu` | `lhu rt, offset(base)` | Load halfword unsigned | `lhu $1, 0($2)` |
| `sw` | `sw rt, offset(base)` | Store word | `sw $1, 0($2)` |
| `sh` | `sh rt, offset(base)` | Store halfword | `sh $1, 0($2)` |
| `sb` | `sb rt, offset(base)` | Store byte | `sb $1, 0($2)` |
| `lui` | `lui rt, imm` | Load upper immediate | `lui $1, 0x1000` |

### Control Flow

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `j` | `j target` | Jump | `j label` |
| `jal` | `jal target` | Jump and link | `jal function` |
| `jr` | `jr rs` | Jump register | `jr $31` |
| `jalr` | `jalr rd, rs` | Jump and link register | `jalr $31, $2` |
| `beq` | `beq rs, rt, offset` | Branch if equal | `beq $1, $2, label` |
| `bne` | `bne rs, rt, offset` | Branch if not equal | `bne $1, $2, label` |
| `bgtz` | `bgtz rs, offset` | Branch if greater than zero | `bgtz $1, label` |
| `bgez` | `bgez rs, offset` | Branch if greater than or equal to zero | `bgez $1, label` |
| `bltz` | `bltz rs, offset` | Branch if less than zero | `bltz $1, label` |
| `blez` | `blez rs, offset` | Branch if less than or equal to zero | `blez $1, label` |

### Comparison

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `slt` | `slt rd, rs, rt` | Set if less than | `slt $1, $2, $3` |
| `slti` | `slti rt, rs, imm` | Set if less than immediate | `slti $1, $2, 100` |
| `sltiu` | `sltiu rt, rs, imm` | Set if less than immediate unsigned | `sltiu $1, $2, 100` |

### Special Instructions

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `nop` | `nop` | No operation | `nop` |
| `syscall` | `syscall` | System call | `syscall` |
| `break` | `break code` | Breakpoint | `break 0` |

### Floating-Point Instructions (Optional)

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `add.s` | `add.s fd, fs, ft` | FP add single | `add.s $f0, $f1, $f2` |
| `sub.s` | `sub.s fd, fs, ft` | FP subtract single | `sub.s $f0, $f1, $f2` |
| `mul.s` | `mul.s fd, fs, ft` | FP multiply single | `mul.s $f0, $f1, $f2` |
| `div.s` | `div.s fd, fs, ft` | FP divide single | `div.s $f0, $f1, $f2` |
| `lwc1` | `lwc1 ft, offset(base)` | Load FP single | `lwc1 $f0, 0($1)` |
| `swc1` | `swc1 ft, offset(base)` | Store FP single | `swc1 $f0, 0($1)` |

## System Calls

The simulator supports system calls via the `syscall` instruction. The system call number is placed in register `$v0` ($2), and arguments are placed in registers `$a0-$a3` ($4-$7).

| Call # | Service | Arguments | Result |
|--------|---------|-----------|--------|
| 1 | Print integer | $a0 = integer to print | - |
| 4 | Print string | $a0 = address of string | - |
| 5 | Read integer | - | $v0 = integer read |
| 8 | Read string | $a0 = buffer, $a1 = length | - |
| 10 | Exit | - | Program terminates |
| 11 | Print character | $a0 = character to print | - |
| 12 | Read character | - | $v0 = character read |

## Common Pseudo-Instructions

These are translated by the assembler into one or more actual instructions:

| Pseudo-Instruction | Expansion | Description | Example |
|-------------------|-----------|-------------|---------|
| `move rd, rs` | `add rd, rs, $0` | Move register value | `move $1, $2` |
| `li rt, imm` | `lui rt, imm_hi` + `ori rt, rt, imm_lo` | Load immediate | `li $1, 0x12345678` |
| `la rt, label` | Address calculation | Load address | `la $1, var` |
| `b label` | `beq $0, $0, label` | Unconditional branch | `b loop` |

## Register Conventions

| Register Number | Name | Usage |
|----------------|------|-------|
| $0 | $zero | Always contains 0 |
| $1 | $at | Assembler temporary |
| $2-$3 | $v0-$v1 | Function return values |
| $4-$7 | $a0-$a3 | Function arguments |
| $8-$15 | $t0-$t7 | Temporary registers |
| $16-$23 | $s0-$s7 | Saved registers |
| $24-$25 | $t8-$t9 | More temporary registers |
| $26-$27 | $k0-$k1 | Reserved for OS kernel |
| $28 | $gp | Global pointer |
| $29 | $sp | Stack pointer |
| $30 | $fp | Frame pointer |
| $31 | $ra | Return address |
