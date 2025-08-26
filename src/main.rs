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

fn word_loader(word: &str, color: &str, h_color: &str, mut frame: usize) -> String {
    if frame < 10 || word.chars().count() == 0 {
        return format!("{color}{word}");
    }
    frame -= 10;

    let m_range: usize = 3;

    // pad with m_range spaces so the snake can fully fade out off the right edge.
    let s_word = format!("{word}{}", " ".repeat(m_range));
    let word_len = word.chars().count();
    let len = s_word.chars().count();

    let head = frame * 2 % len;

    let end = (head + 1).min(word_len);
    let start = if head + 1 >= m_range {
        head + 1 - m_range
    } else {
        0
    };

    // might no be strictly needed
    // for window empty (we're entirely in padding), show the plain word
    if start >= end {
        return format!("{color}{word}");
    }

    let left: String = word.chars().take(start).collect();
    let mid: String = word.chars().skip(start).take(end - start).collect();
    let right: String = word.chars().skip(end).collect();

    format!("{color}{left}{h_color}{BOLD}{mid}{REGULAR}{color}{right}")
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
    let tasks = 2000;

    let default_spinner = " ·•✤✻✶✼✽❃✹✺✹❇✶✻✤•·•✤❈❉❊✤✻✼✽❃✶✺✹❇❈❉❊✤•· ";
    let spinner_a = "⣾⣽⣻⢿⡿⣟⣯⣷";
    let spinner_b = " ▏▎▍▌▋▊▉▉▉▉▉▊▊▋▌▍▎";
    let spinner_c = " ▁▂▃▄▅▆▇████▇▆▅▅▆▇█████▇▆▅▅▆▇████▇▆▅▄▃▂▁";

    println!("{HIDE_CURSOR}");
    for t in 0..tasks {
        let rustifying = word_loader("Rustifying…", RED, BRIGHT_RED, t);
        let s_rustifying = spinner_loader(default_spinner, RED, t * 5);

        let compiling = word_loader("Compiling…", GREEN, BRIGHT_GREEN, t);
        let s_compiling = spinner_loader(spinner_a, GREEN, t / 3);

        let borrowchecking = word_loader("Borrowchecking…", BLUE, BRIGHT_BLUE, t);
        let s_borrowchecking = spinner_loader(spinner_b, BLUE, t);

        let working = word_loader("Working…", YELLOW, BRIGHT_YELLOW, t);
        let s_working = spinner_loader(spinner_c, YELLOW, t * 2);

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
