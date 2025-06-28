# MIPS Instruction Set

This document details the comprehensive set of MIPS instructions supported by the VMIPS Simulator, including integer, floating-point, and system call instructions.

## Instruction Formats

MIPS instructions adhere to one of three primary formats:

-   **R-type**: `opcode (6) | rs (5) | rt (5) | rd (5) | shamt (5) | funct (6)`
    -   Used for register-to-register operations.
-   **I-type**: `opcode (6) | rs (5) | rt (5) | immediate (16)`
    -   Used for operations involving a register and a small immediate value, or for load/store instructions.
-   **J-type**: `opcode (6) | target (26)`
    -   Used for unconditional jumps.

## Supported Instructions

### Integer Arithmetic Operations

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `add`       | `add rd, rs, rt` | Add (with overflow) | `add $s0, $s1, $s2` |
| `addu`      | `addu rd, rs, rt` | Add unsigned (no overflow) | `addu $s0, $s1, $s2` |
| `addi`      | `addi rt, rs, imm` | Add immediate (with overflow) | `addi $t0, $s0, 100` |
| `addiu`     | `addiu rt, rs, imm` | Add immediate unsigned (no overflow) | `addiu $t0, $s0, 100` |
| `sub`       | `sub rd, rs, rt` | Subtract (with overflow) | `sub $s0, $s1, $s2` |
| `subu`      | `subu rd, rs, rt` | Subtract unsigned (no overflow) | `subu $s0, $s1, $s2` |
| `mult`      | `mult rs, rt` | Multiply (signed) | `mult $s0, $s1` |
| `multu`     | `multu rs, rt` | Multiply unsigned | `multu $s0, $s1` |
| `div`       | `div rs, rt` | Divide (signed) | `div $s0, $s1` |
| `divu`      | `divu rs, rt` | Divide unsigned | `divu $s0, $s1` |
| `mfhi`      | `mfhi rd` | Move from HI register | `mfhi $t0` |
| `mflo`      | `mflo rd` | Move from LO register | `mflo $t0` |
| `mthi`      | `mthi rs` | Move to HI register | `mthi $t0` |
| `mtlo`      | `mtlo rs` | Move to LO register | `mtlo $t0` |

### Logical Operations

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `and`       | `and rd, rs, rt` | Bitwise AND | `and $t0, $t1, $t2` |
| `andi`      | `andi rt, rs, imm` | Bitwise AND immediate | `andi $t0, $t1, 0xFF` |
| `or`        | `or rd, rs, rt` | Bitwise OR | `or $t0, $t1, $t2` |
| `ori`       | `ori rt, rs, imm` | Bitwise OR immediate | `ori $t0, $t1, 0xFF` |
| `xor`       | `xor rd, rs, rt` | Bitwise XOR | `xor $t0, $t1, $t2` |
| `xori`      | `xori rt, rs, imm` | Bitwise XOR immediate | `xori $t0, $t1, 0xFF` |
| `nor`       | `nor rd, rs, rt` | Bitwise NOR | `nor $t0, $t1, $t2` |

### Shift Operations

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `sll`       | `sll rd, rt, shamt` | Shift left logical | `sll $t0, $t1, 4` |
| `srl`       | `srl rd, rt, shamt` | Shift right logical | `srl $t0, $t1, 4` |
| `sra`       | `sra rd, rt, shamt` | Shift right arithmetic | `sra $t0, $t1, 4` |
| `sllv`      | `sllv rd, rt, rs` | Shift left logical variable | `sllv $t0, $t1, $t2` |
| `srlv`      | `srlv rd, rt, rs` | Shift right logical variable | `srlv $t0, $t1, $t2` |
| `srav`      | `srav rd, rt, rs` | Shift right arithmetic variable | `srav $t0, $t1, $t2` |

### Memory Access Operations

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `lw`        | `lw rt, offset(base)` | Load word | `lw $t0, 0($s0)` |
| `lh`        | `lh rt, offset(base)` | Load halfword (signed) | `lh $t0, 2($s0)` |
| `lb`        | `lb rt, offset(base)` | Load byte (signed) | `lb $t0, 4($s0)` |
| `lbu`       | `lbu rt, offset(base)` | Load byte unsigned | `lbu $t0, 5($s0)` |
| `lhu`       | `lhu rt, offset(base)` | Load halfword unsigned | `lhu $t0, 6($s0)` |
| `sw`        | `sw rt, offset(base)` | Store word | `sw $t0, 0($s0)` |
| `sh`        | `sh rt, offset(base)` | Store halfword | `sh $t0, 2($s0)` |
| `sb`        | `sb rt, offset(base)` | Store byte | `sb $t0, 4($s0)` |
| `lui`       | `lui rt, imm` | Load upper immediate | `lui $t0, 0xABCD` |

### Control Flow Operations

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `j`         | `j target` | Jump | `j my_label` |
| `jal`       | `jal target` | Jump and link | `jal my_function` |
| `jr`        | `jr rs` | Jump register | `jr $ra` |
| `jalr`      | `jalr rd, rs` | Jump and link register | `jalr $ra, $t9` |
| `beq`       | `beq rs, rt, offset` | Branch if equal | `beq $t0, $t1, loop_start` |
| `bne`       | `bne rs, rt, offset` | Branch if not equal | `bne $t0, $t1, loop_end` |
| `bgtz`      | `bgtz rs, offset` | Branch if greater than zero | `bgtz $t0, positive_val` |
| `blez`      | `blez rs, offset` | Branch if less than or equal to zero | `blez $t0, non_positive` |
| `bgez`      | `bgez rs, offset` | Branch if greater than or equal to zero | `bgez $t0, non_negative` |
| `bltz`      | `bltz rs, offset` | Branch if less than zero | `bltz $t0, negative_val` |

### Comparison Operations

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `slt`       | `slt rd, rs, rt` | Set if less than (signed) | `slt $t0, $t1, $t2` |
| `sltu`      | `sltu rd, rs, rt` | Set if less than unsigned | `sltu $t0, $t1, $t2` |
| `slti`      | `slti rt, rs, imm` | Set if less than immediate (signed) | `slti $t0, $t1, 50` |
| `sltiu`     | `sltiu rt, rs, imm` | Set if less than immediate unsigned | `sltiu $t0, $t1, 50` |

### Floating-Point Instructions (Coprocessor 1)

VMIPS Rust provides comprehensive support for MIPS floating-point operations, adhering to the IEEE 754 standard for single-precision (32-bit) floating-point numbers.

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `add.s`     | `add.s fd, fs, ft` | Floating-point add single | `add.s $f0, $f1, $f2` |
| `sub.s`     | `sub.s fd, fs, ft` | Floating-point subtract single | `sub.s $f0, $f1, $f2` |
| `mul.s`     | `mul.s fd, fs, ft` | Floating-point multiply single | `mul.s $f0, $f1, $f2` |
| `div.s`     | `div.s fd, fs, ft` | Floating-point divide single | `div.s $f0, $f1, $f2` |
| `abs.s`     | `abs.s fd, fs` | Floating-point absolute value single | `abs.s $f0, $f1` |
| `neg.s`     | `neg.s fd, fs` | Floating-point negate single | `neg.s $f0, $f1` |
| `mov.s`     | `mov.s fd, fs` | Floating-point move single | `mov.s $f0, $f1` |
| `cvt.s.w`   | `cvt.s.w fd, fs` | Convert word to single-precision float | `cvt.s.w $f0, $f1` |
| `cvt.w.s`   | `cvt.w.s fd, fs` | Convert single-precision float to word | `cvt.w.s $f0, $f1` |
| `c.eq.s`    | `c.eq.s fs, ft` | Compare equal single | `c.eq.s $f0, $f1` |
| `c.lt.s`    | `c.lt.s fs, ft` | Compare less than single | `c.lt.s $f0, $f1` |
| `c.le.s`    | `c.le.s fs, ft` | Compare less than or equal single | `c.le.s $f0, $f1` |
| `lwc1`      | `lwc1 ft, offset(base)` | Load word to Coprocessor 1 | `lwc1 $f0, 0($s0)` |
| `swc1`      | `swc1 ft, offset(base)` | Store word from Coprocessor 1 | `swc1 $f0, 0($s0)` |
| `bc1t`      | `bc1t offset` | Branch on Coprocessor 1 true | `bc1t fp_true` |
| `bc1f`      | `bc1f offset` | Branch on Coprocessor 1 false | `bc1f fp_false` |

### Special Instructions

| Instruction | Format | Description | Example |
|-------------|--------|-------------|---------|
| `nop`       | `nop` | No operation (assembler translates to `sll $0, $0, 0`) | `nop` |
| `syscall`   | `syscall` | System call (for OS services) | `syscall` |
| `break`     | `break code` | Software breakpoint | `break 0x1` |

## System Calls

The simulator supports a subset of MIPS system calls, primarily for basic I/O and program control. The system call number is placed in register `$v0` (`$2`), and arguments are passed in registers `$a0-$a3` (`$4-$7`). Results are returned in `$v0`.

| Call # (`$v0`) | Service | Arguments | Result (`$v0`) |
|----------------|---------|-----------|----------------|
| 1              | Print integer | `$a0` = integer to print | - |
| 4              | Print string | `$a0` = address of null-terminated string | - |
| 5              | Read integer | - | Integer read |
| 8              | Read string | `$a0` = buffer address, `$a1` = buffer length | - |
| 10             | Exit | - | Program terminates |
| 11             | Print character | `$a0` = character to print | - |
| 12             | Read character | - | Character read |

## Common Pseudo-Instructions

These are convenient instructions that the assembler translates into one or more actual MIPS machine instructions. They simplify assembly programming.

| Pseudo-Instruction | Expansion (Example) | Description | Example |
|--------------------|---------------------|-------------|---------|
| `move rd, rs`      | `addu rd, rs, $zero` | Move register value | `move $t0, $s0` |
| `li rt, imm`       | `lui rt, imm_hi` then `ori rt, rt, imm_lo` | Load immediate (32-bit) | `li $t0, 0x12345678` |
| `la rt, label`     | `lui rt, upper(label)` then `ori rt, rt, lower(label)` | Load address of label | `la $t0, my_data` |
| `b label`          | `beq $zero, $zero, label` | Unconditional branch | `b loop_start` |

## Register Conventions

Adhering to MIPS register conventions is crucial for writing portable and maintainable assembly code, especially when interfacing with compiled C/C++ code or operating system services.

| Register Number | Name    | Usage                                    |
|-----------------|---------|------------------------------------------|
| $0              | $zero   | Always contains the value 0              |
| $1              | $at     | Assembler temporary (reserved for assembler) |
| $2-$3           | $v0-$v1 | Function return values                   |
| $4-$7           | $a0-$a3 | Function arguments                       |
| $8-$15          | $t0-$t7 | Temporary registers (not preserved across calls) |
| $16-$23         | $s0-$s7 | Saved registers (preserved across calls) |
| $24-$25         | $t8-$t9 | More temporary registers                 |
| $26-$27         | $k0-$k1 | Reserved for OS kernel                   |
| $28             | $gp     | Global pointer                           |
| $29             | $sp     | Stack pointer                            |
| $30             | $fp     | Frame pointer (or $s8)                   |
| $31             | $ra     | Return address                           |

**Floating-Point Registers (`$f0-$f31`)**:

-   Used for floating-point operations. Specific conventions for argument passing and return values exist but are not strictly enforced by the simulator.

For more detailed information on MIPS architecture and instruction set, refer to standard MIPS documentation.
