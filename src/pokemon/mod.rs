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
    // status_condition: Option<StatusType>, // TODO: Implement status types
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
        // status_condition: Option<StatusType>,
        type_1: Types,
        type_2: Option<Types>,
        hp: u16,
        attack: u16,
        defense: u16,
        special_attack: u16,
        special_defense: u16,
        speed: u16,
        moves: Vec<Move>,
        toxic_counter: u8,
    ) {
        Pokemon {
            name,
            type_1,
            type_2,
            hp,
            status_condition: None,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
            attack_modifier: 1,
            defense_modifier: 1,
            special_attack_modifier: 1,
            special_defense_modifier: 1,
            speed_modifier: 1,
            moves,
            toxic_counter: 0,
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
        // TODO: How does an Option<EnumType> work?
        // This match statement will likely need to be refactored
        match self.status_condition {
            Some(StatusType::Poison) => {
                let poison_damage: u16 = ((self.hp as f64) * 0.125) as u16;
                faint_check(&poison_damage);
            }
            Some(StatusType::Burn) => {
                let burn_damage: u16 = ((self.hp as f64) * 0.0675) as u16;
                faint_check(&burn_damage);
            }
            Some(StatusType::Toxic) => {
                let toxic_damage: u16 =
                    (((self.toxic_counter / 16.0) as f64) * 0.0675 * (self.hp as f64)) as u16;
                self.toxic_counter += 1;
                faint_check(&toxic_damage);
            }
            Some(StatusType::LeechSeed) => {
                let leech_seed_damage: u16 = (self.hp as f64 * 0.125) as u16;
                faint_check(&leech_seed_damage);
            }
            // TODO: Implement the binding moves
            // Some(StatusType::Wrap) => {}
            // Some(StatusType::Bind) => {}
            // Some(StatusType::Clamp) => {}
            None => continue,
        }
    }

    pub fn construct_moveset(&mut self, moves: Vec<String>) {
        // TODO: Let a vector of strings be used to select moves out of the overall moveset of Gen 1
        // Assign moves in order of their placement in the vector.
    }
}
