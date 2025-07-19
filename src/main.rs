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

// main.rs
//
// This file contains the main entry point for the MIPS simulator.
// It provides a command-line interface for running the functional or timing
// simulator with a test program.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use vmips_rust::elf_loader::ElfLoader;
use vmips_rust::functional_simulator::instructions::Instruction;
use vmips_rust::functional_simulator::memory::Memory;
use vmips_rust::functional_simulator::simulator::decode_instruction;
use vmips_rust::functional_simulator::simulator::Simulator as FunctionalSimulator;
use vmips_rust::timing_simulator::config::{BranchPredictorType, CacheConfig, PipelineConfig};
use vmips_rust::timing_simulator::simulator::{ExecutionMode, Simulator as TimingSimulator};
use vmips_rust::timing_simulator::pipeline::PipelineStageStatus;
use vmips_rust::utils::logger::{LogLevel, Logger};

#[derive(Parser)]
#[command(name = "vmips_rust")]
#[command(about = "A MIPS processor simulator written in Rust")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the functional simulator
    Functional {
        /// Input assembly or ELF file
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Memory size in bytes
        #[arg(short, long, default_value = "8192")]
        memory_size: usize,

        /// Log level (error, warn, info, debug)
        #[arg(short, long, default_value = "info")]
        log_level: String,

        /// Output log file
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Load as ELF binary instead of raw assembly
        #[arg(long)]
        elf: bool,
    },
    /// Run the timing simulator
    Timing {
        /// Input assembly or ELF file
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Memory size in bytes
        #[arg(short, long, default_value = "8192")]
        memory_size: usize,

        /// Log level (error, warn, info, debug)
        #[arg(short, long, default_value = "info")]
        log_level: String,

        /// Output log file
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Enable pipeline visualization
        #[arg(short, long)]
        visualize: bool,

        /// Maximum cycles to simulate
        #[arg(long, default_value = "1000")]
        max_cycles: usize,

        /// Load as ELF binary instead of raw assembly
        #[arg(long)]
        elf: bool,
    },
}

// Helper function to load data into memory
fn load_test_data(memory: &mut Memory) {
    // Store some test values in memory using the initialization method
    memory.write_word_init(0x1000, 10);
    memory.write_word_init(0x1004, 20);
    memory.write_word_init(0x1008, 30);
    memory.write_word_init(0x100C, 40);

    println!("Test data loaded into memory at addresses 0x1000-0x100C");
}

// Helper function to load program from file or create test program
fn load_program(
    input_file: Option<&PathBuf>,
    is_elf: bool,
) -> Result<(Vec<u8>, Option<u32>), Box<dyn std::error::Error>> {
    if let Some(file_path) = input_file {
        if is_elf {
            // Load ELF binary
            let elf_loader = ElfLoader::load_file(file_path)?;
            let entry_point = elf_loader.entry_point();

            // For ELF files, we'll return empty program data since the loader
            // will handle loading into memory directly
            Ok((Vec::new(), Some(entry_point)))
        } else {
            // Load raw binary or assembly file
            use std::fs;
            let data = fs::read(file_path)?;
            Ok((data, None))
        }
    } else {
        // Create default test program
        Ok((create_test_program(), None))
    }
}

// Helper function to create a simple test program
fn create_test_program() -> Vec<u8> {
    // Create program as u32 values
    let program_words = vec![
        0x8C021000u32, // lw $2, 0x1000($0)     - Load from 0x1000 (value 10)
        0x8C031004u32, // lw $3, 0x1004($0)     - Load from 0x1004 (value 20)
        0x00431020u32, // add $2, $2, $3        - Add values (10+20=30)
        0xAC021008u32, // sw $2, 0x1008($0)     - Store at 0x1008
        // Multiplication
        0x8C021000u32, // lw $2, 0x1000($0)     - Load again from 0x1000 (value 10)
        0x8C031004u32, // lw $3, 0x1004($0)     - Load again from 0x1004 (value 20)
        0x00430018u32, // mult $2, $3           - Multiply (10*20=200)
        0x00001012u32, // mflo $2               - Get multiplication result
        0xAC02100Cu32, // sw $2, 0x100C($0)     - Store result at 0x100C
        // Explicit termination - multiple NOPs
        0x00000000u32, // nop
        0x00000000u32, // nop
        0x00000000u32, // nop
        0x00000000u32, // nop
        0x00000000u32, // nop
    ];

    // Convert to bytes with explicit endianness
    let mut program_bytes = Vec::with_capacity(program_words.len() * 4);
    for &word in &program_words {
        program_bytes.extend_from_slice(&word.to_le_bytes());
    }

    program_bytes
}

// Helper function to display memory contents
fn display_memory_values(memory: &Memory) {
    println!("\nMemory Contents:");
    println!("Address 0x1000: {:?}", memory.read_word(0x1000));
    println!("Address 0x1004: {:?}", memory.read_word(0x1004));
    println!("Address 0x1008: {:?}", memory.read_word(0x1008));
    println!("Address 0x100C: {:?}", memory.read_word(0x100C));
    println!("Address 0x1010: {:?}", memory.read_word(0x1010));
}

// Run the functional simulator with the given program
fn run_functional_simulator(
    program: &[u8],
    memory_size: usize,
    _entry_point: Option<u32>,
    input_file: Option<&PathBuf>,
    is_elf: bool,
) {
    let mut simulator = FunctionalSimulator::new(memory_size);

    // Handle ELF loading or regular program loading
    if is_elf && input_file.is_some() {
        // Load ELF file directly into memory
        match ElfLoader::load_file(input_file.unwrap()) {
            Ok(elf_loader) => {
                if let Err(e) = elf_loader.load_into_memory(&mut simulator.memory) {
                    eprintln!("Failed to load ELF into memory: {:?}", e);
                    return;
                }
                println!("ELF binary loaded successfully");
                let segments = elf_loader.get_segments();
                for (vaddr, size, flags) in segments {
                    println!(
                        "  Segment: 0x{:08X} - 0x{:08X} (flags: 0x{:X})",
                        vaddr,
                        vaddr + size,
                        flags
                    );
                }
            },
            Err(e) => {
                eprintln!("Failed to load ELF file: {:?}", e);
                return;
            },
        }
    } else {
        // First clear and then initialize memory with test data
        load_test_data(&mut simulator.memory);

        // Debug the program bytes
        println!("Loading program of size {} bytes", program.len());
        if program.len() >= 4 {
            let first_instruction =
                u32::from_le_bytes([program[0], program[1], program[2], program[3]]);
            println!("First instruction: 0x{:08X}", first_instruction);
        }

        // Dump all program instructions for debugging
        println!("Program instructions:");
        for i in (0..program.len()).step_by(4) {
            if i + 3 < program.len() {
                let instruction = u32::from_le_bytes([
                    program[i],
                    program[i + 1],
                    program[i + 2],
                    program[i + 3],
                ]);
                println!("  0x{:04X}: 0x{:08X}", i, instruction);
            }
        }

        // Load program into simulator using write_word_init to bypass permissions
        for i in (0..program.len()).step_by(4) {
            if i + 3 < program.len() {
                let instruction = u32::from_le_bytes([
                    program[i],
                    program[i + 1],
                    program[i + 2],
                    program[i + 3],
                ]);
                simulator.memory.write_word_init(i, instruction);
            }
        }
    }

    println!(
        "Program loaded. PC: 0x{:08X}, SP: 0x{:08X}",
        0,
        simulator.registers.read(29)
    ); // Using 0 as placeholder since pc is private

    // Verify memory values before running
    println!("\nVerifying memory values before execution:");
    println!("Address 0x1000: {:?}", simulator.memory.read_word(0x1000));
    println!("Address 0x1004: {:?}", simulator.memory.read_word(0x1004));
    println!(
        "First instruction at 0x0000: {:?}",
        simulator.memory.read_word(0)
    );

    println!("Running functional simulator...");

    // Run the functional simulator
    simulator.run();

    // Display final state
    println!("\nSimulation completed.");
    println!("Final register values:");
    for i in 0..8 {
        print!("${}: {}\t", i, simulator.registers.read(i));
        if i % 4 == 3 {
            println!();
        }
    }

    // Display memory contents
    display_memory_values(&simulator.memory);
}

// Run the timing simulator with the given program and options
fn run_timing_simulator_with_options(
    program: &[u8],
    memory_size: usize,
    visualize: bool,
    max_cycles: usize,
    entry_point: Option<u32>,
    input_file: Option<&PathBuf>,
    is_elf: bool,
) {
    // Create pipeline configuration with builder pattern
    let pipeline_config = PipelineConfig::new(5)
        .with_latencies(vec![1, 1, 1, 1, 1])
        .with_forwarding(true)
        .with_branch_prediction(true, BranchPredictorType::TwoBit)
        .with_superscalar(1);

    // Fix the CacheConfig initializations
    let instr_cache_config = CacheConfig::new(32768, 4, 64);

    let data_cache_config = CacheConfig::new(32768, 4, 64);

    // Create and initialize the timing simulator
    let mut simulator = TimingSimulator::new(
        pipeline_config,
        instr_cache_config,
        data_cache_config,
        memory_size,
    );

    // Enable visualization based on CLI flag
    simulator.enable_visualization(visualize);

    if visualize {
        // Configure visualization options
        simulator.configure_visualization(true, true);

        // Set visualization format - use Text format for standard output
        use vmips_rust::timing_simulator::visualization::OutputFormat;
        simulator.set_visualization_format(OutputFormat::Text);
    }

    // Handle ELF loading or regular program loading
    if is_elf && input_file.is_some() {
        // Load ELF file directly into memory
        match ElfLoader::load_file(input_file.unwrap()) {
            Ok(elf_loader) => {
                if let Err(e) = elf_loader.load_into_memory(&mut simulator.memory) {
                    eprintln!("Failed to load ELF into memory: {:?}", e);
                    return;
                }
                println!("ELF binary loaded successfully");
                if let Some(entry) = entry_point {
                    simulator.pc = entry;
                    println!("Entry point set to: 0x{:08X}", entry);
                }
            },
            Err(e) => {
                eprintln!("Failed to load ELF file: {:?}", e);
                return;
            },
        }
    } else {
        // Initialize memory with test data
        load_test_data(&mut simulator.memory);

        // Load the program correctly
        println!("Loading program of size {} bytes", program.len());

        // Print program instructions for debugging
        println!("Program instructions:");
        for i in (0..program.len()).step_by(4) {
            if i + 3 < program.len() {
                let instruction = u32::from_le_bytes([
                    program[i],
                    program[i + 1],
                    program[i + 2],
                    program[i + 3],
                ]);
                println!("  0x{:04X}: 0x{:08X}", i, instruction);
            }
        }

        // Copy the program bytes to the beginning of memory using init method
        for i in (0..program.len()).step_by(4) {
            if i + 3 < program.len() {
                let instruction = u32::from_le_bytes([
                    program[i],
                    program[i + 1],
                    program[i + 2],
                    program[i + 3],
                ]);
                simulator.memory.write_word_init(i, instruction);
            }
        }
    }

    // Verify memory values
    println!("\nVerifying memory values before execution:");
    println!("Address 0x1000: {:?}", simulator.memory.read_word(0x1000));
    println!("Address 0x0000: {:?}", simulator.memory.read_word(0));

    println!("Running timing simulator...");

    // Use the provided max_cycles parameter
    let mut cycle_count = 0;

    // Start execution at PC = 0
    simulator.pc = 0;

    println!("Starting execution at PC: 0x{:08X}", simulator.pc);

    // Manual execution loop
    while cycle_count < max_cycles {
        cycle_count += 1;

        // Visualize the pipeline state if enabled
        if visualize && (cycle_count <= 5 || cycle_count % 10 == 0) {
            if let Some(visualization) = &simulator.visualization {
                if let ExecutionMode::InOrder(ref pipeline) = simulator.execution_mode {
                    println!(
                        "{}",
                        visualization.visualize_pipeline(pipeline, cycle_count)
                    );
                }
            }
        }

        // Print current state every 10 cycles
        if cycle_count % 10 == 0 || cycle_count < 5 {
            println!("Cycle {}, PC: 0x{:08X}", cycle_count, simulator.pc);
        }

        // Directly fetch instruction from memory
        let instr_word = match simulator.memory.read_word(simulator.pc as usize) {
            Some(word) => word,
            None => {
                println!("Memory read error at PC: 0x{:08X}", simulator.pc);
                break;
            },
        };

        // If we hit a NOP after executing a few instructions, terminate
        if instr_word == 0 && cycle_count > 5 {
            println!(
                "Reached NOP instruction at PC: 0x{:08X}, terminating",
                simulator.pc
            );
            break;
        }

        // Decode instruction
        let instruction = decode_instruction(instr_word);

        // Update pipeline stages for visualization
        if let ExecutionMode::InOrder(ref mut pipeline) = simulator.execution_mode {
            // Simulate pipeline stages by moving instructions through the pipeline
            // Move instructions from right to left (WB -> MEM -> EX -> ID -> IF)
            for i in (1..pipeline.stages.len()).rev() {
                if let Some(prev_instr) = pipeline.stages[i-1].instruction.clone() {
                    pipeline.stages[i].instruction = Some(prev_instr);
                    pipeline.stages[i].pc = pipeline.stages[i-1].pc;
                    pipeline.stages[i].status = PipelineStageStatus::Busy;
                }
            }
            
            // Add new instruction to fetch stage
            pipeline.stages[0].instruction = Some(instruction.clone());
            pipeline.stages[0].pc = simulator.pc;
            pipeline.stages[0].status = PipelineStageStatus::Busy;
        }

        // Print the instruction being executed
        if cycle_count < 20 {
            println!(
                "Executing 0x{:08X} at PC: 0x{:08X}",
                instr_word, simulator.pc
            );
        }

        // Execute instruction manually
        match instruction {
            Instruction::Add { rd, rs, rt } => {
                let rs_value = simulator.registers.read(rs);
                let rt_value = simulator.registers.read(rt);
                let result = rs_value.wrapping_add(rt_value);
                simulator.registers.write(rd, result);
                println!("  ADD ${} = ${} + ${} = {}", rd, rs, rt, result);
            },
            Instruction::Lw { rt, base, offset } => {
                let base_value = simulator.registers.read(base);
                let address = base_value.wrapping_add(offset as u32);
                match simulator.memory.read_word(address as usize) {
                    Some(value) => {
                        simulator.registers.write(rt, value);
                        println!("  LW ${} = mem[{} + {}] = {}", rt, base, offset, value);
                    },
                    None => {
                        println!("Memory read error at address 0x{:08X}", address);
                        break;
                    },
                }
            },
            Instruction::Sw { rt, base, offset } => {
                let base_value = simulator.registers.read(base);
                let address = base_value.wrapping_add(offset as u32);
                let value = simulator.registers.read(rt);
                if simulator.memory.write_word(address as usize, value) {
                    println!("  SW mem[{} + {}] = ${} = {}", base, offset, rt, value);
                } else {
                    println!("Memory write error at address 0x{:08X}", address);
                    break;
                }
            },
            Instruction::Mult { rs, rt } => {
                let rs_value = simulator.registers.read(rs);
                let rt_value = simulator.registers.read(rt);
                let result = rs_value.wrapping_mul(rt_value);

                // Ensure we have space for LO register
                if simulator.registers.data.len() <= 32 {
                    simulator.registers.data.resize(33, 0);
                }
                simulator.registers.data[32] = result;
                println!("  MULT LO = ${} * ${} = {}", rs, rt, result);
            },
            Instruction::Mflo { rd } => {
                let lo_value = if simulator.registers.data.len() > 32 {
                    simulator.registers.data[32]
                } else {
                    0
                };
                simulator.registers.write(rd, lo_value);
                println!("  MFLO ${} = LO = {}", rd, lo_value);
            },
            Instruction::Beq { rs, rt, offset } => {
                let rs_value = simulator.registers.read(rs);
                let rt_value = simulator.registers.read(rt);
                if rs_value == rt_value {
                    let new_pc = simulator.pc.wrapping_add((offset as u32) << 2);
                    println!("  BEQ ${} == ${}, jumping to 0x{:08X}", rs, rt, new_pc);
                    simulator.pc = new_pc;
                    continue; // Skip PC increment
                } else {
                    println!("  BEQ ${} != ${}, not taken", rs, rt);
                }
            },
            Instruction::Bne { rs, rt, offset } => {
                let rs_value = simulator.registers.read(rs);
                let rt_value = simulator.registers.read(rt);
                if rs_value != rt_value {
                    let new_pc = simulator.pc.wrapping_add((offset as u32) << 2);
                    println!("  BNE ${} != ${}, jumping to 0x{:08X}", rs, rt, new_pc);
                    simulator.pc = new_pc;
                    continue; // Skip PC increment
                } else {
                    println!("  BNE ${} == ${}, not taken", rs, rt);
                }
            },
            Instruction::J { target } => {
                let new_pc = (simulator.pc & 0xF0000000) | (target << 2);
                println!("  J jumping to 0x{:08X}", new_pc);
                simulator.pc = new_pc;
                continue; // Skip PC increment
            },
            Instruction::Jr { rs } => {
                let new_pc = simulator.registers.read(rs);
                println!("  JR ${} jumping to 0x{:08X}", rs, new_pc);
                simulator.pc = new_pc;
                continue; // Skip PC increment
            },
            Instruction::InvalidInstruction => {
                println!(
                    "Invalid instruction 0x{:08X} at PC: 0x{:08X}",
                    instr_word, simulator.pc
                );
                break;
            },
            _ => {
                // Instead of trying to print the Instruction which doesn't implement Debug
                println!("  Unhandled instruction type at PC: 0x{:08X}", simulator.pc);
            },
        }

        // Increment PC to next instruction
        simulator.pc += 4;
    }

    if cycle_count >= max_cycles {
        println!(
            "Reached maximum cycle count ({}). Ending simulation.",
            max_cycles
        );
    }

    println!(
        "Simulation complete after {} cycles. Final PC: 0x{:08X}",
        cycle_count, simulator.pc
    );

    // Display final state
    println!("\nSimulation completed.");
    println!("Final register values:");
    for i in 0..8 {
        print!("${}: {}\t", i, simulator.registers.read(i));
        if i % 4 == 3 {
            println!();
        }
    }

    // Display memory contents
    display_memory_values(&simulator.memory);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Functional {
            input,
            memory_size,
            log_level,
            output,
            elf,
        } => {
            // Parse log level
            let parsed_log_level = match log_level.to_lowercase().as_str() {
                "error" => LogLevel::Error,
                "warn" | "warning" => LogLevel::Warning,
                "info" => LogLevel::Info,
                "debug" => LogLevel::Debug,
                _ => LogLevel::Info,
            };

            // Create logger
            let log_file = output
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .or_else(|| Some("vmips_rust.log".to_string()));

            let mut logger = Logger::new(log_file.as_deref(), parsed_log_level);
            logger.info("Starting VMIPS Rust with functional simulator");

            // Load program from file or create test program
            match load_program(input.as_ref(), elf) {
                Ok((program, entry_point)) => {
                    run_functional_simulator(
                        &program,
                        memory_size,
                        entry_point,
                        input.as_ref(),
                        elf,
                    );
                },
                Err(e) => {
                    eprintln!("Failed to load program: {}", e);
                    return;
                },
            }

            if let Some(log_file) = log_file {
                println!("\nLog file created: {}", log_file);
            }
        },
        Commands::Timing {
            input,
            memory_size,
            log_level,
            output,
            visualize,
            max_cycles,
            elf,
        } => {
            // Parse log level
            let parsed_log_level = match log_level.to_lowercase().as_str() {
                "error" => LogLevel::Error,
                "warn" | "warning" => LogLevel::Warning,
                "info" => LogLevel::Info,
                "debug" => LogLevel::Debug,
                _ => LogLevel::Info,
            };

            // Create logger
            let log_file = output
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .or_else(|| Some("vmips_rust.log".to_string()));

            let mut logger = Logger::new(log_file.as_deref(), parsed_log_level);
            logger.info("Starting VMIPS Rust with timing simulator");

            // Load program from file or create test program
            match load_program(input.as_ref(), elf) {
                Ok((program, entry_point)) => {
                    run_timing_simulator_with_options(
                        &program,
                        memory_size,
                        visualize,
                        max_cycles,
                        entry_point,
                        input.as_ref(),
                        elf,
                    );
                },
                Err(e) => {
                    eprintln!("Failed to load program: {}", e);
                    return;
                },
            }

            if let Some(log_file) = log_file {
                println!("\nLog file created: {}", log_file);
            }
        },
    }
}
