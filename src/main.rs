mod data_types;
mod introduction;

use data_types::{booleans, floating, integers};
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
}
