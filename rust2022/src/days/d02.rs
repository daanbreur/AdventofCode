use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 2;
type Data = Vec<Vec<String>>;

fn get_hand_score(hand: &String) -> u64 {
    if hand == "X" {
        return 1;
    }
    if hand == "Y" {
        return 2;
    }
    if hand == "Z" {
        return 3;
    }
    return 0;
}

fn get_your_hand(play: &String, opponent_hand: &String) -> u64 {
    if play == "X" {
        if opponent_hand == "A" {
            return 3;
        }
        if opponent_hand == "B" {
            return 1;
        }
        if opponent_hand == "C" {
            return 2;
        }
    }
    if play == "Y" {
        if opponent_hand == "A" {
            return 1;
        }
        if opponent_hand == "B" {
            return 2;
        }
        if opponent_hand == "C" {
            return 3;
        }
    }
    if play == "Z" {
        if opponent_hand == "A" {
            return 2;
        }
        if opponent_hand == "B" {
            return 3;
        }
        if opponent_hand == "C" {
            return 1;
        }
    }
    return 0;
}

impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test02.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(15), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        let output: Vec<Vec<String>> = input.lines().map(|x| x.split_whitespace().map(|v| v.to_owned()).collect()).collect();

        (
            Self {},
            output,
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut score = 0;

        for round in data {
            dprintln!("Hands: {:?} | Score: {}", round, score);
            score+=get_hand_score(&round[1]);

            if round[0] == "A" && round[1] == "X" || round[0] == "B" && round[1] == "Y" || round[0] == "C" && round[1] == "Z" {
                score+=3;
                dprintln!("Tie! NewScore: {}", score);
            }

            if round[0] == "A" && round[1] == "Y" || round[0] == "B" && round[1] == "Z" || round[0] == "C" && round[1] == "X" {
                score+=6;
                dprintln!("Win! NewScore: {}", score);
            }

            dprintln!("New Score: {}", score);
        }

        Answer::Number(score as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut score: u64 = 0;

        for round in data {
            score += get_your_hand(&round[1], &round[0]);
            if round[1] == "X" {
                score += 0;
            }
            if round[1] == "Y" {
                score += 3;
            }
            if round[1] == "Z" {
                score += 6;
            }
        }

        Answer::Number(score as u64)
    }
}
