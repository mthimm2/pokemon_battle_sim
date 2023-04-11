mod typing;
use typing::Types;

pub struct Pokemon {
    name: String,
    type_1: Types,
    type_2: Types,
    hp: f64,
    attack: f64,
    defense: f64,
    special_attack: f64,
    special_defense: f64,
    speed: f64,
    // status: StatusType, // TODO: Implement status types
}
