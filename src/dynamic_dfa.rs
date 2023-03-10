use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

fn get_states(json: &Value) -> Vec<String> {
    let input_states: &Vec<Value> = json["states"].as_array().unwrap();

    let mut states: Vec<String> = Vec::new();

    for state in input_states {
        states.push(state.as_str().unwrap().to_string());
    }

    states
}

fn get_tokens(json: &Value) -> Vec<String> {
    let input_tokens: &Vec<Value> = json["tokens"].as_array().unwrap();

    let mut tokens: Vec<String> = Vec::new();

    for token in input_tokens {
        tokens.push(token.as_str().unwrap().to_string());
    }

    tokens
}

fn get_transitions(
    json: &Value,
    states: &Vec<String>,
    tokens: &Vec<String>,
) -> HashMap<String, HashMap<String, String>> {
    let mut dfa: HashMap<String, HashMap<String, String>> = HashMap::new();

    for state in states {
        let mut transition_for_state: HashMap<String, String> = HashMap::new();

        for token in tokens {
            let transition = json["transitions"][state.as_str()][token.as_str()]
                .as_str()
                .unwrap()
                .to_string();
            transition_for_state.insert(token.to_string(), transition);
        }

        dfa.insert(state.to_string(), transition_for_state);
    }

    dfa
}

fn get_json() -> Value {
    // Read input_dfa.json
    let file = File::open("src/input_dfa.json").unwrap();
    let reader = BufReader::new(file);
    // Parse the json file
    let json: Value = serde_json::from_reader(reader).unwrap();

    json
}

fn get_accepted_states(json: &Value) -> HashSet<String> {
    let input_accepted_states: &Vec<Value> = json["accepted_states"].as_array().unwrap();

    let mut accepted_states: HashSet<String> = HashSet::new();

    for state in input_accepted_states {
        accepted_states.insert(state.as_str().unwrap().to_string());
    }

    accepted_states
}

fn get_input_token_array(s: &str, tokens: &Vec<String>) -> Result<Vec<String>, ()> {
    let mut input_token_array: Vec<String> = Vec::new();
    let mut current_token = String::new();

    for char in s.chars() {
        current_token.push(char);
        if tokens.contains(&current_token) {
            input_token_array.push(current_token);
            current_token = String::new();
        } else {
            current_token.push(char);
        }
    }

    if current_token == "" {
        Ok(input_token_array)
    } else {
        Err(())
    }
}

fn validate_input(
    input_token_array: &Vec<String>,
    dfa: &HashMap<String, HashMap<String, String>>,
    initial_state: &String,
    accepted_states: &HashSet<String>,
) -> Result<(), ()> {

    let mut current_state = initial_state.to_string();
    println!("initial state: {}", current_state);

    for token in input_token_array {
        println!("⬇️");
        println!("{}", current_state);
        if dfa[&current_state][token] == "None" {
            return Err(());
        } else {
            current_state = dfa[&current_state][token].to_string();
        }
    }

    if accepted_states.contains(&current_state) {
        Ok(())
    } else {
        Err(())
    }
}

pub fn is_valid(s: &str) -> Result<(), ()> {
    let json = get_json();
    let states = get_states(&json);
    let tokens = get_tokens(&json);
    let accepted_states: HashSet<String> = get_accepted_states(&json);
    let dfa = get_transitions(&json, &states, &tokens);
    let initial_state = json["initial_state"].as_str().unwrap().to_string();
    let input_token_array: Vec<String> = get_input_token_array(s, &tokens)?;
    validate_input(&input_token_array, &dfa, &initial_state, &accepted_states)?;
    return Ok(());
}
