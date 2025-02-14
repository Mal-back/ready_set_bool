use ready_set_bool::ready_set_bool::truth_table::TruthTable;

fn main() {
    let tree = TruthTable::build_from_str("11&");
    println!("{tree:?}");
}
