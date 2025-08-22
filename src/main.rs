use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

const GRAY: &str = "\x1B[38;2;136;136;136m";
const RED: &str = "\x1B[91m";
const RESET: &str = "\x1B[0m";
const BOLD: &str = "\x1B[1m";
const REGULAR: &str = "\x1b[22m";
const BRIGHT_ORANGE: &str = "\x1b[38;2;235;158;127m";

const HIDE_CURSOR: &str = "\x1B[?25l";
const SHOW_CURSOR: &str = "\x1B[?25h";

fn dummy_progress() {
    let tasks = 100;

    let word = "Rustifying…";
    let spinner = " ·•✤✻✶✼✽❃✹✺✹❇✶✻✤•·•✤❈❉❊✤✻✼✽❃✶✺✹❇❈❉❊✤•· ";

    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();

    let spinner_chars: Vec<char> = spinner.chars().collect();
    let spinner_len = spinner_chars.len();

    println!("{HIDE_CURSOR}");
    for t in 0..tasks {
        let spinner_frame = spinner_chars[t % spinner_len];

        let w_position = t % word_len;
        let left: String = word.chars().take(w_position).collect();
        let mid = word_chars[w_position];
        let right: String = word.chars().skip(w_position + 1).collect();
        let word_frame = format!("{RED}{left}{BRIGHT_ORANGE}{BOLD}{mid}{REGULAR}{RED}{right}");

        println!("{}\n", CLEAR);
        println!("      {}{} {} {}    (press ctrl+C to strop){}", RED, spinner_frame, word_frame, GRAY, RESET);
        println!("\n");
        println!("      {}{} {} {}    (press ctrl+C to strop){}", RED, spinner_frame, word_frame, GRAY, RESET);
        sleep(Duration::from_millis(225));
    }
    println!("{SHOW_CURSOR}");
}

fn main() {
    dummy_progress();
}
