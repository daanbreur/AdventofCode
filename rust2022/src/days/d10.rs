use crate::{dprintln, vprintln, dprint, vprint};

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 10;

type Data = Vec<String>;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test10.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(13140), Answer::String(String::new()))
    }

    fn init(input: &str) -> (Self, Data) {
        (
            Self {},
            input
                .lines()
                .map(|x| x.to_string())
                .collect(),
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut answer: i64 = 0;
        let mut instuction_counter = 0;
        let mut register_x: i64 = 1;

        for line in data {
            let mut instruction = line.split(" ");
            let mut inst = instruction.next().unwrap();
            let mut data = instruction.next();
            vprintln!("TAPE: {:?} - INSTRUCTION: {:?} - DATA: {:?}", line, inst, data);
            match inst {
                "noop" => {
                    vprint!("oldIC: {:?} - oldRX: {:?} ", instuction_counter, register_x);
                    instuction_counter += 1;
                    if instuction_counter == 20 || instuction_counter == 60 || instuction_counter == 100 || instuction_counter == 140 || instuction_counter == 180 || instuction_counter == 220 {
                        answer += instuction_counter*register_x;
                    }
                    vprintln!(" |  IC: {:?} - RX: {:?}", instuction_counter, register_x);
                },
                "addx" => {
                    instuction_counter += 1;
                    if instuction_counter == 20 || instuction_counter == 60 || instuction_counter == 100 || instuction_counter == 140 || instuction_counter == 180 || instuction_counter == 220 {
                        answer += instuction_counter*register_x;
                    }
                    instuction_counter += 1;
                    if instuction_counter == 20 || instuction_counter == 60 || instuction_counter == 100 || instuction_counter == 140 || instuction_counter == 180 || instuction_counter == 220 {
                        answer += instuction_counter*register_x;
                    }
                    register_x += data.unwrap().parse::<i64>().unwrap();
                },
                _ => unreachable!(),
            }
        }

        Answer::Number(answer as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        let mut instuction_counter = 0;
        let mut crt_counter = 0;
        let mut register_x: i64 = 1;
        let mut crt_image: String = String::new();

        for line in data {
            let mut instruction = line.split(" ");
            let mut inst = instruction.next().unwrap();
            let mut data = instruction.next();
            vprintln!("TAPE: {:?} - INSTRUCTION: {:?} - DATA: {:?}", line, inst, data);
            match inst {
                "noop" => {
                    vprint!("oldCRT: {:?} - oldIC: {:?} - oldRX: {:?} ", crt_counter, instuction_counter, register_x);
                    instuction_counter += 1;
                    if crt_counter%40 == 0 {
                        crt_image.push('\n');
                    }
                    if ((crt_counter%40) - register_x).abs() <= 1 {
                        crt_image.push('█');
                    } else {
                        crt_image.push(' ');
                    }
                    crt_counter += 1;

                    vprintln!(" |  CRT: {:?} - IC: {:?} - RX: {:?}", crt_counter, instuction_counter, register_x);
                },
                "addx" => {
                    instuction_counter += 1;
                    if crt_counter%40 == 0 {
                        crt_image.push('\n');
                    }
                    if ((crt_counter%40) - register_x).abs() <= 1 {
                        crt_image.push('█');
                    } else {
                        crt_image.push(' ');
                    }
                    crt_counter += 1;

                    instuction_counter += 1;
                    if crt_counter%40 == 0 {
                        crt_image.push('\n');
                    }
                    if ((crt_counter%40) - register_x).abs() <= 1 {
                        crt_image.push('█');
                    } else {
                        crt_image.push(' ');
                    }
                    crt_counter += 1;
                    register_x += data.unwrap().parse::<i64>().unwrap();
                },
                _ => unreachable!(),
            }
        }


        Answer::String(crt_image)
    }
}

//█
// 