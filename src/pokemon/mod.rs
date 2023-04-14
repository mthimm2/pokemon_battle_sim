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
    volatile_status_condition: Option<VolatileStatusType>, // TODO: This might need to be a vector, because you could be bound, seeded, and poisoned simultaneously.
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
        moves: Vec<Move>,
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
            moves,
        }
    }

    fn faint_check(&mut self, damage: u16) {
        if damage >= self.hp {
            self.hp = 0;
            // self.status_condition = Some(StatusType::Fainted);
        } else {
            self.hp -= incoming_damage;
        }
    }

    pub fn take_attack_damage(&mut self, incoming_damage: u16) {
        faint_check(&incoming_damage);
    }

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

    pub fn construct_moveset(&mut self, moves: Vec<String>) {
        // TODO: Let a vector of strings be used to select moves out of the overall moveset of Gen 1
        // Assign moves in order of their placement in the vector.
    }
}
