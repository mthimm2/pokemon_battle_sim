#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use crate::statuses::{Damage, NonVolatileStatusType, Status, VolatileStatusType};
use crate::typing::Types;

pub struct Pokemon {
    pub name: String,
    pub type_1: Types,
    pub type_2: Option<Types>,
    pub max_hp: f64,
    pub hp: f64,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
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
        let name: String = String::new();
        // let results = creator_helper();
        let type_1: Types = Types::Normal;
        let type_2: Option<Types> = None;
        let max_hp: f64 = 0.0;
        let hp: f64 = 0.0;
        let attack: u16 = 0;
        let defense: u16 = 0;
        let special_attack: u16 = 0;
        let special_defense: u16 = 0;
        let speed: u16 = 0;
        let status_conditions: Status = Status {
            non_vol: None,
            vol: Vec::new(),
            turn_count: 0,
        };
        //let mut moves: Vec<Move> = Vec::new();

        Pokemon {
            name,
            type_1,
            type_2,
            max_hp,
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
            status_conditions,
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
        if self.hp - damage < 1.0 {
            self.hp = 0.0;
            self.status_conditions.non_vol = Some(NonVolatileStatusType::Fainted);
        } else {
            self.hp -= damage;
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
        self.faint_check(damages.0);
        self.faint_check(damages.1);
    }

    // Reads the relevant moveset from the file and returns it as a vector of strings
    pub fn construct_moveset(&mut self, _moves: Vec<String>) {
        // TODO: Let a vector of strings be used to select moves out of the overall moveset of Gen 1
        // Assign moves in order of their placement in the vector.
        todo!();
    }

    // TODO: Implement functions surrounding moves
    // PP check and select move for use during the turn
    // pub fn move_pp_check(&mut self, move: Move) -> Move {}
    pub fn move_pp_check(&mut self) {
        // Refer to comment line above for final function header
        todo!();
    }
}
