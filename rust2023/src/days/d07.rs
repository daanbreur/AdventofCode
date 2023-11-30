use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 7;

type Data = Vec<String>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test07.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(95437), Answer::Number(24933642))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input.lines().map(|x| x.to_owned()).collect::<Vec<String>>(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut total_size: u64 = 0;
        let mut stack: Vec<u64> = vec![0];

        for line in data {
            if line.starts_with("$") {
                let command = line.to_string().replace("$ ", "");
                let mut split_command = command.split(" ");
                match split_command.next().unwrap() {
                    "cd" => {
                        match split_command.next().unwrap() {
                            "/" => {},
                            ".." => {
                                let current_dir = stack.pop().unwrap();
                                if current_dir < 100_000 {
                                    total_size += current_dir;
                                }
                                if let Some(new_dir) = stack.last_mut() {
                                    *new_dir += current_dir;
                                }
                            },
                            _dir => {
                                stack.push(0);
                            },
                        }
                    },
                    _ => {},
                }
            } else {
                if let Some(current_dir) = line.split_once(" ") {
                    if let Ok(size) = current_dir.0.parse::<u64>() {
                        if let Some(directory) = stack.last_mut() {
                            *directory += size;
                        }
                    }
                }
            }
        }

        Answer::Number(total_size as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut stack: Vec<i64> = vec![0];
        let mut directory_sizes: Vec<i64> = Vec::new();

        for line in data {
            if line.starts_with("$") {
                let command = line.to_string().replace("$ ", "");
                let mut split_command = command.split(" ");
                match split_command.next().unwrap() {
                    "cd" => {
                        match split_command.next().unwrap() {
                            "/" => {},
                            ".." => {
                                let current_dir = stack.pop().unwrap();
                                if let Some(new_dir) = stack.last_mut() {
                                    *new_dir += current_dir;
                                }
                                directory_sizes.push(current_dir);
                            },
                            _dir => {
                                stack.push(0);
                            },
                        }
                    },
                    _ => {},
                }
            } else {
                if let Some(current_dir) = line.split_once(" ") {
                    if let Ok(size) = current_dir.0.parse::<i64>() {
                        if let Some(directory) = stack.last_mut() {
                            *directory += size;
                        }
                    }
                }
            }
        }

        while stack.len() > 0 { 
            let size = stack.pop().unwrap();
            directory_sizes.push(size);
            if stack.len() > 0 {
                if let Some(directory) = stack.last_mut() {
                    *directory += size;
                }
            }
        }

        let space_remaining = 70_000_000 - directory_sizes.last().unwrap();
        let space_required = 30_000_000 - space_remaining;

        let delete_size: i64 = directory_sizes
            .into_iter()
            .filter(|x| x >= &space_required)
            .min()
            .unwrap();
            

        Answer::Number(delete_size as u64)
    }
}
