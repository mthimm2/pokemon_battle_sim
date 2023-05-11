#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use crate::statuses::{Damage, NonVolatileStatusType, Status, VolatileStatusType};
use crate::typing::Types;

pub struct Pokemon {
    pub name: String,
    pub type_1: Types,
    pub type_2: Option<Types>,
    pub max_hp: f64,
    pub hp: f64,
    pub attack: f64,
    pub defense: f64,
    pub special_attack: f64,
    pub special_defense: f64,
    pub speed: f64,
    pub status_conditions: Status,
    pub attack_modifier: f64,
    pub defense_modifier: f64,
    pub special_attack_modifier: f64,
    pub special_defense_modifier: f64,
    pub speed_modifier: f64,
    // moves: Vec<Move>,
    // TODO: Critical Hit modifier field
    // TODO: Evasion modifier field
}

impl Pokemon {
    pub fn new() -> Pokemon {
        // TODO: Make this function one that takes in a HashMap of all constructed pokemon, that being handled separately
        // By a utility function in an external file.
        // Pass in only a name and this retrieves the Pokemon with all of its preset stats and moves.
        Pokemon {
            name: String::from("New"),
            type_1: Types::Normal,
            type_2: None,
            max_hp: 0.0,
            hp: 0.0,
            attack: 0.0,
            defense: 0.0,
            special_attack: 0.0,
            special_defense: 0.0,
            speed: 0.0,
            status_conditions: Status {
                non_vol: None,
                vol: Vec::new(),
                turn_count: 0,
            },
            attack_modifier: 1.0,
            defense_modifier: 1.0,
            special_attack_modifier: 1.0,
            special_defense_modifier: 1.0,
            speed_modifier: 1.0,
            // moves: Vec::new(),
        }
    }

    // Performs the job of checking the incoming move's damage against current hp
    // Subtracts off health and faints the Pokemon as needed
    fn faint_check(&mut self, damage: f64) {
        self.hp = (self.hp - damage).max(0.0);
        if self.hp < 1.0 {
            self.hp = 0.0;
            self.status_conditions.non_vol = Some(NonVolatileStatusType::Fainted);
        }
    }

    // Wrapper function for the faint check in the context of direct damage
    pub fn take_attack_damage(&mut self, incoming_damage: f64) {
        self.faint_check(incoming_damage);
    }

    // Wrapper function for the faint check in the context of status damage
    pub fn take_status_damage(&mut self) {
        let damages = self
            .status_conditions
            .damage(self.max_hp, self.attack, self.defense);
        self.faint_check(damages.0 + damages.1);
    }

    // Reads the relevant moveset from the file and returns it as a vector of strings
    pub fn construct_moveset(&mut self, _moves: Vec<String>) {
        // TODO: Let a vector of strings be used to select moves out of the overall moveset of Gen 1
        // Assign moves in order of their placement in the vector.
        todo!();
    }

    // Function that will allow the user to select a move from the pokemon's moveset
    // pub fn select_move(&self, choice: u8) -> Move {}
}
