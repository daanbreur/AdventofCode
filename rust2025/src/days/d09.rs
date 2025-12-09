use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 9;

#[derive(Debug, Clone)]
pub struct Point {
    x: u64,
    y: u64
}

impl Point {
    fn get_area_with_other_point(&self, point: &Point) -> u64 {
        let dx = (self.x as i64 - point.x as i64).abs() as u64;
        dprintln!("dx: {:?}", dx);

        let dy = (self.y as i64 - point.y as i64).abs() as u64;
        dprintln!("dy: {:?}", dy);

        let area: u64 = (dx + 1) * (dy + 1);
        dprintln!("area: {:?}", area);

        area
    }
}

type Data = Vec<Point>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(include_str!("test_inputs/test09.txt"))
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(50), Answer::Number(24))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|v| {
                    let (x, y) = v.split_once(",").unwrap();
                    Point {
                        x: x.parse::<u64>().expect("error while parsing input."),
                        y: y.parse::<u64>().expect("error while parsing input."),
                    }
                })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut highest: u64 = 0;

        for i in 0..data.len() {
            for j in 0..data.len() {
                let area = data[i].get_area_with_other_point(&data[j]);
                if area > highest {
                    highest = area;
                }
            }
        }
        
        Answer::Number(highest)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(data.len() as u64)
    }
}
