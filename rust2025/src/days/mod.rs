use aoc_macro::mod_days;
use std::time::{Duration, Instant};

// Thanks to andi-makes with his AoC project https://github.com/andi-makes/aoc2021,
// this system is heavily inspired by his system.

pub struct Day<const DAY: u8>;

#[derive(Debug, Clone, PartialEq)]
pub enum Answer {
    Number(u64),
    String(String),
    Bitmap(Vec<Vec<bool>>),
}

impl Answer {
    fn append_per_line(str: String, prefix: &str) -> String {
        str.lines()
            .map(|v| prefix.to_owned() + v + "\n")
            .collect::<String>()
            .strip_suffix('\n')
            .unwrap()
            .to_owned()
    }

    fn bm_get(bm: &[Vec<bool>], x: usize, y: usize) -> bool {
        if let Some(line) = bm.get(y) {
            if let Some(v) = line.get(x) {
                return *v;
            }
            return false;
        }
        false
    }

    fn minify_bitmap(bm: &Vec<Vec<bool>>) -> String {
        let height = bm.len();
        let width = bm[0].len();

        let mut out = String::new();

        for y in (0..height).step_by(2) {
            if y != 0 {
                out += "\n"
            }
            for x in (0..width).step_by(2) {
                let m = (
                    Self::bm_get(bm, x, y),
                    Self::bm_get(bm, x + 1, y),
                    Self::bm_get(bm, x, y + 1),
                    Self::bm_get(bm, x + 1, y + 1),
                );

                out += match m {
                    (false, false, false, false) => " ",
                    (true, false, false, false) => "▘",
                    (false, true, false, false) => "▝",
                    (true, true, false, false) => "▀",
                    (false, false, true, false) => "▖",
                    (true, false, true, false) => "▌",
                    (false, true, true, false) => "▞",
                    (true, true, true, false) => "▛",
                    (false, false, false, true) => "▗",
                    (true, false, false, true) => "▚",
                    (false, true, false, true) => "▐",
                    (true, true, false, true) => "▜",
                    (false, false, true, true) => "▄",
                    (true, false, true, true) => "▙",
                    (false, true, true, true) => "▟",
                    (true, true, true, true) => "█",
                }
            }
        }

        Self::append_per_line(out, "\t\t")
    }
}

impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "{}", s),
            Self::Bitmap(bm) => {
                writeln!(f).unwrap();
                write!(f, "{}", Self::minify_bitmap(bm))
            }
        }
    }
}

impl From<u64> for Answer {
    fn from(n: u64) -> Self {
        Self::Number(n)
    }
}

impl From<String> for Answer {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

pub trait DayImpl<T>
where
    T: Clone,
{
    /// Parses the test input.
    fn init_test() -> (Self, T)
    where
        Self: Sized;

    fn expected_results() -> (Answer, Answer);

    /// Parse input
    fn init(input: &str) -> (Self, T)
    where
        Self: Sized;

    /// Compute part 1
    fn one(&self, data: &mut T) -> Answer;

    /// Compue part 2
    fn two(&self, data: &mut T) -> Answer;

    /// Parse input and messure the time it took
    fn init_timed(input: &str) -> ((Self, T), Duration)
    where
        Self: Sized,
    {
        let s = Instant::now();
        (Self::init(input), s.elapsed())
    }

    /// Compute part 1 and messure the time it took
    fn one_timed(&self, data: &mut T) -> (Answer, Duration) {
        let s = Instant::now();
        (self.one(data), s.elapsed())
    }

    /// Compute part 2 and messure the time it took
    fn two_timed(&self, data: &mut T) -> (Answer, Duration) {
        let s = Instant::now();
        (self.two(data), s.elapsed())
    }

    /// Compute both parts
    fn run(input: &str) -> (Answer, Answer)
    where
        Self: Sized,
    {
        let (day, mut data) = Self::init(input);
        (day.one(&mut data.clone()), day.two(&mut data))
    }

    /// Init and compute part 1
    fn run_one(input: &str) -> Answer
    where
        Self: Sized,
    {
        let (day, mut data) = Self::init(input);
        day.one(&mut data)
    }

    /// Init and compute part 1
    fn run_two(input: &str) -> Answer
    where
        Self: Sized,
    {
        let (day, mut data) = Self::init(input);
        day.two(&mut data)
    }

    /// Init and compute part 1
    fn run_one_timed(input: &str) -> (Answer, Duration, Duration)
    where
        Self: Sized,
    {
        let ((day, mut data), init_t) = Self::init_timed(input);
        let (one, one_t) = day.one_timed(&mut data);
        (one, init_t, one_t)
    }

    /// Init and compute part 1
    fn run_two_timed(input: &str) -> (Answer, Duration, Duration)
    where
        Self: Sized,
    {
        let ((day, mut data), init_t) = Self::init_timed(input);
        let (two, two_t) = day.two_timed(&mut data);
        (two, init_t, two_t)
    }

    /// Compute both parts, and messure the time each step took
    fn run_timed(input: &str) -> (Answer, Answer, Duration, Duration, Duration)
    where
        Self: Sized,
    {
        let ((day, mut data), i_t) = Self::init_timed(input);
        let (one, one_t) = day.one_timed(&mut data.clone());
        let (two, two_t) = day.two_timed(&mut data);

        (one, two, i_t, one_t, two_t)
    }

    /// Test part one
    fn test_one() -> (bool, Answer, Answer)
    where
        Self: Sized,
    {
        let (day, mut data) = Self::init_test();
        let one = day.one(&mut data);

        let (one_e, _) = Self::expected_results();

        (one_e == one, one, one_e)
    }

    /// Test part two
    fn test_two() -> (bool, Answer, Answer)
    where
        Self: Sized,
    {
        let (day, mut data) = Self::init_test();
        let two = day.two(&mut data);

        let (_, two_e) = Self::expected_results();

        (two_e == two, two, two_e)
    }

    /// Run both tests
    fn test() -> ((bool, Answer, Answer), (bool, Answer, Answer))
    where
        Self: Sized,
    {
        let (day, mut data) = Self::init_test();
        let one = day.one(&mut data.clone());
        let two = day.two(&mut data);

        let (one_e, two_e) = Self::expected_results();

        ((one_e == one, one, one_e), (two_e == two, two, two_e))
    }
}

mod_days!();
