// main_assembler.rs

use vmips_rust::assembler::Assembler;
use vmips_rust::utils::logger::{LogLevel, Logger};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    // Create logger
    let log_file = "vmips_assembler.log";
    let mut logger = Logger::new(Some(log_file), LogLevel::Debug);
    logger.info("Starting VMIPS Rust Assembler");
    
    if args.len() < 2 {
        print_usage(&args[0]);
        return Ok(());
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "assemble" | "a" => {
            if args.len() < 3 {
                println!("Error: No input file specified.");
                print_usage(&args[0]);
                return Ok(());
            }
            
            let input_file = &args[2];
            let output_file = if args.len() >= 4 { &args[3] } else { "a.out" };
            
            println!("Assembling {} to {}", input_file, output_file);
            
            let mut assembler = Assembler::new();
            match assembler.assemble_file(input_file) {
                Ok(binary_data) => {
                    let binary: Vec<u8> = binary_data;
                    let mut file = File::create(output_file)?;
                    file.write_all(&binary)?;
                    println!("Assembly successful.");
                },
                Err(err) => {
                    println!("Assembly error: {}", err);
                    return Ok(());
                },
            }
        },
        "run" | "r" => {
            if args.len() < 3 {
                println!("Error: No input file specified.");
                print_usage(&args[0]);
                return Ok(());
            }
            
            let input_file = &args[2];
            let simulator_type = if args.len() >= 4 { &args[3] } else { "functional" };
            
            println!("Assembling and running {} with {} simulator", input_file, simulator_type);
            
            // First assemble the file to memory
            let mut assembler = Assembler::new();
            let binary: Vec<u8> = match assembler.assemble_file(input_file) {
                Ok(binary_data) => binary_data,
                Err(err) => {
                    println!("Assembly error: {}", err);
                    return Ok(());
                },
            };
            
            // Then run the binary with the specified simulator
            run_simulator(&binary, simulator_type);
        },
        "interactive" | "i" => {
            run_interactive_mode();
        },
        "--help" | "-h" => {
            print_usage(&args[0]);
        },
        _ => {
            println!("Error: Unknown command '{}'", command);
            print_usage(&args[0]);
        },
    }
    
    Ok(())
}

fn print_usage(program_name: &str) {
    println!("Usage: {} <command> [options]", program_name);
    println!("Commands:");
    println!("  assemble, a <input.s> [output.bin]    Assemble a MIPS assembly file to binary");
    println!("  run, r <input.s> [simulator_type]     Assemble and run a MIPS assembly file");
    println!("  interactive, i                        Start an interactive MIPS assembly session");
    println!("  --help, -h                            Show this help message");
    println!("");
    println!("Simulator types:");
    println!("  functional                            Use the functional simulator (default)");
    println!("  timing                                Use the timing simulator");
}

fn run_simulator(binary: &[u8], simulator_type: &str) {
    use vmips_rust::functional_simulator::simulator::Simulator as FunctionalSimulator;
    use vmips_rust::timing_simulator::config::{CacheConfig, PipelineConfig, BranchPredictorType};
    use vmips_rust::timing_simulator::simulator::Simulator as TimingSimulator;
    
    match simulator_type {
        "functional" => {
            let memory_size = 8192;
            let mut simulator = FunctionalSimulator::new(memory_size);
            
            // Load the binary into memory
            simulator.load_program(binary);
            
            // Run the simulator
            println!("Running functional simulator...");
            simulator.run();
            
            // Display results
            println!("\nSimulation completed.");
            println!("Final register values:");
            println!("{}", simulator.registers.dump_registers());
        },
        "timing" => {
            // Create pipeline configuration using the builder pattern
            let pipeline_config = PipelineConfig::new(5)
                .with_latencies(vec![1, 1, 1, 1, 1])
                .with_forwarding(true)
                .with_branch_prediction(true, BranchPredictorType::TwoBit);
            
            // Create cache configurations
            let l1_cache_config = CacheConfig::new(32768, 4, 64);
            let l2_cache_config = CacheConfig::new(262144, 8, 64);
            
            // Create simulator
            let mut simulator = TimingSimulator::new(
                pipeline_config,
                l1_cache_config,
                l2_cache_config,
                8192, // memory size
            );
            
            // Load the binary into memory
            let program_bytes = binary;
            
            // Skip the header (first 8 bytes) which contains data and text section sizes
            let data_size = u32::from_le_bytes([
                program_bytes[0], program_bytes[1], program_bytes[2], program_bytes[3]
            ]) as usize;
            
            let text_size = u32::from_le_bytes([
                program_bytes[4], program_bytes[5], program_bytes[6], program_bytes[7]
            ]) as usize;
            
            // Load data section
            for (i, &byte) in program_bytes[8..8 + data_size].iter().enumerate() {
                simulator.memory.write_byte(i, byte);
            }
            
            // Load text section
            for (i, chunk) in program_bytes[8 + data_size..8 + data_size + text_size].chunks_exact(4).enumerate() {
                let instr = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                simulator.memory.write_word(0x1000 + i * 4, instr);
            }
            
            // Set PC to start of text section
            simulator.pc = 0x1000;
            
            // Run the simulator
            println!("Running timing simulator...");
            simulator.run();
            
            // Display results
            println!("\nSimulation completed.");
            println!("Final register values:");
            println!("{}", simulator.registers.dump_registers());
        },
        _ => {
            println!("Error: Unknown simulator type '{}'", simulator_type);
            println!("Valid simulator types are 'functional' and 'timing'");
        },
    }
}

fn run_interactive_mode() {
    println!("VMIPS Rust Interactive Assembler");
    println!("Type MIPS assembly code, one instruction per line.");
    println!("Enter an empty line to assemble and run the program.");
    println!("Type 'exit' or 'quit' to quit.");
    
    let stdin = io::stdin();
    let mut program = String::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        
        let line = line.trim();
        
        if line.is_empty() {
            if !program.is_empty() {
                println!("Assembling and running program...");
                
                // Wrap the program in .text section
                let full_program = format!(".text\n{}", program);
                
                let mut assembler = Assembler::new();
                match assembler.assemble_string(&full_program) {
                    Ok(binary_data) => {
                        let binary: Vec<u8> = binary_data;
                        run_simulator(&binary, "functional");
                        program.clear();
                    },
                    Err(err) => {
                        println!("Assembly error: {}", err);
                    },
                }
            }
        } else if line == "exit" || line == "quit" {
            break;
        } else {
            program.push_str(line);
            program.push('\n');
        }
    }
}