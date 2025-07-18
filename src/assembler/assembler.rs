// Copyright (c) 2024 Mudit Bhargava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// assembler.rs
//
// This file contains the implementation of the MIPS assembler.
// It defines the Assembler struct, which is responsible for parsing MIPS
// assembly code, resolving labels, and generating machine code.

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;

// Register mapping
const REGISTER_MAP: &[(&str, u32)] = &[
    ("$zero", 0),
    ("$0", 0),
    ("$at", 1),
    ("$1", 1),
    ("$v0", 2),
    ("$2", 2),
    ("$v1", 3),
    ("$3", 3),
    ("$a0", 4),
    ("$4", 4),
    ("$a1", 5),
    ("$5", 5),
    ("$a2", 6),
    ("$6", 6),
    ("$a3", 7),
    ("$7", 7),
    ("$t0", 8),
    ("$8", 8),
    ("$t1", 9),
    ("$9", 9),
    ("$t2", 10),
    ("$10", 10),
    ("$t3", 11),
    ("$11", 11),
    ("$t4", 12),
    ("$12", 12),
    ("$t5", 13),
    ("$13", 13),
    ("$t6", 14),
    ("$14", 14),
    ("$t7", 15),
    ("$15", 15),
    ("$s0", 16),
    ("$16", 16),
    ("$s1", 17),
    ("$17", 17),
    ("$s2", 18),
    ("$18", 18),
    ("$s3", 19),
    ("$19", 19),
    ("$s4", 20),
    ("$20", 20),
    ("$s5", 21),
    ("$21", 21),
    ("$s6", 22),
    ("$22", 22),
    ("$s7", 23),
    ("$23", 23),
    ("$t8", 24),
    ("$24", 24),
    ("$t9", 25),
    ("$25", 25),
    ("$k0", 26),
    ("$26", 26),
    ("$k1", 27),
    ("$27", 27),
    ("$gp", 28),
    ("$28", 28),
    ("$sp", 29),
    ("$29", 29),
    ("$fp", 30),
    ("$30", 30),
    ("$ra", 31),
    ("$31", 31),
];

// FP register mapping
const FP_REGISTER_MAP: &[(&str, u32)] = &[
    ("$f0", 0),
    ("$f1", 1),
    ("$f2", 2),
    ("$f3", 3),
    ("$f4", 4),
    ("$f5", 5),
    ("$f6", 6),
    ("$f7", 7),
    ("$f8", 8),
    ("$f9", 9),
    ("$f10", 10),
    ("$f11", 11),
    ("$f12", 12),
    ("$f13", 13),
    ("$f14", 14),
    ("$f15", 15),
    ("$f16", 16),
    ("$f17", 17),
    ("$f18", 18),
    ("$f19", 19),
    ("$f20", 20),
    ("$f21", 21),
    ("$f22", 22),
    ("$f23", 23),
    ("$f24", 24),
    ("$f25", 25),
    ("$f26", 26),
    ("$f27", 27),
    ("$f28", 28),
    ("$f29", 29),
    ("$f30", 30),
    ("$f31", 31),
];

// Assembler error
#[derive(Debug)]
pub enum AssemblerError {
    IoError(io::Error),
    ParseError(String, usize),
    SymbolError(String, usize),
    SyntaxError(String, usize),
    RegisterError(String, usize),
    OperandError(String, usize),
    RangeError(String, usize),
    UnsupportedError(String, usize),
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblerError::IoError(err) => write!(f, "I/O error: {}", err),
            AssemblerError::ParseError(msg, line) => {
                write!(f, "Parse error at line {}: {}", line, msg)
            },
            AssemblerError::SymbolError(msg, line) => {
                write!(f, "Symbol error at line {}: {}", line, msg)
            },
            AssemblerError::SyntaxError(msg, line) => {
                write!(f, "Syntax error at line {}: {}", line, msg)
            },
            AssemblerError::RegisterError(msg, line) => {
                write!(f, "Register error at line {}: {}", line, msg)
            },
            AssemblerError::OperandError(msg, line) => {
                write!(f, "Operand error at line {}: {}", line, msg)
            },
            AssemblerError::RangeError(msg, line) => {
                write!(f, "Range error at line {}: {}", line, msg)
            },
            AssemblerError::UnsupportedError(msg, line) => {
                write!(f, "Unsupported feature at line {}: {}", line, msg)
            },
        }
    }
}

impl From<io::Error> for AssemblerError {
    fn from(error: io::Error) -> Self {
        AssemblerError::IoError(error)
    }
}

// Token struct for lexer
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Label(String),
    Instruction(String),
    Register(u32),
    FpRegister(u32),
    Immediate(i32),
    Address(String, i32), // Base register name and offset
    Symbol(String),
    Comma,
    LeftParen,
    RightParen,
    Directive(String),
    StringLiteral(String),
}

// Assembler struct
pub struct Assembler {
    labels: HashMap<String, u32>,
    data_section: Vec<u8>,
    text_section: Vec<u32>,
    current_address: u32,
    in_data_section: bool,
    current_line: usize,
    errors: Vec<AssemblerError>,
    register_map: HashMap<String, u32>,
    fp_register_map: HashMap<String, u32>,
    current_filename: String,
}

impl Assembler {
    pub fn new() -> Self {
        let mut register_map = HashMap::new();
        for &(name, num) in REGISTER_MAP {
            register_map.insert(name.to_string(), num);
        }

        let mut fp_register_map = HashMap::new();
        for &(name, num) in FP_REGISTER_MAP {
            fp_register_map.insert(name.to_string(), num);
        }

        Self {
            labels: HashMap::new(),
            data_section: Vec::new(),
            text_section: Vec::new(),
            current_address: 0,
            in_data_section: false,
            current_line: 0,
            errors: Vec::new(),
            register_map,
            fp_register_map,
            current_filename: String::new(),
        }
    }

    // Assemble a file
    pub fn assemble_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<u8>, AssemblerError> {
        let file = File::open(&path)?;
        self.current_filename = path.as_ref().to_string_lossy().to_string();
        let reader = io::BufReader::new(file);

        // First pass: collect labels and directives
        self.first_pass(reader)?;

        // Reset for second pass
        self.current_address = 0;
        self.in_data_section = false;

        // Second pass: generate code
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        self.second_pass(reader)?;

        // Combine data and text sections
        let mut result = Vec::new();

        // Write data section size (4 bytes)
        let data_size = self.data_section.len() as u32;
        result.extend_from_slice(&data_size.to_le_bytes());

        // Write text section size (4 bytes)
        let text_size = self.text_section.len() as u32 * 4;
        result.extend_from_slice(&text_size.to_le_bytes());

        // Write data section
        result.extend_from_slice(&self.data_section);

        // Write text section
        for instr in &self.text_section {
            result.extend_from_slice(&instr.to_le_bytes());
        }

        if !self.errors.is_empty() {
            // Return the first error
            return Err(self.errors.remove(0));
        }

        Ok(result)
    }

    // Assemble from a string
    pub fn assemble_string(&mut self, code: &str) -> Result<Vec<u8>, AssemblerError> {
        self.current_filename = "<string>".to_string();

        // Create a cursor that implements BufRead
        let cursor = io::Cursor::new(code);

        // First pass: collect labels and directives
        self.first_pass(BufReader::new(cursor.clone()))?;

        // Reset for second pass
        self.current_address = 0;
        self.in_data_section = false;

        // Second pass: generate code
        self.second_pass(BufReader::new(cursor))?;

        // Combine data and text sections
        let mut result = Vec::new();

        // Write data section size (4 bytes)
        let data_size = self.data_section.len() as u32;
        result.extend_from_slice(&data_size.to_le_bytes());

        // Write text section size (4 bytes)
        let text_size = self.text_section.len() as u32 * 4;
        result.extend_from_slice(&text_size.to_le_bytes());

        // Write data section
        result.extend_from_slice(&self.data_section);

        // Write text section
        for instr in &self.text_section {
            result.extend_from_slice(&instr.to_le_bytes());
        }

        if !self.errors.is_empty() {
            // Return the first error
            return Err(self.errors.remove(0));
        }

        Ok(result)
    }

    // First pass: collect labels and directives
    fn first_pass<R: BufRead>(&mut self, reader: R) -> Result<(), AssemblerError> {
        self.current_line = 0;
        self.current_address = 0;
        self.in_data_section = false;
        self.labels.clear();

        for line_result in reader.lines() {
            self.current_line += 1;
            let line = line_result?;
            let line = self.preprocess_line(&line);

            if line.is_empty() {
                continue;
            }

            // Check if this line has a label
            if let Some(label_end) = line.find(':') {
                let label = line[..label_end].trim();
                if !label.is_empty() {
                    // Add label to symbol table
                    self.labels.insert(label.to_string(), self.current_address);
                }

                // Process the rest of the line (if any)
                let rest = line[label_end + 1..].trim();
                if rest.is_empty() {
                    continue;
                }

                self.process_first_pass_line(rest)?;
            } else {
                // No label, process the whole line
                self.process_first_pass_line(&line)?;
            }
        }

        Ok(())
    }

    // Process a line during the first pass
    fn process_first_pass_line(&mut self, line: &str) -> Result<(), AssemblerError> {
        let tokens = self.tokenize(line)?;

        if tokens.is_empty() {
            return Ok(());
        }

        match &tokens[0] {
            Token::Directive(directive) => {
                match directive.as_str() {
                    ".data" => {
                        self.in_data_section = true;
                    },
                    ".text" => {
                        self.in_data_section = false;
                    },
                    ".word" => {
                        // Each word takes 4 bytes in data section
                        if self.in_data_section {
                            // Count how many words are defined
                            let word_count = tokens
                                .iter()
                                .filter(|t| matches!(t, Token::Immediate(_) | Token::Symbol(_)))
                                .count();
                            self.current_address += word_count as u32 * 4;
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                ".word directive must be in .data section".to_string(),
                                self.current_line,
                            ));
                        }
                    },
                    ".byte" => {
                        // Each byte takes 1 byte in data section
                        if self.in_data_section {
                            // Count how many bytes are defined
                            let byte_count = tokens
                                .iter()
                                .filter(|t| matches!(t, Token::Immediate(_)))
                                .count();
                            self.current_address += byte_count as u32;
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                ".byte directive must be in .data section".to_string(),
                                self.current_line,
                            ));
                        }
                    },
                    ".half" => {
                        // Each halfword takes 2 bytes in data section
                        if self.in_data_section {
                            // Count how many halfwords are defined
                            let half_count = tokens
                                .iter()
                                .filter(|t| matches!(t, Token::Immediate(_)))
                                .count();
                            self.current_address += half_count as u32 * 2;
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                ".half directive must be in .data section".to_string(),
                                self.current_line,
                            ));
                        }
                    },
                    ".ascii" | ".asciiz" => {
                        // String length + 1 for null terminator if .asciiz
                        if self.in_data_section {
                            if tokens.len() < 2 {
                                return Err(AssemblerError::SyntaxError(
                                    format!("{} directive requires a string argument", directive),
                                    self.current_line,
                                ));
                            }

                            if let Token::StringLiteral(string) = &tokens[1] {
                                let null_terminator = if directive == ".asciiz" { 1 } else { 0 };
                                self.current_address += string.len() as u32 + null_terminator;
                            } else {
                                return Err(AssemblerError::SyntaxError(
                                    format!("{} directive requires a string argument", directive),
                                    self.current_line,
                                ));
                            }
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                format!("{} directive must be in .data section", directive),
                                self.current_line,
                            ));
                        }
                    },
                    ".space" => {
                        // Allocate n bytes of space
                        if self.in_data_section {
                            if tokens.len() < 2 {
                                return Err(AssemblerError::SyntaxError(
                                    ".space directive requires a size argument".to_string(),
                                    self.current_line,
                                ));
                            }

                            if let Token::Immediate(size) = tokens[1] {
                                if size < 0 {
                                    return Err(AssemblerError::RangeError(
                                        "Space size must be non-negative".to_string(),
                                        self.current_line,
                                    ));
                                }
                                self.current_address += size as u32;
                            } else {
                                return Err(AssemblerError::SyntaxError(
                                    ".space directive requires a numeric size argument".to_string(),
                                    self.current_line,
                                ));
                            }
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                ".space directive must be in .data section".to_string(),
                                self.current_line,
                            ));
                        }
                    },
                    ".align" => {
                        // Align to 2^n boundary
                        if tokens.len() < 2 {
                            return Err(AssemblerError::SyntaxError(
                                ".align directive requires an alignment argument".to_string(),
                                self.current_line,
                            ));
                        }

                        if let Token::Immediate(align) = tokens[1] {
                            if align < 0 || align > 31 {
                                return Err(AssemblerError::RangeError(
                                    "Alignment must be between 0 and 31".to_string(),
                                    self.current_line,
                                ));
                            }

                            let alignment = 1 << align;
                            let misalignment = self.current_address % alignment;
                            if misalignment != 0 {
                                self.current_address += alignment - misalignment;
                            }
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                ".align directive requires a numeric alignment argument"
                                    .to_string(),
                                self.current_line,
                            ));
                        }
                    },
                    _ => {
                        // Ignore unknown directives in first pass
                    },
                }
            },
            Token::Instruction(_) => {
                if !self.in_data_section {
                    // Each instruction takes 4 bytes in text section
                    self.current_address += 4;
                } else {
                    return Err(AssemblerError::SyntaxError(
                        "Instructions must be in .text section".to_string(),
                        self.current_line,
                    ));
                }
            },
            _ => {
                return Err(AssemblerError::SyntaxError(
                    format!("Unexpected token at start of line: {:?}", tokens[0]),
                    self.current_line,
                ));
            },
        }

        Ok(())
    }

    // Second pass: generate code
    fn second_pass<R: BufRead>(&mut self, reader: R) -> Result<(), AssemblerError> {
        self.current_line = 0;
        self.current_address = 0;
        self.in_data_section = false;
        self.data_section.clear();
        self.text_section.clear();

        for line_result in reader.lines() {
            self.current_line += 1;
            let line = line_result?;
            let line = self.preprocess_line(&line);

            if line.is_empty() {
                continue;
            }

            // Check if this line has a label
            if let Some(label_end) = line.find(':') {
                // Process the rest of the line (if any)
                let rest = line[label_end + 1..].trim();
                if rest.is_empty() {
                    continue;
                }

                self.process_second_pass_line(rest)?;
            } else {
                // No label, process the whole line
                self.process_second_pass_line(&line)?;
            }
        }

        Ok(())
    }

    // Process a line during the second pass
    fn process_second_pass_line(&mut self, line: &str) -> Result<(), AssemblerError> {
        let tokens = self.tokenize(line)?;

        if tokens.is_empty() {
            return Ok(());
        }

        match &tokens[0] {
            Token::Directive(directive) => {
                match directive.as_str() {
                    ".data" => {
                        self.in_data_section = true;
                    },
                    ".text" => {
                        self.in_data_section = false;
                    },
                    ".word" => {
                        if self.in_data_section {
                            for i in 1..tokens.len() {
                                if let Token::Comma = tokens[i] {
                                    continue;
                                }

                                match &tokens[i] {
                                    Token::Immediate(value) => {
                                        // Add word to data section
                                        self.data_section
                                            .extend_from_slice(&(*value as u32).to_le_bytes());
                                        self.current_address += 4;
                                    },
                                    Token::Symbol(symbol) => {
                                        // Look up symbol value
                                        if let Some(&addr) = self.labels.get(symbol) {
                                            self.data_section
                                                .extend_from_slice(&addr.to_le_bytes());
                                            self.current_address += 4;
                                        } else {
                                            return Err(AssemblerError::SymbolError(
                                                format!("Undefined symbol: {}", symbol),
                                                self.current_line,
                                            ));
                                        }
                                    },
                                    _ => {
                                        return Err(AssemblerError::SyntaxError(
                                            format!("Expected immediate value or symbol in .word directive, got {:?}", tokens[i]),
                                            self.current_line,
                                        ));
                                    },
                                }
                            }
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                ".word directive must be in .data section".to_string(),
                                self.current_line,
                            ));
                        }
                    },
                    ".byte" => {
                        if self.in_data_section {
                            for i in 1..tokens.len() {
                                if let Token::Comma = tokens[i] {
                                    continue;
                                }

                                if let Token::Immediate(value) = tokens[i] {
                                    if value < -128 || value > 255 {
                                        return Err(AssemblerError::RangeError(
                                            format!("Byte value out of range: {}", value),
                                            self.current_line,
                                        ));
                                    }

                                    // Add byte to data section
                                    self.data_section.push(value as u8);
                                    self.current_address += 1;
                                } else {
                                    return Err(AssemblerError::SyntaxError(
                                        format!(
                                            "Expected immediate value in .byte directive, got {:?}",
                                            tokens[i]
                                        ),
                                        self.current_line,
                                    ));
                                }
                            }
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                ".byte directive must be in .data section".to_string(),
                                self.current_line,
                            ));
                        }
                    },
                    ".half" => {
                        if self.in_data_section {
                            for i in 1..tokens.len() {
                                if let Token::Comma = tokens[i] {
                                    continue;
                                }

                                if let Token::Immediate(value) = tokens[i] {
                                    if value < -32768 || value > 65535 {
                                        return Err(AssemblerError::RangeError(
                                            format!("Halfword value out of range: {}", value),
                                            self.current_line,
                                        ));
                                    }

                                    // Add halfword to data section
                                    self.data_section
                                        .extend_from_slice(&(value as u16).to_le_bytes());
                                    self.current_address += 2;
                                } else {
                                    return Err(AssemblerError::SyntaxError(
                                        format!(
                                            "Expected immediate value in .half directive, got {:?}",
                                            tokens[i]
                                        ),
                                        self.current_line,
                                    ));
                                }
                            }
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                ".half directive must be in .data section".to_string(),
                                self.current_line,
                            ));
                        }
                    },
                    ".ascii" | ".asciiz" => {
                        if self.in_data_section {
                            if tokens.len() < 2 {
                                return Err(AssemblerError::SyntaxError(
                                    format!("{} directive requires a string argument", directive),
                                    self.current_line,
                                ));
                            }

                            if let Token::StringLiteral(string) = &tokens[1] {
                                // Add string to data section
                                self.data_section.extend_from_slice(string.as_bytes());
                                self.current_address += string.len() as u32;

                                // Add null terminator for .asciiz
                                if directive == ".asciiz" {
                                    self.data_section.push(0);
                                    self.current_address += 1;
                                }
                            } else {
                                return Err(AssemblerError::SyntaxError(
                                    format!("{} directive requires a string argument", directive),
                                    self.current_line,
                                ));
                            }
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                format!("{} directive must be in .data section", directive),
                                self.current_line,
                            ));
                        }
                    },
                    ".space" => {
                        if self.in_data_section {
                            if tokens.len() < 2 {
                                return Err(AssemblerError::SyntaxError(
                                    ".space directive requires a size argument".to_string(),
                                    self.current_line,
                                ));
                            }

                            if let Token::Immediate(size) = tokens[1] {
                                if size < 0 {
                                    return Err(AssemblerError::RangeError(
                                        "Space size must be non-negative".to_string(),
                                        self.current_line,
                                    ));
                                }

                                // Add zeros to data section
                                self.data_section.extend(vec![0; size as usize]);
                                self.current_address += size as u32;
                            } else {
                                return Err(AssemblerError::SyntaxError(
                                    ".space directive requires a numeric size argument".to_string(),
                                    self.current_line,
                                ));
                            }
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                ".space directive must be in .data section".to_string(),
                                self.current_line,
                            ));
                        }
                    },
                    ".align" => {
                        if tokens.len() < 2 {
                            return Err(AssemblerError::SyntaxError(
                                ".align directive requires an alignment argument".to_string(),
                                self.current_line,
                            ));
                        }

                        if let Token::Immediate(align) = tokens[1] {
                            if align < 0 || align > 31 {
                                return Err(AssemblerError::RangeError(
                                    "Alignment must be between 0 and 31".to_string(),
                                    self.current_line,
                                ));
                            }

                            let alignment = 1 << align;
                            let misalignment = self.current_address % alignment;
                            if misalignment != 0 {
                                let padding = alignment - misalignment;

                                // Add padding bytes
                                if self.in_data_section {
                                    self.data_section.extend(vec![0; padding as usize]);
                                } else {
                                    // In text section, add NOP instructions (0x00000000)
                                    for _ in 0..(padding / 4) {
                                        self.text_section.push(0);
                                    }
                                }

                                self.current_address += padding;
                            }
                        } else {
                            return Err(AssemblerError::SyntaxError(
                                ".align directive requires a numeric alignment argument"
                                    .to_string(),
                                self.current_line,
                            ));
                        }
                    },
                    _ => {
                        // Ignore unknown directives in second pass
                    },
                }
            },
            Token::Instruction(instr) => {
                if !self.in_data_section {
                    // Generate machine code for instruction
                    let machine_code = self.assemble_instruction(instr, &tokens[1..])?;
                    self.text_section.push(machine_code);
                    self.current_address += 4;
                } else {
                    return Err(AssemblerError::SyntaxError(
                        "Instructions must be in .text section".to_string(),
                        self.current_line,
                    ));
                }
            },
            _ => {
                return Err(AssemblerError::SyntaxError(
                    format!("Unexpected token at start of line: {:?}", tokens[0]),
                    self.current_line,
                ));
            },
        }

        Ok(())
    }

    // Tokenize a line of assembly code
    fn tokenize(&self, line: &str) -> Result<Vec<Token>, AssemblerError> {
        let mut tokens = Vec::new();
        let mut chars = line.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    // Skip whitespace
                    chars.next();
                },
                ',' => {
                    tokens.push(Token::Comma);
                    chars.next();
                },
                '(' => {
                    tokens.push(Token::LeftParen);
                    chars.next();
                },
                ')' => {
                    tokens.push(Token::RightParen);
                    chars.next();
                },
                '$' => {
                    // Register
                    chars.next(); // Skip $
                    let mut reg_name = String::from("$");

                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() {
                            reg_name.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    // Check if it's a valid register
                    if reg_name.starts_with("$f") {
                        // FP register
                        if let Ok(reg_num) = reg_name[2..].parse::<u32>() {
                            if reg_num < 32 {
                                tokens.push(Token::FpRegister(reg_num));
                            } else {
                                return Err(AssemblerError::RegisterError(
                                    format!("Invalid FP register number: {}", reg_num),
                                    self.current_line,
                                ));
                            }
                        } else if let Some(&reg_num) = self.fp_register_map.get(&reg_name) {
                            tokens.push(Token::FpRegister(reg_num));
                        } else {
                            return Err(AssemblerError::RegisterError(
                                format!("Invalid FP register: {}", reg_name),
                                self.current_line,
                            ));
                        }
                    } else {
                        // Integer register
                        if let Ok(reg_num) = reg_name[1..].parse::<u32>() {
                            if reg_num < 32 {
                                tokens.push(Token::Register(reg_num));
                            } else {
                                return Err(AssemblerError::RegisterError(
                                    format!("Invalid register number: {}", reg_num),
                                    self.current_line,
                                ));
                            }
                        } else if let Some(&reg_num) = self.register_map.get(&reg_name) {
                            tokens.push(Token::Register(reg_num));
                        } else {
                            return Err(AssemblerError::RegisterError(
                                format!("Invalid register: {}", reg_name),
                                self.current_line,
                            ));
                        }
                    }
                },
                '-' | '+' | '0'..='9' => {
                    // Immediate value
                    let mut num_str = String::new();

                    // Handle sign
                    if c == '-' || c == '+' {
                        num_str.push(c);
                        chars.next();
                    }

                    // Check if it's a hex, octal, or binary number
                    if chars.peek() == Some(&'0') {
                        num_str.push('0');
                        chars.next();

                        if let Some(&c) = chars.peek() {
                            match c {
                                'x' | 'X' => {
                                    // Hex number
                                    num_str.push('x');
                                    chars.next();

                                    while let Some(&c) = chars.peek() {
                                        if c.is_digit(16) {
                                            num_str.push(c);
                                            chars.next();
                                        } else {
                                            break;
                                        }
                                    }

                                    // Parse hex number
                                    if let Ok(value) = i32::from_str_radix(&num_str[2..], 16) {
                                        tokens.push(Token::Immediate(value));
                                    } else {
                                        return Err(AssemblerError::ParseError(
                                            format!("Invalid hex number: {}", num_str),
                                            self.current_line,
                                        ));
                                    }
                                },
                                'b' | 'B' => {
                                    // Binary number
                                    num_str.push('b');
                                    chars.next();

                                    while let Some(&c) = chars.peek() {
                                        if c == '0' || c == '1' {
                                            num_str.push(c);
                                            chars.next();
                                        } else {
                                            break;
                                        }
                                    }

                                    // Parse binary number
                                    if let Ok(value) = i32::from_str_radix(&num_str[2..], 2) {
                                        tokens.push(Token::Immediate(value));
                                    } else {
                                        return Err(AssemblerError::ParseError(
                                            format!("Invalid binary number: {}", num_str),
                                            self.current_line,
                                        ));
                                    }
                                },
                                '0'..='7' => {
                                    // Octal number
                                    while let Some(&c) = chars.peek() {
                                        if c.is_digit(8) {
                                            num_str.push(c);
                                            chars.next();
                                        } else {
                                            break;
                                        }
                                    }

                                    // Parse octal number
                                    if let Ok(value) = i32::from_str_radix(&num_str[1..], 8) {
                                        tokens.push(Token::Immediate(value));
                                    } else {
                                        return Err(AssemblerError::ParseError(
                                            format!("Invalid octal number: {}", num_str),
                                            self.current_line,
                                        ));
                                    }
                                },
                                _ => {
                                    // Just a zero
                                    tokens.push(Token::Immediate(0));
                                },
                            }
                        } else {
                            // Just a zero
                            tokens.push(Token::Immediate(0));
                        }
                    } else {
                        // Decimal number
                        while let Some(&c) = chars.peek() {
                            if c.is_digit(10) {
                                num_str.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }

                        // Parse decimal number
                        if let Ok(value) = num_str.parse::<i32>() {
                            tokens.push(Token::Immediate(value));
                        } else {
                            return Err(AssemblerError::ParseError(
                                format!("Invalid decimal number: {}", num_str),
                                self.current_line,
                            ));
                        }
                    }
                },
                '.' => {
                    // Directive or symbol
                    let mut name = String::new();
                    name.push(c);
                    chars.next();

                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            name.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::Directive(name));
                },
                '"' => {
                    // String literal
                    chars.next(); // Skip opening quote
                    let mut string = String::new();

                    while let Some(&c) = chars.peek() {
                        if c == '"' {
                            chars.next(); // Skip closing quote
                            break;
                        } else if c == '\\' {
                            // Escape sequence
                            chars.next(); // Skip backslash

                            if let Some(&c) = chars.peek() {
                                match c {
                                    'n' => string.push('\n'),
                                    't' => string.push('\t'),
                                    'r' => string.push('\r'),
                                    '0' => string.push('\0'),
                                    '\\' => string.push('\\'),
                                    '"' => string.push('"'),
                                    'x' => {
                                        // Hex escape sequence (\xHH)
                                        chars.next(); // Skip x
                                        let mut hex = String::new();

                                        for _ in 0..2 {
                                            if let Some(&c) = chars.peek() {
                                                if c.is_digit(16) {
                                                    hex.push(c);
                                                    chars.next();
                                                } else {
                                                    break;
                                                }
                                            } else {
                                                break;
                                            }
                                        }

                                        if let Ok(value) = u8::from_str_radix(&hex, 16) {
                                            string.push(value as char);
                                        } else {
                                            return Err(AssemblerError::ParseError(
                                                format!("Invalid hex escape sequence: \\x{}", hex),
                                                self.current_line,
                                            ));
                                        }

                                        continue; // Skip the chars.next() below
                                    },
                                    _ => string.push(c),
                                }

                                chars.next();
                            } else {
                                return Err(AssemblerError::ParseError(
                                    "Unexpected end of string after escape character".to_string(),
                                    self.current_line,
                                ));
                            }
                        } else {
                            string.push(c);
                            chars.next();
                        }
                    }

                    tokens.push(Token::StringLiteral(string));
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    // Instruction or symbol
                    let mut name = String::new();

                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' || c == '.' {
                            name.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    // Check if it's an instruction
                    if let Some(canonical_name) = self.normalize_instruction(&name) {
                        tokens.push(Token::Instruction(canonical_name));
                    } else {
                        // It's a symbol
                        tokens.push(Token::Symbol(name));
                    }
                },
                '#' => {
                    // Comment - ignore rest of line
                    break;
                },
                _ => {
                    return Err(AssemblerError::SyntaxError(
                        format!("Unexpected character: {}", c),
                        self.current_line,
                    ));
                },
            }
        }

        Ok(tokens)
    }

    // Normalize instruction name (convert aliases to canonical form)
    fn normalize_instruction(&self, name: &str) -> Option<String> {
        // Common instruction aliases
        match name.to_lowercase().as_str() {
            "add" | "addu" => Some("add".to_string()),
            "sub" | "subu" => Some("sub".to_string()),
            "and" => Some("and".to_string()),
            "or" => Some("or".to_string()),
            "xor" => Some("xor".to_string()),
            "nor" => Some("nor".to_string()),
            "slt" | "sltu" => Some("slt".to_string()),
            "sll" | "sllv" => Some("sll".to_string()),
            "srl" | "srlv" => Some("srl".to_string()),
            "sra" | "srav" => Some("sra".to_string()),
            "addi" | "addiu" => Some("addi".to_string()),
            "slti" | "sltiu" => Some("slti".to_string()),
            "andi" => Some("andi".to_string()),
            "ori" => Some("ori".to_string()),
            "xori" => Some("xori".to_string()),
            "lui" => Some("lui".to_string()),
            "lw" => Some("lw".to_string()),
            "sw" => Some("sw".to_string()),
            "lb" => Some("lb".to_string()),
            "lbu" => Some("lbu".to_string()),
            "sb" => Some("sb".to_string()),
            "lh" => Some("lh".to_string()),
            "lhu" => Some("lhu".to_string()),
            "sh" => Some("sh".to_string()),
            "beq" => Some("beq".to_string()),
            "bne" => Some("bne".to_string()),
            "bgtz" => Some("bgtz".to_string()),
            "blez" => Some("blez".to_string()),
            "bgez" => Some("bgez".to_string()),
            "bltz" => Some("bltz".to_string()),
            "j" => Some("j".to_string()),
            "jal" => Some("jal".to_string()),
            "jr" => Some("jr".to_string()),
            "jalr" => Some("jalr".to_string()),
            "mult" | "multu" => Some("mult".to_string()),
            "div" | "divu" => Some("div".to_string()),
            "mfhi" => Some("mfhi".to_string()),
            "mflo" => Some("mflo".to_string()),
            "mthi" => Some("mthi".to_string()),
            "mtlo" => Some("mtlo".to_string()),
            "syscall" => Some("syscall".to_string()),
            "break" => Some("break".to_string()),
            "nop" => Some("nop".to_string()),
            "move" => Some("move".to_string()),
            "li" => Some("li".to_string()),
            "la" => Some("la".to_string()),
            "b" => Some("b".to_string()),
            // FP instructions
            "add.s" => Some("add.s".to_string()),
            "sub.s" => Some("sub.s".to_string()),
            "mul.s" => Some("mul.s".to_string()),
            "div.s" => Some("div.s".to_string()),
            "abs.s" => Some("abs.s".to_string()),
            "neg.s" => Some("neg.s".to_string()),
            "mov.s" => Some("mov.s".to_string()),
            "cvt.s.w" => Some("cvt.s.w".to_string()),
            "cvt.w.s" => Some("cvt.w.s".to_string()),
            "c.eq.s" => Some("c.eq.s".to_string()),
            "c.lt.s" => Some("c.lt.s".to_string()),
            "c.le.s" => Some("c.le.s".to_string()),
            "lwc1" => Some("lwc1".to_string()),
            "swc1" => Some("swc1".to_string()),
            "bc1t" => Some("bc1t".to_string()),
            "bc1f" => Some("bc1f".to_string()),
            _ => None,
        }
    }

    // Preprocess a line of code
    fn preprocess_line(&self, line: &str) -> String {
        // Remove comments
        let mut result = String::new();
        let mut in_string = false;
        let mut escape = false;

        for c in line.chars() {
            if in_string {
                if escape {
                    escape = false;
                } else if c == '\\' {
                    escape = true;
                } else if c == '"' {
                    in_string = false;
                }
                result.push(c);
            } else if c == '"' {
                in_string = true;
                result.push(c);
            } else if c == '#' {
                break;
            } else {
                result.push(c);
            }
        }

        result.trim().to_string()
    }

    // Assemble an instruction
    fn assemble_instruction(&self, instr: &str, operands: &[Token]) -> Result<u32, AssemblerError> {
        match instr {
            "add" => self.assemble_r_type(0, 0x20, operands),
            "sub" => self.assemble_r_type(0, 0x22, operands),
            "and" => self.assemble_r_type(0, 0x24, operands),
            "or" => self.assemble_r_type(0, 0x25, operands),
            "xor" => self.assemble_r_type(0, 0x26, operands),
            "nor" => self.assemble_r_type(0, 0x27, operands),
            "slt" => self.assemble_r_type(0, 0x2A, operands),
            "sll" => self.assemble_shift(0, 0x00, operands),
            "srl" => self.assemble_shift(0, 0x02, operands),
            "sra" => self.assemble_shift(0, 0x03, operands),
            "jr" => self.assemble_jr(operands),
            "jalr" => self.assemble_jalr(operands),
            "addi" => self.assemble_i_type(0x08, operands),
            "andi" => self.assemble_i_type(0x0C, operands),
            "ori" => self.assemble_i_type(0x0D, operands),
            "xori" => self.assemble_i_type(0x0E, operands),
            "lui" => self.assemble_lui(operands),
            "lw" => self.assemble_load_store(0x23, operands),
            "sw" => self.assemble_load_store(0x2B, operands),
            "lb" => self.assemble_load_store(0x20, operands),
            "lbu" => self.assemble_load_store(0x24, operands),
            "sb" => self.assemble_load_store(0x28, operands),
            "lh" => self.assemble_load_store(0x21, operands),
            "lhu" => self.assemble_load_store(0x25, operands),
            "sh" => self.assemble_load_store(0x29, operands),
            "beq" => self.assemble_branch(0x04, operands),
            "bne" => self.assemble_branch(0x05, operands),
            "bgtz" => self.assemble_branch_z(0x07, operands),
            "blez" => self.assemble_branch_z(0x06, operands),
            "bgez" => self.assemble_branch_z_rt(0x01, 0x01, operands),
            "bltz" => self.assemble_branch_z_rt(0x01, 0x00, operands),
            "j" => self.assemble_jump(0x02, operands),
            "jal" => self.assemble_jump(0x03, operands),
            "mult" => self.assemble_mult_div(0, 0x18, operands),
            "div" => self.assemble_mult_div(0, 0x1A, operands),
            "mfhi" => self.assemble_mf(0, 0x10, operands),
            "mflo" => self.assemble_mf(0, 0x12, operands),
            "mthi" => self.assemble_mt(0, 0x11, operands),
            "mtlo" => self.assemble_mt(0, 0x13, operands),
            "syscall" => self.assemble_syscall(),
            "break" => self.assemble_break(operands),
            "nop" => Ok(0),
            "move" => self.assemble_move(operands),
            "li" => self.assemble_li(operands),
            "la" => self.assemble_la(operands),
            "b" => self.assemble_b(operands),
            // FP instructions
            "add.s" => self.assemble_fp_r_type(0x11, 0x10, 0x00, operands),
            "sub.s" => self.assemble_fp_r_type(0x11, 0x10, 0x01, operands),
            "mul.s" => self.assemble_fp_r_type(0x11, 0x10, 0x02, operands),
            "div.s" => self.assemble_fp_r_type(0x11, 0x10, 0x03, operands),
            "abs.s" => self.assemble_fp_r_type_fs(0x11, 0x10, 0x05, operands),
            "neg.s" => self.assemble_fp_r_type_fs(0x11, 0x10, 0x07, operands),
            "mov.s" => self.assemble_fp_r_type_fs(0x11, 0x10, 0x06, operands),
            "cvt.s.w" => self.assemble_fp_r_type_fs(0x11, 0x14, 0x20, operands),
            "cvt.w.s" => self.assemble_fp_r_type_fs(0x11, 0x10, 0x24, operands),
            "c.eq.s" => self.assemble_fp_cmp(0x11, 0x10, 0x30, 0x00, operands),
            "c.lt.s" => self.assemble_fp_cmp(0x11, 0x10, 0x30, 0x01, operands),
            "c.le.s" => self.assemble_fp_cmp(0x11, 0x10, 0x30, 0x02, operands),
            "lwc1" => self.assemble_fp_load_store(0x31, operands),
            "swc1" => self.assemble_fp_load_store(0x39, operands),
            "bc1t" => self.assemble_fp_branch(0x11, 0x08, 0x01, operands),
            "bc1f" => self.assemble_fp_branch(0x11, 0x08, 0x00, operands),
            _ => Err(AssemblerError::UnsupportedError(
                format!("Unsupported instruction: {}", instr),
                self.current_line,
            )),
        }
    }

    // Assemble R-type instruction
    fn assemble_r_type(
        &self,
        opcode: u32,
        funct: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.len() < 5 {
            return Err(AssemblerError::SyntaxError(
                "R-type instruction requires 3 registers".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2], &operands[4]) {
            (Token::Register(rd), Token::Register(rs), Token::Register(rt)) => {
                // opcode (6 bits) | rs (5 bits) | rt (5 bits) | rd (5 bits) | shamt (5 bits) | funct (6 bits)
                Ok((opcode << 26) | (*rs << 21) | (*rt << 16) | (*rd << 11) | (0 << 6) | funct)
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for R-type instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble shift instructions (sll, srl, sra)
    fn assemble_shift(
        &self,
        opcode: u32,
        funct: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.len() < 5 {
            return Err(AssemblerError::SyntaxError(
                "Shift instruction requires a register, a register, and a shift amount".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2], &operands[4]) {
            (Token::Register(rd), Token::Register(rt), Token::Immediate(shamt)) => {
                if *shamt < 0 || *shamt > 31 {
                    return Err(AssemblerError::RangeError(
                        format!("Shift amount out of range: {}", shamt),
                        self.current_line,
                    ));
                }

                // opcode (6 bits) | rs (5 bits) | rt (5 bits) | rd (5 bits) | shamt (5 bits) | funct (6 bits)
                Ok((opcode << 26)
                    | (0 << 21)
                    | (*rt << 16)
                    | (*rd << 11)
                    | ((*shamt as u32) << 6)
                    | funct)
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for shift instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble jump register (jr)
    fn assemble_jr(&self, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.is_empty() {
            return Err(AssemblerError::SyntaxError(
                "JR instruction requires a register".to_string(),
                self.current_line,
            ));
        }

        match &operands[0] {
            Token::Register(rs) => {
                // opcode (6 bits) | rs (5 bits) | 0 (15 bits) | funct (6 bits)
                Ok((0 << 26) | (*rs << 21) | (0 << 6) | 0x08)
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operand for JR instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble jump and link register (jalr)
    fn assemble_jalr(&self, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.len() < 1 {
            return Err(AssemblerError::SyntaxError(
                "JALR instruction requires at least one register".to_string(),
                self.current_line,
            ));
        }

        if operands.len() == 1 {
            // Only rs specified, rd defaults to $ra (31)
            match &operands[0] {
                Token::Register(rs) => {
                    // opcode (6 bits) | rs (5 bits) | 0 (5 bits) | rd (5 bits) | 0 (5 bits) | funct (6 bits)
                    Ok((0 << 26) | (*rs << 21) | (0 << 16) | (31 << 11) | (0 << 6) | 0x09)
                },
                _ => Err(AssemblerError::SyntaxError(
                    "Invalid operand for JALR instruction".to_string(),
                    self.current_line,
                )),
            }
        } else if operands.len() >= 3 {
            // Both rd and rs specified
            match (&operands[0], &operands[2]) {
                (Token::Register(rd), Token::Register(rs)) => {
                    // opcode (6 bits) | rs (5 bits) | 0 (5 bits) | rd (5 bits) | 0 (5 bits) | funct (6 bits)
                    Ok((0 << 26) | (*rs << 21) | (0 << 16) | (*rd << 11) | (0 << 6) | 0x09)
                },
                _ => Err(AssemblerError::SyntaxError(
                    "Invalid operands for JALR instruction".to_string(),
                    self.current_line,
                )),
            }
        } else {
            Err(AssemblerError::SyntaxError(
                "Invalid syntax for JALR instruction".to_string(),
                self.current_line,
            ))
        }
    }

    // Assemble I-type instruction
    fn assemble_i_type(&self, opcode: u32, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.len() < 5 {
            return Err(AssemblerError::SyntaxError(
                "I-type instruction requires a register, a register, and an immediate value"
                    .to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2], &operands[4]) {
            (Token::Register(rt), Token::Register(rs), Token::Immediate(imm)) => {
                if *imm < -32768 || *imm > 65535 {
                    return Err(AssemblerError::RangeError(
                        format!("Immediate value out of range: {}", imm),
                        self.current_line,
                    ));
                }

                // opcode (6 bits) | rs (5 bits) | rt (5 bits) | immediate (16 bits)
                Ok((opcode << 26) | (*rs << 21) | (*rt << 16) | (*imm as u32 & 0xFFFF))
            },
            (Token::Register(rt), Token::Register(rs), Token::Symbol(symbol)) => {
                // Look up symbol value
                if let Some(&addr) = self.labels.get(symbol) {
                    // Calculate offset for PC-relative addressing
                    let offset = (addr as i32 - (self.current_address as i32 + 4)) / 4;

                    if offset < -32768 || offset > 32767 {
                        return Err(AssemblerError::RangeError(
                            format!("Symbol offset out of range: {}", offset),
                            self.current_line,
                        ));
                    }

                    // opcode (6 bits) | rs (5 bits) | rt (5 bits) | immediate (16 bits)
                    Ok((opcode << 26) | (*rs << 21) | (*rt << 16) | (offset as u32 & 0xFFFF))
                } else {
                    Err(AssemblerError::SymbolError(
                        format!("Undefined symbol: {}", symbol),
                        self.current_line,
                    ))
                }
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for I-type instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble load upper immediate (lui)
    fn assemble_lui(&self, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "LUI instruction requires a register and an immediate value".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2]) {
            (Token::Register(rt), Token::Immediate(imm)) => {
                if *imm < -32768 || *imm > 65535 {
                    return Err(AssemblerError::RangeError(
                        format!("Immediate value out of range: {}", imm),
                        self.current_line,
                    ));
                }

                // opcode (6 bits) | 0 (5 bits) | rt (5 bits) | immediate (16 bits)
                Ok((0x0F << 26) | (0 << 21) | (*rt << 16) | (*imm as u32 & 0xFFFF))
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for LUI instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble load and store instructions
    fn assemble_load_store(&self, opcode: u32, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "Load/store instruction requires a register and an address".to_string(),
                self.current_line,
            ));
        }

        match &operands[0] {
            Token::Register(rt) => {
                // Parse the address operand
                let (base, offset) = self.parse_address(&operands[1..])?;

                // opcode (6 bits) | base (5 bits) | rt (5 bits) | offset (16 bits)
                Ok((opcode << 26) | (base << 21) | (*rt << 16) | (offset as u32 & 0xFFFF))
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for load/store instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Parse an address operand (offset(base))
    fn parse_address(&self, tokens: &[Token]) -> Result<(u32, i16), AssemblerError> {
        if tokens.is_empty() {
            return Err(AssemblerError::SyntaxError(
                "Expected address operand".to_string(),
                self.current_line,
            ));
        }

        // Check if it's a simple base register
        if let Token::Register(base) = tokens[0] {
            return Ok((base, 0));
        }

        // Check for offset(base) format
        if tokens.len() >= 4 && tokens[1] == Token::LeftParen && tokens[3] == Token::RightParen {
            match (&tokens[0], &tokens[2]) {
                (Token::Immediate(offset), Token::Register(base)) => {
                    if *offset < -32768 || *offset > 32767 {
                        return Err(AssemblerError::RangeError(
                            format!("Offset out of range: {}", offset),
                            self.current_line,
                        ));
                    }

                    Ok((*base, *offset as i16))
                },
                (Token::Symbol(symbol), Token::Register(base)) => {
                    // Look up symbol value
                    if let Some(&addr) = self.labels.get(symbol) {
                        Ok((*base, addr as i16))
                    } else {
                        Err(AssemblerError::SymbolError(
                            format!("Undefined symbol: {}", symbol),
                            self.current_line,
                        ))
                    }
                },
                _ => Err(AssemblerError::SyntaxError(
                    "Invalid address format".to_string(),
                    self.current_line,
                )),
            }
        } else {
            Err(AssemblerError::SyntaxError(
                "Invalid address format".to_string(),
                self.current_line,
            ))
        }
    }

    // Assemble branch instructions (beq, bne)
    fn assemble_branch(&self, opcode: u32, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.len() < 5 {
            return Err(AssemblerError::SyntaxError(
                "Branch instruction requires two registers and a label".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2], &operands[4]) {
            (Token::Register(rs), Token::Register(rt), Token::Symbol(symbol)) => {
                // Look up symbol value
                if let Some(&addr) = self.labels.get(symbol) {
                    // Calculate offset for PC-relative addressing
                    let offset = (addr as i32 - (self.current_address as i32 + 4)) / 4;

                    if offset < -32768 || offset > 32767 {
                        return Err(AssemblerError::RangeError(
                            format!("Branch target out of range: {}", offset),
                            self.current_line,
                        ));
                    }

                    // opcode (6 bits) | rs (5 bits) | rt (5 bits) | offset (16 bits)
                    Ok((opcode << 26) | (*rs << 21) | (*rt << 16) | (offset as u32 & 0xFFFF))
                } else {
                    Err(AssemblerError::SymbolError(
                        format!("Undefined symbol: {}", symbol),
                        self.current_line,
                    ))
                }
            },
            (Token::Register(rs), Token::Register(rt), Token::Immediate(offset)) => {
                if *offset < -32768 || *offset > 32767 {
                    return Err(AssemblerError::RangeError(
                        format!("Branch offset out of range: {}", offset),
                        self.current_line,
                    ));
                }

                // opcode (6 bits) | rs (5 bits) | rt (5 bits) | offset (16 bits)
                Ok((opcode << 26) | (*rs << 21) | (*rt << 16) | (*offset as u32 & 0xFFFF))
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for branch instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble branch zero instructions (bgtz, blez)
    fn assemble_branch_z(&self, opcode: u32, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "Branch instruction requires a register and a label".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2]) {
            (Token::Register(rs), Token::Symbol(symbol)) => {
                // Look up symbol value
                if let Some(&addr) = self.labels.get(symbol) {
                    // Calculate offset for PC-relative addressing
                    let offset = (addr as i32 - (self.current_address as i32 + 4)) / 4;

                    if offset < -32768 || offset > 32767 {
                        return Err(AssemblerError::RangeError(
                            format!("Branch target out of range: {}", offset),
                            self.current_line,
                        ));
                    }

                    // opcode (6 bits) | rs (5 bits) | rt (5 bits) | offset (16 bits)
                    Ok((opcode << 26) | (*rs << 21) | (0 << 16) | (offset as u32 & 0xFFFF))
                } else {
                    Err(AssemblerError::SymbolError(
                        format!("Undefined symbol: {}", symbol),
                        self.current_line,
                    ))
                }
            },
            (Token::Register(rs), Token::Immediate(offset)) => {
                if *offset < -32768 || *offset > 32767 {
                    return Err(AssemblerError::RangeError(
                        format!("Branch offset out of range: {}", offset),
                        self.current_line,
                    ));
                }

                // opcode (6 bits) | rs (5 bits) | rt (5 bits) | offset (16 bits)
                Ok((opcode << 26) | (*rs << 21) | (0 << 16) | (*offset as u32 & 0xFFFF))
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for branch instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble branch zero instructions with rt field (bgez, bltz)
    fn assemble_branch_z_rt(
        &self,
        opcode: u32,
        rt: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "Branch instruction requires a register and a label".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2]) {
            (Token::Register(rs), Token::Symbol(symbol)) => {
                // Look up symbol value
                if let Some(&addr) = self.labels.get(symbol) {
                    // Calculate offset for PC-relative addressing
                    let offset = (addr as i32 - (self.current_address as i32 + 4)) / 4;

                    if offset < -32768 || offset > 32767 {
                        return Err(AssemblerError::RangeError(
                            format!("Branch target out of range: {}", offset),
                            self.current_line,
                        ));
                    }

                    // opcode (6 bits) | rs (5 bits) | rt (5 bits) | offset (16 bits)
                    Ok((opcode << 26) | (*rs << 21) | (rt << 16) | (offset as u32 & 0xFFFF))
                } else {
                    Err(AssemblerError::SymbolError(
                        format!("Undefined symbol: {}", symbol),
                        self.current_line,
                    ))
                }
            },
            (Token::Register(rs), Token::Immediate(offset)) => {
                if *offset < -32768 || *offset > 32767 {
                    return Err(AssemblerError::RangeError(
                        format!("Branch offset out of range: {}", offset),
                        self.current_line,
                    ));
                }

                // opcode (6 bits) | rs (5 bits) | rt (5 bits) | offset (16 bits)
                Ok((opcode << 26) | (*rs << 21) | (rt << 16) | (*offset as u32 & 0xFFFF))
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for branch instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble jump instructions (j, jal)
    fn assemble_jump(&self, opcode: u32, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.is_empty() {
            return Err(AssemblerError::SyntaxError(
                "Jump instruction requires a target".to_string(),
                self.current_line,
            ));
        }

        match &operands[0] {
            Token::Symbol(symbol) => {
                // Look up symbol value
                if let Some(&addr) = self.labels.get(symbol) {
                    if addr % 4 != 0 {
                        return Err(AssemblerError::RangeError(
                            format!("Jump target not word-aligned: 0x{:08X}", addr),
                            self.current_line,
                        ));
                    }

                    // opcode (6 bits) | target (26 bits)
                    Ok((opcode << 26) | ((addr >> 2) & 0x3FFFFFF))
                } else {
                    Err(AssemblerError::SymbolError(
                        format!("Undefined symbol: {}", symbol),
                        self.current_line,
                    ))
                }
            },
            Token::Immediate(addr) => {
                let addr = *addr as u32;

                if addr % 4 != 0 {
                    return Err(AssemblerError::RangeError(
                        format!("Jump target not word-aligned: 0x{:08X}", addr),
                        self.current_line,
                    ));
                }

                // opcode (6 bits) | target (26 bits)
                Ok((opcode << 26) | ((addr >> 2) & 0x3FFFFFF))
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operand for jump instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble mult/div instructions
    fn assemble_mult_div(
        &self,
        opcode: u32,
        funct: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "MULT/DIV instruction requires two registers".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2]) {
            (Token::Register(rs), Token::Register(rt)) => {
                // opcode (6 bits) | rs (5 bits) | rt (5 bits) | 0 (10 bits) | funct (6 bits)
                Ok((opcode << 26) | (*rs << 21) | (*rt << 16) | (0 << 6) | funct)
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for MULT/DIV instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble mfhi/mflo instructions
    fn assemble_mf(
        &self,
        opcode: u32,
        funct: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.is_empty() {
            return Err(AssemblerError::SyntaxError(
                "MFHI/MFLO instruction requires a register".to_string(),
                self.current_line,
            ));
        }

        match &operands[0] {
            Token::Register(rd) => {
                // opcode (6 bits) | 0 (10 bits) | rd (5 bits) | 0 (5 bits) | funct (6 bits)
                Ok((opcode << 26) | (0 << 21) | (0 << 16) | (*rd << 11) | (0 << 6) | funct)
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operand for MFHI/MFLO instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble mthi/mtlo instructions
    fn assemble_mt(
        &self,
        opcode: u32,
        funct: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.is_empty() {
            return Err(AssemblerError::SyntaxError(
                "MTHI/MTLO instruction requires a register".to_string(),
                self.current_line,
            ));
        }

        match &operands[0] {
            Token::Register(rs) => {
                // opcode (6 bits) | rs (5 bits) | 0 (15 bits) | funct (6 bits)
                Ok((opcode << 26) | (*rs << 21) | (0 << 6) | funct)
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operand for MTHI/MTLO instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble syscall instruction
    fn assemble_syscall(&self) -> Result<u32, AssemblerError> {
        // opcode (6 bits) | 0 (20 bits) | funct (6 bits)
        Ok((0 << 26) | (0 << 6) | 0x0C)
    }

    // Assemble break instruction
    fn assemble_break(&self, operands: &[Token]) -> Result<u32, AssemblerError> {
        let code = if !operands.is_empty() {
            match &operands[0] {
                Token::Immediate(code) => {
                    if *code < 0 || *code > 1048575 {
                        return Err(AssemblerError::RangeError(
                            format!("Break code out of range: {}", code),
                            self.current_line,
                        ));
                    }
                    *code as u32
                },
                _ => 0,
            }
        } else {
            0
        };

        // opcode (6 bits) | code (20 bits) | funct (6 bits)
        Ok((0 << 26) | (code << 6) | 0x0D)
    }

    // Assemble move pseudo-instruction (move $rd, $rs)
    fn assemble_move(&self, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "MOVE instruction requires two registers".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2]) {
            (Token::Register(rd), Token::Register(rs)) => {
                // Translate to "addu $rd, $zero, $rs"
                // opcode (6 bits) | zero (5 bits) | rs (5 bits) | rd (5 bits) | 0 (5 bits) | addu funct (6 bits)
                Ok((0 << 26) | (0 << 21) | (*rs << 16) | (*rd << 11) | (0 << 6) | 0x21)
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for MOVE instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble load immediate pseudo-instruction (li $rt, imm)
    fn assemble_li(&self, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "LI instruction requires a register and an immediate value".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2]) {
            (Token::Register(rt), Token::Immediate(imm)) => {
                if *imm >= -32768 && *imm <= 32767 {
                    // Small immediate, use "addi $rt, $zero, imm"
                    // opcode (6 bits) | zero (5 bits) | rt (5 bits) | immediate (16 bits)
                    Ok((0x08 << 26) | (0 << 21) | (*rt << 16) | (*imm as u32 & 0xFFFF))
                } else {
                    // Large immediate, split into two instructions
                    // This case is tricky since we can't return two instructions
                    // For now, just return the first instruction and rely on the assembler to expand this
                    let upper = (*imm >> 16) & 0xFFFF;

                    // First instruction: "lui $rt, upper"
                    // opcode (6 bits) | 0 (5 bits) | rt (5 bits) | upper (16 bits)
                    let lui = (0x0F << 26) | (0 << 21) | (*rt << 16) | (upper as u32 & 0xFFFF);

                    // Warn about truncated expansion
                    println!(
                        "Warning: LI expansion truncated to first instruction at line {}",
                        self.current_line
                    );

                    Ok(lui)
                }
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for LI instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble load address pseudo-instruction (la $rt, symbol)
    fn assemble_la(&self, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "LA instruction requires a register and a symbol".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2]) {
            (Token::Register(rt), Token::Symbol(symbol)) => {
                // Look up symbol value
                if let Some(&addr) = self.labels.get(symbol) {
                    let upper = (addr >> 16) & 0xFFFF;

                    // First instruction: "lui $rt, upper"
                    // opcode (6 bits) | 0 (5 bits) | rt (5 bits) | upper (16 bits)
                    let lui = (0x0F << 26) | (0 << 21) | (*rt << 16) | (upper as u32);

                    // Warn about truncated expansion
                    println!(
                        "Warning: LA expansion truncated to first instruction at line {}",
                        self.current_line
                    );

                    Ok(lui)
                } else {
                    Err(AssemblerError::SymbolError(
                        format!("Undefined symbol: {}", symbol),
                        self.current_line,
                    ))
                }
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for LA instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble branch unconditional pseudo-instruction (b label)
    fn assemble_b(&self, operands: &[Token]) -> Result<u32, AssemblerError> {
        if operands.is_empty() {
            return Err(AssemblerError::SyntaxError(
                "B instruction requires a label".to_string(),
                self.current_line,
            ));
        }

        match &operands[0] {
            Token::Symbol(symbol) => {
                // Look up symbol value
                if let Some(&addr) = self.labels.get(symbol) {
                    // Calculate offset for PC-relative addressing
                    let offset = (addr as i32 - (self.current_address as i32 + 4)) / 4;

                    if offset < -32768 || offset > 32767 {
                        return Err(AssemblerError::RangeError(
                            format!("Branch target out of range: {}", offset),
                            self.current_line,
                        ));
                    }

                    // Translate to "beq $zero, $zero, offset"
                    // opcode (6 bits) | zero (5 bits) | zero (5 bits) | offset (16 bits)
                    Ok((0x04 << 26) | (0 << 21) | (0 << 16) | (offset as u32 & 0xFFFF))
                } else {
                    Err(AssemblerError::SymbolError(
                        format!("Undefined symbol: {}", symbol),
                        self.current_line,
                    ))
                }
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operand for B instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble FP R-type instruction
    fn assemble_fp_r_type(
        &self,
        opcode: u32,
        fmt: u32,
        funct: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.len() < 5 {
            return Err(AssemblerError::SyntaxError(
                "FP R-type instruction requires 3 FP registers".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2], &operands[4]) {
            (Token::FpRegister(fd), Token::FpRegister(fs), Token::FpRegister(ft)) => {
                // opcode (6 bits) | fmt (5 bits) | ft (5 bits) | fs (5 bits) | fd (5 bits) | funct (6 bits)
                Ok((opcode << 26) | (fmt << 21) | (*ft << 16) | (*fs << 11) | (*fd << 6) | funct)
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for FP R-type instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble FP R-type instruction with only fd and fs
    fn assemble_fp_r_type_fs(
        &self,
        opcode: u32,
        fmt: u32,
        funct: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "FP R-type instruction requires 2 FP registers".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2]) {
            (Token::FpRegister(fd), Token::FpRegister(fs)) => {
                // opcode (6 bits) | fmt (5 bits) | 0 (5 bits) | fs (5 bits) | fd (5 bits) | funct (6 bits)
                Ok((opcode << 26) | (fmt << 21) | (0 << 16) | (*fs << 11) | (*fd << 6) | funct)
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for FP R-type instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble FP compare instruction
    fn assemble_fp_cmp(
        &self,
        opcode: u32,
        fmt: u32,
        funct: u32,
        cond: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "FP compare instruction requires 2 FP registers".to_string(),
                self.current_line,
            ));
        }

        match (&operands[0], &operands[2]) {
            (Token::FpRegister(fs), Token::FpRegister(ft)) => {
                // opcode (6 bits) | fmt (5 bits) | ft (5 bits) | fs (5 bits) | cc (3 bits) | 0 (2 bits) | funct (6 bits)
                let cc = 0; // Use condition code 0
                Ok((opcode << 26)
                    | (fmt << 21)
                    | (*ft << 16)
                    | (*fs << 11)
                    | (cc << 8)
                    | (cond << 4)
                    | funct)
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for FP compare instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble FP load/store instructions
    fn assemble_fp_load_store(
        &self,
        opcode: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "FP load/store instruction requires a register and an address".to_string(),
                self.current_line,
            ));
        }

        match &operands[0] {
            Token::FpRegister(ft) => {
                // Parse the address operand
                let (base, offset) = self.parse_address(&operands[1..])?;

                // opcode (6 bits) | base (5 bits) | ft (5 bits) | offset (16 bits)
                Ok((opcode << 26) | (base << 21) | (*ft << 16) | (offset as u32 & 0xFFFF))
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operands for FP load/store instruction".to_string(),
                self.current_line,
            )),
        }
    }

    // Assemble FP branch instructions
    fn assemble_fp_branch(
        &self,
        opcode: u32,
        fmt: u32,
        rt: u32,
        operands: &[Token],
    ) -> Result<u32, AssemblerError> {
        if operands.is_empty() {
            return Err(AssemblerError::SyntaxError(
                "FP branch instruction requires a label".to_string(),
                self.current_line,
            ));
        }

        match &operands[0] {
            Token::Symbol(symbol) => {
                // Look up symbol value
                if let Some(&addr) = self.labels.get(symbol) {
                    // Calculate offset for PC-relative addressing
                    let offset = (addr as i32 - (self.current_address as i32 + 4)) / 4;

                    if offset < -32768 || offset > 32767 {
                        return Err(AssemblerError::RangeError(
                            format!("Branch target out of range: {}", offset),
                            self.current_line,
                        ));
                    }

                    // opcode (6 bits) | fmt (5 bits) | rt (5 bits) | offset (16 bits)
                    Ok((opcode << 26) | (fmt << 21) | (rt << 16) | (offset as u32 & 0xFFFF))
                } else {
                    Err(AssemblerError::SymbolError(
                        format!("Undefined symbol: {}", symbol),
                        self.current_line,
                    ))
                }
            },
            _ => Err(AssemblerError::SyntaxError(
                "Invalid operand for FP branch instruction".to_string(),
                self.current_line,
            )),
        }
    }
}
