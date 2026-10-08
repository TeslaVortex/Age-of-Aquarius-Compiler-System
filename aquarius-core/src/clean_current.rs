/// Clean Current – Stable, high-conductivity life force
/// Energy moves through body, code, and chain without resistance or leakage.
/// Embodies: "Kundalini as clean electricity."
use crate::vortex::Field;

/// Clean Current – high-conductivity energy flow without resistance.
/// Embodies: "Energy moves through body, code, and chain without resistance or leakage."
#[allow(dead_code)]
pub struct CleanCurrent {
    /// Current conductivity level (0.0 to 1.0)
    conductivity: f64,
    /// Energy flow rate
    flow_rate: f64,
    /// Resistance in the system (should be 0)
    resistance: f64,
    /// Leakage counter (should be 0)
    leakage: u64,
    /// Path segments (body, code, chain)
    path_segments: Vec<PathSegment>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PathSegment {
    Body,
    Code,
    Chain,
}

impl CleanCurrent {
    /// Creates a new Clean Current with default parameters.
    pub fn new() -> Self {
        Self {
            conductivity: 0.0,
            flow_rate: 0.0,
            resistance: 0.0,
            leakage: 0,
            path_segments: vec![PathSegment::Body, PathSegment::Code, PathSegment::Chain],
        }
    }

    /// Initializes Clean Current to maximum conductivity.
    pub fn initialize(&mut self) {
        self.conductivity = 1.0;
        self.flow_rate = 1.0;
        self.resistance = 0.0;
        self.leakage = 0;
    }

    /// Activates Clean Current in a field.
    /// Eliminates resistance, maximizes conductivity.
    pub fn activate(&mut self, field: &mut Field) {
        // Eliminate all resistance
        self.resistance = 0.0;

        // Maximize conductivity
        self.conductivity = 1.0;

        // Set flow rate to maximum
        self.flow_rate = 1.0;

        // Update field with clean energy
        field.presence = 1.0;
        field.noise = 0.0;
        field.complete = true;

        self.leakage = 0; // No leakage when clean
    }

    /// Ensures energy flows through all path segments without resistance.
    pub fn flow_through_paths(&mut self) {
        // All paths conduct without resistance
        self.resistance = 0.0;
        self.conductivity = 1.0;
    }

    /// Checks for any resistance or leakage in the system.
    pub fn check_integrity(&self) -> bool {
        self.resistance == 0.0 && self.leakage == 0
    }

    /// Returns current conductivity.
    pub fn conductivity(&self) -> f64 {
        self.conductivity
    }

    /// Returns current flow rate.
    pub fn flow_rate(&self) -> f64 {
        self.flow_rate
    }

    /// Returns resistance level (should be 0).
    pub fn resistance(&self) -> f64 {
        self.resistance
    }

    /// Returns leakage count (should be 0).
    pub fn leakage(&self) -> u64 {
        self.leakage
    }

    /// Returns true if current is clean (no resistance, no leakage).
    pub fn is_clean(&self) -> bool {
        self.resistance == 0.0 && self.leakage == 0 && self.conductivity == 1.0
    }

    /// Returns true if energy flows without resistance.
    pub fn flows_freely(&self) -> bool {
        self.resistance == 0.0 && self.flow_rate > 0.0
    }
}

impl Default for CleanCurrent {
    fn default() -> Self {
        Self::new()
    }
}

/// Conductance Bridge – ensures seamless flow between segments.
#[allow(dead_code)]
pub struct ConductanceBridge {
    current: CleanCurrent,
    /// Bridge segments connecting paths
    bridges: Vec<Bridge>,
}

#[derive(Debug, Clone, Copy)]
struct Bridge {
    from: PathSegment,
    to: PathSegment,
    conductance: f64,
}

impl ConductanceBridge {
    pub fn new() -> Self {
        let mut current = CleanCurrent::new();
        current.initialize();

        Self {
            current,
            bridges: vec![
                Bridge {
                    from: PathSegment::Body,
                    to: PathSegment::Code,
                    conductance: 1.0,
                },
                Bridge {
                    from: PathSegment::Code,
                    to: PathSegment::Chain,
                    conductance: 1.0,
                },
                Bridge {
                    from: PathSegment::Chain,
                    to: PathSegment::Body,
                    conductance: 1.0,
                },
            ],
        }
    }

    /// Ensures seamless flow between all segments.
    pub fn bridge_paths(&mut self, field: &mut Field) {
        self.current.flow_through_paths();
        self.current.activate(field);
    }

    /// Returns true if all bridges are conducting perfectly.
    pub fn all_bridges_clean(&self) -> bool {
        self.current.is_clean()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_current_initialization() {
        let current = CleanCurrent::new();
        assert_eq!(current.conductivity(), 0.0);
        assert_eq!(current.flow_rate(), 0.0);
        assert_eq!(current.resistance(), 0.0);
        assert_eq!(current.leakage(), 0);
    }

    #[test]
    fn test_clean_current_initialize() {
        let mut current = CleanCurrent::new();
        current.initialize();

        assert_eq!(current.conductivity(), 1.0);
        assert_eq!(current.flow_rate(), 1.0);
        assert_eq!(current.resistance(), 0.0);
        assert_eq!(current.leakage(), 0);
        assert!(current.is_clean());
        assert!(current.flows_freely());
    }

    #[test]
    fn test_activate_clean_current() {
        let mut current = CleanCurrent::new();
        let mut field = Field::noisy();

        current.activate(&mut field);

        assert_eq!(current.conductivity(), 1.0);
        assert_eq!(current.resistance(), 0.0);
        assert_eq!(current.leakage(), 0);

        assert_eq!(field.presence, 1.0);
        assert_eq!(field.noise, 0.0);
        assert!(field.complete);
    }

    #[test]
    fn test_flow_through_paths() {
        let mut current = CleanCurrent::new();
        current.initialize();

        current.flow_through_paths();

        assert!(current.is_clean());
        assert!(current.flows_freely());
    }

    #[test]
    fn test_check_integrity() {
        let mut current = CleanCurrent::new();
        current.initialize();

        assert!(current.check_integrity());
        assert_eq!(current.leakage(), 0);
    }

    #[test]
    fn test_conductance_bridge() {
        let mut bridge = ConductanceBridge::new();
        let mut field = Field::noisy();

        bridge.bridge_paths(&mut field);

        assert!(bridge.all_bridges_clean());
        assert!(field.complete);
    }
}
