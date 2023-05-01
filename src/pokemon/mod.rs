#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use crate::statuses::{Damage, NonVolatileStatusType, VolatileStatusType};
use crate::typing::Types;

#[derive(Debug)]
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
    non_volatile_status_condition: Option<NonVolatileStatusType>,
    volatile_status_condition: Option<VolatileStatusType>, // TODO: This needs to be a vector, because you could be bound, seeded, and confused simultaneously.
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
        let mut name: String = String::new();
        // let results = creator_helper();
        let mut type_1: Types = Types::Normal;
        let mut type_2: Option<Types> = None;
        let mut max_hp: f64 = 0.0;
        let mut hp: f64 = 0.0;
        let mut attack: u16 = 0;
        let mut defense: u16 = 0;
        let mut special_attack: u16 = 0;
        let mut special_defense: u16 = 0;
        let mut speed: u16 = 0;
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
            non_volatile_status_condition: None,
            volatile_status_condition: None,
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
            self.non_volatile_status_condition = Some(NonVolatileStatusType::Fainted);
        } else {
            self.hp -= damage;
        }
    }

    // Handles assigning a non-volatile status condition to the Pokemon on the field.
    // If no status condition, assigns a status condition
    // If a status condition already exists, prints out the prompts
    // Panics if a fainted Pokemon is still on the field
    fn non_volatile_status_check(&mut self, incoming_status: NonVolatileStatusType) {
        match &self.non_volatile_status_condition {
            // TODO: Put in handling for burn halving attack and paralysis halving speed.
            None => self.non_volatile_status_condition = Some(incoming_status),
            // If we're already statused and someone's trying to status us again,
            // we print out that we're already afflicted with status X
            Some(non_volatile_condition) => {
                print!("{} is already ", self.name);
                match non_volatile_condition {
                    NonVolatileStatusType::Freeze(turn_count) => println!("Frozen!"),
                    NonVolatileStatusType::Paralysis => println!("Paralyzed!"),
                    NonVolatileStatusType::Burn => println!("Burned!"),
                    NonVolatileStatusType::Sleep(turn_count) => println!("Sleeping!"),
                    NonVolatileStatusType::Fainted => panic!("We should not be here!"),
                    _ => println!("Poisoned!"), // Toxic and Poison case
                }
            }
        }
    }

    // Handles assigning a volatile status condition to the Pokemon on the field
    // TODO: Figure out how to handle a vector of volatile status conditions
    fn volatile_status_check(&mut self, incoming_status: VolatileStatusType) {
        match self.volatile_status_condition {
            None => self.volatile_status_condition = Some(incoming_status),
            Some(volatile_condition) => {
                println!("But it failed!");
                // if incoming_status == volatile_condition {
                //     print!("{} is already ", self.name);
                //     match incoming_status {
                //         Bound => print!("Bound!\n"),
                //         Confusion => print!("Confused!\n"),
                //         Flinch => continue,
                //         Seeded => print!("Seeded!\n"),
                //         Rampage => continue, // TODO: This might not be correct for a move like Thrash
                //         Charging => continue, // TODO: This might not be correct for a move like Sky Attack
                //         Recharging => continue, // TODO: This might not be correct for a move like Hyper Beam
                //     }
                // } else {}
            }
        }
    }

    // Wrapper function for the faint check in the context of direct damage
    pub fn take_attack_damage(&mut self, incoming_damage: f64) {
        self.faint_check(incoming_damage);
    }

    // Wrapper function for the faint check in the context of status damage
    pub fn take_status_damage(&mut self) {
        // Damage from non-volatile statuses first
        let mut non_volatile_status_damage: f64 = 0.0;
        let mut volatile_status_damage: f64 = 0.0;

        match self.non_volatile_status_condition.take() {
            Some(non_volatile_condition) => {
                match non_volatile_condition {
                    NonVolatileStatusType::Poison => {
                        non_volatile_status_damage =
                            non_volatile_condition.status_damage(&self.max_hp);
                        self.non_volatile_status_condition = Some(NonVolatileStatusType::Poison);
                    }
                    NonVolatileStatusType::Burn => {
                        non_volatile_status_damage =
                            non_volatile_condition.status_damage(&self.max_hp);
                        self.non_volatile_status_condition = Some(NonVolatileStatusType::Burn);
                    }
                    NonVolatileStatusType::Toxic(toxic_counter) => {
                        // (toxic_counter / 16) % per turn
                        // let toxic_damage: f64 = (*toxic_counter as f64) * 0.0625 * self.max_hp;
                        non_volatile_status_damage =
                            non_volatile_condition.status_damage(&self.max_hp);
                        self.non_volatile_status_condition =
                            Some(NonVolatileStatusType::Toxic(toxic_counter + 1));
                    }
                    NonVolatileStatusType::Freeze(turn_count) => {
                        non_volatile_status_damage =
                            non_volatile_condition.status_damage(&self.max_hp);
                        self.non_volatile_status_condition =
                            Some(NonVolatileStatusType::Freeze(turn_count + 1));
                    }
                    NonVolatileStatusType::Paralysis => {
                        non_volatile_status_damage =
                            non_volatile_condition.status_damage(&self.max_hp);
                        self.non_volatile_status_condition = Some(NonVolatileStatusType::Paralysis);
                    }
                    NonVolatileStatusType::Sleep(turn_count) => {
                        non_volatile_status_damage =
                            non_volatile_condition.status_damage(&self.max_hp);
                        self.non_volatile_status_condition =
                            Some(NonVolatileStatusType::Sleep(turn_count + 1));
                    }
                    NonVolatileStatusType::Fainted => {
                        panic!("We should not be here! We should have been fainted!");
                    }
                }
            }
            None => non_volatile_status_damage = 0.0,
        }

        // Leave it to each type to calculate its damage, then pass that to the faint_check function
        self.faint_check(non_volatile_status_damage);

        // Damage from volatile statuses second
        match self.volatile_status_condition.take() {
            Some(volatile_condition) => {
                match volatile_condition {
                    VolatileStatusType::Seeded => {
                        volatile_status_damage = volatile_condition.status_damage(&self.max_hp);
                        self.volatile_status_condition = Some(VolatileStatusType::Seeded);
                    }
                    VolatileStatusType::Bound(turn_count) => {
                        // This covers bind, wrap, and clamp
                        volatile_status_damage = volatile_condition.status_damage(&self.max_hp);
                        self.volatile_status_condition =
                            Some(VolatileStatusType::Bound(turn_count + 1));
                    }
                    VolatileStatusType::Confusion(turn_count) => {
                        volatile_status_damage = volatile_condition.status_damage(&self.max_hp);
                        self.volatile_status_condition =
                            Some(VolatileStatusType::Confusion(turn_count + 1));
                    }
                    VolatileStatusType::Flinch => {
                        volatile_status_damage = volatile_condition.status_damage(&self.max_hp);
                        self.volatile_status_condition = Some(VolatileStatusType::Flinch);
                    }
                    VolatileStatusType::Rampage(turn_count) => {
                        volatile_status_damage = volatile_condition.status_damage(&self.max_hp);
                        self.volatile_status_condition =
                            Some(VolatileStatusType::Rampage(turn_count + 1));
                    }
                    VolatileStatusType::Charging(turn_count) => {
                        volatile_status_damage = volatile_condition.status_damage(&self.max_hp);
                        self.volatile_status_condition =
                            Some(VolatileStatusType::Charging(turn_count + 1));
                    }
                    VolatileStatusType::Recharging(turn_count) => {
                        volatile_status_damage = volatile_condition.status_damage(&self.max_hp);
                        self.volatile_status_condition =
                            Some(VolatileStatusType::Recharging(turn_count + 1));
                    }
                }
            }
            None => self.faint_check(0.0),
        }
        self.faint_check(volatile_status_damage);
    }

    // Reads the relevant moveset from the file and returns it as a vector of strings
    pub fn construct_moveset(&mut self, moves: Vec<String>) {
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

    #[test]
    fn test_create_pokemon() {
        let mut bulbasaur: Pokemon = creator_helper();
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
        bulbasaur.hp = 100.0;
        bulbasaur.non_volatile_status_condition = None;
        bulbasaur.take_attack_damage(84.5);
        assert_eq!(bulbasaur.hp, 15.5);
        assert_eq!(bulbasaur.non_volatile_status_condition, None);
        assert_ne!(
            bulbasaur.non_volatile_status_condition,
            Some(NonVolatileStatusType::Fainted)
        );

        // Take damage such that health < 1.0
        bulbasaur.hp = 100.0;
        bulbasaur.non_volatile_status_condition = None;
        bulbasaur.take_attack_damage(99.5);
        assert_eq!(bulbasaur.hp, 0.0);
        assert_eq!(
            bulbasaur.non_volatile_status_condition,
            Some(NonVolatileStatusType::Fainted)
        );

        // Take damage == current health
        bulbasaur.hp = 100.0;
        bulbasaur.non_volatile_status_condition = None;
        bulbasaur.take_attack_damage(100.0);
        assert_eq!(bulbasaur.hp, 0.0);
        assert_eq!(
            bulbasaur.non_volatile_status_condition,
            Some(NonVolatileStatusType::Fainted)
        );
    }

    #[test]
    fn test_poison_damage() {
        let mut bulbasaur: Pokemon = creator_helper();

        // Single-instance status damage test
        bulbasaur.hp = 100.0;
        bulbasaur.non_volatile_status_condition = None;
        bulbasaur.non_volatile_status_check(NonVolatileStatusType::Poison);
        bulbasaur.take_status_damage();
        assert_eq!(bulbasaur.hp, 87.5);

        // Multi-instance poison damage and faint test
        bulbasaur.hp = 100.0;
        bulbasaur.non_volatile_status_condition = None;
        bulbasaur.non_volatile_status_check(NonVolatileStatusType::Poison);
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
            bulbasaur.non_volatile_status_condition,
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
        bulbasaur.hp = 100.0;
        bulbasaur.non_volatile_status_condition = None;

        bulbasaur.non_volatile_status_check(NonVolatileStatusType::Toxic(1));
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.non_volatile_status_condition,
            Some(NonVolatileStatusType::Toxic(2))
        );
        assert_eq!(bulbasaur.hp, 93.75);
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.non_volatile_status_condition,
            Some(NonVolatileStatusType::Toxic(3))
        );
        assert_eq!(bulbasaur.hp, 81.25);
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.non_volatile_status_condition,
            Some(NonVolatileStatusType::Toxic(4))
        );
        assert_eq!(bulbasaur.hp, 62.5);
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.non_volatile_status_condition,
            Some(NonVolatileStatusType::Toxic(5))
        );
        assert_eq!(bulbasaur.hp, 37.5);
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.non_volatile_status_condition,
            Some(NonVolatileStatusType::Toxic(6))
        );
        assert_eq!(bulbasaur.hp, 6.25);

        // Faint here
        bulbasaur.take_status_damage();
        assert_eq!(
            bulbasaur.non_volatile_status_condition,
            Some(NonVolatileStatusType::Fainted)
        );
        assert_eq!(bulbasaur.hp, 0.0);

        // TODO: Write test for when Toxic hits between 7-16 in intensity.
        bulbasaur.hp = 100.0;
        bulbasaur.non_volatile_status_condition = Some(NonVolatileStatusType::Toxic(7));
        bulbasaur.take_status_damage();
    }
}
