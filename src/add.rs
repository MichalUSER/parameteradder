use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::color;

use crate::{get_input, flush, print_error};
use crate::write::write_file;

fn print_parameters(input_vec: Vec<&str>) {
    let input_vec_size = input_vec.len() - 1 as usize;
    for (i, e) in input_vec.iter().enumerate() {
        if i < input_vec_size {
            print!("{}, ", e);
        } else {
            print!("{}\n", e);
        }
    }
}

pub fn start(br: BufReader<File>) {
    let input = get_input("Enter grub paramater you want to add: ");
    let input_vec: Vec<&str> = input.split(' ').collect();
    if input.len() > 0 {
        add_parameters(input_vec, br);
    } else {
        print_error("Error adding grub parameter");
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
        print_error("Error adding grub parameters");
    } else {
        print!("{}Added grub parameters ", color::Fg(color::Green));
        print_parameters(input_vec);
        print!("{}", color::Fg(color::Reset));
        flush();
    }
}
