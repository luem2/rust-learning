mod control_flow;
mod data_types;
mod functions;
mod introduction;

use control_flow::{and_or_operators, basics, loops, match_construct};
use data_types::{arrays, booleans, characters, enums, floating, integers, structs, tuples};
use functions::{parameters_arguments, return_values, statements_expressions};
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

    // functions
    parameters_arguments::main();
    statements_expressions::main();
    return_values::main();

    // control flow
    basics::main();
    and_or_operators::main();
    match_construct::main();
    loops::main();
}
