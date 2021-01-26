mod loader;

use loader::load;

use std::fs::File;
use std::io;
//use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

fn flush() {
    io::stdout().flush().expect("Error flushing");
}

fn get_input(text: &str) -> String {
    print!("{}", text);
    flush();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error getting input");
    input
}

fn convert_strings_to_big_string(content: Vec<String>) -> String {
    let mut big_string = String::new();
    for mut line in content.into_iter() {
        line.push_str("\n");
        big_string.push_str(line.as_str());
    }
    big_string
}

fn write_file(content: Vec<String>) {
    let big_string = convert_strings_to_big_string(content);
    if let Ok(file) = File::create("fake_grub") {
        let mut buffer = BufWriter::new(file);
        buffer
            .write_all(big_string.as_bytes())
            .expect("Error occured while writing to the file.");
        buffer.flush().expect("Error while flushing.")
    } else {
        println!("Error occured while loading the file.");
    }
}

// GRUB_CMDLINE_LINUX_DEFAULT
fn edit_parameters(input_vec: Vec<&str>, _br: &BufReader<File>) {
    print!("Added grub parameters ");
    let input_vec_size = input_vec.len() - 1 as usize;
    for (i, e) in input_vec.iter().enumerate() {
        if i < input_vec_size {
            print!("{}, ", *e);
        } else {
            print!("{}", *e);
        }
    }
    flush();
}

fn main() {
    if let Some(br) = load() {
        let input = get_input("Enter grub paramater you want to add: ");
        let input_vec: Vec<&str> = input.split(' ').collect();
        if input_vec.len() > 0 {
            edit_parameters(input_vec, &br);
        } else {
            println!("Error adding grub parameter");
        }
    } else {
        println!("Error: grub file not found at /etc/default/grub");
    }
}
