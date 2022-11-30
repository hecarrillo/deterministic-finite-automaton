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

fn is_the_accepted_state(state: State) -> bool {
    match state {
        State::PairZero | State::PairZeroOne => true,
        _ => false,
    }
}

pub fn is_valid(s: &str) -> bool {
    let mut state = State::Start;
    println!("_______STATES________\n");
    println!("{:?}", state);
    for c in s.chars() {
        if state == State::End {
            return false;
        }
        state = transition(state, c);
        println!("⬇️");
        println!("{:?}", state);
    }
    if is_the_accepted_state(state) {
        return true;
    }
    return false;
}
