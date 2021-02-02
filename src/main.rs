mod loader;
mod edit;

use loader::load;
use edit::edit_parameters;

use std::io;
use std::io::BufRead;
use std::io::Write;
use termion::color;

fn flush() {
    io::stdout().flush().expect("Error flushing");
}

fn get_input(text: &str) -> String {
    print!("{}", text);
    flush();
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    if let Some(br) = load() {
        let input = get_input("Enter grub paramater you want to add: ");
        let input_vec: Vec<&str> = input.split(' ').collect();
        if input.len() > 0 {
            edit_parameters(input_vec, br);
        } else {
            println!("{}Error adding grub parameter", color::Fg(color::Red));
        }
    } else {
        println!("Error: grub file not found at /etc/default/grub");
    }
}
