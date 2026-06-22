use std::collections::HashMap;
use colored::*;


mod words;

pub fn search(word: &str, vec: &Vec<&str>) -> HashMap<String, Vec<usize>> {
    let mi: HashMap<String, Vec<usize>> = words::hash(&vec);
    let mut jj:HashMap<String, Vec<usize>> = HashMap::new();
    for (key , value) in mi{
        if word == key{
            jj.insert(key, value);
        }
    }
   jj
}


pub fn sentence(string: &str , word:&str) -> String {
    let mut back = String::new();
   let sentence = string.split_whitespace().collect::<Vec<&str>>();
    for o in sentence {
        if o == word {
            back = back + &o.red().bold().to_string() + " ";
        }
        else {
            back = back + &o + " ";
        }
    }
    back
}