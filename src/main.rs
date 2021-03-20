mod add;
mod loader;
mod remove;
mod write;

use loader::load;

use std::env;
use std::io;
use std::io::BufRead;
use std::io::Write;
use termion::color;

fn flush() {
    io::stdout().flush().expect("Error flushing");
}

pub fn print_error(text: &str) {
    println!("{}{}{}",
        color::Fg(color::Red),
        text,
        color::Fg(color::Reset)
    );
}

pub fn print_success(text: &str) {
    println!("{}{}{}",
        color::Fg(color::Green),
        text,
        color::Fg(color::Reset)
    );
}

pub fn get_input(text: &str) -> String {
    print!("{}", text);
    flush();
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    if let Some(br) = load() {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            print_error("Incorrect program parameter");
        } else {
            match args[1].as_str() {
                "add" => add::start(br),
                "remove" => remove::start(br),
                _ => print_error("Incorrect program parameter"),
            }
        }
    } else {
        print_error("Error: grub file not found at /etc/default/grub");
    }
}
