use std::collections::HashMap;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 3;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Home {
    x: i64,
    y: i64,
}

type Data = Vec<char>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test03.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(2), Answer::Number(11))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .trim()
                .chars()
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut visited: HashMap<Home, u64> = HashMap::new();
        let mut home = Home { x: 0, y: 0 };

        visited.insert(home, 1);

        for instruction in data {
            home = match instruction {
                '^' => Home {
                    x: home.x,
                    y: home.y + 1,
                },
                'v' => Home {
                    x: home.x,
                    y: home.y - 1,
                },
                '>' => Home {
                    x: home.x + 1,
                    y: home.y,
                },
                '<' => Home {
                    x: home.x - 1,
                    y: home.y,
                },
                _ => panic!("Unrecognized instruction: {}", instruction),
            };

            let mut visited_count: u64 = 0;
            match visited.get(&home) {
                Some(val) => visited_count = *val + 1,
                None => (),
            }
            visited.insert(home, visited_count);
        }

        Answer::Number(visited.len() as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut visited: HashMap<Home, u64> = HashMap::new();
        let mut home = Home { x: 0, y: 0 };
        let mut home_2 = Home { x: 0, y: 0 };

        visited.insert(home, 1);

        for i in 0..data.len()-1 {
            if i%2 == 0 {
                home = match data[i] {
                    '^' => Home {
                        x: home.x,
                        y: home.y + 1,
                    },
                    'v' => Home {
                        x: home.x,
                        y: home.y - 1,
                    },
                    '>' => Home {
                        x: home.x + 1,
                        y: home.y,
                    },
                    '<' => Home {
                        x: home.x - 1,
                        y: home.y,
                    },
                    _ => panic!("Unrecognized instruction: {}", data[i]),
                };

                let mut visited_count: u64 = 0;
                match visited.get(&home) {
                    Some(val) => visited_count = *val + 1,
                    None => (),
                }
                visited.insert(home, visited_count);
            } else {
                home_2 = match data[i] {
                    '^' => Home {
                        x: home_2.x,
                        y: home_2.y + 1,
                    },
                    'v' => Home {
                        x: home_2.x,
                        y: home_2.y - 1,
                    },
                    '>' => Home {
                        x: home_2.x + 1,
                        y: home_2.y,
                    },
                    '<' => Home {
                        x: home_2.x - 1,
                        y: home_2.y,
                    },
                    _ => panic!("Unrecognized instruction: {}", data[i]),
                };

                let mut visited_count: u64 = 0;
                match visited.get(&home_2) {
                    Some(val) => visited_count = *val + 1,
                    None => (),
                }
                visited.insert(home_2, visited_count);
            }
        }

        Answer::Number(visited.len() as u64)
    }
}
