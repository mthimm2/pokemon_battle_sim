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
    let frozen: Option<NonVolatileStatusType::Freeze> = Some(NonVolatileStatusType::Freeze(0));
    match frozen {
        Some(NonVolatileStatusType::Freeze) => {
            println!("Freeze turns {}", Freeze);
        }
    }
}

#[test]
fn test_volatile_status() {}
