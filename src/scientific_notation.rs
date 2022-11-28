// State machine that checks if a string is a valid scientific number notation
// (e.g. 1.23e-4, 1.23e+4, 1.23e4, 1.23E-4, 1.23E+4, 1.23E4)


// State machine states
#[derive(PartialEq)]
#[derive(Debug)]
enum State {
    Start,
    Sign,
    Integer,
    Decimal,
    Exponent,
    ExponentSign,
    ExponentInteger,
    End,
}

// State machine transitions

// Transition from Start state
fn start(c: char) -> State {
    match c {
        '+' | '-' => State::Sign,
        '0'..='9' => State::Integer,
        '.' => State::Decimal,
        _ => State::End,
    }
}

// Transition from Sign state
fn sign(c: char) -> State {
    match c {
        '0'..='9' => State::Integer,
        '.' => State::Decimal,
        _ => State::End,
    }
}

// Transition from Integer state
fn integer(c: char) -> State {
    match c {
        '0'..='9' => State::Integer,
        '.' => State::Decimal,
        'e' | 'E' => State::Exponent,
        _ => State::End,
    }
}

// Transition from Decimal state
fn decimal(c: char) -> State {
    match c {
        '0'..='9' => State::Integer,
        'e' | 'E' => State::Exponent,
        _ => State::End,
    }
}

// Transition from Exponent state
fn exponent(c: char) -> State {
    match c {
        '+' | '-' => State::ExponentSign,
        '0'..='9' => State::ExponentInteger,
        _ => State::End,
    }
}

// Transition from ExponentSign state
fn exponent_sign(c: char) -> State {
    match c {
        '0'..='9' => State::ExponentInteger,
        _ => State::End,
    }
}

// Transition from ExponentInteger state
fn exponent_integer(c: char) -> State {
    match c {
        '0'..='9' => State::ExponentInteger,
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
        State::Sign => sign(c),
        State::Integer => integer(c),
        State::Decimal => decimal(c),
        State::Exponent => exponent(c),
        State::ExponentSign => exponent_sign(c),
        State::ExponentInteger => exponent_integer(c),
        State::End => end(c),
    }
}

// State machine
pub fn is_valid_scientific_notation(s: &str) -> bool {
    let mut state = State::Start;
    // print curr state
    print!("State: {:?}", state);
    for c in s.chars() {
        state = transition(state, c);
        // print curr state
        print!("-> State: {:?}", state);
    }
    println!();
    state == State::Integer || state == State::Decimal || state == State::ExponentInteger
}