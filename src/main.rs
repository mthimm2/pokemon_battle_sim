mod pokemon;
mod statuses;
mod typing;
use statuses::{NonVolatileStatusType, VolatileStatusType};
use std::collections::HashMap;
use typing::*;

fn main() {
    let _type_chart: HashMap<Types, HashMap<Types, f64>> = construct_type_chart();
    // println!("Hello, world!");
}
