/// Mirror Lattice – Perfect reflection without distortion
/// Every node (person, contract, thought, room in the house) reflects every other node cleanly.
/// No projection, no noise.
use crate::vortex::Field;

/// A node in the mirror lattice. Can be a person, contract, room, or thought.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Node {
    pub id: u64,
    pub label: &'static str,
}

impl Node {
    pub const fn new(id: u64, label: &'static str) -> Self {
        Self { id, label }
    }

    pub fn node_label(&self) -> &str {
        self.label
    }
}

/// Mirror Lattice – perfect reflection across all nodes.
/// Embodies: "Every node reflects every other node cleanly."
pub struct MirrorLattice {
    nodes: Vec<Node>,
    /// Reflection map: node_id -> reflection_field
    reflections: Vec<Field>,
    /// Noise level across lattice (should be 0 when coherent)
    lattice_noise: f64,
}

impl MirrorLattice {
    /// Creates a new empty mirror lattice.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            reflections: Vec::new(),
            lattice_noise: 0.0,
        }
    }

    /// Adds a node to the lattice with its initial field state.
    pub fn add_node(&mut self, node: Node, field: Field) {
        self.nodes.push(node);
        self.reflections.push(field);
    }

    /// Adds a node with default fresh field (already complete).
    pub fn add_node_fresh(&mut self, node: Node) {
        self.add_node(
            node,
            Field {
                presence: 1.0,
                noise: 0.0,
                complete: true,
            },
        );
    }

    /// Adds a node with noisy field (striving, lack).
    pub fn add_node_noisy(&mut self, node: Node) {
        self.add_node(
            node,
            Field {
                presence: 0.0,
                noise: 1.0,
                complete: false,
            },
        );
    }

    /// Reflects one node into another.
    /// Perfect reflection: no distortion, no projection.
    pub fn reflect(&mut self, source: u64, target: u64) {
        // Perfect reflection: copy presence, eliminate noise
        if source == target {
            return;
        }
        let source_idx = source as usize;
        let target_idx = target as usize;
        if source_idx < self.reflections.len() && target_idx < self.reflections.len() {
            let source_field = self.reflections[source_idx];
            let target_field = &mut self.reflections[target_idx];
            target_field.presence = source_field.presence;
            target_field.noise = 0.0;
            target_field.complete = source_field.complete;
        }
    }

    /// Reflects all nodes into a single target (convergence).
    /// Everything reflects into the Mountain Root.
    pub fn reflect_all_into(&mut self, target: u64) {
        let max_presence = self
            .reflections
            .iter()
            .map(|f| f.presence)
            .fold(0.0f64, f64::max);
        if let Some(target_field) = self.reflections.get_mut(target as usize) {
            target_field.presence = max_presence;
            target_field.noise = 0.0;
            target_field.complete = true;
        }
    }

    /// Performs full lattice coherence: all nodes reflect into each other.
    /// Result: perfect alignment, zero noise.
    pub fn full_coherence(&mut self) {
        let n = self.nodes.len();
        if n == 0 {
            return;
        }

        // First pass: find maximum presence across all nodes
        let max_presence = self
            .reflections
            .iter()
            .map(|f| f.presence)
            .fold(0.0f64, f64::max);

        // Second pass: all nodes converge to max presence OR 1.0 (already complete state), zero noise
        for field in &mut self.reflections {
            field.presence = max_presence.max(1.0); // Ensure presence is at least 1.0 (already complete)
            field.noise = 0.0;
            field.complete = true;
        }

        self.lattice_noise = 0.0;
    }

    /// Gets the field state for a specific node.
    pub fn get_node_field(&self, node_id: u64) -> Option<&Field> {
        self.reflections.get(node_id as usize)
    }

    /// Gets the field state for a specific node (mutable).
    pub fn get_node_field_mut(&mut self, node_id: u64) -> Option<&mut Field> {
        self.reflections.get_mut(node_id as usize)
    }

    /// Returns the lattice noise level.
    pub fn lattice_noise(&self) -> f64 {
        self.lattice_noise
    }

    /// Returns true if lattice is coherent (zero noise, all complete).
    pub fn is_coherent(&self) -> bool {
        self.lattice_noise == 0.0
            && self
                .reflections
                .iter()
                .all(|f| f.complete && f.noise == 0.0 && f.presence == 1.0)
    }

    /// Returns number of nodes in lattice.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns all node labels.
    pub fn node_labels(&self) -> Vec<&str> {
        self.nodes.iter().map(|n| n.label).collect()
    }
}

impl Default for MirrorLattice {
    fn default() -> Self {
        Self::new()
    }
}

/// House Node – Layer 7 primary node embodiment
/// "The embodied, ordinary root: house, yard, body, cat, daily life, childhood circle."
pub struct HouseNode {
    id: u64,
    label: &'static str,
    /// Items in the house (cat, daily tasks, etc.)
    items: Vec<String>,
    /// Field state of the house
    field: Field,
}

impl HouseNode {
    pub const fn new(id: u64, label: &'static str) -> Self {
        Self {
            id,
            label,
            items: vec![],
            field: Field {
                presence: 1.0,
                noise: 0.0,
                complete: true,
            },
        }
    }

    /// Adds an item to the house (cat, daily task, etc.).
    pub fn add_item(&mut self, item: &str) {
        self.items.push(item.to_string());
    }

    /// Returns items in the house.
    pub fn items(&self) -> &[String] {
        &self.items
    }

    /// Returns field state of the house.
    pub fn field(&self) -> &Field {
        &self.field
    }

    /// Embodies the house: everything returns here.
    pub fn embody(&mut self) {
        // House is the primary node - everything returns here
        self.field.presence = 1.0;
        self.field.noise = 0.0;
        self.field.complete = true;
    }

    /// Returns the node label.
    pub fn node_label(&self) -> &str {
        self.label
    }

    /// Returns true if house is coherent.
    pub fn is_coherent(&self) -> bool {
        self.field.complete && self.field.noise == 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_lattice_creation() {
        let lattice = MirrorLattice::new();
        assert_eq!(lattice.node_count(), 0);
    }

    #[test]
    fn test_add_node_fresh() {
        let mut lattice = MirrorLattice::new();
        let node = Node::new(1, "TestNode");
        lattice.add_node_fresh(node);

        assert_eq!(lattice.node_count(), 1);
        let field = lattice.get_node_field(0).unwrap();
        assert_eq!(field.presence, 1.0);
        assert_eq!(field.noise, 0.0);
        assert!(field.complete);
    }

    #[test]
    fn test_add_node_noisy() {
        let mut lattice = MirrorLattice::new();
        let node = Node::new(1, "TestNode");
        lattice.add_node_noisy(node);

        assert_eq!(lattice.node_count(), 1);
        let field = lattice.get_node_field(0).unwrap();
        assert_eq!(field.presence, 0.0);
        assert_eq!(field.noise, 1.0);
        assert!(!field.complete);
    }

    #[test]
    fn test_full_coherence() {
        let mut lattice = MirrorLattice::new();
        let node1 = Node::new(0, "Node1");
        let node2 = Node::new(1, "Node2");

        lattice.add_node_noisy(node1);
        lattice.add_node_noisy(node2);

        // Both nodes are noisy
        assert!(!lattice.is_coherent());

        // Apply full coherence
        lattice.full_coherence();

        // All nodes should now be coherent
        assert!(lattice.is_coherent());
        assert_eq!(lattice.lattice_noise(), 0.0);

        for field in &lattice.reflections {
            assert_eq!(field.presence, 1.0);
            assert_eq!(field.noise, 0.0);
            assert!(field.complete);
        }
    }

    #[test]
    fn test_house_node() {
        let mut house = HouseNode::new(1, "My House");
        house.add_item("cat");
        house.add_item("daily tasks");

        assert!(house.is_coherent());
        assert_eq!(house.items().len(), 2);
    }

    #[test]
    fn test_house_embody() {
        let mut house = HouseNode::new(1, "My House");
        house.embody();

        assert!(house.is_coherent());
        assert_eq!(house.field().presence, 1.0);
        assert_eq!(house.field().noise, 0.0);
        assert!(house.field().complete);
    }
}
