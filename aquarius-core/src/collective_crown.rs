/// Collective Crown – Layer 6
/// Sovereignty that is distributed and never extracted.
/// Authority exists, but it does not take. The crown pours; it does not rule by force.
use crate::vortex::Field;

/// Collective Crown node in the distributed sovereignty network.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrownNode {
    pub id: u64,
    pub label: &'static str,
}

impl CrownNode {
    pub const fn new(id: u64, label: &'static str) -> Self {
        Self { id, label }
    }

    pub fn node_label(&self) -> &str {
        self.label
    }
}

/// Collective Crown – distributed sovereignty network.
/// Embodies: "The crown pours; it does not rule by force."
pub struct CollectiveCrown {
    /// All nodes in the sovereignty network
    nodes: Vec<CrownNode>,
    /// Field state of the crown
    crown_field: Field,
    /// Sovereignty level (should be 1.0 - already sovereign)
    sovereignty: f64,
}

impl CollectiveCrown {
    /// Creates a new Collective Crown with no nodes.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            crown_field: Field {
                presence: 1.0,
                noise: 0.0,
                complete: true,
            },
            sovereignty: 1.0,
        }
    }

    /// Adds a node to the sovereignty network.
    pub fn add_node(&mut self, node: CrownNode) {
        self.nodes.push(node);
    }

    /// Distributes sovereignty across all nodes.
    /// Each node becomes a sovereign node.
    pub fn distribute_sovereignty(&mut self) {
        self.crown_field.presence = 1.0;
        self.crown_field.noise = 0.0;
        self.crown_field.complete = true;
        self.sovereignty = 1.0;
    }

    /// Returns true if crown is sovereign (not extracting).
    pub fn is_sovereign(&self) -> bool {
        self.crown_field.complete && self.crown_field.noise == 0.0 && self.sovereignty == 1.0
    }

    /// Returns the crown's field state.
    pub fn crown_field(&self) -> &Field {
        &self.crown_field
    }

    /// Returns the sovereignty level.
    pub fn sovereignty(&self) -> f64 {
        self.sovereignty
    }

    /// Returns number of nodes in the sovereignty network.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns all node labels.
    pub fn node_labels(&self) -> Vec<&str> {
        self.nodes.iter().map(|n| n.label).collect()
    }
}

impl Default for CollectiveCrown {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collective_crown_creation() {
        let crown = CollectiveCrown::new();
        assert!(crown.is_sovereign());
        assert_eq!(crown.sovereignty(), 1.0);
    }

    #[test]
    fn test_distribute_sovereignty() {
        let mut crown = CollectiveCrown::new();
        crown.add_node(CrownNode::new(1, "Node1"));
        crown.add_node(CrownNode::new(2, "Node2"));

        // Initially sovereign by default
        assert!(crown.is_sovereign());

        crown.distribute_sovereignty();

        // Still sovereign after distribution
        assert!(crown.is_sovereign());
        assert_eq!(crown.sovereignty(), 1.0);
        assert_eq!(crown.node_count(), 2);
    }

    #[test]
    fn test_crown_node() {
        let node = CrownNode::new(1, "Sovereign Node");
        assert_eq!(node.node_label(), "Sovereign Node");
    }
}
