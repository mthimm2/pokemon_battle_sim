use crate::statuses::{NonVolatileStatusType, VolatileStatusType};
use crate::typing::Types;

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
    non_volatile_status_condition: Option<NonVolatileStatusType>,
    volatile_status_condition: Option<VolatileStatusType>, // TODO: This needs to be a vector, because you could be bound and seeded simultaneously.

    // TODO: These should be floats. Think about Growl, etc lowering attack.
    attack_modifier: u8,
    defense_modifier: u8,
    special_attack_modifier: u8,
    special_defense_modifier: u8,
    speed_modifier: u8,
    // moves: Vec<Move>,
}

impl Pokemon {
    pub fn new(
        &self,
        name: String,
        type_1: Types,
        type_2: Option<Types>,
        hp: u16,
        attack: u16,
        defense: u16,
        special_attack: u16,
        special_defense: u16,
        speed: u16,
    ) -> Pokemon {
        // TODO: Make helper functions to handle all of these fields being instantiated
        Pokemon {
            name,
            type_1,
            type_2,
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
            non_volatile_status_condition: None,
            volatile_status_condition: None,
            attack_modifier: 1,
            defense_modifier: 1,
            special_attack_modifier: 1,
            special_defense_modifier: 1,
            speed_modifier: 1,
            // moves: Vec::new(),
        }
    }

    // Performs the job of checking the incoming move's damage against current hp
    // Subtracts off health and faints the Pokemon as needed
    fn faint_check(&mut self, damage: u16) {
        if damage >= self.hp {
            self.hp = 0;
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
            None => self.non_volatile_status_condition = Some(incoming_status),
            Some(non_volatile_condition) => {
                print!("{} is already ", self.name);
                match non_volatile_condition {
                    NonVolatileStatusType::Freeze(turn_count) => print!("Frozen!\n"),
                    NonVolatileStatusType::Paralysis => print!("Paralyzed!\n"),
                    NonVolatileStatusType::Burn => print!("Burned!\n"),
                    NonVolatileStatusType::Sleep(turn_count) => print!("Sleeping!\n"),
                    NonVolatileStatusType::Fainted => panic!("We should not be here!"),
                    _ => print!("Poisoned!\n"), // Toxic and Poison case
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
    pub fn take_attack_damage(&mut self, incoming_damage: u16) {
        self.faint_check(incoming_damage);
    }

    // Wrapper function for the faint check in the context of status damage
    pub fn take_status_damage(&mut self) {
        // Damage from non-volatile statuses first
        match &self.non_volatile_status_condition {
            Some(non_volatile_condition) => {
                match non_volatile_condition {
                    NonVolatileStatusType::Poison => {
                        // TODO: Need a new field to facilitate correct poison and burn damage
                        // Call field max_hp
                        let poison_damage: u16 = ((self.hp as f64) * 0.125) as u16; // 1/8th
                        self.faint_check(poison_damage);
                    }
                    NonVolatileStatusType::Burn => {
                        let burn_damage: u16 = ((self.hp as f64) * 0.0675) as u16; // 1/16th
                        self.faint_check(burn_damage);
                    }
                    NonVolatileStatusType::Toxic(toxic_counter) => {
                        // (toxic_counter / 16) % per turn
                        let toxic_damage: u16 =
                            (((toxic_counter / 16) as f64) * 0.0675 * (self.hp as f64)) as u16;
                        self.non_volatile_status_condition =
                            Some(NonVolatileStatusType::Toxic(toxic_counter + 1));
                        self.faint_check(toxic_damage);
                    }
                    _ => self.faint_check(0),
                }
            }
            None => self.faint_check(0),
        }
        // Damage from volatile statuses second
        match &self.volatile_status_condition {
            Some(volatile_condition) => {
                match volatile_condition {
                    VolatileStatusType::Seeded => {
                        let leech_seed_damage: u16 = (self.hp as f64 * 0.125) as u16;
                        self.faint_check(leech_seed_damage);
                    }
                    VolatileStatusType::Bound(turn_count) => {
                        // This covers bind, wrap, and clamp
                        let bind_damage: u16 = (self.hp as f64 * 0.0675) as u16;
                        self.faint_check(bind_damage);
                    }
                    VolatileStatusType::Confusion(turn_count) => {
                        todo!(); // TODO: Implement confusion damage check
                    }
                    _ => self.faint_check(0),
                }
            }
            None => self.faint_check(0),
        }
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
mod pokemon_tests {
    use super::*;
    #[test]
    fn test_create_pokemon() {
        println!("This worked!");
    }
}
