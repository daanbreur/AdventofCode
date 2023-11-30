use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 20;

type Data = Vec<u64>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test20.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(0), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| v.parse::<u64>().expect("error while parsing input."))
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        Answer::Number(data.len() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.len() as u64)
    }
}
