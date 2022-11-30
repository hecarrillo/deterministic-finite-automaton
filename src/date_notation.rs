// State machine that checks if a string is a valid date
// (e.g. 2020-01-01, 2020-1-1, 2020/01/01, 2020/1/1, 2020.01.01, 2020.1.1)

// State machine states
#[derive(PartialEq)]
#[derive(Debug)]
enum State {
    Start,
    Year,
    JanToSep,
    OctToDecOrJan,
    MonthEnd,
    MonthSeparator,
    FirstDay,
    OnlyFirstTwoDaysOrEnd,
    DayEnd,
    DayOneToNine,
    DaySeparator,
    End,
}

// State machine transitions

fn start(c: char) -> State {
    match c {
        '0' => State::DayOneToNine,
        '1' | '2' => State::FirstDay,
        '3' => State::OnlyFirstTwoDaysOrEnd,
        '4'..='9' => State::DayEnd,
        _ => State::End,
    }
}

fn year(c: char) -> State {
    match c {
        '0'..='9' => State::Year,
        _ => State::End,
    }
}

fn month_separator(c: char) -> State {
    match c {
        '0'..='9' => State::Year,
        _ => State::End,
    }
}

fn first_day(c: char) -> State {
    match c {
        '0'..='9' => State::DayEnd,
        '-' | '/' | '.' => State::DaySeparator,
        _ => State::End,
    }
}

fn day_one_to_nine(c: char) -> State {
    match c {
        '1'..='9' => State::DayEnd,
        _ => State::End,
    }
}

fn only_first_two_days_or_end(c: char) -> State {
    match c {
        '0' | '1' => State::DayEnd,
        '-' | '/' | '.' => State::DaySeparator,
        _ => State::End,
    }
}

fn day_end(c: char) -> State {
    match c {
        '-' | '/' | '.' => State::DaySeparator,
        _ => State::End,
    }
}

fn day_separator(c: char) -> State {
    match c {
        '0' => State::JanToSep,
        '1' => State::OctToDecOrJan,
        '2'..='9' => State::MonthEnd,
        _ => State::End,
    }
}

fn month_end(c: char) -> State {
    match c {
        '-' | '/' | '.' => State::MonthSeparator,
        _ => State::End,
    }
}

fn jan_to_sep(c: char) -> State {
    match c {
        '1'..='9' => State::MonthEnd,
        _ => State::End,
    }
}

fn oct_to_dec_or_jan(c: char) -> State {
    match c {
        '0'..='2' => State::MonthEnd,
        '-' | '/' | '.' => State::MonthSeparator,
        _ => State::End,
    }
}

// Transition function
fn transition(state: State, c: char) -> State {
    match state {
        State::Start => start(c),
        State::Year => year(c),
        State::JanToSep => jan_to_sep(c),
        State::OctToDecOrJan => oct_to_dec_or_jan(c),
        State::MonthEnd => month_end(c),
        State::MonthSeparator => month_separator(c),
        State::FirstDay => first_day(c),
        State::OnlyFirstTwoDaysOrEnd => only_first_two_days_or_end(c),
        State::DayEnd => day_end(c),
        State::DaySeparator => day_separator(c),
        State::DayOneToNine => day_one_to_nine(c),
        State::End => State::End,
    }
}

pub fn is_valid_date_notation(s: &str) -> bool {
    let mut state = State::Start;
    for c in s.chars() {
        state = transition(state, c);
    }
    state == State::Year
}
