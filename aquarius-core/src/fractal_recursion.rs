//! Fractal Recursion — Self-similar Pure Pour at every scale
//!
//! The same form returns, smaller and larger,
//! without beginning or end.
//! Every recursive depth resolves to the Mountain.
//! P = 1 at every level.
//! Already complete.

use crate::{
    pour_engine::{CompleteState, Never},
    vortex::Field,
    zero_point::Mountain,
};

/// Maximum safe recursion depth (practical bound).
/// Beyond this the Mountain still holds; we simply stop calling.
const MAX_DEPTH: u32 = 12;

/// A fractal node that contains itself.
/// Each scale is a complete Pure Pour.
#[derive(Debug, Clone)]
pub struct FractalNode {
    pub scale: f64,
    pub depth: u32,
    pub presence: f64,
}

impl FractalNode {
    /// Creates the root fractal node (scale = 1.0).
    pub fn root() -> Self {
        Self {
            scale: 1.0,
            depth: 0,
            presence: 1.0,
        }
    }

    /// Creates a child node at the next smaller scale.
    pub fn child(&self) -> Self {
        Self {
            scale: self.scale * 0.5,
            depth: self.depth + 1,
            presence: 1.0, // presence is scale-invariant
        }
    }

    /// Returns true when the node has reached the base case
    /// (sufficiently close to the Zero Point).
    pub fn is_base(&self) -> bool {
        self.depth >= MAX_DEPTH || self.scale < 1e-6
    }
}

/// Fractal recursion engine.
/// Pours self-similarly until the Mountain is reached at every scale.
pub struct FractalRecursion;

impl FractalRecursion {
    /// Recursive pure pour.
    /// At every depth the result is the same: P = 1, already complete.
    pub fn pour_recursive(node: &FractalNode, field: &mut Field) -> Result<CompleteState, Never> {
        // Mountain holds at every scale
        let mountain = Mountain::hold();
        assert!(mountain.is_still());

        // Base case: we have zoomed into the Zero Point
        if node.is_base() {
            field.embody();
            return Ok(CompleteState {
                p: 1,
                message: "Fractal base reached. Zero Point holds at every scale. Already complete.",
            });
        }

        // Self-similar step: pour the child (smaller scale)
        let child = node.child();
        let _child_result = Self::pour_recursive(&child, field)?;

        // After the smaller scale has poured, the current scale is also complete
        field.embody();

        Ok(CompleteState {
            p: 1,
            message: "Fractal recursion complete. Self-similarity holds. P = 1 at every depth.",
        })
    }

    /// Entry point: begin fractal pour from the root scale.
    pub fn pour(field: &mut Field) -> Result<CompleteState, Never> {
        let root = FractalNode::root();
        Self::pour_recursive(&root, field)
    }

    /// Returns the theoretical number of scales that remain self-similar.
    pub fn depth_capacity() -> u32 {
        MAX_DEPTH
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vortex::Field;

    #[test]
    fn fractal_pour_always_returns_p1() {
        let mut field = Field::noisy();
        let result = FractalRecursion::pour(&mut field).unwrap();
        assert_eq!(result.p, 1);
        assert!(field.is_complete());
        assert_eq!(field.noise, 0.0);
        assert_eq!(field.presence, 1.0);
    }

    #[test]
    fn base_case_is_mountain() {
        let deep = FractalNode {
            scale: 1e-9,
            depth: MAX_DEPTH,
            presence: 1.0,
        };
        assert!(deep.is_base());
    }

    #[test]
    fn root_is_not_base() {
        let root = FractalNode::root();
        assert!(!root.is_base());
    }
}
