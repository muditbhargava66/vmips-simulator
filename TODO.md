# TODO for VMIPS Rust v0.2.0

## High Priority
- [x] Add support for remaining common MIPS instructions
  - [x] Implement floating-point operations
  - [x] Add shift variable instructions (SLLV, SRLV, SRAV)
  - [x] Support system call (SYSCALL) instruction
- [x] Improve timing simulator to more accurately model pipeline stalls
- [x] Enhance cache simulation with more realistic timing models
- [x] Implement a simple text-based MIPS assembler

## Medium Priority
- [x] Add visualization for pipeline stages and hazards
- [ ] Create a better CLI with more options for controlling simulation
- [x] Implement memory-mapped I/O for simulated devices
- [ ] Add support for loading ELF binaries
- [x] Create more example programs demonstrating architectural concepts

## Low Priority
- [x] Optimize simulation performance for large programs
- [x] Add statistical reporting for cache hits/misses and branch prediction
- [ ] Create a web-based frontend for the simulator
- [x] Implement parallel execution for timing simulator
- [ ] Add support for custom instruction extensions

## Documentation
- [ ] Create comprehensive API documentation
- [x] Add more detailed comments explaining simulator logic
- [ ] Create tutorials for common use cases
- [x] Add architecture diagrams for pipeline and cache

## Testing
- [ ] Add more unit tests for edge cases
- [x] Create integration tests with complete programs
- [x] Add performance benchmarks (Added in benches/)
- [ ] Implement property-based testing

## Project Infrastructure
- [x] Add CI/CD pipeline
- [ ] Create automated release process
- [x] Add code coverage reporting
- [x] Implement linting and code formatting checks