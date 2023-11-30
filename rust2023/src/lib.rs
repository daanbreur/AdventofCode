// Nightly features
// #![feature(int_abs_diff)]

use crate::days::Answer;
use crate::days::Day;
use crate::days::DayImpl;
use aoc_macro::*;
use colored::*;
use lazy_static::lazy_static;
use mut_static::MutStatic;
use std::time::Duration;

mod days;

#[derive(Debug, Clone, PartialEq)]
pub enum Verbosity {
    None,
    Verbose,
    Developement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Part {
    One,
    Two,
    Both,
}

#[derive(Debug, Clone)]
pub struct Settings {
    verbosity: Verbosity,
}

impl Settings {
    fn set_verbosity(&mut self, new: Verbosity) {
        self.verbosity = new;
    }
}

lazy_static! {
    static ref VERBOSITY: MutStatic<Settings> = MutStatic::from(Settings {
        verbosity: Verbosity::None
    });
}

pub fn set_verbosity(new_verbosity: Verbosity) {
    let mut handle = VERBOSITY.write().unwrap();
    handle.set_verbosity(new_verbosity);
}

pub fn get_verbosity() -> Verbosity {
    VERBOSITY.read().unwrap().verbosity.clone()
}

#[macro_export]
macro_rules! vprint {
    ($($arg:tt)*) => {
        if $crate::get_verbosity() ==  $crate::Verbosity::Verbose || $crate::get_verbosity() ==  $crate::Verbosity::Developement {
            print!(
                $($arg)*
            )
        }
    };
}

#[macro_export]
macro_rules! vprintln {
    ($($arg:tt)*) => {
        if $crate::get_verbosity() ==  $crate::Verbosity::Verbose || $crate::get_verbosity() ==  $crate::Verbosity::Developement {
            println!(
                $($arg)*
            )
        }
    };
}

#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => {
        if $crate::get_verbosity() == $crate::Verbosity::Developement  {
            print!(
                $($arg)*
            )
        }
    };
}

#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => {
        if $crate::get_verbosity() == $crate::Verbosity::Developement  {
            println!(
                $($arg)*
            )
        }
    };
}

fn dynamic_range_time_format(d: &Duration) -> String {
    let nanos = d.as_nanos();

    if nanos < 1000 {
        // less than one microsecond
        format!("{} ns", nanos)
    } else if nanos < 100000 {
        // less than 10 microseconds
        format!("{:.3} µs", nanos as f64 / 1000.0)
    } else if nanos < 1000000 {
        // less than one millisecond
        format!("{} µs", nanos / 1000)
    } else if nanos < 10000000 {
        // less than 10 milliseconds
        format!("{:.3} ms", nanos as f64 / 1000000.0)
    } else if nanos < 1000000000 {
        // less than a second
        format!("{} ms", nanos / 1000000)
    } else if nanos < 10000000000 {
        // less than 10 seconds
        format!("{:.3} s", nanos as f64 / 1000000000.0)
    } else {
        // more than 10 seconds
        format!("{} s", nanos / 1000000000)
    }
}

pub fn run_day(day: u8, part: Part, input: &String) {
    println!("{} Day {}", "Starting".green().bold(), day);
    println!("{}", "-----------------------".green().bold());
    let (one, two, init_t, one_t, two_t) = match part {
        Part::Both => match_and_run_day_both!(),
        Part::One => {
            let (one, init_t, one_t) = match_and_run_day_one!();
            (one, Answer::Number(0), init_t, one_t, Duration::ZERO)
        }
        Part::Two => {
            let (two, init_t, two_t) = match_and_run_day_two!();
            (Answer::Number(0), two, init_t, Duration::ZERO, two_t)
        }
    };

    println!("{}:", "Results".green().bold());
    println!(
        "\t{}: {}",
        "Parsing time".green(),
        dynamic_range_time_format(&init_t).bold().blue()
    );
    if part == Part::Both || part == Part::One {
        println!("\t{}:", "Part 1".green());
        println!("\t\tSolution: {}", format!("{}", one).bold().blue());
        println!(
            "\t\tTook:     {}",
            dynamic_range_time_format(&one_t).bold().blue()
        );
    }
    if part == Part::Both || part == Part::Two {
        println!("\t{}:", "Part 2".green());
        println!("\t\tSolution: {}", format!("{}", two).bold().blue());
        println!(
            "\t\tTook:     {}",
            dynamic_range_time_format(&two_t).bold().blue()
        );
    }
}

pub fn test_day(day: u8, part: Part) -> bool {
    println!("{} Day {}", "Testing".green().bold(), day);
    println!("{}", "-----------------------".green().bold());
    match part {
        Part::Both => {
            let ((one_p, one_r, one_e), (two_p, two_r, two_e)) = match_and_test_day_both!();

            println!("{}:", "Results".green().bold());
            println!(
                "\t{}: {}",
                "Part 1".green(),
                match one_p {
                    true => {
                        "PASSED".green().bold()
                    }
                    false => {
                        "FAILED".red().bold()
                    }
                }
            );
            println!("\t\tResult:   {}", format!("{}", one_r).bold().blue());
            println!("\t\tExpected: {}", format!("{}", one_e).bold().blue());

            println!(
                "\t{}: {}",
                "Part 2".green(),
                match two_p {
                    true => {
                        "PASSED".green().bold()
                    }
                    false => {
                        "FAILED".red().bold()
                    }
                }
            );
            println!("\t\tResult:   {}", format!("{}", two_r).bold().blue());
            println!("\t\tExpected: {}", format!("{}", two_e).bold().blue());

            !(!two_p || !one_p)
        }
        Part::One => {
            let (one_p, one_r, one_e) = match_and_test_day_one!();

            println!("{}:", "Results".green().bold());
            println!(
                "\t{}: {}",
                "Part 1".green(),
                match one_p {
                    true => {
                        "PASSED".green().bold()
                    }
                    false => {
                        "FAILED".red().bold()
                    }
                }
            );
            println!("\t\tResult:   {}", format!("{}", one_r).bold().blue());
            println!("\t\tExpected: {}", format!("{}", one_e).bold().blue());

            !one_p
        }
        Part::Two => {
            let (two_p, two_r, two_e) = match_and_test_day_two!();

            println!("{}:", "Results".green().bold());
            println!(
                "\t{}: {}",
                "Part 2".green(),
                match two_p {
                    true => {
                        "PASSED".green().bold()
                    }
                    false => {
                        "FAILED".red().bold()
                    }
                }
            );
            println!("\t\tResult:   {}", format!("{}", two_r).bold().blue());
            println!("\t\tExpected: {}", format!("{}", two_e).bold().blue());

            !two_p
        }
    }
}