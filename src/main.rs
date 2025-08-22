use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

const GRAY: &str = "\x1B[37m";
const RED: &str = "\x1B[91m";
const BRIGHT_RED: &str = "\x1B[92m";
const YELLOW: &str = "\x1B[93m";
const RESET: &str = "\x1B[0m";
const BOLD: &str = "\x1B[1m";
const BRIGHT_ORANGE: &str = "\x1b[38;2;235;158;127m";

const HIDE_CURSOR: &str = "\x1B[?25l";
const SHOW_CURSOR: &str = "\x1B[?25h";

fn word_progress() {
    let word = "Wizarding…";
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    for letter in 0..word_len {
        let position = letter % word_len;
        //let frame = word_chars[position];
        let left: String = word.chars().take(position).collect();
        let mid = word_chars[position];
        let right: String = word.chars().skip(position + 1).collect();
        let frame = format!("{}{}{}{}{}{}", RED, left, BRIGHT_ORANGE, mid, RED, right);
        println!("{}\n {}", CLEAR, frame);
        sleep(Duration::from_millis(300));
    }
}

fn progress() {
    let tasks = 50;
    let spinner = " ·•✤✻✶✼✽❃✹✺✹❇✶❈❉❊✤✻✼✽❃✶✺✹❇❈❉❊✤•· ";
    let spinner_chars: Vec<char> = spinner.chars().collect();
    let spinner_len = spinner_chars.len();

    println!("{HIDE_CURSOR}");
    for t in 0..tasks {
        let frame = spinner_chars[t % spinner_len];
        println!("{}\n      {}{} Wizarding… ", CLEAR, RED, frame);
        sleep(Duration::from_millis(250));
    }
    println!("{SHOW_CURSOR}");
}

fn main() {
    word_progress();
}
