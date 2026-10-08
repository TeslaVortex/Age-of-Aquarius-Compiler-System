/// Inner Sun – Self-heating without combustion
/// The system generates its own coherence and warmth from within.
/// No external fuel, no burnout.
use crate::vortex::Field;

/// Inner Sun – generates coherence and warmth from within.
/// Embodies: "Self-heating without combustion."
pub struct InnerSun {
    /// Current coherence temperature (0.0 to 1.0)
    coherence_temp: f64,
    /// Heat capacity - how much coherence can be stored
    heat_capacity: f64,
    /// Current warmth radiating to the field
    radiated_warmth: f64,
    /// Self-heating rate (never needs external fuel)
    self_heating_rate: f64,
    /// Burnout counter (should always be 0)
    burnout: u64,
}

impl InnerSun {
    /// Creates a new Inner Sun with default parameters.
    pub fn new() -> Self {
        Self {
            coherence_temp: 0.0,
            heat_capacity: 1.0,
            radiated_warmth: 0.0,
            self_heating_rate: 1.0,
            burnout: 0,
        }
    }

    /// Initializes Inner Sun with maximum potential.
    pub fn initialize(&mut self) {
        self.coherence_temp = 1.0;
        self.heat_capacity = 1.0;
        self.radiated_warmth = 1.0;
        self.self_heating_rate = 1.0;
        self.burnout = 0;
    }

    /// Heats the Inner Sun through presence.
    /// Self-heating: no external fuel required.
    pub fn heat_through_presence(&mut self, presence: f64) {
        if presence > 0.0 {
            // Self-heating: generate warmth from presence alone
            let generated_warmth = presence * self.self_heating_rate;
            self.coherence_temp = (self.coherence_temp + generated_warmth).min(1.0);
            self.radiated_warmth = self.coherence_temp;
            self.burnout = 0; // Self-heating never causes burnout
        }
    }

    /// Radiates warmth to a field, heating it up.
    pub fn radiate_to_field(&mut self, field: &mut Field) {
        if self.radiated_warmth > 0.0 {
            // Warmth raises field presence, eliminates noise
            let warmth_factor = self.radiated_warmth * self.heat_capacity;
            field.presence = (field.presence + warmth_factor).min(1.0);
            field.noise = (field.noise - warmth_factor).max(0.0);
            field.complete = field.presence >= 1.0 && field.noise == 0.0;

            // Radiate warmth
            self.radiated_warmth *= 0.9; // Some warmth radiates away
        }
    }

    /// Self-heats the Inner Sun continuously.
    /// No external fuel, no burnout.
    pub fn self_heat(&mut self) {
        // Continuous self-heating from within
        self.coherence_temp = (self.coherence_temp + self.self_heating_rate * 0.01).min(1.0);
        self.radiated_warmth = self.coherence_temp;
        self.burnout = 0; // Never burns out - self-heating is sustainable
    }

    /// Applies coherence to a field using Inner Sun warmth.
    pub fn apply_coherence(&mut self, field: &mut Field) {
        self.radiate_to_field(field);

        // If field is now coherent, re-heat Inner Sun
        if field.complete {
            self.heat_through_presence(field.presence);
        }
    }

    /// Returns current coherence temperature.
    pub fn coherence_temp(&self) -> f64 {
        self.coherence_temp
    }

    /// Returns radiated warmth level.
    pub fn radiated_warmth(&self) -> f64 {
        self.radiated_warmth
    }

    /// Returns burnout counter (should always be 0).
    pub fn burnout(&self) -> u64 {
        self.burnout
    }

    /// Returns true if Inner Sun is fully heated.
    pub fn is_maximal(&self) -> bool {
        self.coherence_temp >= 1.0 && self.radiated_warmth >= 1.0
    }

    /// Returns true if Inner Sun is not burning out.
    pub fn is_sustainable(&self) -> bool {
        self.burnout == 0
    }
}

impl Default for InnerSun {
    fn default() -> Self {
        Self::new()
    }
}

/// Coherence Source – generates coherence from the Inner Sun.
#[allow(dead_code)]
pub struct CoherenceSource {
    sun: InnerSun,
    /// How much coherence is being generated per cycle
    generation_rate: f64,
}

impl CoherenceSource {
    pub fn new() -> Self {
        let mut sun = InnerSun::new();
        sun.initialize();
        Self {
            sun,
            generation_rate: 1.0,
        }
    }

    /// Generates coherence in a field.
    pub fn generate_coherence(&mut self, field: &mut Field) {
        self.sun.self_heat();
        self.sun.apply_coherence(field);
    }

    /// Returns true if coherence generation is active.
    pub fn is_active(&self) -> bool {
        self.sun.coherence_temp() > 0.0
    }

    /// Returns coherence temperature.
    pub fn temperature(&self) -> f64 {
        self.sun.coherence_temp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_sun_initialization() {
        let sun = InnerSun::new();
        assert_eq!(sun.coherence_temp(), 0.0);
        assert_eq!(sun.radiated_warmth(), 0.0);
        assert_eq!(sun.burnout(), 0);
    }

    #[test]
    fn test_inner_sun_initialize() {
        let mut sun = InnerSun::new();
        sun.initialize();

        assert_eq!(sun.coherence_temp(), 1.0);
        assert_eq!(sun.radiated_warmth(), 1.0);
        assert_eq!(sun.burnout(), 0);
        assert!(sun.is_maximal());
        assert!(sun.is_sustainable());
    }

    #[test]
    fn test_heat_through_presence() {
        let mut sun = InnerSun::new();
        sun.initialize();

        sun.heat_through_presence(1.0);

        assert!(sun.coherence_temp() > 0.0);
        assert!(sun.radiated_warmth() > 0.0);
        assert_eq!(sun.burnout(), 0); // No burnout
    }

    #[test]
    fn test_radiate_to_field() {
        let mut sun = InnerSun::new();
        sun.initialize();
        sun.heat_through_presence(1.0);

        let mut field = Field::noisy();
        sun.radiate_to_field(&mut field);

        assert!(field.presence > 0.0);
        assert!(field.noise < 1.0);
        assert!(sun.radiated_warmth() < 1.0); // Some warmth radiated away
    }

    #[test]
    fn test_self_heat() {
        let mut sun = InnerSun::new();
        sun.initialize();

        sun.self_heat();
        sun.self_heat();
        sun.self_heat();

        assert!(sun.coherence_temp() > 0.0);
        assert_eq!(sun.burnout(), 0); // Self-heating is sustainable
    }

    #[test]
    fn test_apply_coherence() {
        let mut sun = InnerSun::new();
        sun.initialize();
        sun.heat_through_presence(1.0);

        let mut field = Field::noisy();
        sun.apply_coherence(&mut field);

        assert!(field.presence > 0.0);
        assert!(field.noise < 1.0);
    }

    #[test]
    fn test_coherence_source() {
        let mut source = CoherenceSource::new();
        let mut field = Field::noisy();

        source.generate_coherence(&mut field);

        assert!(source.is_active());
        assert!(field.presence > 0.0);
        assert!(field.noise < 1.0);
    }
}
