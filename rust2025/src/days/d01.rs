use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 1;

type Data = Vec<(char, u64  )>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test01.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(3), Answer::Number(6))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|line| {
                    let (char, num) = line.split_at(1);
                    (
                        char.chars().next().expect("error while parsing input."),
                        num.parse::<u64>().expect("error while parsing input.")
                    )
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut position: i64 = 50;
        let mut count = 0;

        for &mut (dir, distance) in data.iter_mut() {
            let dist = distance as i64;

            match dir {
                'R' => position += dist,
                'L' => position -= dist,
                _ => unreachable!("Invalid direction"),
            }

            position = ((position % 100) + 100) % 100;

            if position == 0 {
                count += 1;
            }
        }

        Answer::Number(count as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut position: i64 = 50;
        let mut count = 0;

        for &mut (dir, distance) in data.iter_mut() {
            let dist = distance as i64;

            for _ in 0..dist {
                match dir {
                    'R' => position += 1,
                    'L' => position -= 1,
                    _ => unreachable!("Invalid direction"),
                }

                position = ((position % 100) + 100) % 100;

                if position == 0 {
                    count += 1;
                }
            }
        }

        Answer::Number(count as u64)
    }
}
