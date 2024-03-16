mod data_types;
mod introduction;

use data_types::{arrays, booleans, characters, enums, floating, integers, structs, tuples};
use introduction::{constants, shadowing, variables};

fn main() {
    // introduction
    variables::main();
    shadowing::main();
    constants::main();

    // data types
    integers::main();
    floating::main();
    booleans::main();
    characters::main();
    tuples::main();
    arrays::main();
    structs::main();
    enums::main();
}
