use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 2;

type Data = Vec<(u64, u64)>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test02.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(1227775554), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input.split(",")
            .filter(|v| !v.is_empty())
            .map(|v| {
                let (start, end) = v.split_once("-").expect("error while parsing input.");
                (
                    start.parse::<u64>().expect("error while parsing input."),
                    end.parse::<u64>().expect("error while parsing input."),
                )
            })
            .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut count: u64 = 0;

        for &mut (start, end) in data.iter_mut() {
            for id in start..=end {
                dprintln!("[{} - {}] id: {}", start, end, id);

                let id_str: String = id.to_string();
                let id_length = id_str.len();

                if id_length % 2 == 1 { continue; };

                dprintln!("  length: {}", id_length);

                let (left, right) = id_str.split_at(id_length / 2);
                
                dprintln!("  left: {}, right: {}", left, right);

                if left == right {
                    dprintln!("  match found: {}", id);
                    count += id;
                }
            }
        }

        Answer::Number(count)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.len() as u64)
    }
}
