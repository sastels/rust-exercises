use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut out: BTreeMap<char, i32> = BTreeMap::new();
    for (val, keys) in h {
        for key in keys {
            out.insert((*key).to_lowercase().collect::<Vec<_>>()[0], *val);
        }
    }
    out
}
