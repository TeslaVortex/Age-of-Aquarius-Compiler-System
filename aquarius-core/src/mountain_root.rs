use crate::mirror_lattice::{HouseNode, MirrorLattice, Node};
/// Mountain Root – The embodied, ordinary root
/// House, yard, body, cat, daily life, childhood circle.
/// This is the primary node. Everything returns here.
use crate::vortex::Field;

/// Returning Item - item that returns to the root
pub struct ReturningItem {
    name: &'static str,
    field: Field,
}

/// Mountain Root – the primary node where everything returns.
/// Embodies: "The embodied, ordinary root: house, yard, body, cat, daily life, childhood circle."
pub struct MountainRoot {
    /// The house as primary node
    house: HouseNode,
    /// Mirror lattice containing all nodes
    lattice: MirrorLattice,
    /// Field state of the root
    root_field: Field,
    /// Items that return to the root (cat, daily tasks, etc.)
    returning_items: Vec<ReturningItem>,
}

impl MountainRoot {
    /// Creates a new Mountain Root with a fresh house.
    pub fn new() -> Self {
        let mut house = HouseNode::new(1, "Mountain Root");
        house.add_item("cat");
        house.add_item("daily life");
        house.add_item("childhood circle");

        Self {
            house,
            lattice: MirrorLattice::new(),
            root_field: Field {
                presence: 1.0,
                noise: 0.0,
                complete: true,
            },
            returning_items: vec![
                ReturningItem {
                    name: "cat",
                    field: Field {
                        presence: 1.0,
                        noise: 0.0,
                        complete: true,
                    },
                },
                ReturningItem {
                    name: "daily tasks",
                    field: Field {
                        presence: 1.0,
                        noise: 0.0,
                        complete: true,
                    },
                },
                ReturningItem {
                    name: "thoughts",
                    field: Field {
                        presence: 1.0,
                        noise: 0.0,
                        complete: true,
                    },
                },
            ],
        }
    }

    /// Embodies the Mountain Root – everything returns here.
    pub fn embody(&mut self) {
        // House embodies itself
        self.house.embody();

        // All returning items embody into the root
        for item in &mut self.returning_items {
            item.field.presence = 1.0;
            item.field.noise = 0.0;
            item.field.complete = true;
        }

        // Root field is always complete
        self.root_field.presence = 1.0;
        self.root_field.noise = 0.0;
        self.root_field.complete = true;

        // All lattice nodes reflect into the root
        self.lattice.reflect_all_into(1); // Node 1 is the house
    }

    /// Adds a node to the lattice that returns to the root.
    pub fn add_returning_node(&mut self, node: Node, field: Field) {
        self.lattice.add_node(node, field);
        self.returning_items.push(ReturningItem {
            name: node.label,
            field,
        });
    }

    /// All nodes in lattice reflect into the Mountain Root.
    pub fn all_return_to_root(&mut self) {
        self.lattice.reflect_all_into(1); // Node 1 is the house
        self.house.embody();
    }

    /// Returns the house node (primary).
    pub fn house(&self) -> &HouseNode {
        &self.house
    }

    /// Returns the house node (mutable).
    pub fn house_mut(&mut self) -> &mut HouseNode {
        &mut self.house
    }

    /// Returns the mirror lattice.
    pub fn lattice(&self) -> &MirrorLattice {
        &self.lattice
    }

    /// Returns the root field.
    pub fn root_field(&self) -> &Field {
        &self.root_field
    }

    /// Returns all returning items.
    pub fn returning_items(&self) -> &[ReturningItem] {
        &self.returning_items
    }

    /// Returns true if Mountain Root is coherent.
    pub fn is_coherent(&self) -> bool {
        self.house.is_coherent() && self.root_field.complete && self.lattice.is_coherent()
    }

    /// Returns true if everything has returned to the root.
    pub fn everything_returned(&self) -> bool {
        // The Mountain Root is always coherent - it is the embodiment
        // of "everything returns here". The house embodies itself,
        // all items return, and the root field is complete.
        self.house.is_coherent() && self.root_field.complete
    }

    /// Returns the primary node ID (house).
    pub fn primary_node_id(&self) -> u64 {
        1 // House is always node 1
    }

    /// Returns label of the primary node.
    pub fn primary_node_label(&self) -> &str {
        self.house.node_label()
    }
}

impl Default for MountainRoot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mountain_root_creation() {
        let root = MountainRoot::new();
        assert_eq!(root.primary_node_label(), "Mountain Root");
        assert!(root.is_coherent());
    }

    #[test]
    fn test_mountain_root_embody() {
        let mut root = MountainRoot::new();
        root.embody();

        assert!(root.is_coherent());
        assert_eq!(root.root_field().presence, 1.0);
        assert_eq!(root.root_field().noise, 0.0);
        assert!(root.root_field().complete);
        assert!(root.house().is_coherent());
    }

    #[test]
    fn test_add_returning_node() {
        let mut root = MountainRoot::new();
        let node = Node::new(1, "Thought");
        root.add_returning_node(
            node,
            Field {
                presence: 1.0,
                noise: 0.0,
                complete: true,
            },
        );

        assert_eq!(root.lattice().node_count(), 1);
        assert!(root.returning_items().len() > 0);
    }

    #[test]
    fn test_all_return_to_root() {
        let mut root = MountainRoot::new();
        root.all_return_to_root();

        assert!(root.is_coherent());
        assert!(root.everything_returned());
    }

    #[test]
    fn test_mountain_root_coherence() {
        let root = MountainRoot::new();
        assert!(root.is_coherent());
        assert!(root.everything_returned());
    }
}
