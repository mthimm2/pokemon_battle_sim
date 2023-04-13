#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonVolatileStatusType {
    Freeze(u8),
    Paralysis,
    Poison,
    Burn,
    Toxic(u8),
    Sleep(u8),
}

// Only Gen 1 volatile status types are included below.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    // All non-volatile status types
    let paralyze: Option<NonVolatileStatusType> = Some(NonVolatileStatusType::Paralysis);
    let poison: Option<NonVolatileStatusType> = Some(NonVolatileStatusType::Poison);
    let burn: Option<NonVolatileStatusType> = Some(NonVolatileStatusType::Burn);
    let toxic: Option<NonVolatileStatusType> = Some(NonVolatileStatusType::Toxic(1));
    let sleep: Option<NonVolatileStatusType> = Some(NonVolatileStatusType::Sleep(1));
    let mut frozen: Option<NonVolatileStatusType> = Some(NonVolatileStatusType::Freeze(1));

    // Frozen test
    match frozen {
        Some(NonVolatileStatusType::Freeze(turn_count)) => {
            // Check initial value of the enum and then increment via reassignment and check again
            assert_eq!(1, turn_count);
            frozen = Some(NonVolatileStatusType::Freeze(turn_count + 1));
            assert_eq!(frozen, Some(NonVolatileStatusType::Freeze(2)));
        }
        _ => panic!("Frozen test failed."),
    }
    assert_ne!(frozen, Some(NonVolatileStatusType::Sleep(2)));
    assert_ne!(frozen, Some(NonVolatileStatusType::Paralysis));
    assert_ne!(frozen, Some(NonVolatileStatusType::Burn));
    assert_ne!(frozen, Some(NonVolatileStatusType::Poison));
    assert_ne!(frozen, Some(NonVolatileStatusType::Toxic(1)));
    assert_ne!(frozen, Some(NonVolatileStatusType::Freeze(1)));

    // Paralysis test
    match paralyze {
        Some(NonVolatileStatusType::Paralysis) => {
            assert_eq!(paralyze, Some(NonVolatileStatusType::Paralysis));
        }
        _ => panic!("Paralysis test failed."),
    }
    assert_ne!(paralyze, Some(NonVolatileStatusType::Sleep(2)));
    assert_ne!(paralyze, Some(NonVolatileStatusType::Burn));
    assert_ne!(paralyze, Some(NonVolatileStatusType::Poison));
    assert_ne!(paralyze, Some(NonVolatileStatusType::Toxic(1)));
    assert_ne!(paralyze, Some(NonVolatileStatusType::Freeze(1)));

    // Poison test
    match poison {
        Some(NonVolatileStatusType::Poison) => {
            assert_eq!(poison, Some(NonVolatileStatusType::Poison))
        }
        _ => panic!("Poison test failed."),
    }
    assert_ne!(paralyze, Some(NonVolatileStatusType::Sleep(2)));
    assert_ne!(paralyze, Some(NonVolatileStatusType::Burn));
    assert_ne!(paralyze, Some(NonVolatileStatusType::Paralysis));
    assert_ne!(paralyze, Some(NonVolatileStatusType::Toxic(1)));
    assert_ne!(paralyze, Some(NonVolatileStatusType::Freeze(1)));

    // Burn test
    match burn {
        Some(NonVolatileStatusType::Burn) => {
            assert_eq!(burn, Some(NonVolatileStatusType::Burn))
        }
        _ => panic!("Burn test failed."),
    }
    assert_ne!(burn, Some(NonVolatileStatusType::Sleep(78)));
    assert_ne!(burn, Some(NonVolatileStatusType::Poison));
    assert_ne!(burn, Some(NonVolatileStatusType::Paralysis));
    assert_ne!(burn, Some(NonVolatileStatusType::Toxic(21)));
    assert_ne!(burn, Some(NonVolatileStatusType::Freeze(0)));

    // Toxic test
    match toxic {
        Some(NonVolatileStatusType::Toxic(toxic_count)) => {
            assert_eq!(toxic, Some(NonVolatileStatusType::Toxic(1)));
            toxic = Some(NonVolatileStatusType::Toxic(2));
            assert_eq!(toxic, Some(NonVolatileStatusType::Toxic(2)));
            assert_ne!(toxic, Some(NonVolatileStatusType::Toxic(3)));
        }
        _ => panic!("Toxic test failed."),
    }
    assert_ne!(toxic, Some(NonVolatileStatusType::Sleep(78)));
    assert_ne!(toxic, Some(NonVolatileStatusType::Poison));
    assert_ne!(toxic, Some(NonVolatileStatusType::Paralysis));
    assert_ne!(toxic, Some(NonVolatileStatusType::Burn));
    assert_ne!(toxic, Some(NonVolatileStatusType::Freeze(0)));
    assert_ne!(toxic, Some(NonVolatileStatusType::Toxic(38)));

    // Sleep test
    match sleep {
        Some(NonVolatileStatusType::Sleep(sleep_count)) => {
            assert_eq!(toxic, Some(NonVolatileStatusType::Sleep(1)));
            sleep = Some(NonVolatileStatusType::Sleep(2));
            assert_eq!(toxic, Some(NonVolatileStatusType::Sleep(2)));
            assert_ne!(toxic, Some(NonVolatileStatusType::Sleep(3)));
        }
        _ => panic!("Sleep test failed."),
    }
    assert_ne!(sleep, Some(NonVolatileStatusType::Sleep(78)));
    assert_ne!(sleep, Some(NonVolatileStatusType::Poison));
    assert_ne!(sleep, Some(NonVolatileStatusType::Paralysis));
    assert_ne!(sleep, Some(NonVolatileStatusType::Burn));
    assert_ne!(sleep, Some(NonVolatileStatusType::Freeze(0)));
    assert_ne!(sleep, Some(NonVolatileStatusType::Toxic(38)));
}

#[test]
fn test_volatile_status() {}
