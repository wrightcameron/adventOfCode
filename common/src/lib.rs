use serde::Deserialize;
use serde_json;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Solution {
    id: String,
    first: i64,
    second: i64,
}

pub fn get_solution(day: String, problem: i8) -> i64 {
    let json_string = fs::read_to_string("data/solutions.json").expect("JSON file doesn't exist!");
    let json: Vec<Solution> =
        serde_json::from_str(&json_string).expect("JSON was not well-formatted");
    let solution = json.iter().find(|x| x.id == day).unwrap();
    return if problem == 1 {
        solution.first
    } else {
        solution.second
    };
}
