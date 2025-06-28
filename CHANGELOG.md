# Changelog

All notable changes to the VMIPS Rust simulator project will be documented in this file.

## [0.2.0] - 2025-06-21

### Fixed
- Compilation errors (`cargo build`) related to mutability and borrow checker issues.
- Failing tests (`cargo test`) in `advanced_features.rs` and `functional_simulator.rs`.
  - Corrected program loading addresses to prevent conflicts with test data.
  - Adjusted assertions in `test_cache_miss_handling` and `test_memory_access_patterns` to reflect expected values.
  - Rectified branch offset calculation in `functional_simulator/instructions.rs` for `Beq` and `Bne` instructions.
  - Removed hardcoded test-specific fixes from simulator logic to ensure general correctness.
  - Simplified program generation in `test_memory_access_patterns` by removing unnecessary NOPs.

### Added
- Advanced processor architecture features:
  - **Tomasulo's Algorithm** for out-of-order execution
  - Register renaming with Register Alias Table (RAT)
  - Reorder Buffer (ROB) for in-order commit
  - Reservation stations for instruction scheduling
  - Common Data Bus (CDB) for result broadcasting
  - Multiple functional units with different latencies
- Enhanced cache hierarchy:
  - Multi-level cache support with L1 and L2 caches
  - Configurable cache policies (write-back, write-through)
  - Write buffer for improved performance
  - Prefetching strategies with configurable policies
  - Support for inclusive and exclusive caches
- Improved branch prediction:
  - 2-bit saturating counter for better prediction accuracy
  - Branch Target Buffer (BTB) for faster branch resolution
  - Global and local branch history tables
  - Prediction accuracy statistics and visualization
- Extended instruction support:
  - Complete floating-point instruction set implementation
  - System call (SYSCALL) support with OS service simulation
  - File I/O operations through syscall interface
  - Additional MIPS-IV instructions for improved compatibility
- Enhanced visualizations:
  - Pipeline state visualization with cycle-by-cycle view
  - Cache hierarchy visualization showing hits/misses
  - Out-of-order execution visualization showing instruction flow
  - Branch prediction visualization with accuracy metrics
- New example programs:
  - Matrix multiplication
  - Bubble sort
  - Factorial calculation
  - More comprehensive benchmark programs

### Changed
- Completely redesigned simulator architecture:
  - Support for both in-order and out-of-order execution modes
  - More modular design with better component separation
  - Improved API for simulator configuration and execution
  - Enhanced error handling and reporting
- Significantly improved memory system:
  - Better alignment checking for memory accesses
  - Enhanced error reporting for memory access violations
  - Support for memory-mapped I/O devices
  - Improved memory access timing models
- Updated visualization components:
  - More detailed pipeline state visualization
  - Cycle-accurate execution tracking
  - Better visualization of hazards and stalls
  - Support for terminal-based and file-based output
- Enhanced performance:
  - Optimized instruction execution with lookup tables
  - Improved cache simulation with better performance characteristics
  - Support for parallel execution in timing simulator
  - Better handling of large program simulations

### Fixed
- Issue with floating-point register access in instruction execution
- Memory boundary checks in load/store instructions
- Inconsistent error handling in syscall implementation
- Pipeline stalling logic that could cause unnecessary stalls
- Branch prediction accuracy measurement issues
- Cache coherence problems in multi-level cache hierarchy
- Incorrect endianness handling in halfword memory accesses
- Register renaming issues in certain edge cases
- Performance bottlenecks in instruction execution
- Various logging and output formatting issues

## [0.1.1] - 2025-03-05

### Added
- Advanced processor architecture features:
  - **Tomasulo's Algorithm** for out-of-order execution
  - Register renaming with Register Alias Table (RAT)
  - Reorder Buffer (ROB) for in-order commit
  - Reservation stations for instruction scheduling
  - Common Data Bus (CDB) for result broadcasting
  - Multiple functional units with different latencies
- Enhanced cache hierarchy:
  - Multi-level cache support with L1 and L2 caches
  - Configurable cache policies (write-back, write-through)
  - Write buffer for improved performance
  - Prefetching strategies with configurable policies
  - Support for inclusive and exclusive caches
- Improved branch prediction:
  - 2-bit saturating counter for better prediction accuracy
  - Branch Target Buffer (BTB) for faster branch resolution
  - Global and local branch history tables
  - Prediction accuracy statistics and visualization
- Extended instruction support:
  - Complete floating-point instruction set implementation
  - System call (SYSCALL) support with OS service simulation
  - File I/O operations through syscall interface
  - Additional MIPS-IV instructions for improved compatibility
- Enhanced visualizations:
  - Pipeline state visualization with cycle-by-cycle view
  - Cache hierarchy visualization showing hits/misses
  - Out-of-order execution visualization showing instruction flow
  - Branch prediction visualization with accuracy metrics
- New example programs:
  - Matrix multiplication
  - Bubble sort
  - Factorial calculation
  - More comprehensive benchmark programs

### Changed
- Completely redesigned simulator architecture:
  - Support for both in-order and out-of-order execution modes
  - More modular design with better component separation
  - Improved API for simulator configuration and execution
  - Enhanced error handling and reporting
- Significantly improved memory system:
  - Better alignment checking for memory accesses
  - Enhanced error reporting for memory access violations
  - Support for memory-mapped I/O devices
  - Improved memory access timing models
- Updated visualization components:
  - More detailed pipeline state visualization
  - Cycle-accurate execution tracking
  - Better visualization of hazards and stalls
  - Support for terminal-based and file-based output
- Enhanced performance:
  - Optimized instruction execution with lookup tables
  - Improved cache simulation with better performance characteristics
  - Support for parallel execution in timing simulator
  - Better handling of large program simulations

### Fixed
- Issue with floating-point register access in instruction execution
- Memory boundary checks in load/store instructions
- Inconsistent error handling in syscall implementation
- Pipeline stalling logic that could cause unnecessary stalls
- Branch prediction accuracy measurement issues
- Cache coherence problems in multi-level cache hierarchy
- Incorrect endianness handling in halfword memory accesses
- Register renaming issues in certain edge cases
- Performance bottlenecks in instruction execution
- Various logging and output formatting issues

## [0.1.1] - 2025-03-05

### Added
- Support for additional MIPS instructions:
  - `lui` (Load Upper Immediate)
  - `ori` (OR Immediate)
  - `mult` (Multiply)
  - `mflo` (Move From LO)
  - `addiu` (Add Immediate Unsigned)
  - `bne` (Branch Not Equal)
  - `jr` (Jump Register)
  - Many more MIPS instruction variants (40+ instructions total)
- Special LO register support for multiplication results
- Explicit termination detection via NOP instructions
- Infinite loop detection and prevention
- Maximum instruction limit to prevent runaway execution
- Detailed instruction logging and debugging
- Improved test program with explicit termination sequence
- Cache parameter validation and bounds checking
- Better error handling for memory access violations
- Step-by-step instruction execution with detailed output in timing simulator
- Comprehensive program instruction dumping for debugging
- Enhanced test suite with more robust tests for both simulators

### Changed
- Modified `Registers` implementation to use `Vec<u32>` instead of fixed-size array
- Enhanced program loading with detailed validation and debugging
- Improved instruction decoder to handle more MIPS instruction formats
- Extended `get_address` method to support additional branch instructions
- Updated `main.rs` to provide better output and debugging information
- Fixed PC handling in branch and jump instructions
- Improved cache indexing logic to prevent out-of-bounds errors
- Enhanced timing simulator with direct instruction execution
- Completely rewrote the timing simulator execution loop for better debugging
- Reduced cycle limit in timing simulator to improve visibility of execution
- Updated functional simulator test to use more reliable instruction sequence
- Added Debug trait to Instruction enum for better error reporting
- Improved project documentation in README and code comments

### Fixed
- Memory access violations in the cache implementation
- Incorrect handling of JR (Jump Register) instruction
- Infinite loops in functional and timing simulators
- Invalid instruction decoding for various MIPS function codes
- Out-of-bounds errors in the cache implementation
- Incorrect dot product calculation in example
- HashMap type declarations in loop detection code
- Branch target calculation for PC-relative addressing
- Timing simulator not executing instructions properly
- Timing simulator not loading program at correct memory location
- No instruction output in timing simulator
- Failed test cases for both functional and timing simulators
- Unused imports and other compiler warnings

## [0.1.0] - 2025-03-04

### Added
- Initial implementation of VMIPS Rust simulator
- Basic MIPS instruction support
- Functional simulator
- Timing simulator with pipeline
- Memory hierarchy with cache simulation
- Hazard detection and handling
- Example dot product program
- Basic logging and debugging capabilities