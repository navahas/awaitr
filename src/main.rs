use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

const RESET: &str = "\x1B[0m";
const BOLD: &str = "\x1B[1m";
const REGULAR: &str = "\x1b[22m";

const GRAY: &str = "\x1B[38;2;136;136;136m";
const RED: &str = "\x1B[91m";
const BRIGHT_ORANGE: &str = "\x1b[38;2;235;158;127m";
const GREEN: &str = "\x1b[38;2;163;190;140m";
const BRIGHT_GREEN: &str = "\x1b[38;2;186;210;157m";
const BLUE: &str = "\x1b[38;2;174;198;207m";
const BRIGHT_BLUE: &str = "\x1b[38;2;167;199;231m";
const YELLOW: &str = "\x1b[38;2;241;216;165m";
const BRIGHT_YELLOW: &str = "\x1b[38;2;240;223;174m";

const HIDE_CURSOR: &str = "\x1B[?25l";
const SHOW_CURSOR: &str = "\x1B[?25h";

fn word_loader(word: &str, color: &str, h_color: &str, frame: usize) -> String {
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    let w_position = frame % word_len;
    let left: String = word.chars().take(w_position).collect();
    let mid = word_chars[w_position];
    let right: String = word.chars().skip(w_position + 1).collect();
    let word_frame = format!("{color}{left}{h_color}{BOLD}{mid}{REGULAR}{color}{right}");
    word_frame
}

fn dummy_progress() {
    let tasks = 100;

    let spinner = " ·•✤✻✶✼✽❃✹✺✹❇✶✻✤•·•✤❈❉❊✤✻✼✽❃✶✺✹❇❈❉❊✤•· ";
    let spinner_chars: Vec<char> = spinner.chars().collect();
    let spinner_len = spinner_chars.len();


    println!("{HIDE_CURSOR}");
    for t in 0..tasks {
        let spinner_frame = spinner_chars[t % spinner_len];

        let rustifying = word_loader("Rustifying…", RED, BRIGHT_ORANGE, t);
        let compiling = word_loader("Compiling…", GREEN, BRIGHT_GREEN, t);
        let borrowchecking = word_loader("Borrowchecking…", BLUE, BRIGHT_BLUE, t);
        let working = word_loader("Working…", YELLOW, BRIGHT_YELLOW, t);

        println!("{}\n", CLEAR);
        println!(
            "{:width$}{}{} {} {}",
            "", RED, spinner_frame, rustifying, RESET, width = 11
        );

        println!("\n");
        println!(
            "{:width$}{}{} {} {}",
            "", GREEN, spinner_frame, compiling, RESET, width = 11
        );

        println!("\n");
        println!(
            "{:width$}{}{} {} {}",
            "", BLUE, spinner_frame, borrowchecking, RESET, width = 11
        );

        println!("\n");
        println!(
            "{:width$}{}{} {} {}",
            "", YELLOW, spinner_frame, working, RESET, width = 11
        );

        println!("\n");
        println!(
            "{:width$}{}(press crtl+C to exit){}",
            "", GRAY, RESET, width = 11
        );

        sleep(Duration::from_millis(225));
    }
    println!("{SHOW_CURSOR}");
}

fn main() {
    dummy_progress();
}
