// main.rs
use std::env;
use vmips_rust::functional_simulator::simulator::Simulator as FunctionalSimulator;
use vmips_rust::functional_simulator::memory::Memory;
use vmips_rust::timing_simulator::simulator::Simulator as TimingSimulator;
use vmips_rust::timing_simulator::config::{CacheConfig, ReplacementPolicy};
use vmips_rust::utils::logger::{Logger, LogLevel};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: vmips_rust <simulator_type> [options]");
        println!("Simulator types:");
        println!("  - functional: Run the functional simulator");
        println!("  - timing: Run the timing simulator");
        return;
    }

    let simulator_type = &args[1];
    let log_file = "vmips_rust.log";
    let mut logger = Logger::new(Some(log_file), LogLevel::Debug);

    match simulator_type.as_str() {
        "functional" => {
            let memory_size = 8192; // Adjust the memory size as needed
            let mut simulator = FunctionalSimulator::new(memory_size);
    
            // Load program into memory
            let program = vec![
                0x00000000u32, // nop
                0x00000000u32, // nop
                0x8C020000u32, // lw $2, 0($0)
                0x8C030004u32, // lw $3, 4($0)
                0x00430820u32, // add $1, $2, $3
                0xAC010008u32, // sw $1, 8($0)
            ];
            let program_bytes = unsafe {
                std::slice::from_raw_parts(
                    program.as_ptr() as *const u8,
                    program.len() * std::mem::size_of::<u32>(),
                )
            };
            simulator.load_program(program_bytes);
    
            // Run the functional simulator
            simulator.run();
    
            // Log the final state
            logger.info(&format!("Registers: {:?}", simulator.registers));
            logger.info(&format!("Memory: {:?}", simulator.memory.data));
        }
        "timing" => {
            let pipeline_config = vmips_rust::timing_simulator::config::PipelineConfig {
                num_stages: 5,
                stage_latencies: vec![1, 1, 1, 1, 1],
            };
            let instr_cache_config = CacheConfig {
                size: 2048, // Increase the cache size
                associativity: 4,
                block_size: 64,
                replacement_policy: ReplacementPolicy::LRU,
            };
            let data_cache_config = CacheConfig {
                size: 2048, // Increase the cache size
                associativity: 4,
                block_size: 64,
                replacement_policy: ReplacementPolicy::LRU,
            };
            let memory_size = 8192; // Adjust the memory size as needed
            let mut simulator = TimingSimulator::new(
                pipeline_config,
                instr_cache_config,
                data_cache_config,
                memory_size, // Pass memory_size instead of memory
            );

            // Load program into memory
            let program = vec![
                0x00000000, // Add your program instructions here
                0x00000000,
                0x00000000,
                // ...
            ];
            simulator.memory.data[..program.len()].copy_from_slice(&program);

            // Run the timing simulator
            simulator.run();

            // Log the final state
            logger.info(&format!("Registers: {:?}", simulator.registers));
            logger.info(&format!("Memory: {:?}", simulator.memory.data));
        }
        _ => {
            println!("Invalid simulator type: {}", simulator_type);
            return;
        }
    }

    println!("Simulation completed. Log file: {}", log_file);
}