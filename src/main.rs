pub mod control_flow;
pub mod data_types;
pub mod functions;
pub mod introduction;
pub mod memory_management;

use introduction::{constants, shadowing, variables};

fn main() {
    // introduction
    variables::main();
    shadowing::main();
    constants::main();
}
