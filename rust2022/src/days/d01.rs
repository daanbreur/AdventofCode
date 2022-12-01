use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 1;

type Data = Vec<Vec<u64>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test01.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(24000), Answer::Number(45000))
    }

    fn init(input: &str) -> (Self, Data) {
        let mut output: Vec<Vec<u64>> = Vec::new();

        input.split("\n\n").for_each(|x| {
            let data: Vec<u64> = x.lines().map(|y| {
                y.parse::<u64>().expect("error while parsing input.")
            }).collect();
            output.push(data);
        });

        (
            Self {},
            output
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut answer: u64 = 0;
        data.iter().for_each(|data| {
            let mut sum: u64 = 0;
            data.iter().for_each(|n| {
                sum += n;
            });
            if sum > answer {
                answer = sum;
            }
        });

        Answer::Number(answer as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut answer: u64 = 0;
        data.iter().for_each(|data| {
            let mut sum: u64 = 0;
            data.iter().for_each(|n| {
                sum += n;
            });
            if sum > answer {
                answer = sum;
            }
        });

        Answer::Number(answer as u64)
    }
}