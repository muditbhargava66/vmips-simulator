// main.rs
use std::env;
use vmips_rust::functional_simulator::instructions::Instruction;
use vmips_rust::functional_simulator::memory::Memory;
use vmips_rust::functional_simulator::simulator::decode_instruction;
use vmips_rust::functional_simulator::simulator::Simulator as FunctionalSimulator;
use vmips_rust::timing_simulator::config::{
    CacheConfig, PipelineConfig, BranchPredictorType
};
use vmips_rust::timing_simulator::simulator::{Simulator as TimingSimulator, ExecutionMode};
use vmips_rust::utils::logger::{LogLevel, Logger};

// Helper function to load data into memory
fn load_test_data(memory: &mut Memory) {
    // Store some test values in memory using the initialization method
    memory.write_word_init(0x1000, 10);
    memory.write_word_init(0x1004, 20);
    memory.write_word_init(0x1008, 30);
    memory.write_word_init(0x100C, 40);

    println!("Test data loaded into memory at addresses 0x1000-0x100C");
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
fn run_functional_simulator(program: &[u8], memory_size: usize) {
    let mut simulator = FunctionalSimulator::new(memory_size);

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
            let instruction =
                u32::from_le_bytes([program[i], program[i + 1], program[i + 2], program[i + 3]]);
            println!("  0x{:04X}: 0x{:08X}", i, instruction);
        }
    }

    // Load program into simulator using write_word_init to bypass permissions
    for i in (0..program.len()).step_by(4) {
        if i + 3 < program.len() {
            let instruction = u32::from_le_bytes([
                program[i], program[i + 1], program[i + 2], program[i + 3],
            ]);
            simulator.memory.write_word_init(i, instruction);
        }
    }

    println!("Program loaded. PC: 0x{:08X}, SP: 0x{:08X}", 
         0, simulator.registers.read(29)); // Using 0 as placeholder since pc is private

    // Verify memory values before running
    println!("\nVerifying memory values before execution:");
    println!("Address 0x1000: {:?}", simulator.memory.read_word(0x1000));
    println!("Address 0x1004: {:?}", simulator.memory.read_word(0x1004));
    println!("First instruction at 0x0000: {:?}", simulator.memory.read_word(0));

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

// Run the timing simulator with the given program
fn run_timing_simulator(program: &[u8], memory_size: usize) {
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

    // Enable visualization (new feature)
    simulator.enable_visualization(true);
    
    // Configure visualization options (new feature)
    simulator.configure_visualization(true, true);
    
    // Set visualization format - use Text format for standard output (new feature)
    use vmips_rust::timing_simulator::visualization::OutputFormat;
    simulator.set_visualization_format(OutputFormat::Text);

    // Initialize memory with test data
    load_test_data(&mut simulator.memory);

    // Load the program correctly
    println!("Loading program of size {} bytes", program.len());

    // Print program instructions for debugging
    println!("Program instructions:");
    for i in (0..program.len()).step_by(4) {
        if i + 3 < program.len() {
            let instruction =
                u32::from_le_bytes([program[i], program[i + 1], program[i + 2], program[i + 3]]);
            println!("  0x{:04X}: 0x{:08X}", i, instruction);
        }
    }

    // Copy the program bytes to the beginning of memory using init method
    for i in (0..program.len()).step_by(4) {
        if i + 3 < program.len() {
            let instruction = u32::from_le_bytes([
                program[i], program[i + 1], program[i + 2], program[i + 3],
            ]);
            simulator.memory.write_word_init(i, instruction);
        }
    }
    
    // Verify memory values
    println!("\nVerifying memory values before execution:");
    println!("Address 0x1000: {:?}", simulator.memory.read_word(0x1000));
    println!("Address 0x0000: {:?}", simulator.memory.read_word(0));

    println!("Running timing simulator...");

    // Set a maximum number of cycles to prevent infinite loops
    let max_cycles = 100;
    let mut cycle_count = 0;

    // Start execution at PC = 0
    simulator.pc = 0;

    println!("Starting execution at PC: 0x{:08X}", simulator.pc);

    // Manual execution loop
    while cycle_count < max_cycles {
        cycle_count += 1;

        // Visualize the pipeline state if enabled (new feature)
        if cycle_count <= 5 || cycle_count % 10 == 0 {
            if let Some(visualization) = &simulator.visualization {
                if let ExecutionMode::InOrder(ref pipeline) = simulator.execution_mode {
                    println!("{}", visualization.visualize_pipeline(pipeline, cycle_count));
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
    let args: Vec<String> = env::args().collect();

    let mut simulator_type = "functional";
    let mut memory_size = 8192;

    // Parse command line arguments
    if args.len() > 1 {
        simulator_type = &args[1];
    }

    // Allow specifying memory size as second argument
    if args.len() > 2 {
        if let Ok(size) = args[2].parse::<usize>() {
            memory_size = size;
        }
    }

    // Create logger
    let log_file = "vmips_rust.log";
    let mut logger = Logger::new(Some(log_file), LogLevel::Debug);
    logger.info(&format!(
        "Starting VMIPS Rust with {} simulator",
        simulator_type
    ));

    // Create test program
    let program = create_test_program();

    // Run appropriate simulator based on command line argument
    match simulator_type {
        "functional" => {
            run_functional_simulator(&program, memory_size);
        },
        "timing" => {
            run_timing_simulator(&program, memory_size);
        },
        _ => {
            println!("Invalid simulator type: {}", simulator_type);
            println!("Usage: vmips_rust <simulator_type> [memory_size]");
            println!("Simulator types:");
            println!("  - functional: Run the functional simulator");
            println!("  - timing: Run the timing simulator");
            return;
        },
    }

    println!("\nLog file created: {}", log_file);
}
