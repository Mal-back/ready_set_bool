use truth_table::TruthTable;

mod error;
mod operation;
mod tree_node;
mod truth_table;

fn main() {
    println!("Hello, world!");
    let tree = TruthTable::build_from_str("AA&");
}
