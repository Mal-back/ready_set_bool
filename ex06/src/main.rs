use truth_table::TruthTable;

mod error;
mod operation;
mod tree_node;
mod truth_table;

fn main() {
    let mut tree = TruthTable::build_from_str("AB&C&D&").unwrap();
    tree.turn_into_conjuctive_normal_form();
    tree.print_rpn_from_tree();
}
