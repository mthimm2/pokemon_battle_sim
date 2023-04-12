mod typing;
use std::collections::HashMap;
use typing::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StatusTypes {
    Toxic,
    Burn,
    Poison,
}

fn main() {
    let _type_chart: HashMap<Types, HashMap<Types, f64>> = construct_type_chart();
    let useless: Option<StatusTypes> = Some(StatusTypes::Burn);
    match useless {
        Some(StatusTypes::Toxic) => println!("I'm afficted with toxic."),
        Some(StatusTypes::Burn) => println!("I'm afficted with burn."),
        Some(StatusTypes::Poison) => println!("I'm afficted poison."),
        None => println!("All good."),
    }
    println!("Hello, world!");
}
