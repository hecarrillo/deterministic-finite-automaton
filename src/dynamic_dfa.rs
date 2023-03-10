// import the serde_json crate
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn validate(s: &str) {
    // create a hashmap of transitions
    let mut dfa: HashMap<String, HashMap<String, String>> = HashMap::new();
    // Read input_dfa.json
    let file = File::open("src/input_dfa.json").unwrap();
    let reader = BufReader::new(file);
    // Parse the json file
    let json: Value = serde_json::from_reader(reader).unwrap();
    // log json
    println!("{:?}", json);

    let states = json["states"].as_array().unwrap();
    // put the states in a vector
    let mut states: Vec<String> = Vec::new();
    for state in states {
        states.push(state.as_str().unwrap().to_string());
    }

    // log states
    println!("{:?}", states);

    // For each state, get the transitions in the "transitions" object in json and store the transitions in the hashmap

    for state in states {
        let state_transitions = state["transitions"].as_array().unwrap();
        for transition in state_transitions {
            let to = transition["to"].as_str().unwrap();
            let input = transition["input"].as_str().unwrap();
            let transition = Transition {
                to: to.to_string(),
                input: input.to_string(),
            };
            transitions.insert(state_name.to_string(), transition);
        }
    }
}
