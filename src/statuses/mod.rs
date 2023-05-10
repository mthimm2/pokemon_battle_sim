use rand::Rng;

pub trait Damage {
    fn status_damage(&self, max_hp: f64, turn_count: u8, attack: u16, defense: u16) -> f64;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NonVolatileStatusType {
    Freeze,
    Paralysis,
    Poison,
    Burn,
    Toxic,
    Sleep,
    Fainted,
}

impl Damage for NonVolatileStatusType {
    fn status_damage(&self, max_hp: f64, turn_count: u8, _attack: u16, _defense: u16) -> f64 {
        match &self {
            Self::Poison => 0.125 * max_hp,
            Self::Burn => 0.0625 * max_hp,
            Self::Toxic => (turn_count as f64) * 0.0625 * max_hp,
            Self::Fainted => panic!("Fainted damage is not calculated."),
            _ => 0.0, // Sleep, Freeze, Paralysis
        }
    }
}

// Only Gen 1 volatile status types are included below.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum VolatileStatusType {
    Bound,
    Confusion,
    Flinch,
    Seeded,
    Rampage,    // Used for Thrash and Petal Dance to lock the user in.
    Charging, // Used for Sky Attack, Solar Beam, Fly, Dig, Razor Wind, and Skull Bash to stop attacking for the first turn
    Recharging, // Used for Hyper Beam to stop attacking for one turn
}

impl Damage for VolatileStatusType {
    fn status_damage(&self, max_hp: f64, _turn_count: u8, attack: u16, defense: u16) -> f64 {
        match &self {
            Self::Bound => max_hp * 0.0625,
            Self::Confusion => {
                // Damage formula reference: https://bulbapedia.bulbagarden.net/wiki/Generation_V
                let mut rng = rand::thread_rng();
                let level_part: f64 = (2.0 * 100.0) / 5.0;
                let power: f64 = 40.0;
                let attack_over_defense: f64 = attack as f64 / defense as f64;
                let numerator: f64 = level_part * power * attack_over_defense;
                let denominator: f64 = 50.0;
                f64::floor((numerator / denominator) + 2.0 * rng.gen_range(0.85..1.0))
            }
            Self::Seeded => max_hp * 0.125,
            _ => 0.0,
        }
    }
}

pub struct Status {
    pub non_vol: Option<NonVolatileStatusType>,
    pub vol: Vec<Option<(VolatileStatusType, u8)>>, // (volatile_status, turn_count_unique_to_that_specific_volatile_status)
    pub turn_count: u8,
}

impl Status {
    pub fn damage(&mut self, max_hp: f64, attack: u16, defense: u16) -> (f64, f64) {
        let mut non_vol_and_vol_damage: (f64, f64) = (0.0, 0.0);

        match &self.non_vol {
            Some(non_vol_status) => {
                // We always run the damage calc for any non-volatile status condition
                non_vol_and_vol_damage.0 =
                    non_vol_status.status_damage(max_hp, self.turn_count, attack, defense);
                match non_vol_status {
                    // If we're hit with a non-volatile status, we can perform the damage calc
                    NonVolatileStatusType::Sleep
                    | NonVolatileStatusType::Freeze
                    | NonVolatileStatusType::Toxic => {
                        // If we're toxiced, slept, or frozen, we increment turn count
                        self.turn_count += 1;
                    }
                    _ => {} // Poison, burn, para, and faint don't increment turn count
                }
            }
            None => {} // No action taken if we're not statused
        }

        self.vol.iter_mut().for_each(|vol_status| {
            match vol_status {
                Some(condition) => {
                    // Run the damage calc for any valid volatile status condition
                    non_vol_and_vol_damage.1 +=
                        condition
                            .0
                            .status_damage(max_hp, self.turn_count, attack, defense);

                    // These conditions all have a relevant turn count associated with
                    // them that we want to increment
                    match condition.0 {
                        VolatileStatusType::Bound
                        | VolatileStatusType::Confusion
                        | VolatileStatusType::Flinch
                        | VolatileStatusType::Charging
                        | VolatileStatusType::Recharging
                        | VolatileStatusType::Rampage => {
                            condition.1 += 1;
                        }
                        VolatileStatusType::Seeded => {} // No individual turn count increment here
                    }
                }
                None => {} // No action taken if we're not statused
            }
        });
        non_vol_and_vol_damage
    }

    // Handles assigning a non-volatile status condition to the Pokemon on the field.
    // If no status condition, assigns a status condition
    // If a status condition already exists, prints out the prompts
    // Panics if a fainted Pokemon is still on the field
    pub fn non_volatile_status_check(
        &mut self,
        incoming_status: NonVolatileStatusType,
        name: &str,
    ) {
        match &self.non_vol {
            None => {
                self.non_vol = Some(incoming_status);
                self.turn_count = 1;
            }
            // If we're already statused and someone's trying to status us again,
            // we print out that we're already afflicted with status X
            Some(non_volatile_condition) => {
                print!("{} is already ", name);
                match non_volatile_condition {
                    NonVolatileStatusType::Freeze => println!("Frozen!"),
                    NonVolatileStatusType::Paralysis => println!("Paralyzed!"),
                    NonVolatileStatusType::Burn => println!("Burned!"),
                    NonVolatileStatusType::Sleep => println!("Sleeping!"),
                    NonVolatileStatusType::Fainted => panic!("We should not be here!"),
                    _ => println!("Poisoned!"), // Toxic and Poison case
                }
            }
        }
    }

    // Handles assigning a volatile status condition to the Pokemon on the field
    pub fn volatile_status_check(&mut self, incoming_status: VolatileStatusType) {
        if !self
            .vol
            .iter()
            .any(|status| incoming_status == status.as_ref().unwrap().0)
        {
            self.vol.push(Some((incoming_status, 1)));
        } else {
            println!("But it failed!");
        }
    }
}
