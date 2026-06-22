use std::collections::HashMap;

pub fn hash(vec: &[&str]) -> HashMap<String, Vec<usize>> {
    let mut map: HashMap<String, Vec<usize>> = HashMap::new();
    for (line_num, line) in vec.iter().enumerate() {
        for word in line.split_whitespace() {
            map.entry(word.to_string()).or_insert_with(Vec::new).push(line_num + 1);
        }
    }
    return map;
}