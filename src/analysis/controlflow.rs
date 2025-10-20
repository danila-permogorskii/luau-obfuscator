//! Control flow analysis

use crate::parser::ParseResult;
use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use std::collections::HashMap;

/// Basic block in control flow graph
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub successors: Vec<usize>,
    pub predecessors: Vec<usize>,
}

/// Control flow graph
#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    pub blocks: HashMap<usize, BasicBlock>,
    pub entry_block: usize,
    pub exit_blocks: Vec<usize>,
}

/// Analyzes control flow
pub struct ControlFlowAnalyzer {
    next_block_id: usize,
}

impl ControlFlowAnalyzer {
    pub fn new() -> Self {
        Self { next_block_id: 0 }
    }

    /// Build control flow graph from parsed code
    pub fn analyze(&self, _parse_result: &ParseResult) -> Result<ControlFlowGraph> {
        // Create simple linear control flow for now
        // In a complete implementation, this would analyze:
        // - if/then/else branches
        // - loops (for, while, repeat)
        // - function calls
        // - return statements
        
        let entry_block = BasicBlock {
            id: 0,
            start_line: 1,
            end_line: usize::MAX,
            successors: Vec::new(),
            predecessors: Vec::new(),
        };

        let mut blocks = HashMap::new();
        blocks.insert(0, entry_block);

        Ok(ControlFlowGraph {
            blocks,
            entry_block: 0,
            exit_blocks: vec![0],
        })
    }

    fn create_block(&mut self, start_line: usize) -> BasicBlock {
        let id = self.next_block_id;
        self.next_block_id += 1;

        BasicBlock {
            id,
            start_line,
            end_line: start_line,
            successors: Vec::new(),
            predecessors: Vec::new(),
        }
    }
}

impl Default for ControlFlowAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlFlowGraph {
    /// Get all blocks in the graph
    pub fn all_blocks(&self) -> Vec<&BasicBlock> {
        self.blocks.values().collect()
    }

    /// Get block by ID
    pub fn get_block(&self, id: usize) -> Option<&BasicBlock> {
        self.blocks.get(&id)
    }

    /// Add edge between blocks
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if let Some(from_block) = self.blocks.get_mut(&from) {
            from_block.successors.push(to);
        }
        if let Some(to_block) = self.blocks.get_mut(&to) {
            to_block.predecessors.push(from);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_basic_block() {
        let mut analyzer = ControlFlowAnalyzer::new();
        let block = analyzer.create_block(1);
        
        assert_eq!(block.id, 0);
        assert_eq!(block.start_line, 1);
        assert_eq!(block.end_line, 1);
    }

    #[test]
    fn test_control_flow_graph_creation() {
        let mut cfg = ControlFlowGraph {
            blocks: HashMap::new(),
            entry_block: 0,
            exit_blocks: vec![0],
        };

        let block1 = BasicBlock {
            id: 0,
            start_line: 1,
            end_line: 5,
            successors: Vec::new(),
            predecessors: Vec::new(),
        };

        cfg.blocks.insert(0, block1);
        assert_eq!(cfg.blocks.len(), 1);
        assert!(cfg.get_block(0).is_some());
    }

    #[test]
    fn test_add_edge() {
        let mut cfg = ControlFlowGraph {
            blocks: HashMap::new(),
            entry_block: 0,
            exit_blocks: vec![1],
        };

        cfg.blocks.insert(
            0,
            BasicBlock {
                id: 0,
                start_line: 1,
                end_line: 5,
                successors: Vec::new(),
                predecessors: Vec::new(),
            },
        );

        cfg.blocks.insert(
            1,
            BasicBlock {
                id: 1,
                start_line: 6,
                end_line: 10,
                successors: Vec::new(),
                predecessors: Vec::new(),
            },
        );

        cfg.add_edge(0, 1);

        assert_eq!(cfg.get_block(0).unwrap().successors.len(), 1);
        assert_eq!(cfg.get_block(1).unwrap().predecessors.len(), 1);
    }
}
