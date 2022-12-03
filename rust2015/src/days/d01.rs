use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 1;

type Data = Vec<char>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test01.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(3), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input.trim().chars().collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut answer = 0;

        for char in data {
            if char.eq_ignore_ascii_case(&'(') {
                answer += 1;
            }
            if char.eq_ignore_ascii_case(&')') {
                answer -= 1;
            }
        }

        Answer::Number(answer as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut level: i64 = 0;
        let mut answer: u64 = 0;

        for i in 1..data.len() {
            let char: char = data.get(i-1).unwrap().to_owned();
            if char.eq_ignore_ascii_case(&'(') {
                level += 1;
            }
            if char.eq_ignore_ascii_case(&')') {
                level -= 1;
            }
            if level <= -1 {
                answer = i as u64;
                break;
            }
        }

        Answer::Number(answer as u64)
    }
}