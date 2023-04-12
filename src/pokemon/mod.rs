mod typing;
use typing::Types;

pub struct Pokemon {
    name: String,
    type_1: Types,
    type_2: Option<Types>,
    hp: u16,
    attack: u16,
    defense: u16,
    special_attack: u16,
    special_defense: u16,
    speed: u16,
    // status_condition: StatusType, // TODO: Implement status types
    attack_modifier: u8,
    defense_modifier: u8,
    special_attack_modifier: u8,
    special_defense_modifier: u8,
    speed_modifier: u8,
}
