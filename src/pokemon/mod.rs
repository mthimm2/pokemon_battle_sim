mod typing;
use typing::Types;

mod statuses;
use statuses::{NonVolatileStatusType, VolatileStatusType};

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
    attack_modifier: u8,
    defense_modifier: u8,
    special_attack_modifier: u8,
    special_defense_modifier: u8,
    speed_modifier: u8,
    moves: Vec<Move>,
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
            moves: Vec::new(),
        }
    }

    // Performs the job of checking the incoming move's damage against current hp
    // Subtracts off health and faints the Pokemon as needed
    fn faint_check(&mut self, damage: u16) {
        if damage >= self.hp {
            self.hp = 0;
            self.status_condition = Some(NonVolatileStatusType::Fainted);
        } else {
            self.hp -= incoming_damage;
        }
    }

    // Handles assigning a non-volatile status condition to the Pokemon on the field.
    // If no status condition, assigns a status condition
    // If a status condition already exists, prints out the prompts
    // Panics if a fainted Pokemon is still on the field
    fn non_volatile_status_check(&mut self, incoming_status: NonVolatileStatusType) {
        match self.non_volatile_status_condition {
            None => self.non_volatile_status_condition = Some(incoming_status),
            Some(non_volatile_condition) => {
                print!("{} is already ", self.name);
                match non_volatile_condition {
                    Freeze => print("Frozen!\n"),
                    Paralysis => print("Paralyzed!\n"),
                    Burn => print("Burned!\n"),
                    Sleep => print("Sleeping!\n"),
                    Fainted => panic!("We should not be here!"),
                    _ => print("Poisoned!\n"), // Toxic case
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
        faint_check(&incoming_damage);
    }

    // Wrapper function for the faint check in the context of status damage
    pub fn take_status_damage(&mut self) {
        match self.status_condition {
            Some(NonVolatileStatusType::Poison) => {
                let poison_damage: u16 = ((self.hp as f64) * 0.125) as u16; // 1/8th
                faint_check(&poison_damage);
            }
            Some(NonVolatileStatusType::Burn) => {
                let burn_damage: u16 = ((self.hp as f64) * 0.0675) as u16; // 1/16th
                faint_check(&burn_damage);
            }
            Some(NonVolatileStatusType::Toxic(toxic_counter)) => {
                // (toxic_counter / 16) % per turn
                let toxic_damage: u16 =
                    (((self.toxic_counter / 16.0) as f64) * 0.0675 * (self.hp as f64)) as u16;
                self.non_volatile_status_condition =
                    Some(NonVolatileStatusType::Toxic(toxic_counter + 1));
                faint_check(&toxic_damage);
            }
            Some(VolatileStatusType::Seeded) => {
                let leech_seed_damage: u16 = (self.hp as f64 * 0.125) as u16;
                faint_check(&leech_seed_damage);
            }
            Some(VolatileStatusType::Bound) => {
                // This covers bind, wrap, and clamp
                let bind_damage: u16 = (self.hp as f64 * 0.0675) as u16;
                faint_check(&bind_damage);
            }
            None => continue,
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
#[test]
fn test_create_pokemon() {
    todo!();
}
