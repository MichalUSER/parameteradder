use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io;
use std::io::Write;
use termion::color;

use crate::write::write_file;
use crate::get_input;

// TODO
// Refactor code
// Make it look and feel nicer
// Add a README.md

fn flush() {
    io::stdout().flush().expect("Error flushing");
}

fn print_parameters(input_vec: Vec<&str>) {
    let input_vec_size = input_vec.len() - 1 as usize;
    for (i, e) in input_vec.iter().enumerate() {
        if i < input_vec_size {
            print!("{}, ", e);
        } else {
            print!("{}\n", e);
        }
    }
    flush();
}

pub fn start(br: BufReader<File>) {
        let input = get_input("Enter grub paramater you want to add: ");
        let input_vec: Vec<&str> = input.split(' ').collect();
        if input.len() > 0 {
            add_parameters(input_vec, br);
        } else {
            println!("{}Error adding grub parameter", color::Fg(color::Red));
        }
}

fn add_parameters(input_vec: Vec<&str>, br: BufReader<File>) {
    let mut lines: Vec<String> = Vec::new();
    let mut error = true;
    for eresult in br.lines().into_iter() {
        let e = eresult.unwrap();
        let split: Vec<&str> = e.split('=').collect();
        let mut line = String::new();
        if split.len() > 1 {
            if split[0] == "GRUB_CMDLINE_LINUX_DEFAULT" {
                error = false;
                line.push_str(split[0..].join("=").as_str());
                line.pop();
                line = format!("{} {}{}", line, input_vec.join(" "), '"');
            } else {
                line = split.join("=");
            }
        } else {
            line = e;
        }
        lines.push(line);
    }
    write_file(lines);
    if error {
        println!("{}Error adding grub parameters", color::Fg(color::Red));
    } else {
        print!("{}Added grub parameters ", color::Fg(color::Green));
        print_parameters(input_vec);
    }
}
