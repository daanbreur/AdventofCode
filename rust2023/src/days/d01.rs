use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 1;

type Data = Vec<String>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test01.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(142), Answer::Number(281))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| v.parse::<String>().expect("error while parsing input."))
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut answer: u64 = 0;
        data.iter().for_each(|n| {
            let mut wholenumber: u64 = 0;
            let mut first: String = "".to_owned();
            let mut last: String = "".to_owned();

            let char_vec: Vec<char> = n.chars().collect();
            dprintln!("{:?}", char_vec);

            for v in char_vec {
                if (v.is_digit(10)) {
                    if (first == "") {
                        first = v.to_string();
                    }
                    last = v.to_string();
                }
            }

            dprintln!("{:?} {:?}", first, last);

            wholenumber = (first + &last)
                .parse::<u64>()
                .expect("My fwend something failed");

            dprintln!("{:?}", wholenumber);
            answer += wholenumber;
        });

        Answer::Number(answer as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut answer: u64 = 0;
        data.iter().for_each(|n| {
            let mut test: String = n.clone().to_owned();
            test = test.replace("one", "one1one");
            test = test.replace("two", "two2two");
            test = test.replace("three", "three3three");
            test = test.replace("four", "four4four");
            test = test.replace("five", "five5five");
            test = test.replace("six", "six6six");
            test = test.replace("seven", "seven7seven");
            test = test.replace("eight", "eight8eight");
            test = test.replace("nine", "nine9nine");

            let mut wholenumber: u64 = 0;
            let mut first: String = "".to_owned();
            let mut last: String = "".to_owned();

            let char_vec: Vec<char> = test.chars().collect();
            dprintln!("{:?}", char_vec);

            for v in char_vec {
                if (v.is_digit(10)) {
                    if (first == "") {
                        first = v.to_string();
                    }
                    last = v.to_string();
                }
            }

            dprintln!("{:?} {:?}", first, last);

            wholenumber = (first + &last)
                .parse::<u64>()
                .expect("My fwend something failed");

            dprintln!("{:?}", wholenumber);
            answer += wholenumber;
        });

        Answer::Number(answer as u64)
    }
}
