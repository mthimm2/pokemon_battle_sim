#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[cfg(test)]
mod tests {
    mod pokemon_tests;
    mod status_tests;
}

mod pokemon;
mod statuses;
mod typing;

use pokemon::Pokemon;
use statuses::{NonVolatileStatusType, VolatileStatusType};
use std::collections::HashMap;
use typing::*;

fn main() {
    // Need type chart, pokemon, and moves to start things off.
    let _type_chart: HashMap<Types, HashMap<Types, f64>> = construct_type_chart();
    // let _pokemon: HashMap<String, Pokemon> = construct_pokemon();
    // let _moves: HashMap<String, Move> = construct_moves();
}
