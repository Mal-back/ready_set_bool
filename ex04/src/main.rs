use truth_table::TruthTable;

mod error;
mod operation;
mod tree_node;
mod truth_table;
fn main() {
    let tree = TruthTable::build_from_str("AB|C&").unwrap();
    tree.compute_truth_table();
}
