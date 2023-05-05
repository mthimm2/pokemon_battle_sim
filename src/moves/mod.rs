use crate::pokemon::Pokemon;
use crate::typing::Types;

trait Damage {
    fn damage(&self, target: &Pokemon) -> f64;
}

pub enum MoveTypes {
    Physical,
    Special,
    Status,
}

impl Damage for MoveTypes {
    // TODO: Implement damage calculation for various move types
    fn damage(&self, target: &Pokemon) -> f64 {
        match self.phys_spec_stat {
            MoveTypes::Physical => {}
            MoveTypes::Special => {}
            MoveTypes::Status => {}
        }
    }
}

pub struct Move {
    power: f64,
    pp: u8,
    chart_type: Types,
    phys_spec_stat: MoveTypes,
    name: String,
    description: String,
}

// TODO: Implement move struct
impl Move {}
