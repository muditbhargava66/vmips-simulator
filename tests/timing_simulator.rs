// tests/timing_simulator.rs
use vmips_rust::timing_simulator::simulator::Simulator;
use vmips_rust::timing_simulator::config::{PipelineConfig, CacheConfig, ReplacementPolicy};

#[test]
fn test_timing_simulator() {
    let pipeline_config = PipelineConfig {
        num_stages: 5,
        stage_latencies: vec![1, 1, 1, 1, 1],
    };
    let instr_cache_config = CacheConfig {
        size: 256,
        associativity: 2,
        block_size: 64,
        replacement_policy: ReplacementPolicy::LRU,
    };
    let data_cache_config = CacheConfig {
        size: 512,
        associativity: 4,
        block_size: 64,
        replacement_policy: ReplacementPolicy::LRU,
    };
    let memory_size = 1024;
    let mut simulator = Simulator::new(
        pipeline_config,
        instr_cache_config,
        data_cache_config,
        memory_size,
    );

    // Load a test program into memory
    let program = vec![
        0x00000000, // Add your test program instructions here
        0x00000000,
        0x00000000,
        // ...
    ];
    simulator.memory.data[..program.len()].copy_from_slice(&program);

    // Run the timing simulator
    simulator.run();

    // Add your test assertions here
    // assert_eq!(simulator.registers.read(/* Register number */), /* Expected value */);
    // assert_eq!(simulator.memory.read_word(/* Memory address */), /* Expected value */);
    assert_eq!(simulator.registers.read(2), 42);
    assert_eq!(simulator.memory.read_word(0x1000), 0xDEADBEEF);
    // ...
}