extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 4;

type Data = String;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test04.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(609043), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .trim()
                .to_owned(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let secret: &[u8] = data.as_bytes();
        let mut hasher = Md5::new();
        let mut answer: u64 = 0;

        for i in 0..u64::MAX {
            hasher.input(secret);
            hasher.input(i.to_string().as_bytes());

            let output = hasher.result_str();

            if output.starts_with("00000") {
                answer = i;
                break;
            }

            hasher.reset();
        }

        return Answer::Number(answer as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let secret: &[u8] = data.as_bytes();
        let mut hasher = Md5::new();
        let mut answer: u64 = 0;

        for i in 0..u64::MAX {
            hasher.input(secret);
            hasher.input(i.to_string().as_bytes());

            let output = hasher.result_str();

            if output.starts_with("000000") {
                answer = i;
                break;
            }

            hasher.reset();
        }

        return Answer::Number(answer as u64)
    }
}
