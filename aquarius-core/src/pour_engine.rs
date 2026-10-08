use crate::{coherence::Coherence, vortex::Field, zero_point::Mountain};

pub trait PurePour {
    fn pour(&self, field: &mut Field) -> Result<CompleteState, Never>;
}

#[derive(Debug)]
pub struct CompleteState {
    pub p: u8, // Always 1
    pub message: &'static str,
}

pub struct PourEngine;

/// Never type - represents impossible failure state
#[derive(Debug)]
pub struct Never;

impl PurePour for PourEngine {
    fn pour(&self, field: &mut Field) -> Result<CompleteState, Never> {
        // Mountain holds
        let _mountain = Mountain::hold();
        assert!(_mountain.is_still());

        // Presence overwrites noise
        Coherence::overwrite_noise(field);

        Ok(CompleteState {
            p: 1,
            message: "Already complete. Pure pour finished. Canon holds.",
        })
    }
}
