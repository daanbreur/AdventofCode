use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 4;

type Data = Vec<Vec<bool>>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test04.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(13), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '@' => true,
                            '.' => false,
                            _ => panic!("Unexpected character in input"),
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        dprintln!("input data: {:?}", data);

        let mut count: u64 = 0;

        let (w, h) = (data[0].len(), data.len());

        for i in 0..h {
            for j in 0..w {
                if !data[i][j] { continue; }

                let mut neighbors = 0;

                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 { continue; }

                        let neighbor_i = i as isize + di;
                        let neighbor_j = j as isize + dj;

                        if neighbor_i < 0 || neighbor_i >= h as isize || neighbor_j < 0 || neighbor_j >= w as isize {
                            continue;
                        }

                        if data[neighbor_i as usize][neighbor_j as usize] {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    count += 1;
                }
            }
        }
        
        Answer::Number(count)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.len() as u64)
    }
}
