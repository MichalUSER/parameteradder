use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
//use std::io;
//use std::io::Write;
use termion::color;

use crate::write::write_file;
use crate::get_input;

fn print_parameters(params: &Vec<&str>) {
    let mut counter = 0;
    for p in params.iter() {
        println!("{}{}:{} {}", color::Fg(color::Red), counter, color::Fg(color::Reset), p);
        counter += 1;
    }
}

fn ask_user(params: &Vec<&str>) -> Option<Vec<Option<usize>>> {
    print_parameters(params);
    let i = get_input("Which ones do you want to delete? ");
    let split: Vec<&str> = i.split(' ').collect();
    let mut error = false;
    if split.len() > 0 {
        let nums: Vec<Option<usize>> = split.into_iter().map(|e| {
            if let Ok(e) = e.parse() { Some(e) }
            else {
                error = true;
                None
            }
        }).collect();
        if error { None }
        else { Some(nums) }
    } else {
        None
    }
}

pub fn start(br: BufReader<File>) {
    let mut lines: Vec<String> = Vec::new();
    let mut error = true;
    for eresult in br.lines().into_iter() {
        let e = eresult.unwrap();
        let split: Vec<&str> = e.split('=').collect();
        let line;
        if split.len() > 1 {
            if split[0] == "GRUB_CMDLINE_LINUX_DEFAULT" {
                error = false;
                let mut right_side = split[1..].join("=").to_string();
                let r_len = right_side.len() - 1;
                right_side.drain(0..1);
                right_side.drain(r_len-1..r_len);
                let mut params: Vec<&str> = right_side.split(' ').collect();
                if let Some(v) = ask_user(&params) {
                    for n in v.into_iter() {
                        if let Some(n) = n {
                            params.remove(n);
                        }
                        else { error = true; }
                    }
                } else { error = true; }
                line = format!("GRUB_CMDLINE_LINUX_DEFAULT={}{}{}", '"', params.join(" ").as_str(), '"');
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
        println!("{}Error removing grub parameters", color::Fg(color::Red));
    } else {
        println!("{}Successfully removed grub parameters", color::Fg(color::Green));
    }
}
