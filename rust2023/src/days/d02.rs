use crate::{dprintln, vprintln};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 2;

#[derive(Debug, Clone, Copy)]
pub struct Hand {
    red: u64,
    green: u64,
    blue: u64,
}

impl From<&str> for Hand {
    fn from(data: &str) -> Self {
        let mut output = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        for color in data.split(", ") {
            let data = color.split_once(" ").unwrap();
            let number = data.0.parse::<u64>().unwrap();
            let color = data.1;

            match color {
                "red" => output.red = number,
                "green" => output.green = number,
                "blue" => output.blue = number,
                _ => panic!("unexpected input"),
            }
        }

        output
    }
}

type Data = Vec<Vec<Hand>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test02.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(8), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| {
                    v.split_once(": ")
                        .unwrap()
                        .1
                        .split("; ")
                        .map(|hand| hand.into())
                        .collect()
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut answer: u64 = 0;

        for (id, value) in data.iter().enumerate() {
            let mut game_possible = true;

            // dprintln!("{:?} {:?}", id, value);

            for hand in value {
                if hand.red > 12 || hand.green > 13 || hand.blue > 14 {
                    game_possible = false;
                }
            }

            if game_possible {
                answer += (id + 1) as u64;
            }
        }

        Answer::Number(answer as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.len() as u64)
    }
}
