use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 5;

type Data = (Vec<Vec<char>>, Vec<(i32, i32, i32)>);
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test05.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::String(String::from("CMZ")), Answer::String(String::from("MCD")))
    }

    fn init(input: &str) -> (Self, Data) {
        
        let mut two_parts_input = input.split("\n\n");
        let mut stack_lines = two_parts_input.next().unwrap().lines().collect::<Vec<&str>>();

        let stack_indexes = stack_lines.pop().unwrap().trim();
        let amount_of_stacks = stack_indexes.split(' ').collect::<Vec<&str>>().pop().unwrap().parse::<i32>().unwrap();
        let mut stacks: Vec<Vec<char>> = Vec::new();
        for _ in 0..amount_of_stacks {
            stacks.push(Vec::new());
        }
        while let Some(s) = stack_lines.pop() {
            let mut chars = s.clone().chars();
            let mut stack_num = 0;
            while let (Some(_), Some(item), Some(_)) = (chars.next(), chars.next(), chars.next()) {
                if item != ' '{
                    stacks[stack_num].push(item)
                }
                stack_num += 1;
                chars.next();
            }
        }

        dprintln!("{:?}", stacks);

        let move_lines = two_parts_input.next().unwrap().lines().collect::<Vec<&str>>();
        let moves: Vec<(i32, i32, i32)> = move_lines.into_iter().map(|line| {
            let mut split_line = line.split(' ');
            split_line.next();
            let amount = split_line.next().unwrap().parse::<i32>().unwrap();
            split_line.next();
            let from = split_line.next().unwrap().parse::<i32>().unwrap();
            split_line.next();
            let to = split_line.next().unwrap().parse::<i32>().unwrap();
            (amount, from, to)
        }).collect();

        (
            Self {},
            (stacks, moves),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut output: String = String::new();
        
        let data = &mut data.clone();

        let stacks = &mut data.0;
        let instructions = &data.1;

        for instruction in instructions {
            for _ in 0..instruction.0 {
                dprintln!("{:?} ; {:?}", stacks, instruction);
                let tmp = stacks[(instruction.1-1) as usize].pop().unwrap();
                stacks[(instruction.2-1) as usize].push(tmp);
            }
        }

        for stack in stacks {
            output.push(stack.pop().unwrap())
        }

        Answer::String(output)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut output: String = String::new();
        
        let data = &mut data.clone();

        let stacks = &mut data.0;
        let instructions = &data.1;

        for instruction in instructions {
            let mut tmp_stack = Vec::new();
            for _ in 0..instruction.0 {
                dprintln!("{:?} ; {:?}", stacks, instruction);
                let tmp = stacks[(instruction.1-1) as usize].pop().unwrap();
                tmp_stack.push(tmp);
            }
            tmp_stack.reverse();
            for tmp in tmp_stack {
                stacks[(instruction.2-1) as usize].push(tmp);
            }
        }

        for stack in stacks {
            output.push(stack.pop().unwrap())
        }

        Answer::String(output)
    }
}
