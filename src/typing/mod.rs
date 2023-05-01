use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Types {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

pub fn construct_type_chart() -> HashMap<Types, HashMap<Types, f64>> {
    let mut type_chart: HashMap<Types, HashMap<Types, f64>> = HashMap::new();

    // Add damage multipliers for Normal type attacks
    let mut normal_multipliers: HashMap<Types, f64> = HashMap::new();
    normal_multipliers.insert(Types::Rock, 0.5);
    normal_multipliers.insert(Types::Ghost, 0.0);
    type_chart.insert(Types::Normal, normal_multipliers);

    // Add damage multipliers for Fire type attacks
    let mut fire_multipliers: HashMap<Types, f64> = HashMap::new();
    fire_multipliers.insert(Types::Fire, 0.5);
    fire_multipliers.insert(Types::Water, 0.5);
    fire_multipliers.insert(Types::Grass, 2.0);
    fire_multipliers.insert(Types::Ice, 2.0);
    fire_multipliers.insert(Types::Bug, 2.0);
    fire_multipliers.insert(Types::Rock, 0.5);
    fire_multipliers.insert(Types::Dragon, 0.5);
    fire_multipliers.insert(Types::Steel, 2.0);
    type_chart.insert(Types::Fire, fire_multipliers);

    // Add damage multipliers for Water type attacks
    let mut water_multipliers: HashMap<Types, f64> = HashMap::new();
    water_multipliers.insert(Types::Fire, 2.0);
    water_multipliers.insert(Types::Water, 0.5);
    water_multipliers.insert(Types::Grass, 0.5);
    water_multipliers.insert(Types::Ground, 2.0);
    water_multipliers.insert(Types::Rock, 2.0);
    water_multipliers.insert(Types::Dragon, 0.5);
    type_chart.insert(Types::Water, water_multipliers);

    // Add damage multipliers for Grass type attacks
    let mut grass_multipliers: HashMap<Types, f64> = HashMap::new();
    grass_multipliers.insert(Types::Fire, 0.5);
    grass_multipliers.insert(Types::Water, 2.0);
    grass_multipliers.insert(Types::Grass, 0.5);
    grass_multipliers.insert(Types::Poison, 0.5);
    grass_multipliers.insert(Types::Ground, 2.0);
    grass_multipliers.insert(Types::Flying, 0.5);
    grass_multipliers.insert(Types::Bug, 0.5);
    grass_multipliers.insert(Types::Rock, 2.0);
    grass_multipliers.insert(Types::Dragon, 0.5);
    grass_multipliers.insert(Types::Steel, 0.5);
    type_chart.insert(Types::Grass, grass_multipliers);

    // Add damage multipliers for Electric type attacks
    let mut electric_multipliers: HashMap<Types, f64> = HashMap::new();
    electric_multipliers.insert(Types::Water, 2.0);
    electric_multipliers.insert(Types::Electric, 0.5);
    electric_multipliers.insert(Types::Grass, 0.5);
    electric_multipliers.insert(Types::Ground, 0.0);
    electric_multipliers.insert(Types::Flying, 2.0);
    electric_multipliers.insert(Types::Dragon, 0.5);
    type_chart.insert(Types::Electric, electric_multipliers);

    // Add damage multipliers for Ice type attacks
    let mut ice_multipliers: HashMap<Types, f64> = HashMap::new();
    ice_multipliers.insert(Types::Fire, 0.5);
    ice_multipliers.insert(Types::Water, 0.5);
    ice_multipliers.insert(Types::Grass, 2.0);
    ice_multipliers.insert(Types::Ice, 0.5);
    ice_multipliers.insert(Types::Ground, 2.0);
    ice_multipliers.insert(Types::Flying, 2.0);
    ice_multipliers.insert(Types::Dragon, 2.0);
    ice_multipliers.insert(Types::Steel, 0.5);
    type_chart.insert(Types::Ice, ice_multipliers);

    // Add damage multipliers for Fighting type attacks
    let mut fighting_multipliers: HashMap<Types, f64> = HashMap::new();
    fighting_multipliers.insert(Types::Normal, 2.0);
    fighting_multipliers.insert(Types::Ice, 2.0);
    fighting_multipliers.insert(Types::Poison, 0.5);
    fighting_multipliers.insert(Types::Flying, 0.5);
    fighting_multipliers.insert(Types::Psychic, 0.5);
    fighting_multipliers.insert(Types::Bug, 0.5);
    fighting_multipliers.insert(Types::Rock, 2.0);
    fighting_multipliers.insert(Types::Ghost, 0.0);
    fighting_multipliers.insert(Types::Dark, 2.0);
    fighting_multipliers.insert(Types::Steel, 2.0);
    fighting_multipliers.insert(Types::Fairy, 0.5);
    type_chart.insert(Types::Fighting, fighting_multipliers);

    // Add damage multipliers for Poison type attacks
    let mut poison_multipliers: HashMap<Types, f64> = HashMap::new();
    poison_multipliers.insert(Types::Grass, 2.0);
    poison_multipliers.insert(Types::Poison, 0.5);
    poison_multipliers.insert(Types::Ground, 0.5);
    poison_multipliers.insert(Types::Rock, 0.5);
    poison_multipliers.insert(Types::Ghost, 0.5);
    poison_multipliers.insert(Types::Steel, 0.0);
    poison_multipliers.insert(Types::Fairy, 2.0);
    type_chart.insert(Types::Poison, poison_multipliers);

    // Add damage multipliers for Ground type attacks
    let mut ground_multipliers: HashMap<Types, f64> = HashMap::new();
    ground_multipliers.insert(Types::Fire, 2.0);
    ground_multipliers.insert(Types::Electric, 2.0);
    ground_multipliers.insert(Types::Grass, 0.5);
    ground_multipliers.insert(Types::Poison, 2.0);
    ground_multipliers.insert(Types::Flying, 0.0);
    ground_multipliers.insert(Types::Bug, 0.5);
    ground_multipliers.insert(Types::Rock, 2.0);
    ground_multipliers.insert(Types::Steel, 2.0);
    type_chart.insert(Types::Ground, ground_multipliers);

    // Add damage multipliers for Flying type attacks
    let mut flying_multipliers: HashMap<Types, f64> = HashMap::new();
    flying_multipliers.insert(Types::Electric, 0.5);
    flying_multipliers.insert(Types::Grass, 2.0);
    flying_multipliers.insert(Types::Fighting, 2.0);
    flying_multipliers.insert(Types::Bug, 2.0);
    flying_multipliers.insert(Types::Rock, 0.5);
    flying_multipliers.insert(Types::Steel, 0.5);
    type_chart.insert(Types::Flying, flying_multipliers);

    // Add damage multipliers for Psychic type attacks
    let mut psychic_multipliers: HashMap<Types, f64> = HashMap::new();
    psychic_multipliers.insert(Types::Fighting, 2.0);
    psychic_multipliers.insert(Types::Poison, 2.0);
    psychic_multipliers.insert(Types::Psychic, 0.5);
    psychic_multipliers.insert(Types::Dark, 0.0);
    psychic_multipliers.insert(Types::Steel, 0.5);
    type_chart.insert(Types::Psychic, psychic_multipliers);

    // Add damage multipliers for Bug type attacks
    let mut bug_multipliers: HashMap<Types, f64> = HashMap::new();
    bug_multipliers.insert(Types::Fire, 0.5);
    bug_multipliers.insert(Types::Grass, 2.0);
    bug_multipliers.insert(Types::Fighting, 0.5);
    bug_multipliers.insert(Types::Poison, 0.5);
    bug_multipliers.insert(Types::Flying, 0.5);
    bug_multipliers.insert(Types::Psychic, 2.0);
    bug_multipliers.insert(Types::Ghost, 0.5);
    bug_multipliers.insert(Types::Dark, 2.0);
    bug_multipliers.insert(Types::Steel, 0.5);
    bug_multipliers.insert(Types::Fairy, 0.5);
    type_chart.insert(Types::Bug, bug_multipliers);

    // Add damage multipliers for Rock type attacks
    let mut rock_multipliers: HashMap<Types, f64> = HashMap::new();
    rock_multipliers.insert(Types::Fire, 2.0);
    rock_multipliers.insert(Types::Ice, 2.0);
    rock_multipliers.insert(Types::Fighting, 0.5);
    rock_multipliers.insert(Types::Ground, 0.5);
    rock_multipliers.insert(Types::Flying, 2.0);
    rock_multipliers.insert(Types::Bug, 2.0);
    rock_multipliers.insert(Types::Steel, 0.5);
    type_chart.insert(Types::Rock, rock_multipliers);

    // Add damage multipliers for Ghost type attacks
    let mut ghost_multipliers: HashMap<Types, f64> = HashMap::new();
    ghost_multipliers.insert(Types::Normal, 0.0);
    ghost_multipliers.insert(Types::Psychic, 2.0);
    ghost_multipliers.insert(Types::Ghost, 2.0);
    ghost_multipliers.insert(Types::Dark, 0.5);
    type_chart.insert(Types::Ghost, ghost_multipliers);

    // Add damage multipliers for Dragon type attacks
    let mut dragon_multipliers: HashMap<Types, f64> = HashMap::new();
    dragon_multipliers.insert(Types::Dragon, 2.0);
    dragon_multipliers.insert(Types::Steel, 0.5);
    dragon_multipliers.insert(Types::Fairy, 0.0);
    type_chart.insert(Types::Dragon, dragon_multipliers);

    // Add damage multipliers for Dark type attacks
    let mut dark_multipliers: HashMap<Types, f64> = HashMap::new();
    dark_multipliers.insert(Types::Fighting, 0.5);
    dark_multipliers.insert(Types::Psychic, 2.0);
    dark_multipliers.insert(Types::Ghost, 2.0);
    dark_multipliers.insert(Types::Dark, 0.5);
    dark_multipliers.insert(Types::Fairy, 0.5);
    type_chart.insert(Types::Dark, dark_multipliers);

    // Add damage multipliers for Steel type attacks
    let mut steel_multipliers: HashMap<Types, f64> = HashMap::new();
    steel_multipliers.insert(Types::Fire, 0.5);
    steel_multipliers.insert(Types::Water, 0.5);
    steel_multipliers.insert(Types::Electric, 0.5);
    steel_multipliers.insert(Types::Ice, 2.0);
    steel_multipliers.insert(Types::Rock, 2.0);
    steel_multipliers.insert(Types::Steel, 0.5);
    steel_multipliers.insert(Types::Fairy, 2.0);
    type_chart.insert(Types::Steel, steel_multipliers);

    // Add damage multipliers for Fairy type attacks
    let mut fairy_multipliers: HashMap<Types, f64> = HashMap::new();
    fairy_multipliers.insert(Types::Fighting, 2.0);
    fairy_multipliers.insert(Types::Poison, 0.5);
    fairy_multipliers.insert(Types::Bug, 2.0);
    fairy_multipliers.insert(Types::Steel, 0.5);
    type_chart.insert(Types::Fairy, fairy_multipliers);

    // Print the type chart
    // for (type1, multipliers) in &type_chart {
    //     println!("{:?}:", type1);
    //     for (type2, multiplier) in multipliers {
    //         println!("  {:?}: {}", type2, multiplier);
    //     }
    // }
    type_chart
}
