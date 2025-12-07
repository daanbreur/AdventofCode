use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 5;

#[derive(Debug, Clone)]
pub struct Inventory {
    fresh_ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

impl Inventory {
    fn is_fresh(&self, ingredient: u64) -> bool {
        for &(start, end) in &self.fresh_ranges {
            if start <= ingredient && ingredient <= end {
                return true;
            }
        }

        false
    }
}

type Data = Inventory;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test05.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(3), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        let (fresh_ranges, ingredients) = input.split_once("\n\n").expect("error while parsing input. no double newline found, are you on windows?");

        (
            Self {},
            Inventory {
                fresh_ranges: fresh_ranges
                    .lines()
                    .map(|line| {
                        let (start, end) = line.split_once('-').unwrap();
                        (start.parse::<u64>().expect("error while parsing input."), end.parse::<u64>().expect("error while parsing input."))
                    })
                    .collect(),
                ingredients: ingredients
                    .lines()
                    .map(|line| line.parse::<u64>().expect("error while parsing input."))
                    .collect(),
            },
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        dprintln!("data: {:?}", data);
        let mut count: u64 = 0;

        for ingredient in &data.ingredients {
            count += if data.is_fresh(*ingredient) { 1 } else { 0 };
        }

        Answer::Number(count as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0 as u64)
    }
}
