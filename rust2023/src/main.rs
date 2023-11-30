use aoc23::{run_day, set_verbosity, test_day, Part, Verbosity};
use clap::{App, AppSettings, Arg, SubCommand};
use colored::*;
use reqwest::blocking::Client;
use reqwest::cookie::Jar;
use reqwest::header::USER_AGENT;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;

// NOTE: Since this CLI was coded using clap in december of 2021, there seem to
// have been huge breaking changes, requiring me to rewrite the CLI completely
// to be able to update Clap. As I am on a limited time schedule RN, I will
// probably do this later on.
// 2023-11-29: still haven't done this, maybe I'll do is this year.
// TODO: Rewrite CLI and update Clap

fn main() {
    let matches = App::new("Advent Of Code 2023")
        .author("LeMoonStar <webmaster@unitcore.de>")
        .about("My Advent Of Code 2023 solutions.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("day")
                .help("The number of the day to execute")
                .required(true)
                .takes_value(true)
                .validator(|v| match v.parse::<u8>() {
                    Ok(day) => {
                        if 0 < day && day <= 25 {
                            Ok(())
                        } else {
                            Err("The day must be between 1 and 25.".to_string())
                        }
                    }
                    Err(_) => Err("The day must be a number between 1 and 25.".to_string()),
                }),
        )
        .arg(
            Arg::with_name("part")
                .help("Specifies the part of the day to compute.")
                .long("part")
                .short("p")
                .default_value("b")
                .possible_values(&["1", "2", "b"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Print verbose information")
                .long("verbose")
                .short("v")
                .conflicts_with("development"))
        .arg(
            Arg::with_name("development")
                .help("Print development information")
                .long("dev")
                .short("d")
                .conflicts_with("verbose"))
        .subcommand(
            SubCommand::with_name("test").about("Test the day with the example input data."),
        )
        .subcommand(
            SubCommand::with_name("auto")
                .about("Automatically download input from AoC using the provided session and run the solution.")
                .arg(Arg::with_name("session")
                    .help("The AoC browser session string. If not provided, uses the AOC_SESSION environment variable.")
                    .short("s")
                    .long("session")
                    .takes_value(true))
                .arg(Arg::with_name("no_cache")
                    .help("Don't cache the input, and delete any current cache for this day.")
                    .short("N")
                    .long("no-cache")))
        .subcommand(
            SubCommand::with_name("run")
                .about("Use either a file or stdin as input and run the solution.")
                .arg(Arg::with_name("file")
                    .help("Specify a file to be used as input, otherwise use stdin.")
                    .short("f")
                    .long("file")
                    .takes_value(true)
            )
        )
        .get_matches();

    if cfg!(debug_assertions) {
        println!(
            "{}",
            "This binary was built in debug mode. To improve performance, please add --release to the build command."
                .red()
                .bold()
        );
    }

    let day = matches
        .value_of("day")
        .unwrap()
        .parse::<u8>()
        .expect("Failed to parse day argument.");

    let part: Part = match matches.value_of("part") {
        Some("1") => Part::One,
        Some("2") => Part::Two,
        Some("b") => Part::Both,
        _ => panic!("unexpected part argument."),
    };

    if matches.args.contains_key("verbose") {
        set_verbosity(Verbosity::Verbose);
    }

    if matches.args.contains_key("development") {
        set_verbosity(Verbosity::Development);
    }

    match matches.subcommand() {
        ("run", c_matches) => {
            let input = match c_matches {
                Some(c_matches) => {
                    if let Some(f) = c_matches.value_of("file") {
                        fs::read_to_string(Path::new(f)).expect("Error while reading input file")
                    } else {
                        get_stdin_day_input(day)
                    }
                }
                None => get_stdin_day_input(day),
            };
            run_day(day, part, &input);
        }
        ("auto", c_matches) => {
            let session: String = match c_matches {
                Some(c_matches) => match c_matches.value_of("session") {
                    Some(v) => v.to_owned(),
                    None => env::var("AOC_SESSION").expect("Neither a session argument nor the AOC_SESSION environment variable were provided."),
                },
                None => env::var("AOC_SESSION").expect("Neither a session argument nor the AOC_SESSION environment variable were provided."),
            };
            let cache = if let Some(c_matches) = c_matches {
                !c_matches.args.contains_key("no_cache")
            } else {
                true
            };

            let input = get_auto_input(day, &session, cache);
            run_day(day, part, &input);
        }
        ("test", _) => {
            if !test_day(day, part) {
                std::process::exit(1);
            }
        }
        _ => panic!("Unexpected Subcommand."),
    }
}

fn get_stdin_day_input(day: u8) -> String {
    let mut input = String::new();
    let stdin = std::io::stdin();

    println!(
        "Please paste your input for day {}, and then press {}",
        day,
        match cfg!(windows) {
            true => "CTRL-Z",
            _ => "CTRL-D",
        }
    );

    loop {
        match stdin.read_line(&mut input) {
            Ok(l) => {
                if l == 0 {
                    break;
                }
            }
            Err(err) => panic!("Error encountered while trying to read stdin: {}", err),
        }
    }

    input
}

fn download_input(day: u8, session: &String) -> Result<String, reqwest::Error> {
    println!("Downloading input for day {}", day);

    let cookie_jar = Jar::default();
    cookie_jar.add_cookie_str(
        format!("session={}; Domain=adventofcode.com", session).as_ref(),
        &"https://adventofcode.com/".parse::<reqwest::Url>().unwrap(),
    );
    let client = Client::builder()
        .https_only(true)
        .cookie_provider(Arc::new(cookie_jar))
        .build()?;

    let response = client
        .get(format!("https://adventofcode.com/2023/day/{}/input", day))
        .header(
            USER_AGENT,
            "https://github.com/daanbreur/AdventofCode by daan@daanbreur.systems",
        )
        .send()?;

    if !response.status().is_success() {
        panic!("Server error or invalid session.");
    }

    response.text()
}

fn get_auto_input(day: u8, session: &String, cache: bool) -> String {
    let cache_str = &format!("./.aoc23_cache/input{:02}.txt", day);
    let cache_path: &Path = Path::new(cache_str);
    match cache {
        true => match fs::read_to_string(cache_path) {
            Ok(input) => input,
            Err(_) => match download_input(day, session) {
                Ok(input) => {
                    let _ = fs::create_dir(Path::new("./.aoc23_cache"));
                    match fs::write(cache_path, &input) {
                        Ok(_) => {}
                        Err(err) => println!("Warning! couldn't save input cache!{:?}", err),
                    }
                    input
                }
                Err(err) => {
                    panic!("Error while downloading input: {:?}", err);
                }
            },
        },
        false => {
            let _ = fs::remove_file(cache_path);
            match download_input(day, session) {
                Ok(input) => input,
                Err(err) => {
                    panic!("Error while downloading input: {:?}", err);
                }
            }
        }
    }
}
