use std::collections::hash_map;

pub struct truth_table<'a> {
    value_map: hash_map<char, u8>,
    formula: &'a str,
}
