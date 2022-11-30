// State machine that checks if a string is a valid combination without once in a row and with pair # of zeros
// (e.g. 10000, 10101, 001001010, 00, 010)


// State machine states
#[derive(PartialEq)]
#[derive(Debug)]
enum State {
    Start,
    UnpairZero,
    One,
    PairZero,
    PairZeroOne,
    UnpairZeroOne,
    End,
}

// State machine transitions

// Transition from Start state
fn start(c: char) -> State {
    match c {
        '0' => State::UnpairZero,
        '1' => State::One,
        _ => State::End,
    }
}

// Transition from UnpairZero state
fn unpair_zero(c: char) -> State {
    match c {
        '0' => State::PairZero,
        '1' => State::UnpairZeroOne,
        _ => State::End,
    }
}

// Transition from One state
fn one(c: char) -> State {
    match c {
        '0' => State::UnpairZero,
        '1' => State::End,
        _ => State::End,
    }
}

// Transition from PairZero state
fn pair_zero(c: char) -> State {
    match c {
        '0' => State::UnpairZero,
        '1' => State::PairZeroOne,
        _ => State::End,
    }
}

// Transition from PairZeroOne state
fn pair_zero_one(c:char) -> State{
    match c {
        '0' => State::UnpairZero,
        '1' => State::End,
        _ => State::End,
    }
}

// Transition from UnpairZeroOne state
fn unpair_zero_one(c: char) -> State {
    match c {
        '0' => State::PairZero,
        '1' => State::End,
        _ => State::End,
    }
}

// Transition from End state
fn end(_: char) -> State {
    State::End
}

// State machine transition function
fn transition(state: State, c: char) -> State {
    match state {
        State::Start => start(c),
        State::UnpairZero => unpair_zero(c),
        State::One => one(c),
        State::PairZero => pair_zero(c),
        State::PairZeroOne => pair_zero_one(c),
        State::UnpairZeroOne => unpair_zero_one(c),
        State::End => end(c),
    }
}

// State machine
pub fn is_valid_pair_zeros(s: &str) -> bool {
    let mut state = State::Start;
    // print curr state
    print!("State: {:?}", state);
    for c in s.chars() {
        state = transition(state, c);
        // print curr state
        println!("-> State: {:?}", state);
    }
    println!();
    state == State::PairZero || state == State::PairZeroOne
}