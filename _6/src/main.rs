use ::std::collections::HashMap;

mod input;

fn backmapify(orbits_str: &str) -> HashMap<&str, &str> {
    orbits_str
        .split("\n")
        .map(|s| s.split(")"))
        .map(|splt| splt.collect())
        .map(|splt: Vec<&str>| (splt[1], splt[0]))
        .collect()
}

fn get_com_path<'a>(orbit_map: &HashMap<&'a str, &'a str>, key: &'a str) -> Vec<&'a str> {
    let mut v = Vec::new();
    let mut key = key;
    while orbit_map.contains_key(key) {
        key = orbit_map[key];
        v.push(key); // push after getting
    }
    v
}

fn get_checksum(orbit_map: &HashMap<&str, &str>) -> i32 {
    orbit_map
        .keys()
        .map(|k| get_com_path(orbit_map, k))
        .map(|v| v.len() as i32)
        .sum()
}

fn get_shortest_path(orbit_map: &HashMap<&str, &str>) -> i32 {
    let p_you = get_com_path(&orbit_map, "YOU");
    let p_san = get_com_path(&orbit_map, "SAN");
    let p_you_len = p_you.len();
    let p_san_len = p_san.len();
    for i in 1..p_you_len {
        if p_you[p_you_len-i] != p_san[p_san_len-i] {
            return (p_you_len-i + p_san_len-i + 2) as i32;
        }
    }
    0
}

fn main() {
    let backwards_tree = backmapify(input::PROGRAM_INPUT);
    println!("Task 1 / checksum: {}", get_checksum(&backwards_tree));
    println!("Task 2 / shortest path: {}", get_shortest_path(&backwards_tree));
}
