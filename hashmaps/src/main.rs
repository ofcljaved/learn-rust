use std::collections::HashMap;

fn main() {
    let tup_vec = vec![
        (String::from("father"), 32),
        (String::from("father"), 38),
        (String::from("mother"), 18),
    ];
    let map = generate_map(tup_vec);
    println!("{:?}", map);
}

fn generate_map(pairs: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
    let mut new_map = HashMap::new();
    for (key, val) in pairs {
        let entry = new_map.entry(key).or_insert(vec![]);
        entry.push(val)
    }
    new_map
}
