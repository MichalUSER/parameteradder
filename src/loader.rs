use std::fs::File;
use std::io::BufReader;

// check if path exists
//fn path_exists(path: &str) -> bool {
//fs::metadata(path).is_ok()
//}

//pub fn load() -> Option<String> {
//if path_exists("/etc/default/grub") {
//Some("exists".to_string())
//} else {
//None
//}
//}

pub fn load() -> Option<BufReader<File>> {
    if let Ok(f) = File::open("fake_grub") {
        Some(BufReader::new(f))
    } else {
        None
    }
}
