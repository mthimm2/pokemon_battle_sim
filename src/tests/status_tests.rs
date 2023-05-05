use crate::statuses::{NonVolatileStatusType, Status, VolatileStatusType};

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
    let res: (f64, f64) = status_tester.damage(100.0, 100, 100);
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
    let res: (f64, f64) = status_tester.damage(100.0, 100, 100);
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
    let res: (f64, f64) = status_tester.damage(100.0, 100, 100);
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
    let res: (f64, f64) = status_tester.damage(100.0, 100, 100);
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
    let res: (f64, f64) = status_tester.damage(100.0, 100, 100);
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
    let res: (f64, f64) = status_tester.damage(100.0, 100, 100);
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
    status_tester.damage(100.0, 100, 100);
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
// TODO: Write tests for volatile status conditions
fn test_volatile_status() {
    // All volatile status types
    let mut test_status = Status {
        non_vol: None,
        vol: Vec::new(),
        turn_count: 0,
    };
    let mut damage_result: (f64, f64) = (0.0, 0.0);

    // BOUND TEST
    test_status.volatile_status_check(VolatileStatusType::Bound);
    assert_eq!(test_status.turn_count, 0);
    assert_eq!(test_status.vol[0].as_ref().unwrap().1, 1);
    damage_result = test_status.damage(100.0, 100, 100);
    assert!(test_status.vol.len() == 1);
    assert_eq!(damage_result.0, 0.0);
    assert_eq!(damage_result.1, 6.25);
    match &test_status.vol[0] {
        Some(volatile_tuple) => {
            assert_eq!(volatile_tuple.0, VolatileStatusType::Bound);
            assert_eq!(volatile_tuple.1, 2);
        }
        None => panic!("Bind test failed."),
    }

    // SEEDED TEST
    test_status.volatile_status_check(VolatileStatusType::Seeded);
    assert_eq!(test_status.turn_count, 0);
    assert_eq!(test_status.vol[1].as_ref().unwrap().1, 1);
    damage_result = test_status.damage(100.0, 100, 100);
    assert!(test_status.vol.len() == 2);
    assert_eq!(damage_result.0, 0.0);
    assert_eq!(damage_result.1, 18.75);
    match &test_status.vol[1] {
        Some(volatile_tuple) => {
            assert_eq!(volatile_tuple.0, VolatileStatusType::Seeded);
            assert_eq!(volatile_tuple.1, 1);
        }
        None => panic!("Seeded test failed."),
    }

    // FLINCH TEST
    test_status.volatile_status_check(VolatileStatusType::Flinch);
    assert_eq!(test_status.turn_count, 0);
    assert_eq!(test_status.vol[2].as_ref().unwrap().1, 1);
    damage_result = test_status.damage(100.0, 100, 100);
    assert!(test_status.vol.len() == 3);
    assert_eq!(damage_result.0, 0.0);
    assert_eq!(damage_result.1, 18.75);
    match &test_status.vol[2] {
        Some(volatile_tuple) => {
            assert_eq!(volatile_tuple.0, VolatileStatusType::Flinch);
            assert_eq!(volatile_tuple.1, 2);
        }
        None => panic!("Flinch test failed."),
    }

    // RAMPAGE TEST
    test_status.volatile_status_check(VolatileStatusType::Rampage);
    assert_eq!(test_status.turn_count, 0);
    assert_eq!(test_status.vol[3].as_ref().unwrap().1, 1);
    damage_result = test_status.damage(100.0, 100, 100);
    assert!(test_status.vol.len() == 4);
    assert_eq!(damage_result.0, 0.0);
    assert_eq!(damage_result.1, 18.75);
    match &test_status.vol[3] {
        Some(volatile_tuple) => {
            assert_eq!(volatile_tuple.0, VolatileStatusType::Rampage);
            assert_eq!(volatile_tuple.1, 2);
        }
        None => panic!("Rampage test failed."),
    }

    // CHARGING TEST
    test_status.volatile_status_check(VolatileStatusType::Charging);
    assert_eq!(test_status.turn_count, 0);
    assert_eq!(test_status.vol[4].as_ref().unwrap().1, 1);
    damage_result = test_status.damage(100.0, 100, 100);
    assert!(test_status.vol.len() == 5);
    assert_eq!(damage_result.0, 0.0);
    assert_eq!(damage_result.1, 18.75);
    match &test_status.vol[4] {
        Some(volatile_tuple) => {
            assert_eq!(volatile_tuple.0, VolatileStatusType::Charging);
            assert_eq!(volatile_tuple.1, 2);
        }
        None => panic!("Charging test failed."),
    }

    // RECHARGING TEST
    test_status.volatile_status_check(VolatileStatusType::Recharging);
    assert_eq!(test_status.turn_count, 0);
    assert_eq!(test_status.vol[5].as_ref().unwrap().1, 1);
    damage_result = test_status.damage(100.0, 100, 100);
    assert!(test_status.vol.len() == 6);
    assert_eq!(damage_result.0, 0.0);
    assert_eq!(damage_result.1, 18.75);
    match &test_status.vol[5] {
        Some(volatile_tuple) => {
            assert_eq!(volatile_tuple.0, VolatileStatusType::Recharging);
            assert_eq!(volatile_tuple.1, 2);
        }
        None => panic!("Recharging test failed."),
    }

    // CONFUSION TEST
    test_status.volatile_status_check(VolatileStatusType::Confusion);
    assert_eq!(test_status.turn_count, 0);
    assert_eq!(test_status.vol[6].as_ref().unwrap().1, 1);
    damage_result = test_status.damage(100.0, 100, 100);
    assert!(test_status.vol.len() == 7);
    assert_eq!(damage_result.0, 0.0);
    assert_eq!(damage_result.1, 51.75); // Self attack does 33 damage with 100 attack and defense. Random factor does not significantly influence this.
    match &test_status.vol[6] {
        Some(volatile_tuple) => {
            assert_eq!(volatile_tuple.0, VolatileStatusType::Confusion);
            assert_eq!(volatile_tuple.1, 2);
        }
        None => panic!("Confusion test failed."),
    }
}
