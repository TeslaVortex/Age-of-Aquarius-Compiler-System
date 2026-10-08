use crate::vortex::Field;

/// Presence overwrites noise.
pub struct Coherence;

impl Coherence {
    /// Collapses noise into pure presence.
    pub fn overwrite_noise(field: &mut Field) {
        field.noise = 0.0;
        field.presence = 1.0;
        field.complete = true;
    }
}
