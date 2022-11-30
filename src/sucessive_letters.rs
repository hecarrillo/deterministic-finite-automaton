// State machine that checks if a string is a valid combination with at least least two succesive same letters
// (e.g. aacdad, abcdabccd, aa, abcdd, adbbac)


// State machine states
#[derive(PartialEq)]
#[derive(Debug)]
enum State {
    Start,
    OneA,
    OneB,
    OneC,
    OneD,
    Accept,
    End,
}

// State machine transitions

// Transition from Start state
fn start(c: char) -> State {
    match c {
        'a' => State::OneA,
        'b' => State::OneB,
        'c' => State::OneC,
        'd' => State::OneD,
        _ => State::End,
    }
}

// Transition from OneA state
fn one_a(c: char) -> State {
    match c {
        'a' => State::Accept,
        'b' => State::OneB,
        'c' => State::OneC,
        'd' => State::OneD,
        _ => State::End,
    }
}

// Transition from OneB state
fn one_b(c: char) -> State {
    match c {
        'a' => State::OneA,
        'b' => State::Accept,
        'c' => State::OneC,
        'd' => State::OneD,
        _ => State::End,
    }
}

// Transition from OneC state
fn one_c(c: char) -> State {
    match c {
        'a' => State::OneA,
        'b' => State::OneB,
        'c' => State::Accept,
        'd' => State::OneD,
        _ => State::End,
    }
}

// Transition from OneD state
fn one_d(c:char) -> State{
    match c {
        'a' => State::OneA,
        'b' => State::OneB,
        'c' => State::OneC,
        'd' => State::Accept,
        _ => State::End,
    }
}

// Transition from Accept state
fn accept(c: char) -> State {
    match c {
        'a' | 'b' | 'c' | 'd' => State::Accept,
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
        State::OneA => one_a(c),
        State::OneB => one_b(c),
        State::OneC => one_c(c),
        State::OneD => one_d(c),
        State::Accept => accept(c),
        State::End => end(c),
    }
}

fn is_the_accepted_state(state: State) -> bool {
    match state {
        State::Accept => true,
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
