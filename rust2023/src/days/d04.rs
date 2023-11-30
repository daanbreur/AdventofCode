use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 4;

type Data = Vec<Vec<Vec<u64>>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test04.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(2), Answer::Number(4))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| {
                    v
                        .split(",")
                        .map(|x| {
                            x.split("-").map(|n| {
                                n.parse::<u64>().expect("error while parsing input.")
                            }).collect()
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut answer: u64 = 0;

        for pair in data {
            if (pair[0][0] <= pair[1][1] && pair[0][0] >= pair[1][0]) && (pair[0][1] <= pair[1][1] && pair[0][1] >= pair[1][0]) {
                answer += 1;
            } else if (pair[1][0] <= pair[0][1] && pair[1][0] >= pair[0][0]) && (pair[1][1] <= pair[0][1] && pair[1][1] >= pair[0][0]) {
                answer += 1;
            }
        }

        Answer::Number(answer as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut answer: u64 = 0;

        for pair in data {
            if pair[0][0] <= pair[1][1] && pair[0][1] >= pair[1][0] {
                answer += 1;
            } else if pair[1][0] <= pair[0][1] && pair[1][1] >= pair[0][0] {
                answer += 1;
            }
        }

        Answer::Number(answer as u64)
    }
}
