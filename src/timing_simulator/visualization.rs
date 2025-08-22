// Copyright (c) 2024 Mudit Bhargava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// visualization.rs
//
// This file contains the pipeline visualization logic for the timing simulator.
// It provides different output formats (text, CSV, JSON) to visualize the
// pipeline state at each cycle.

use crate::functional_simulator::instructions::Instruction;
use crate::timing_simulator::pipeline::{Pipeline, PipelineStageStatus};

#[derive(Clone)]
pub struct PipelineVisualization {
    pub show_cycle_info: bool,
    pub show_hazards: bool,
    pub show_instruction_flow: bool,
    pub colorize_output: bool,
    pub output_format: OutputFormat,
}

#[derive(Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Text,
    CSV,
    JSON,
}

impl PipelineVisualization {
    pub fn new() -> Self {
        Self {
            show_cycle_info: true,
            show_hazards: true,
            show_instruction_flow: true,
            colorize_output: true,
            output_format: OutputFormat::Text,
        }
    }

    pub fn visualize_pipeline(&self, pipeline: &Pipeline, cycle: usize) -> String {
        match self.output_format {
            OutputFormat::Text => self.visualize_text(pipeline, cycle),
            OutputFormat::CSV => self.visualize_csv(pipeline, cycle),
            OutputFormat::JSON => self.visualize_json(pipeline, cycle),
        }
    }

    fn visualize_text(&self, pipeline: &Pipeline, cycle: usize) -> String {
        let mut result = String::new();

        // Header
        if self.show_cycle_info {
            result.push_str(&format!("=== Pipeline State at Cycle {} ===\n", cycle));
        }

        // Pipeline diagram
        result.push_str("+-------+-------+-------+-------+-------+\n");
        result.push_str("| Fetch | Decode| Exec  | Mem   | Write |\n");
        result.push_str("+-------+-------+-------+-------+-------+\n");

        // Stage content - Build a single row with content for each stage
        let mut stage_content = String::from("| ");

        for stage in &pipeline.stages {
            let content = match &stage.instruction {
                Some(instr) => {
                    // Format the instruction to fit in a 5-char space
                    let instr_str = self.format_instruction(instr);
                    format!("{:<5}", instr_str)
                },
                None => "     ".to_string(),
            };

            // Add status indicator
            let status_indicator = match stage.status {
                PipelineStageStatus::Empty => " ",
                PipelineStageStatus::Busy => {
                    if stage.cycles_remaining > 0 {
                        &format!("{}c", stage.cycles_remaining)
                    } else {
                        "B"
                    }
                },
                PipelineStageStatus::Stalled => "S",
                PipelineStageStatus::Flushed => "F",
            };

            stage_content.push_str(&format!("{}{} | ", content, status_indicator));
        }

        result.push_str(&stage_content);
        result.push_str("\n");

        result.push_str("+-------+-------+-------+-------+-------+\n");

        // Hazard information
        if self.show_hazards {
            let hazards = self.get_active_hazards(pipeline);
            if !hazards.is_empty() {
                result.push_str("\nActive Hazards:\n");
                for hazard in hazards {
                    result.push_str(&format!("- {}\n", hazard));
                }
            }
        }

        result
    }

    fn visualize_csv(&self, pipeline: &Pipeline, cycle: usize) -> String {
        let mut result = String::new();

        // Header (only for first cycle)
        if cycle == 1 {
            result.push_str("Cycle,Fetch,Decode,Execute,Memory,Writeback,Hazards\n");
        }

        // Pipeline state
        result.push_str(&format!("{},", cycle));

        // Each stage
        for stage in &pipeline.stages {
            let content = match &stage.instruction {
                Some(instr) => format!("{}", self.format_instruction(instr)),
                None => "".to_string(),
            };

            result.push_str(&format!("\"{}\",", content));
        }

        // Hazards
        let hazards = self.get_active_hazards(pipeline);
        result.push_str(&format!("\"{}\"\n", hazards.join("; ")));

        result
    }

    fn visualize_json(&self, pipeline: &Pipeline, cycle: usize) -> String {
        let mut result = String::new();

        // Start JSON object
        result.push_str("{\n");
        result.push_str(&format!("  \"cycle\": {},\n", cycle));
        result.push_str("  \"stages\": [\n");

        // Each stage
        for (i, stage) in pipeline.stages.iter().enumerate() {
            result.push_str("    {\n");

            // Stage type
            result.push_str(&format!("      \"type\": \"{:?}\",\n", stage.stage_type));

            // Stage status
            result.push_str(&format!("      \"status\": \"{:?}\",\n", stage.status));

            // Instruction
            if let Some(instr) = &stage.instruction {
                result.push_str(&format!(
                    "      \"instruction\": \"{}\",\n",
                    self.format_instruction(instr)
                ));
            } else {
                result.push_str("      \"instruction\": null,\n");
            }

            // PC
            result.push_str(&format!("      \"pc\": \"0x{:08X}\"", stage.pc));

            // Additional details
            if stage.cycles_remaining > 0 {
                result.push_str(&format!(
                    ",\n      \"cyclesRemaining\": {}",
                    stage.cycles_remaining
                ));
            }

            if let Some(target_reg) = stage.target_register {
                result.push_str(&format!(",\n      \"targetRegister\": {}", target_reg));
            }

            if let Some(memory_address) = stage.memory_address {
                result.push_str(&format!(
                    ",\n      \"memoryAddress\": \"0x{:08X}\"",
                    memory_address
                ));
            }

            result.push_str("\n    }");

            if i < pipeline.stages.len() - 1 {
                result.push_str(",");
            }
            result.push_str("\n");
        }

        result.push_str("  ],\n");

        // Hazards
        result.push_str("  \"hazards\": [\n");
        let hazards = self.get_active_hazards(pipeline);
        for (i, hazard) in hazards.iter().enumerate() {
            result.push_str(&format!("    \"{}\"", hazard));
            if i < hazards.len() - 1 {
                result.push_str(",");
            }
            result.push_str("\n");
        }
        result.push_str("  ]\n");

        // End JSON object
        result.push_str("}\n");

        result
    }

    fn format_instruction(&self, instruction: &Instruction) -> String {
        // Simplified representation based on instruction type
        match instruction {
            // R-type instructions
            Instruction::Add { .. } => "ADD",
            Instruction::Sub { .. } => "SUB",
            Instruction::And { .. } => "AND",
            Instruction::Or { .. } => "OR",
            Instruction::Xor { .. } => "XOR",
            Instruction::Nor { .. } => "NOR",
            Instruction::Slt { .. } => "SLT",
            Instruction::Sll { .. } => "SLL",
            Instruction::Srl { .. } => "SRL",
            Instruction::Sra { .. } => "SRA",
            Instruction::Sllv { .. } => "SLLV",
            Instruction::Srlv { .. } => "SRLV",
            Instruction::Srav { .. } => "SRAV",
            Instruction::Mult { .. } => "MULT",
            Instruction::Div { .. } => "DIV",
            Instruction::Divu { .. } => "DIVU",
            Instruction::Mflo { .. } => "MFLO",
            Instruction::Mfhi { .. } => "MFHI",
            Instruction::Mthi { .. } => "MTHI",
            Instruction::Mtlo { .. } => "MTLO",

            // I-type instructions
            Instruction::Addi { .. } => "ADDI",
            Instruction::Addiu { .. } => "ADDIU",
            Instruction::Andi { .. } => "ANDI",
            Instruction::Ori { .. } => "ORI",
            Instruction::Xori { .. } => "XORI",
            Instruction::Lui { .. } => "LUI",
            Instruction::Slti { .. } => "SLTI",
            Instruction::Sltiu { .. } => "SLTIU",

            // Load/Store instructions
            Instruction::Lw { .. } => "LW",
            Instruction::Lb { .. } => "LB",
            Instruction::Lbu { .. } => "LBU",
            Instruction::Lh { .. } => "LH",
            Instruction::Lhu { .. } => "LHU",
            Instruction::Sw { .. } => "SW",
            Instruction::Sb { .. } => "SB",
            Instruction::Sh { .. } => "SH",

            // Branch instructions
            Instruction::Beq { .. } => "BEQ",
            Instruction::Bne { .. } => "BNE",
            Instruction::Bgtz { .. } => "BGTZ",
            Instruction::Blez { .. } => "BLEZ",
            Instruction::Bltz { .. } => "BLTZ",
            Instruction::Bgez { .. } => "BGEZ",

            // Jump instructions
            Instruction::J { .. } => "J",
            Instruction::Jal { .. } => "JAL",
            Instruction::Jr { .. } => "JR",
            Instruction::Jalr { .. } => "JALR",

            // Special instructions
            Instruction::Nop => "NOP",
            Instruction::InvalidInstruction => "INVALID",

            // Catch-all for any missing instructions
            _ => "??",
        }
        .to_string()
    }

    fn get_active_hazards(&self, pipeline: &Pipeline) -> Vec<String> {
        let mut hazards = Vec::new();

        for (hazard_type, count) in &pipeline.hazard_stats {
            if *count > 0 {
                hazards.push(format!("{:?}: {}", hazard_type, count));
            }
        }

        hazards
    }
}
