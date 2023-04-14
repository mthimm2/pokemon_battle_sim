#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NonVolatileStatusType {
    Freeze(u8),
    Paralysis,
    Poison,
    Burn,
    Toxic(u8),
    Sleep(u8),
    Fainted,
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
    let fainted: Option<NonVolatileStatusType> = Some(NonVolatileStatusType::Fainted);
    let mut toxic: Option<NonVolatileStatusType> = Some(NonVolatileStatusType::Toxic(1));
    let mut sleep: Option<NonVolatileStatusType> = Some(NonVolatileStatusType::Sleep(1));
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
    assert_ne!(poison, Some(NonVolatileStatusType::Sleep(2)));
    assert_ne!(poison, Some(NonVolatileStatusType::Burn));
    assert_ne!(poison, Some(NonVolatileStatusType::Paralysis));
    assert_ne!(poison, Some(NonVolatileStatusType::Toxic(1)));
    assert_ne!(poison, Some(NonVolatileStatusType::Freeze(1)));

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
        Some(NonVolatileStatusType::Toxic(_toxic_count)) => {
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
        Some(NonVolatileStatusType::Sleep(_sleep_count)) => {
            assert_eq!(sleep, Some(NonVolatileStatusType::Sleep(1)));
            sleep = Some(NonVolatileStatusType::Sleep(2));
            assert_eq!(sleep, Some(NonVolatileStatusType::Sleep(2)));
            assert_ne!(sleep, Some(NonVolatileStatusType::Sleep(3)));
        }
        _ => panic!("Sleep test failed."),
    }
    assert_ne!(sleep, Some(NonVolatileStatusType::Sleep(78)));
    assert_ne!(sleep, Some(NonVolatileStatusType::Poison));
    assert_ne!(sleep, Some(NonVolatileStatusType::Paralysis));
    assert_ne!(sleep, Some(NonVolatileStatusType::Burn));
    assert_ne!(sleep, Some(NonVolatileStatusType::Freeze(0)));
    assert_ne!(sleep, Some(NonVolatileStatusType::Toxic(38)));

    // Fainted test
    match fainted {
        Some(NonVolatileStatusType::Fainted) => {
            assert_eq!(fainted, Some(NonVolatileStatusType::Fainted))
        }
        _ => panic!("Fainted test failed."),
    }
    assert_ne!(fainted, burn);
    assert_ne!(fainted, sleep);
    assert_ne!(fainted, toxic);
    assert_ne!(fainted, poison);
    assert_ne!(fainted, paralyze);
    assert_ne!(fainted, frozen);
    assert_ne!(burn, Some(NonVolatileStatusType::Sleep(78)));
    assert_ne!(burn, Some(NonVolatileStatusType::Poison));
    assert_ne!(burn, Some(NonVolatileStatusType::Paralysis));
    assert_ne!(burn, Some(NonVolatileStatusType::Toxic(21)));
    assert_ne!(burn, Some(NonVolatileStatusType::Freeze(0)));
}

#[test]
fn test_volatile_status() {
    // All volatile status types
    let mut bound: Option<VolatileStatusType> = Some(VolatileStatusType::Bound(1));
    let mut confusion: Option<VolatileStatusType> = Some(VolatileStatusType::Confusion(1));
    let flinch: Option<VolatileStatusType> = Some(VolatileStatusType::Flinch);
    let seeded: Option<VolatileStatusType> = Some(VolatileStatusType::Seeded);
    let mut rampage: Option<VolatileStatusType> = Some(VolatileStatusType::Rampage(1));
    let mut charging: Option<VolatileStatusType> = Some(VolatileStatusType::Charging(1));
    let mut recharging: Option<VolatileStatusType> = Some(VolatileStatusType::Recharging(1));

    // BOUND TEST
    match bound {
        Some(VolatileStatusType::Bound(bind_count)) => {
            assert_eq!(bound, Some(VolatileStatusType::Bound(1)));
            assert_eq!(bind_count, 1);
            bound = Some(VolatileStatusType::Bound(2));
            assert_eq!(bound, Some(VolatileStatusType::Bound(2)));
        }
        _ => panic!("Bind test failed"),
    }
    assert_ne!(bound, confusion);
    assert_ne!(bound, flinch);
    assert_ne!(bound, seeded);
    assert_ne!(bound, rampage);
    assert_ne!(bound, charging);
    assert_ne!(bound, recharging);
    assert_ne!(bound, Some(VolatileStatusType::Confusion(1)));
    assert_ne!(bound, Some(VolatileStatusType::Flinch));
    assert_ne!(bound, Some(VolatileStatusType::Seeded));
    assert_ne!(bound, Some(VolatileStatusType::Rampage(38)));
    assert_ne!(bound, Some(VolatileStatusType::Charging(1)));
    assert_ne!(bound, Some(VolatileStatusType::Recharging(1)));

    // CONFUSION TEST
    match confusion {
        Some(VolatileStatusType::Confusion(confusion_count)) => {
            assert_eq!(confusion, Some(VolatileStatusType::Confusion(1)));
            assert_eq!(confusion_count, 1);
            confusion = Some(VolatileStatusType::Confusion(2));
            assert_eq!(confusion, Some(VolatileStatusType::Confusion(2)));
        }
        _ => panic!("Confusion test failed"),
    }
    assert_ne!(confusion, bound);
    assert_ne!(confusion, flinch);
    assert_ne!(confusion, seeded);
    assert_ne!(confusion, rampage);
    assert_ne!(confusion, charging);
    assert_ne!(confusion, recharging);
    assert_ne!(confusion, Some(VolatileStatusType::Bound(1)));
    assert_ne!(confusion, Some(VolatileStatusType::Flinch));
    assert_ne!(confusion, Some(VolatileStatusType::Seeded));
    assert_ne!(confusion, Some(VolatileStatusType::Rampage(3)));
    assert_ne!(confusion, Some(VolatileStatusType::Charging(1)));
    assert_ne!(confusion, Some(VolatileStatusType::Recharging(1)));

    // FLINCH TEST
    match flinch {
        Some(VolatileStatusType::Flinch) => {
            assert_eq!(flinch, Some(VolatileStatusType::Flinch));
        }
        _ => panic!("Flinch test failed"),
    }
    assert_ne!(flinch, bound);
    assert_ne!(flinch, confusion);
    assert_ne!(flinch, seeded);
    assert_ne!(flinch, rampage);
    assert_ne!(flinch, charging);
    assert_ne!(flinch, recharging);
    assert_ne!(flinch, Some(VolatileStatusType::Bound(1)));
    assert_ne!(flinch, Some(VolatileStatusType::Confusion(1)));
    assert_ne!(flinch, Some(VolatileStatusType::Seeded));
    assert_ne!(flinch, Some(VolatileStatusType::Rampage(3)));
    assert_ne!(flinch, Some(VolatileStatusType::Charging(1)));
    assert_ne!(flinch, Some(VolatileStatusType::Recharging(1)));

    // SEEDED TEST
    match seeded {
        Some(VolatileStatusType::Seeded) => {
            assert_eq!(seeded, Some(VolatileStatusType::Seeded));
        }
        _ => panic!("Seeded test failed"),
    }
    assert_ne!(seeded, bound);
    assert_ne!(seeded, confusion);
    assert_ne!(seeded, flinch);
    assert_ne!(seeded, rampage);
    assert_ne!(seeded, charging);
    assert_ne!(seeded, recharging);
    assert_ne!(seeded, Some(VolatileStatusType::Bound(1)));
    assert_ne!(seeded, Some(VolatileStatusType::Confusion(1)));
    assert_ne!(seeded, Some(VolatileStatusType::Flinch));
    assert_ne!(seeded, Some(VolatileStatusType::Rampage(3)));
    assert_ne!(seeded, Some(VolatileStatusType::Charging(1)));
    assert_ne!(seeded, Some(VolatileStatusType::Recharging(1)));

    // RAMPAGE TEST
    match rampage {
        Some(VolatileStatusType::Rampage(rampage_count)) => {
            assert_eq!(rampage, Some(VolatileStatusType::Rampage(1)));
            assert_eq!(rampage_count, 1);
            rampage = Some(VolatileStatusType::Rampage(2));
            assert_eq!(rampage, Some(VolatileStatusType::Rampage(2)));
        }
        _ => panic!("Rampage test failed"),
    }
    assert_ne!(rampage, bound);
    assert_ne!(rampage, flinch);
    assert_ne!(rampage, seeded);
    assert_ne!(rampage, confusion);
    assert_ne!(rampage, charging);
    assert_ne!(rampage, recharging);
    assert_ne!(rampage, Some(VolatileStatusType::Bound(1)));
    assert_ne!(rampage, Some(VolatileStatusType::Flinch));
    assert_ne!(rampage, Some(VolatileStatusType::Seeded));
    assert_ne!(rampage, Some(VolatileStatusType::Confusion(3)));
    assert_ne!(rampage, Some(VolatileStatusType::Charging(1)));
    assert_ne!(rampage, Some(VolatileStatusType::Recharging(1)));

    // CHARGING TEST
    match charging {
        Some(VolatileStatusType::Charging(charge_count)) => {
            assert_eq!(charging, Some(VolatileStatusType::Charging(1)));
            assert_eq!(charge_count, 1);
            charging = Some(VolatileStatusType::Charging(2));
            assert_eq!(charging, Some(VolatileStatusType::Charging(2)));
        }
        _ => panic!("Charging test failed"),
    }
    assert_ne!(charging, bound);
    assert_ne!(charging, flinch);
    assert_ne!(charging, seeded);
    assert_ne!(charging, confusion);
    assert_ne!(charging, rampage);
    assert_ne!(charging, recharging);
    assert_ne!(charging, Some(VolatileStatusType::Bound(1)));
    assert_ne!(charging, Some(VolatileStatusType::Flinch));
    assert_ne!(charging, Some(VolatileStatusType::Seeded));
    assert_ne!(charging, Some(VolatileStatusType::Confusion(3)));
    assert_ne!(charging, Some(VolatileStatusType::Rampage(1)));
    assert_ne!(charging, Some(VolatileStatusType::Recharging(1)));

    // RECHARGING TEST
    match recharging {
        Some(VolatileStatusType::Recharging(recharge_count)) => {
            assert_eq!(recharging, Some(VolatileStatusType::Recharging(1)));
            assert_eq!(recharge_count, 1);
            recharging = Some(VolatileStatusType::Recharging(2));
            assert_eq!(recharging, Some(VolatileStatusType::Recharging(2)));
        }
        _ => panic!("Recharging test failed"),
    }
    assert_ne!(recharging, bound);
    assert_ne!(recharging, flinch);
    assert_ne!(recharging, seeded);
    assert_ne!(recharging, confusion);
    assert_ne!(recharging, rampage);
    assert_ne!(recharging, charging);
    assert_ne!(recharging, Some(VolatileStatusType::Bound(1)));
    assert_ne!(recharging, Some(VolatileStatusType::Flinch));
    assert_ne!(recharging, Some(VolatileStatusType::Seeded));
    assert_ne!(recharging, Some(VolatileStatusType::Confusion(3)));
    assert_ne!(recharging, Some(VolatileStatusType::Rampage(1)));
    assert_ne!(recharging, Some(VolatileStatusType::Charging(1)));
}
