//! Control flow flattening transformation

use super::FlattenedBlock;
use crate::analysis::{BasicBlock, ControlFlowGraph};
use anyhow::Result;
use std::collections::HashMap;

/// Control flow flattener
pub struct ControlFlowFlattener {
    state_var_name: String,
}

impl ControlFlowFlattener {
    pub fn new() -> Self {
        Self {
            state_var_name: "_state".to_string(),
        }
    }

    /// Flatten control flow graph into state machine
    pub fn flatten(&self, cfg: &ControlFlowGraph) -> Result<Vec<FlattenedBlock>> {
        let mut flattened = Vec::new();

        // Convert each basic block into a state machine case
        for (block_id, block) in &cfg.blocks {
            let state_code = self.generate_state_case(*block_id, block, cfg)?;
            
            flattened.push(FlattenedBlock {
                block_id: *block_id,
                state_machine_code: state_code,
            });
        }

        log::debug!("Flattened {} control flow blocks", flattened.len());
        Ok(flattened)
    }

    /// Generate state machine case for a basic block
    fn generate_state_case(
        &self,
        block_id: usize,
        block: &BasicBlock,
        cfg: &ControlFlowGraph,
    ) -> Result<String> {
        let mut code = format!("if {} == {} then\n", self.state_var_name, block_id);
        
        // Add block body placeholder
        code.push_str(&format!("    -- Block {} body goes here\n", block_id));
        
        // Determine next state
        if block.successors.is_empty() {
            // Terminal block
            code.push_str(&format!("    {} = -1 -- exit\n", self.state_var_name));
        } else if block.successors.len() == 1 {
            // Single successor (unconditional jump)
            code.push_str(&format!("    {} = {}\n", self.state_var_name, block.successors[0]));
        } else {
            // Multiple successors (conditional jump)
            // For now, just pick first successor
            // In a full implementation, this would preserve branch conditions
            code.push_str(&format!(
                "    {} = {} -- conditional branch\n",
                self.state_var_name, block.successors[0]
            ));
        }
        
        code.push_str("end\n");
        Ok(code)
    }

    /// Generate complete state machine wrapper
    pub fn generate_state_machine(
        &self,
        flattened: &[FlattenedBlock],
        entry_block: usize,
    ) -> String {
        let mut code = String::new();
        
        // Initialize state variable
        code.push_str(&format!("local {} = {}\n", self.state_var_name, entry_block));
        
        // Create state machine loop
        code.push_str(&format!("while {} >= 0 do\n", self.state_var_name));
        
        // Add all state cases
        for flattened_block in flattened {
            code.push_str("    ");
            code.push_str(&flattened_block.state_machine_code);
        }
        
        code.push_str("end\n");
        code
    }

    /// Add opaque predicates to confuse analysis
    pub fn add_opaque_predicate(&self, condition: &str, true_branch: usize, false_branch: usize) -> String {
        // Opaque predicates are conditions that always evaluate to true/false
        // but are difficult to determine statically
        format!(
            "if {} then\n    {} = {}\nelse\n    {} = {}\nend\n",
            condition, self.state_var_name, true_branch, self.state_var_name, false_branch
        )
    }
}

impl Default for ControlFlowFlattener {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cfg() -> ControlFlowGraph {
        let mut blocks = HashMap::new();
        
        blocks.insert(
            0,
            BasicBlock {
                id: 0,
                start_line: 1,
                end_line: 5,
                successors: vec![1],
                predecessors: vec![],
            },
        );
        
        blocks.insert(
            1,
            BasicBlock {
                id: 1,
                start_line: 6,
                end_line: 10,
                successors: vec![2, 3],
                predecessors: vec![0],
            },
        );
        
        blocks.insert(
            2,
            BasicBlock {
                id: 2,
                start_line: 11,
                end_line: 15,
                successors: vec![],
                predecessors: vec![1],
            },
        );

        ControlFlowGraph {
            blocks,
            entry_block: 0,
            exit_blocks: vec![2],
        }
    }

    #[test]
    fn test_control_flow_flattening() {
        let flattener = ControlFlowFlattener::new();
        let cfg = create_test_cfg();
        
        let flattened = flattener.flatten(&cfg).unwrap();
        
        assert_eq!(flattened.len(), 3);
        assert!(flattened[0].state_machine_code.contains("if _state == 0"));
    }

    #[test]
    fn test_state_machine_generation() {
        let flattener = ControlFlowFlattener::new();
        let cfg = create_test_cfg();
        let flattened = flattener.flatten(&cfg).unwrap();
        
        let state_machine = flattener.generate_state_machine(&flattened, 0);
        
        assert!(state_machine.contains("local _state = 0"));
        assert!(state_machine.contains("while _state >= 0 do"));
        assert!(state_machine.contains("end"));
    }

    #[test]
    fn test_terminal_block() {
        let flattener = ControlFlowFlattener::new();
        let mut blocks = HashMap::new();
        
        blocks.insert(
            0,
            BasicBlock {
                id: 0,
                start_line: 1,
                end_line: 5,
                successors: vec![], // No successors = terminal
                predecessors: vec![],
            },
        );

        let cfg = ControlFlowGraph {
            blocks,
            entry_block: 0,
            exit_blocks: vec![0],
        };
        
        let flattened = flattener.flatten(&cfg).unwrap();
        
        assert!(flattened[0].state_machine_code.contains("_state = -1"));
    }

    #[test]
    fn test_opaque_predicate() {
        let flattener = ControlFlowFlattener::new();
        let predicate = flattener.add_opaque_predicate("x > 0", 1, 2);
        
        assert!(predicate.contains("if x > 0 then"));
        assert!(predicate.contains("_state = 1"));
        assert!(predicate.contains("_state = 2"));
    }
}
