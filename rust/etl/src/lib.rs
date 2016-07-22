use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    input.iter()
         .fold(BTreeMap::new(), |mut acc, (key, val)| {
             val.into_iter().fold(&mut acc, |acci, x| {acci.insert(x.to_string().to_lowercase(), *key); acci});
             acc
         })
}