use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 2;

type Data = Vec<Vec<u64>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test02.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(101), Answer::Number(48))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| {
                    v.split("x").map(|v| v.parse::<u64>().expect("error while parsing input.")).collect()
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut sum: u64 = 0;

        for paper in data.into_iter() {
            let mut sides: Vec<u64> = Vec::new();
            sides.push(paper[0] * paper[1]);
            sides.push(paper[1] * paper[2]);
            sides.push(paper[2] * paper[0]);
            sides.iter().for_each(|x| sum += 2*x);
            sum += sides.iter().min().unwrap();
        }

        Answer::Number(sum as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut sum: u64 = 0;

        for ribbon in data.into_iter() {
            let mut sides: Vec<u64> = Vec::new();
            sides.push(2*ribbon[0] + 2*ribbon[1]);
            sides.push(2*ribbon[1] + 2*ribbon[2]);
            sides.push(2*ribbon[2] + 2*ribbon[0]);
            sum += sides.iter().min().unwrap();
            sum += ribbon[0]*ribbon[1]*ribbon[2];
        }

        Answer::Number(sum as u64)
    }
}
