use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

const RESET: &str = "\x1B[0m";
const BOLD: &str = "\x1B[1m";
const REGULAR: &str = "\x1b[22m";

const GRAY: &str = "\x1B[38;2;136;136;136m";
const RED: &str = "\x1B[91m";
const BRIGHT_RED: &str = "\x1b[38;2;235;158;127m";
const GREEN: &str = "\x1b[38;2;163;190;140m";
const BRIGHT_GREEN: &str = "\x1b[38;2;186;210;157m";
const BLUE: &str = "\x1b[38;2;174;198;207m";
const BRIGHT_BLUE: &str = "\x1b[38;2;224;233;236m";
const YELLOW: &str = "\x1b[38;2;241;216;165m";
const BRIGHT_YELLOW: &str = "\x1b[38;2;240;223;174m";

const HIDE_CURSOR: &str = "\x1B[?25l";
const SHOW_CURSOR: &str = "\x1B[?25h";
const FPMS: u64 = 218;

fn fpms() {
    sleep(Duration::from_millis(FPMS));
}

fn word_loader(word: &str, color: &str, h_color: &str, frame: usize) -> String {
    let w_position = frame % word.len();
    let range = 2;
    let w_range = w_position + range;

    let end_bound = if w_range > word.len() {
        word.len()
    } else {
        w_range
    };

    let left = word.chars().take(w_position).collect::<String>();
    let (mid, right) = if w_position == 0 {
        let m = word.chars().take(1).collect::<String>();
        let r = word.chars().skip(1).collect::<String>();
        (m, r)
    } else {
        let m = word.chars()
            .skip(w_position)
            .take(range)
            .collect::<String>();
        let r = word.chars().skip(end_bound).collect::<String>();
        (m, r)
    };
    let word_frame = format!(
        "{color}{left}{h_color}{BOLD}{mid}{REGULAR}{color}{right}"
    );
    //let word_frame = format!(
    //    "{w_position} {left} {mid} -{color}{left}{h_color}{BOLD}{mid}{REGULAR}{color}{right}"
    //);
    word_frame
}

fn spinner_loader(icons: &str, color: &str, frame: usize) -> String {
    let spinner_chars: Vec<char> = icons.chars().collect();
    let spinner_frame = spinner_chars[frame % spinner_chars.len()];
    let spinner = format!("{color}{spinner_frame}");
    spinner
}

fn cast_loader(spinner: String, word: String) {
    println!("\n");
    println!("{:width$}{} {} {}", "", spinner, word, RESET, width = 11);
}

fn dummy_progress() {
    let tasks = 100;

    let default_spinner = " ·•✤✻✶✼✽❃✹✺✹❇✶✻✤•·•✤❈❉❊✤✻✼✽❃✶✺✹❇❈❉❊✤•· ";
    let spinner_a = "⣾⣽⣻⢿⡿⣟⣯⣷";
    let spinner_b = " ▏▎▍▌▋▊▉▉▉▉▉▊▊▋▌▍▎";
    let spinner_c = " ▁▂▃▄▅▆▇████▇▇▆▅▄▃▁ ";

    println!("{HIDE_CURSOR}");
    for t in 0..tasks {
        let rustifying = word_loader("Rustifying…", RED, BRIGHT_RED, t);
        let s_rustifying = spinner_loader(default_spinner, RED, t);

        let compiling = word_loader("Compiling…", GREEN, BRIGHT_GREEN, t);
        let s_compiling = spinner_loader(spinner_a, GREEN, t);

        let borrowchecking = word_loader("Borrowchecking…", BLUE, BRIGHT_BLUE, t);
        let s_borrowchecking = spinner_loader(spinner_b, BLUE, t);

        let working = word_loader("Working…", YELLOW, BRIGHT_YELLOW, t);
        let s_working = spinner_loader(spinner_c, YELLOW, t);

        println!("{}", CLEAR);

        let _rusty_progress = cast_loader(s_rustifying, rustifying);
        let _compiling_progress = cast_loader(s_compiling, compiling);
        let _borrowchecking_progress = cast_loader(s_borrowchecking, borrowchecking);
        let _working_progress = cast_loader(s_working, working);

        println!("\n");
        println!(
            "{:width$}{}(press crtl+C to exit){}",
            "",
            GRAY,
            RESET,
            width = 11
        );

        fpms();
    }
    println!("{SHOW_CURSOR}");
}

fn main() {
    dummy_progress();
}
