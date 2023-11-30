use std::collections::{HashMap, HashSet};

use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 9;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Motion {
    direction: String,
    times: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
}

type Data = Vec<Motion>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test09.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(13), Answer::Number(36))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|line| line.split_once(" ").unwrap())
                .map(|v| Motion { direction: v.0.to_string(), times: v.1.parse::<u64>().expect("error while parsing input.") })
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut visited: HashMap<Point, u64> = HashMap::new();
        let mut tail: Point = Point { x: 0, y: 0 };
        let mut head: Point = Point { x: 0, y: 0 };
        visited.insert(tail, 1);

        for motion in data.into_iter() {
            for _ in 0..motion.times {
                if motion.direction == String::from("U") {
                    head.y += 1;
                }
                if motion.direction == String::from("D") {
                    head.y -= 1;
                }
                if motion.direction == String::from("L") {
                    head.x -= 1;
                }
                if motion.direction == String::from("R") {
                    head.x += 1;
                }
                
                let diff_x = (head.x - tail.x).abs() > 1;
                let diff_y = (head.y - tail.y).abs() > 1;

                if diff_x || diff_y {
                    if head.x > tail.x {
                        tail = Point {
                            x: tail.x+1,
                            y: tail.y,
                        }
                    } else if head.x < tail.x {
                        tail = Point {
                            x: tail.x-1,
                            y: tail.y,
                        }
                    }

                    if head.y > tail.y {
                        tail = Point {
                            x: tail.x,
                            y: tail.y+1,
                        }
                    } else if head.y < tail.y {
                        tail = Point {
                            x: tail.x,
                            y: tail.y-1,
                        }
                    }
                }

                let mut visited_count: u64 = 1;
                match visited.get(&tail) {
                    Some(val) => visited_count = *val + 1,
                    None => (),
                }
                visited.insert(tail, visited_count);
            }
        }

        Answer::Number(visited.len() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut visited: HashSet<Point> = HashSet::new();

        let mut tails: [Point; 10] = [Point { x: 0, y: 0 },Point { x: 0, y: 0 },Point { x: 0, y: 0 },Point { x: 0, y: 0 },Point { x: 0, y: 0 },Point { x: 0, y: 0 },Point { x: 0, y: 0 },Point { x: 0, y: 0 },Point { x: 0, y: 0 },Point { x: 0, y: 0 }];
        visited.insert(tails[9]);

        for motion in data.into_iter() {
            for _ in 0..motion.times {
                if motion.direction == String::from("U") {
                    tails[0].y += 1;
                }
                if motion.direction == String::from("D") {
                    tails[0].y -= 1;
                }
                if motion.direction == String::from("L") {
                    tails[0].x -= 1;
                }
                if motion.direction == String::from("R") {
                    tails[0].x += 1;
                }
                
                for i in 1..10 {
                    let diff_x = (tails[i-1].x - tails[i].x).abs() > 1;
                    let diff_y = (tails[i-1].y - tails[i].y).abs() > 1;

                    if diff_x || diff_y {
                        if tails[i-1].x > tails[i].x {
                            tails[i] = Point {
                                x: tails[i].x+1,
                                y: tails[i].y,
                            };
                        } else if tails[i-1].x < tails[i].x {
                            tails[i] = Point {
                                x: tails[i].x-1,
                                y: tails[i].y,
                            };
                        }

                        if tails[i-1].y > tails[i].y {
                            tails[i] = Point {
                                x: tails[i].x,
                                y: tails[i].y+1,
                            };
                        } else if tails[i-1].y < tails[i].y {
                            tails[i] = Point {
                                x: tails[i].x,
                                y: tails[i].y-1,
                            };
                        }
                    }
                }

                visited.insert(tails[9]);
            }
        }
        Answer::Number(visited.len() as u64)
    }
}
