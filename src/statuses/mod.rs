pub trait Damage {
    fn status_damage(&self, max_hp: &f64, turn_count: &u8) -> f64;
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
    fn status_damage(&self, max_hp: &f64, turn_count: &u8) -> f64 {
        match &self {
            Self::Poison => 0.125 * max_hp,
            Self::Burn => 0.0625 * max_hp,
            Self::Toxic => (*turn_count as f64) * 0.0625 * max_hp,
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
    fn status_damage(&self, max_hp: &f64, _turn_count: &u8) -> f64 {
        match &self {
            Self::Bound => max_hp * 0.0625,
            Self::Confusion => 0.0, // TODO: Implement confusion damage
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
    pub fn damage(&mut self, max_hp: &f64) -> (f64, f64) {
        let mut non_vol_and_vol_damage: (f64, f64) = (0.0, 0.0);
        match &self.non_vol {
            Some(non_vol_status) => match non_vol_status {
                NonVolatileStatusType::Poison => {
                    non_vol_and_vol_damage.0 =
                        non_vol_status.status_damage(max_hp, &self.turn_count)
                }
                NonVolatileStatusType::Burn => {
                    non_vol_and_vol_damage.0 =
                        non_vol_status.status_damage(max_hp, &self.turn_count)
                }
                NonVolatileStatusType::Toxic => {
                    non_vol_and_vol_damage.0 =
                        non_vol_status.status_damage(max_hp, &self.turn_count);
                    self.turn_count += 1;
                }
                NonVolatileStatusType::Fainted => {
                    // This panics in the trait implementaiton
                    non_vol_and_vol_damage.0 =
                        non_vol_status.status_damage(max_hp, &self.turn_count);
                }
                NonVolatileStatusType::Paralysis => {
                    non_vol_and_vol_damage.0 =
                        non_vol_status.status_damage(max_hp, &self.turn_count);
                }
                _ => {
                    // Sleep and Freeze cases
                    non_vol_and_vol_damage.0 =
                        non_vol_status.status_damage(max_hp, &self.turn_count);
                    self.turn_count += 1;
                }
            },
            None => non_vol_and_vol_damage.0 = 0.0,
        }

        self.vol.iter_mut().for_each(|vol_status| {
            match vol_status {
                Some(condition) => match condition.0 {
                    VolatileStatusType::Bound => {
                        non_vol_and_vol_damage.1 +=
                            condition.0.status_damage(max_hp, &self.turn_count);
                        condition.1 += 1;
                    }
                    VolatileStatusType::Seeded => {
                        non_vol_and_vol_damage.1 +=
                            condition.0.status_damage(max_hp, &self.turn_count);
                        condition.1 += 1;
                    }
                    VolatileStatusType::Confusion => {}
                    _ => {
                        // Flinch, Charging, Recharging, Rampage case
                        non_vol_and_vol_damage.1 +=
                            condition.0.status_damage(max_hp, &self.turn_count);
                        condition.1 += 1;
                    }
                },
                None => non_vol_and_vol_damage.1 = 0.0,
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
        name: &String,
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
        let mut new_status_flag: bool = true;
        self.vol.iter().for_each(|status| {
            if incoming_status == status.as_ref().unwrap().0 {
                println!("But it failed!");
                new_status_flag = false;
            }
        });
        if new_status_flag {
            self.vol.push(Some((incoming_status, 1)));
        }
    }
}

#[cfg(test)]
mod status_types_tests {
    use super::*;
    #[test]
    #[should_panic]
    fn test_non_volatile_status() {
        // All non-volatile status types
        let mut status_tester: Status = Status {
            non_vol: Some(NonVolatileStatusType::Freeze),
            vol: Vec::new(),
            turn_count: 1,
        };

        // Frozen test
        match status_tester.non_vol {
            Some(NonVolatileStatusType::Freeze) => {
                assert_eq!(status_tester.turn_count, 1);
            }
            _ => panic!("Freeze status did not work."),
        }
        let res: (f64, f64) = status_tester.damage(&100.0);
        assert_ne!(
            status_tester.non_vol,
            Some(NonVolatileStatusType::Paralysis)
        );
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Burn));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Poison));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Sleep));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Toxic));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Fainted));
        assert_eq!(res.0, 0.0);
        assert_eq!(res.1, 0.0);
        assert_eq!(status_tester.turn_count, 2);

        // Paralysis test
        status_tester.non_vol = Some(NonVolatileStatusType::Paralysis);
        status_tester.turn_count = 1;
        let res: (f64, f64) = status_tester.damage(&100.0);
        match status_tester.non_vol {
            Some(NonVolatileStatusType::Paralysis) => {
                assert_eq!(status_tester.turn_count, 1);
            }
            _ => panic!("Paralysis status did not work."),
        }
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Freeze));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Burn));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Poison));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Sleep));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Toxic));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Fainted));
        assert_eq!(res.0, 0.0);
        assert_eq!(res.1, 0.0);
        assert_eq!(status_tester.turn_count, 1);

        // Poison test
        status_tester.non_vol = Some(NonVolatileStatusType::Poison);
        status_tester.turn_count = 1;
        let res: (f64, f64) = status_tester.damage(&100.0);
        match status_tester.non_vol {
            Some(NonVolatileStatusType::Poison) => {
                assert_eq!(status_tester.turn_count, 1);
            }
            _ => panic!("Poison status did not work."),
        }
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Freeze));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Burn));
        assert_ne!(
            status_tester.non_vol,
            Some(NonVolatileStatusType::Paralysis)
        );
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Sleep));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Toxic));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Fainted));
        assert_eq!(res.0, 12.5);
        assert_eq!(res.1, 0.0);
        assert_eq!(status_tester.turn_count, 1);

        // Burn test
        status_tester.non_vol = Some(NonVolatileStatusType::Burn);
        status_tester.turn_count = 1;
        let res: (f64, f64) = status_tester.damage(&100.0);
        match status_tester.non_vol {
            Some(NonVolatileStatusType::Burn) => {
                assert_eq!(status_tester.turn_count, 1);
            }
            _ => panic!("Burn status did not work."),
        }
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Freeze));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Poison));
        assert_ne!(
            status_tester.non_vol,
            Some(NonVolatileStatusType::Paralysis)
        );
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Sleep));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Toxic));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Fainted));
        assert_eq!(res.0, 6.25);
        assert_eq!(res.1, 0.0);
        assert_eq!(status_tester.turn_count, 1);

        // Toxic test
        status_tester.non_vol = Some(NonVolatileStatusType::Toxic);
        status_tester.turn_count = 4;
        let res: (f64, f64) = status_tester.damage(&100.0);
        match status_tester.non_vol {
            Some(NonVolatileStatusType::Toxic) => {
                assert_eq!(status_tester.turn_count, 5);
            }
            _ => panic!("Toxic status did not work."),
        }
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Freeze));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Burn));
        assert_ne!(
            status_tester.non_vol,
            Some(NonVolatileStatusType::Paralysis)
        );
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Sleep));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Poison));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Fainted));
        assert_eq!(res.0, 25.0);
        assert_eq!(res.1, 0.0);
        assert_eq!(status_tester.turn_count, 5);

        // Sleep test
        status_tester.non_vol = Some(NonVolatileStatusType::Sleep);
        status_tester.turn_count = 1;
        let res: (f64, f64) = status_tester.damage(&100.0);
        match status_tester.non_vol {
            Some(NonVolatileStatusType::Sleep) => {
                assert_eq!(status_tester.turn_count, 2);
            }
            _ => panic!("Sleep status did not work."),
        }
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Freeze));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Burn));
        assert_ne!(
            status_tester.non_vol,
            Some(NonVolatileStatusType::Paralysis)
        );
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Poison));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Toxic));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Fainted));
        assert_eq!(res.0, 0.0);
        assert_eq!(res.1, 0.0);
        assert_eq!(status_tester.turn_count, 2);

        // Fainted test
        status_tester.non_vol = Some(NonVolatileStatusType::Fainted);
        status_tester.turn_count = 1;
        let res: (f64, f64) = status_tester.damage(&100.0);
        match status_tester.non_vol {
            Some(NonVolatileStatusType::Fainted) => {
                assert_eq!(status_tester.turn_count, 1);
            }
            _ => panic!("Fainted status did not work."),
        }
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Freeze));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Burn));
        assert_ne!(
            status_tester.non_vol,
            Some(NonVolatileStatusType::Paralysis)
        );
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Sleep));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Toxic));
        assert_ne!(status_tester.non_vol, Some(NonVolatileStatusType::Poison));
        assert_eq!(res.0, 0.0);
        assert_eq!(res.1, 0.0);
        assert_eq!(status_tester.turn_count, 1);
    }

    #[test]
    fn test_volatile_status() {
        // All volatile status types
        let mut test_status = Status {
            non_vol: None,
            vol: Vec::new(),
            turn_count: 0,
        };

        // BOUND TEST

        // CONFUSION TEST

        // FLINCH TEST

        // SEEDED TEST

        // RAMPAGE TEST

        // CHARGING TEST

        // RECHARGING TEST
    }
}
