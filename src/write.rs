use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use termion::color;

fn convert_strings_to_big_string(content: Vec<String>) -> String {
    let mut big_string = String::new();
    for mut line in content.into_iter() {
        line.push_str("\n");
        big_string.push_str(line.as_str());
    }
    big_string
}

pub fn write_file(lines: Vec<String>) {
    let big_string = convert_strings_to_big_string(lines);
    if let Ok(file) = File::create("/etc/default/grub") {
        let mut buffer = BufWriter::new(file);
        buffer
            .write_all(big_string.as_bytes())
            .expect("Error occured while writing to a file.");
        buffer.flush().expect("Error while flushing.");
        println!("{}Successfully made changes.", color::Fg(color::Green));
    } else {
        println!("{}Error occured while creating the file.", color::Fg(color::Red));
    }
}
