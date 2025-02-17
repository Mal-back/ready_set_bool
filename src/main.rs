use ready_set_bool::ready_set_bool::truth_table::TruthTable;

fn main() {
    let tree = TruthTable::build_from_str("AB&C|").unwrap();
    tree.compute_truth_table();
}
