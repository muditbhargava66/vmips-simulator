// main.rs
use std::env;
use vmips_rust::functional_simulator::simulator::Simulator as FunctionalSimulator;
use vmips_rust::timing_simulator::simulator::Simulator as TimingSimulator;
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
            let memory_size = 1024; // Adjust the memory size as needed
            let mut simulator = FunctionalSimulator::new(memory_size);

            // Load program into memory
            let program = vec![
                0x00000000, // Add your program instructions here
                0x00000000,
                0x00000000,
                // ...
            ];
            simulator.load_program(&program);

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
            let instr_cache_config = vmips_rust::timing_simulator::config::CacheConfig {
                size: 256,
                associativity: 2,
                block_size: 64,
                replacement_policy: vmips_rust::timing_simulator::config::ReplacementPolicy::LRU,
            };
            let data_cache_config = vmips_rust::timing_simulator::config::CacheConfig {
                size: 512,
                associativity: 4,
                block_size: 64,
                replacement_policy: vmips_rust::timing_simulator::config::ReplacementPolicy::LRU,
            };
            let memory_size = 1024; // Adjust the memory size as needed
            let mut simulator = TimingSimulator::new(
                pipeline_config,
                instr_cache_config,
                data_cache_config,
                memory_size,
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