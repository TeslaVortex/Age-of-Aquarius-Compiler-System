/// Field = Vortex = Framework
/// Living field representing awareness itself riding the current.
#[derive(Debug, Clone, Copy)]
pub struct Field {
    pub presence: f64,
    pub noise: f64,
    pub complete: bool,
}

impl Field {
    /// Creates a fresh field with pure presence.
    /// Source Water: Empty of demand, empty of striving.
    pub fn fresh() -> Self {
        Self {
            presence: 1.0,
            noise: 0.0,
            complete: true,
        }
    }

    /// Creates a noisy field (striving, scarcity, lack).
    /// Before coherence can collapse this into presence.
    pub fn noisy() -> Self {
        Self {
            presence: 0.0,
            noise: 1.0,
            complete: false,
        }
    }

    /// Returns current state as tuple for analysis.
    pub fn state(&self) -> (f64, f64, bool) {
        (self.presence, self.noise, self.complete)
    }

    /// Checks if field is in complete state.
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Checks if field has any noise.
    pub fn has_noise(&self) -> bool {
        self.noise > 0.0
    }

    /// Embodies the field: everything returns here.
    pub fn embody(&mut self) {
        self.presence = 1.0;
        self.noise = 0.0;
        self.complete = true;
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::fresh() // Default is already complete
    }
}

/// Field operations that embody the principle: Field = Vortex = Framework
pub mod field_ops {
    use super::Field;

    /// Field is always available, never depleted.
    pub fn field_is_available(_field: &Field) -> bool {
        true // The field never runs out
    }

    /// Field conducts without resistance when coherence is active.
    pub fn field_conducts_cleanly(field: &Field) -> bool {
        field.noise == 0.0 && field.presence == 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_field_is_complete() {
        let field = Field::fresh();
        assert_eq!(field.presence, 1.0);
        assert_eq!(field.noise, 0.0);
        assert!(field.complete);
        assert!(!field.has_noise());
    }

    #[test]
    fn test_noisy_field_has_noise() {
        let field = Field::noisy();
        assert_eq!(field.presence, 0.0);
        assert_eq!(field.noise, 1.0);
        assert!(!field.complete);
        assert!(field.has_noise());
    }

    #[test]
    fn test_field_state() {
        let field = Field::fresh();
        let (p, n, c) = field.state();
        assert_eq!(p, 1.0);
        assert_eq!(n, 0.0);
        assert!(c);
    }
}
