use crate::pokemon::Pokemon;
use crate::statuses::{NonVolatileStatusType, Status, VolatileStatusType};
use crate::typing::Types;
// use create::moves::Move;

fn creator_helper() -> Pokemon {
    let mut bulbasaur: Pokemon = Pokemon::new();
    bulbasaur.name = String::from("Bulbasaur");
    bulbasaur.type_1 = Types::Grass;
    bulbasaur.type_2 = None;
    bulbasaur.max_hp = 100.0;
    bulbasaur.hp = bulbasaur.max_hp.clone();
    bulbasaur.attack = 100;
    bulbasaur.defense = 100;
    bulbasaur.special_attack = 100;
    bulbasaur.special_defense = 100;
    bulbasaur.speed = 100;
    bulbasaur
}

fn reset(inst: &mut Pokemon) {
    inst.hp = 100.0;
    inst.status_conditions.non_vol = None;
    inst.status_conditions.turn_count = 0;
    inst.status_conditions.vol.clear();
}

#[test]
fn test_create_pokemon() {
    let bulbasaur: Pokemon = creator_helper();
    assert_eq!(bulbasaur.name, "Bulbasaur");
    assert_eq!(bulbasaur.type_1, Types::Grass);
    assert_eq!(bulbasaur.type_2, None);
    assert_eq!(bulbasaur.max_hp, 100.0);
    assert_eq!(bulbasaur.hp, 100.0);
    assert_eq!(bulbasaur.attack, 100);
    assert_eq!(bulbasaur.defense, 100);
    assert_eq!(bulbasaur.special_attack, 100);
    assert_eq!(bulbasaur.special_defense, 100);
    assert_eq!(bulbasaur.speed, 100);
}

#[test]
fn test_take_damage() {
    let mut bulbasaur: Pokemon = creator_helper();
    // Take damage > current health
    bulbasaur.take_attack_damage(897.0);
    assert_eq!(bulbasaur.hp, 0.0);
    bulbasaur.take_attack_damage(500.0);
    assert_eq!(bulbasaur.hp, 0.0);

    // Take damage < current health
    reset(&mut bulbasaur);
    bulbasaur.take_attack_damage(84.5);
    assert_eq!(bulbasaur.hp, 15.5);
    assert_eq!(bulbasaur.status_conditions.non_vol, None);
    assert_ne!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Fainted)
    );

    // Take damage such that health < 1.0
    reset(&mut bulbasaur);
    bulbasaur.take_attack_damage(99.5);
    assert_eq!(bulbasaur.hp, 0.0);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Fainted)
    );

    // Take damage == current health
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = None;
    bulbasaur.take_attack_damage(100.0);
    assert_eq!(bulbasaur.hp, 0.0);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Fainted)
    );
}

#[test]
fn test_poison_damage() {
    let mut bulbasaur: Pokemon = creator_helper();

    // Single-instance status damage test
    reset(&mut bulbasaur);
    bulbasaur
        .status_conditions
        .non_volatile_status_check(NonVolatileStatusType::Poison, &bulbasaur.name);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 87.5);

    // Multi-instance poison damage and faint test
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = None;
    bulbasaur
        .status_conditions
        .non_volatile_status_check(NonVolatileStatusType::Poison, &bulbasaur.name);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 87.5);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 75.0);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 62.5);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 50.0);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 37.5);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 25.0);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Poison)
    );
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 12.5);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 0.0);
}

#[test]
fn test_toxic_damage() {
    let mut bulbasaur: Pokemon = creator_helper();

    bulbasaur
        .status_conditions
        .non_volatile_status_check(NonVolatileStatusType::Toxic, &bulbasaur.name);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 93.75);
    bulbasaur.take_status_damage();
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );
    assert_eq!(bulbasaur.hp, 81.25);
    bulbasaur.take_status_damage();
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );
    assert_eq!(bulbasaur.hp, 62.5);
    bulbasaur.take_status_damage();
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );
    assert_eq!(bulbasaur.hp, 37.5);
    bulbasaur.take_status_damage();
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );
    assert_eq!(bulbasaur.hp, 6.25);

    // Faint here
    bulbasaur.take_status_damage();
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Fainted)
    );
    assert_eq!(bulbasaur.hp, 0.0);

    // 7/16 Toxic hit
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
    bulbasaur.status_conditions.turn_count = 7;
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 56.25);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );

    // 8/16 Toxic hit
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 50.0);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );

    // 9/16 Toxic hit
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 43.75);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );

    // 10/16 Toxic hit
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 37.5);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );

    // 11/16 Toxic hit
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 31.25);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );

    // 12/16 Toxic hit
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 25.0);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );

    // 13/16 Toxic hit
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 18.75);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );

    // 14/16 Toxic hit
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 12.5);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );

    // 15/16 Toxic hit
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 6.25);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Toxic)
    );

    // 16/16 Toxic hit
    bulbasaur.hp = 100.0;
    bulbasaur.status_conditions.non_vol = Some(NonVolatileStatusType::Toxic);
    bulbasaur.take_status_damage();
    assert_eq!(bulbasaur.hp, 0.0);
    assert_eq!(
        bulbasaur.status_conditions.non_vol,
        Some(NonVolatileStatusType::Fainted)
    );
}
