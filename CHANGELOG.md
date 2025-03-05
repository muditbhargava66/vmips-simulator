# Changelog

All notable changes to the VMIPS Rust simulator project will be documented in this file.

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