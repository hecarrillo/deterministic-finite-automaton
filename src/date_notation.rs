// State machine that checks if a string is a valid date
// (e.g. 2020-01-01, 2020-1-1, 2020/01/01, 2020/1/1, 2020.01.01, 2020.1.1)

// State machine states
#[derive(PartialEq)]
#[derive(Debug)]
enum State {
    Start,
    Year,
    YearSeparator,
    Month,
    MonthSeparator,
    Day,
    End,
}

// State machine transitions

// Transition from Start state
fn start(c: char) -> State {
    match c {
        '0'..='9' => State::Year,
        _ => State::End,
    }
}

// Transition from Year state
fn year(c: char) -> State {
    match c {
        '0'..='9' => State::Year,
        '-' | '/' | '.' => State::YearSeparator,
        _ => State::End,
    }
}

// Transition from YearSeparator state
fn year_separator(c: char) -> State {
    match c {
        '0'..='9' => State::Month,
        _ => State::End,
    }
}

// Transition from Month state
fn month(c: char) -> State {
    match c {
        '0'..='9' => State::Month,
        '-' | '/' | '.' => State::MonthSeparator,
        _ => State::End,
    }
}

// Transition from MonthSeparator state
fn month_separator(c: char) -> State {
    match c {
        '0'..='9' => State::Day,
        _ => State::End,
    }
}

// Transition from Day state
fn day(c: char) -> State {
    match c {
        '0'..='9' => State::Day,
        _ => State::End,
    }
}

// Transition function
fn transition(state: State, c: char) -> State {
    match state {
        State::Start => start(c),
        State::Year => year(c),
        State::YearSeparator => year_separator(c),
        State::Month => month(c),
        State::MonthSeparator => month_separator(c),
        State::Day => day(c),
        State::End => State::End,
    }
}

pub fn is_valid_date_notation(s: &str) -> bool {
    let mut state = State::Start;
    for c in s.chars() {
        state = transition(state, c);
    }
    state == State::Day
}
