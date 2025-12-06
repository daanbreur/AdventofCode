use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 3;

type Data = Vec<Vec<u64>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test03.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(357), Answer::Number(3121910778619))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| {
                    v.split("")
                        .filter(|v| !v.is_empty())
                        .map(|n| {
                            n.parse::<u64>().expect("error while parsing input.")
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut result: u64 = 0;    

        for row in data.iter_mut() {
            let mut best: u64 = 0;

            for i in 0..row.len() {
                let first = row[i];

                for j in i+1..row.len() {
                    let second = row[j];

                    let value = first * 10 + second;

                    if value > best {
                        best = value;
                    }
                }
            }
            
            result += best;
        }

        Answer::Number(result)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.len() as u64)
    }
}
