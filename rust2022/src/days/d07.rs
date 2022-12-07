use crate::dprintln;

use super::{Answer, Day, DayImpl};

const CURRENT_DAY: u8 = 7;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum FilesystemItem {
    Directory(Directory),
    File(File),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct File {
    name: String,
    size: u64,
}

impl File {
    pub fn get_size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Directory {
    name: String,
    parent: Option<*mut Directory>,
    children: Vec<FilesystemItem>,
}

impl Directory {
    pub fn add_children(&mut self, item: FilesystemItem) {
        self.children.push(item);
    }

    pub fn get_size(&self) -> u64 {
        let mut size: u64 = 0;

        for item in self.children.iter() {
            match item {
                FilesystemItem::Directory(data) => {
                    size += data.get_size();
                },
                FilesystemItem::File(data) => {
                    size += data.get_size();
                }
            }
        }

        size
    }

    pub fn get_directory(&mut self, name: String) -> Option<&mut Directory> {
        for item in &mut self.children {
            match item {
                FilesystemItem::Directory(data) => {
                    if data.name == name {
                        return Some(data);
                    }
                },
                FilesystemItem::File(_data) => {}
            }
        }

        return None
    }

    pub fn get_parent(&self) -> Option<&Directory> {
        unsafe { self.parent.unwrap().as_ref() }
    }

    pub fn get_mut_parent(&mut self) -> Option<&mut Directory> {
        unsafe { self.parent.unwrap().as_mut() }
    }
}

type Data = Directory;
impl DayImpl<Data> for Day<CURRENT_DAY> {
    fn init_test() -> (Self, Data) {
        Self::init(&include_str!("test_inputs/test07.txt").to_owned())
    }

    fn expected_results() -> (Answer, Answer) {
        (Answer::Number(95437), Answer::Number(0))
    }

    fn init(input: &str) -> (Self, Data) {
        
        let mut root_dir: Directory = Directory {
            parent: None,
            name: String::from(""),
            children: Vec::new()
        };

        let mut current_dir: &mut Directory = &mut root_dir;
        let mut is_ls_response: bool = false;

        for line in input.lines() {
            if line.starts_with("$") {
                is_ls_response = false;
                let command = line.to_string().replace("$ ", "");
                let mut split_command = command.split(" ");
                match split_command.next().unwrap() {
                    "cd" => {
                        match split_command.next().unwrap() {
                            "/" => {
                                current_dir = &mut root_dir;
                            },
                            ".." => {
                                current_dir = current_dir.get_mut_parent().unwrap();
                            },
                            dir => {
                                current_dir = current_dir.get_directory(dir.to_string()).expect("error while parsing input. directory is not defined.");
                            },
                        }
                    },
                    "ls" => {
                        is_ls_response = true;
                    },
                    _ => {},
                }
            }

            if is_ls_response {
                let mut data = line.split(" ");
                let size = data.next().unwrap();
                let name = data.next().unwrap();
                let item;

                if size == "dir" {
                    item = FilesystemItem::Directory(Directory { parent: Some(&mut *current_dir), name: String::from(name), children: Vec::new() });
                } else {
                    item = FilesystemItem::File(File { name: String::from(name), size: size.parse::<u64>().expect("error while parsing input.") });
                }
                current_dir.add_children(item);
            }
        }

        dprintln!("{:?}", root_dir);
        dprintln!("{:?}", root_dir.get_size());

        (
            Self {},
            root_dir,
        )
    }

    fn one(&self, data: &mut Data) -> Answer {
        let mut total_size: u64 = 0;

        for item in &data.children {
            match item {
                FilesystemItem::Directory(data) => {
                    if data.get_size() < 100_000 {
                        total_size += data.get_size();
                    }
                },
                FilesystemItem::File(_data) => {
                    // if data.get_size() < 100_000 {
                    //     total_size += data.get_size();
                    // }
                },
            }
        }

        Answer::Number(total_size as u64)
    }

    fn two(&self, data: &mut Data) -> Answer {
        Answer::Number(0 as u64)
    }
}
