use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io;
use std::io::Write;
use termion::color;

fn flush() {
    io::stdout().flush().expect("Error flushing");
}

fn convert_strings_to_big_string(content: Vec<String>) -> String {
    let mut big_string = String::new();
    for mut line in content.into_iter() {
        line.push_str("\n");
        big_string.push_str(line.as_str());
    }
    big_string
}

fn write_file(lines: Vec<String>) {
    let big_string = convert_strings_to_big_string(lines);
    if let Ok(file) = File::create("fake_grub") {
        let mut buffer = BufWriter::new(file);
        buffer
            .write_all(big_string.as_bytes())
            .expect("Error occured while writing to a file.");
        buffer.flush().expect("Error while flushing.")
    } else {
        println!("Error occured while loading the file.");
    }
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

pub fn edit_parameters(input_vec: Vec<&str>, br: BufReader<File>) {
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
