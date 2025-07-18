use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vmips_rust::functional_simulator::instructions::Instruction;
use vmips_rust::functional_simulator::memory::Memory;
use vmips_rust::functional_simulator::simulator::Simulator as FunctionalSimulator;
use vmips_rust::timing_simulator::config::{BranchPredictorType, CacheConfig, PipelineConfig};
use vmips_rust::timing_simulator::simulator::{ExecutionMode, Simulator as TimingSimulator};

fn create_test_program() -> Vec<u8> {
    let program_words = vec![
        0x8C021000u32, // lw $2, 0x1000($0)
        0x8C031004u32, // lw $3, 0x1004($0)
        0x00431020u32, // add $2, $2, $3
        0xAC021008u32, // sw $2, 0x1008($0)
        0x00000000u32, // nop
    ];

    let mut program_bytes = Vec::with_capacity(program_words.len() * 4);
    for &word in &program_words {
        program_bytes.extend_from_slice(&word.to_le_bytes());
    }

    program_bytes
}

fn functional_simulator_benchmark(c: &mut Criterion) {
    let program = create_test_program();

    c.bench_function("functional_simulator_execution", |b| {
        b.iter(|| {
            let mut simulator = FunctionalSimulator::new(8192);

            // Load test data
            simulator.memory.write_word_init(0x1000, 10);
            simulator.memory.write_word_init(0x1004, 20);

            // Load program
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

            // Run simulation
            simulator.run();

            black_box(simulator.registers.read(2));
        });
    });
}

fn timing_simulator_benchmark(c: &mut Criterion) {
    let program = create_test_program();

    c.bench_function("timing_simulator_execution", |b| {
        b.iter(|| {
            let pipeline_config = PipelineConfig::new(5)
                .with_latencies(vec![1, 1, 1, 1, 1])
                .with_forwarding(true)
                .with_branch_prediction(true, BranchPredictorType::TwoBit)
                .with_superscalar(1);

            let instr_cache_config = CacheConfig::new(32768, 4, 64);
            let data_cache_config = CacheConfig::new(32768, 4, 64);

            let mut simulator =
                TimingSimulator::new(pipeline_config, instr_cache_config, data_cache_config, 8192);

            // Load test data
            simulator.memory.write_word_init(0x1000, 10);
            simulator.memory.write_word_init(0x1004, 20);

            // Load program
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

            // Run for a fixed number of cycles
            simulator.pc = 0;
            for _ in 0..20 {
                simulator.step();
            }

            black_box(simulator.registers.read(2));
        });
    });
}

fn memory_access_benchmark(c: &mut Criterion) {
    c.bench_function("memory_read_write", |b| {
        let mut memory = Memory::new(65536);

        b.iter(|| {
            for i in 0..1000 {
                memory.write_word_init(i * 4, i as u32);
                black_box(memory.read_word(i * 4));
            }
        });
    });
}

criterion_group!(
    benches,
    functional_simulator_benchmark,
    timing_simulator_benchmark,
    memory_access_benchmark
);
criterion_main!(benches);
