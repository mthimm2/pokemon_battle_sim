#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use crate::statuses::{Damage, NonVolatileStatusType, Status, VolatileStatusType};
use crate::typing::Types;

pub struct Pokemon {
    name: String,
    type_1: Types,
    type_2: Option<Types>,
    max_hp: f64,
    hp: f64,
    attack: u16,
    defense: u16,
    special_attack: u16,
    special_defense: u16,
    speed: u16,
    status_conditions: Status,
    attack_modifier: f64,
    defense_modifier: f64,
    special_attack_modifier: f64,
    special_defense_modifier: f64,
    speed_modifier: f64,
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

// TODO: Write tests for each function in the implementation above
#[cfg(test)]
pub mod pokemon_tests {
    use super::*;

    fn creator_helper() -> Pokemon {
        let mut bulbasaur: Pokemon = Pokemon::new();
        bulbasaur.name = String::from("Bulbasaur");
        bulbasaur.type_1 = Types::Grass;
        bulbasaur.type_2 = None;
        bulbasaur.max_hp = 100.0;
        bulbasaur.hp = bulbasaur.max_hp.clone();
        bulbasaur.attack = 100;
        bulbasaur.defense = 100;
        bulbasaur.special_attack = 100;
        bulbasaur.special_defense = 100;
        bulbasaur.speed = 100;
        bulbasaur
    }

    fn reset(inst: &mut Pokemon) {
        inst.hp = 100.0;
        inst.status_conditions.non_vol = None;
        inst.status_conditions.turn_count = 0;
        inst.status_conditions.vol.clear();
    }

    #[test]
    fn test_create_pokemon() {
        let bulbasaur: Pokemon = creator_helper();
        assert_eq!(bulbasaur.name, "Bulbasaur");
        assert_eq!(bulbasaur.type_1, Types::Grass);
        assert_eq!(bulbasaur.type_2, None);
        assert_eq!(bulbasaur.max_hp, 100.0);
        assert_eq!(bulbasaur.hp, 100.0);
        assert_eq!(bulbasaur.attack, 100);
        assert_eq!(bulbasaur.defense, 100);
        assert_eq!(bulbasaur.special_attack, 100);
        assert_eq!(bulbasaur.special_defense, 100);
        assert_eq!(bulbasaur.speed, 100);
    }

    #[test]
    fn test_take_damage() {
        let mut bulbasaur: Pokemon = creator_helper();
        // Take damage > current health
        bulbasaur.take_attack_damage(897.0);
        assert_eq!(bulbasaur.hp, 0.0);
        bulbasaur.take_attack_damage(500.0);
        assert_eq!(bulbasaur.hp, 0.0);

        // Take damage < current health
        reset(&mut bulbasaur);
        bulbasaur.take_attack_damage(84.5);
        assert_eq!(bulbasaur.hp, 15.5);
        assert_eq!(bulbasaur.status_conditions.non_vol, None);
        assert_ne!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Fainted)
        );

        // Take damage such that health < 1.0
        reset(&mut bulbasaur);
        bulbasaur.take_attack_damage(99.5);
        assert_eq!(bulbasaur.hp, 0.0);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Fainted)
        );

        // Take damage == current health
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = None;
        bulbasaur.take_attack_damage(100.0);
        assert_eq!(bulbasaur.hp, 0.0);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Fainted)
        );
    }

    #[test]
    fn test_poison_damage() {
        let mut bulbasaur: Pokemon = creator_helper();

        // Single-instance status damage test
        reset(&mut bulbasaur);
        bulbasaur
            .status_conditions
            .non_volatile_status_check(NonVolatileStatusType::Poison, &bulbasaur.name);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 87.5);

        // Multi-instance poison damage and faint test
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = None;
        bulbasaur
            .status_conditions
            .non_volatile_status_check(NonVolatileStatusType::Poison, &bulbasaur.name);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 87.5);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 75.0);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 62.5);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 50.0);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 37.5);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 25.0);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Poison)
        );
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 12.5);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 0.0);
    }

    #[test]
    fn test_toxic_damage() {
        let mut bulbasaur: Pokemon = creator_helper();

        bulbasaur
            .status_conditions
            .non_volatile_status_check(NonVolatileStatusType::Toxic, &bulbasaur.name);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 93.75);
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );
        assert_eq!(bulbasaur.hp, 81.25);
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );
        assert_eq!(bulbasaur.hp, 62.5);
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );
        assert_eq!(bulbasaur.hp, 37.5);
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );
        assert_eq!(bulbasaur.hp, 6.25);

        // Faint here
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Fainted)
        );
        assert_eq!(bulbasaur.hp, 0.0);

        // 7/16 Toxic hit
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
        bulbasaur.status_conditions.turn_count = 7;
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 56.25);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );

        // 8/16 Toxic hit
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 50.0);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );

        // 9/16 Toxic hit
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 43.75);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );

        // 10/16 Toxic hit
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 37.5);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );

        // 11/16 Toxic hit
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 31.25);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );

        // 12/16 Toxic hit
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 25.0);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );

        // 13/16 Toxic hit
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 18.75);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );

        // 14/16 Toxic hit
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 12.5);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );

        // 15/16 Toxic hit
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 6.25);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Toxic)
        );

        // 16/16 Toxic hit
        bulbasaur.hp = 100.0;
        bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 0.0);
        assert_eq!(
            bulbasaur.status_conditions.non_vol,
            Some(NonVolatileStatusType::Fainted)
        );
    }
}
