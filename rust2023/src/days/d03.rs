use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 3;

type Data = Vec<String>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test03.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(157), Answer::Number(70))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input.split("\n")
                .map(|v| v.to_owned())
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut doubles: Vec<char> = Vec::new();
        let mut sum: u64 = 0;

        for l in data {
            let line: String = l.to_owned();
            let first_half = &line[..line.len()/2];
            let second_half = &line[line.len()/2..];

            dprintln!("input: {}, first: {}, second: {}", line, first_half, second_half);

            for char in first_half.chars() {
                if second_half.contains(char) {
                    doubles.push(char);
                    break;
                }
            }
        }

        for char in doubles {
            if char.is_uppercase() {
                sum += (char as u64) - ('A' as u64) + 27
            } else {
                sum += (char as u64) - ('a' as u64) + 1
            }
        }

        Answer::Number(sum as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut keys: Vec<char> = Vec::new();
        let mut sum: u64 = 0;

        for i in (0..data.len()-1).step_by(3) {
            let line: String = data.get(i).unwrap().to_string();
            for char in line.chars() {
                if data.get(i+1).unwrap().contains(char) && data.get(i+2).unwrap().contains(char) {
                    keys.push(char);
                    dprintln!("input: {}, first: {}, second: {}", line, data.get(i+1).unwrap(), data.get(i+2).unwrap());
                    dprintln!("key: {}", char.to_string());
                    break;
                }
            }
        }

        for char in keys {
            if char.is_uppercase() {
                sum += (char as u64) - ('A' as u64) + 27
            } else {
                sum += (char as u64) - ('a' as u64) + 1
            }
        }

        Answer::Number(sum as u64)
    }
}
