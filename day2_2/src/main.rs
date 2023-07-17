use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
enum Action {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
struct Game {
    ours: Action,
    theirs: Action,
}

impl Game {
    fn from(input: &str) -> Option<Self> {
        let preprocessed_input = input.trim();
        if preprocessed_input.len() == 3
            && input.contains(' ')
            && input.contains(['A', 'B', 'C'])
            && input.contains(['X', 'Y', 'Z'])
        {
            if let Some((t, o)) = preprocessed_input.split(' ').collect_tuple() {
                Some(Self {
                    ours: select_move(o, string_to_action(t)),
                    theirs: string_to_action(t),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn score(&self) -> u32 {
        if self.ours == self.theirs {
            action_to_value(&self.ours) + 3
        } else if ours_winning(&self.ours, &self.theirs) {
            action_to_value(&self.ours) + 6
        } else {
            action_to_value(&self.ours)
        }
    }
}

fn ours_winning(ours: &Action, theirs: &Action) -> bool {
    (*ours == Action::Paper && *theirs == Action::Rock)
        || (*ours == Action::Rock && *theirs == Action::Scissor)
        || (*ours == Action::Scissor && *theirs == Action::Paper)
}

fn action_to_value(action: &Action) -> u32 {
    match action {
        Action::Rock => 1,
        Action::Paper => 2,
        Action::Scissor => 3,
    }
}

fn string_to_action(input: &str) -> Action {
    match input {
        "A" => Action::Rock,
        "B" => Action::Paper,
        "C" => Action::Scissor,
        _ => panic!("Error during line parsing: {}", input),
    }
}

fn select_move(ours: &str, theirs: Action) -> Action {
    match ours {
        "X" => losing_move(theirs),
        "Y" => theirs, // Draw
        "Z" => winning_move(theirs),
        _ => panic!("Error during line parsing: {}", ours),
    }
}

fn losing_move(action: Action) -> Action {
    match action {
        Action::Rock => Action::Scissor,
        Action::Paper => Action::Rock,
        Action::Scissor => Action::Paper,
    }
}

fn winning_move(action: Action) -> Action {
    match action {
        Action::Rock => Action::Paper,
        Action::Paper => Action::Scissor,
        Action::Scissor => Action::Rock,
    }
}

fn main() {
    let result: u32 = include_str!("input.txt")
        .lines()
        .map(Game::from)
        .filter_map(|g| Some(g?.score()))
        .sum();

    println!("{:?}", result)
}
