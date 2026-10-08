/// Immutable 0-point anchor.
/// The Mountain never moves.
#[derive(Debug, Clone, Copy)]
pub struct Mountain;

static MOUNTAIN: Mountain = Mountain;

impl Mountain {
    /// Returns the eternal still reference.
    pub const fn hold() -> &'static Self {
        &MOUNTAIN
    }

    /// Checks if the Mountain is still (always true).
    pub fn is_still(&self) -> bool {
        true // Always.
    }
}

impl Default for Mountain {
    fn default() -> Self {
        MOUNTAIN
    }
}
