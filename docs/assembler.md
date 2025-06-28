# MIPS Assembler

The VMIPS Rust simulator includes a two-pass MIPS assembler that converts human-readable assembly code into machine code (binary executables) that can be run on the functional or timing simulators.

## Overview

The assembler is designed to be straightforward and supports a wide range of MIPS instructions, directives, and common pseudo-instructions. It performs two passes over the assembly code:

1.  **First Pass**: Scans the assembly file to identify and record all labels and their corresponding memory addresses. This builds the symbol table.
2.  **Second Pass**: Uses the symbol table generated in the first pass to translate instructions and directives into their final machine code representation, resolving all forward references.

## Features

-   **Two-Pass Assembly**: Correctly handles forward references to labels.
-   **Instruction Support**: Translates all MIPS instructions supported by the VMIPS simulator (integer, floating-point, control flow, etc.).
-   **Directives**: Supports common assembler directives for data and section management.
-   **Pseudo-Instructions**: Expands common pseudo-instructions into one or more native MIPS instructions for convenience.
-   **Error Reporting**: Provides detailed error messages with line numbers for syntax errors, undefined symbols, and out-of-range immediate values.

## Usage

### Running the Assembler

You can run the assembler using the `main_assembler` binary:

```bash
cargo run --bin main_assembler assemble <input_assembly_file.s> <output_binary_file.bin>
```

**Example:**

```bash
cargo run --bin main_assembler assemble examples/bubble_sort.s bubble_sort.bin
```

This command will read `examples/bubble_sort.s`, assemble it, and save the resulting binary to `bubble_sort.bin`.

### Assembler Header Format

The assembler generates a binary file with a small 8-byte header, followed by the data section and then the text section. This header is used by the simulators to correctly load the program into memory.

| Offset | Size (bytes) | Description |
|--------|--------------|-------------|
| 0      | 4            | Size of the data section (in bytes) |
| 4      | 4            | Size of the text section (in bytes) |
| 8      | Data Size    | Data Section (initialized data) |
| 8 + Data Size | Text Size | Text Section (machine code instructions) |

## Supported Directives

Assembler directives control the placement of code and data in memory, and define data values.

| Directive   | Description                                     | Example                               |
|-------------|-------------------------------------------------|---------------------------------------|
| `.data`     | Marks the beginning of the data segment.        | `.data`                               |
| `.text`     | Marks the beginning of the text (code) segment. | `.text`                               |
| `.word`     | Allocates and initializes 4-byte words.         | `my_var: .word 10, 20, 30`            |
| `.byte`     | Allocates and initializes 1-byte bytes.         | `my_bytes: .byte 0x0A, 0x0B`          |
| `.half`     | Allocates and initializes 2-byte halfwords.     | `my_half: .half 0x1234`               |
| `.ascii`    | Stores an ASCII string (without null terminator). | `my_str: .ascii "Hello"`              |
| `.asciiz`   | Stores a null-terminated ASCII string.          | `my_zstr: .asciiz "World"`            |
| `.space n`  | Allocates `n` bytes of uninitialized space.     | `buffer: .space 100`                  |
| `.align n`  | Aligns the next data/instruction to `2^n` byte boundary. | `.align 2` (aligns to 4-byte boundary) |

## Pseudo-Instructions

The assembler supports several pseudo-instructions that are expanded into one or more native MIPS instructions. These simplify assembly programming.

| Pseudo-Instruction | Description                                     | Example             | Expansion (Conceptual)                               |
|--------------------|-------------------------------------------------|---------------------|------------------------------------------------------|
| `move rd, rs`      | Move value from `rs` to `rd`.                   | `move $t0, $s0`     | `addu $t0, $s0, $zero`                               |
| `li rt, imm`       | Load 32-bit immediate value into `rt`.          | `li $t0, 0x12345678`| `lui $t0, 0x1234` then `ori $t0, $t0, 0x5678`        |
| `la rt, label`     | Load 32-bit address of `label` into `rt`.       | `la $t0, my_data`   | `lui $t0, upper(my_data)` then `ori $t0, $t0, lower(my_data)` |
| `b label`          | Unconditional branch to `label`.                | `b loop_start`      | `beq $zero, $zero, loop_start`                       |

## Error Handling

The assembler provides informative error messages to help debug your assembly code. Errors typically include the type of error, a descriptive message, and the line number where the error occurred.

**Example Error:**

```
Assembly error: Syntax error at line 15: Invalid operand for JR instruction
```

## Next Steps

-   Write your own MIPS assembly programs and assemble them.
-   Run your assembled programs on the [Functional Simulator](functional-simulator.md) or [Timing Simulator](timing-simulator.md).
-   Explore the [Examples](examples.md) directory for more complex assembly programs.
