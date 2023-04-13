pub enum NonVolatileStatusType {
    Freeze(u8),
    Paralysis,
    Poison,
    Burn,
    Toxic(u8),
    Sleep(u8),
}

// Only Gen 1 volatile status types are included below.
pub enum VolatileStatusType {
    Bound(u8),
    Confusion(u8),
    Flinch,
    Seeded,
    Rampage(u8),    // Used for Thrash and Petal Dance to lock the user in.
    Charging(u8), // Used for Sky Attack, Solar Beam, Fly, Dig, Razor Wind, and Skull Bash to stop attacking for the first turn
    Recharging(u8), // Used for Hyper Beam to stop attacking for one turn
}

#[test]
fn test_non_volatile_status() {
    let mut frozen: Option<NonVolatileStatusType> = Some(NonVolatileStatusType::Freeze(1));
    match frozen {
        Some(NonVolatileStatusType::Freeze(turn_count)) => {
            assert_eq!(1, turn_count);
            frozen = Some(NonVolatileStatusType::Freeze(turn_count + 1));
        }
        Some(NonVolatileStatusType::Paralysis) => {}
        Some(NonVolatileStatusType::Poison) => {}
        Some(NonVolatileStatusType::Burn) => {}
        Some(NonVolatileStatusType::Toxic(_toxic_counter)) => {}
        Some(NonVolatileStatusType::Sleep(_sleep_counter)) => {}
        None => println!("a"),
    }
    assert_eq!(frozen, Some(NonVolatileStatusType::Freeze(2)));
}

#[test]
fn test_volatile_status() {}
