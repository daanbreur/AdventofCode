use std::collections::HashSet;

use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 6;

type Data = String;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test06.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(7), Answer::Number(19))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input.trim().to_owned(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut answer: u64 = 0;

        let windows = data.as_bytes().windows(4);
        for (i, window) in windows.into_iter().enumerate() {
            if window.iter().collect::<HashSet<_>>().len() == 4 {
                answer += i as u64;
                break
            }
        }

        answer += 4;

        Answer::Number(answer as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut answer: u64 = 0;

        let windows = data.as_bytes().windows(14);
        for (i, window) in windows.into_iter().enumerate() {
            if window.iter().collect::<HashSet<_>>().len() == 14 {
                answer += i as u64;
                break
            }
        }

        answer += 14;

        Answer::Number(answer as u64)
    }
}
