mod data_types;
mod introduction;

use data_types::integers;
use introduction::{constants, shadowing, variables};

fn main() {
    variables::main();
    shadowing::main();
    constants::main();

    integers::main();
}
