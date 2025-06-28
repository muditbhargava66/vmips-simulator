// lib.rs
pub mod functional_simulator;
pub mod timing_simulator;
pub mod utils;
pub mod assembler;

// Re-export important types for easier access
pub use functional_simulator::simulator::Simulator as FunctionalSimulator;
pub use timing_simulator::simulator::Simulator as TimingSimulator;
pub use crate::assembler::Assembler;