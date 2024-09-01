use std::io::{self, BufRead};
use structopt::StructOpt;
use termion::color::{self, Color};

#[derive(StructOpt)]
struct Cli {
    /// The text to colorize
    text: Option<String>,
}

fn print_colored_text(text: &str) {
    let colors: [&dyn Color; 6] = [
        &color::Red,
        &color::Green,
        &color::Yellow,
        &color::Blue,
        &color::Magenta,
        &color::Cyan,
    ];

    for (i, ch) in text.chars().enumerate() {
        let color = &colors[i % colors.len()];
        print!("{}{}", color::Fg(*color), ch);
    }
    println!("{}", color::Fg(color::Reset));
}

fn main() {
    let args = Cli::from_args();

    if let Some(text) = args.text {
        print_colored_text(&text);
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            print_colored_text(&line.unwrap());
        }
    }
}