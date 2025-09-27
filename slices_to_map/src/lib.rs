use std::collections::HashMap;

pub fn slices_to_map<'a, T: std::cmp::Eq + std::hash::Hash, U>(
    keys: &'a [T],
    values: &'a [U]
) -> HashMap<&'a T, &'a U> {
    let mut map = HashMap::new();
    for i in 0..keys.len().min(values.len()) {
        map.insert(&keys[i], &values[i]);
    }
    map
}
