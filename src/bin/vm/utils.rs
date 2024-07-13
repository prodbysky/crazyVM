use std::io::Read;

pub fn read_binary(name: &str) -> Option<Vec<u8>> {
    let mut file = match std::fs::File::open(name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file {}: {}", name, e);
            return None;
        }
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to read {}: {}", name, e);
            return None;
        }
    }
    let mut buf = vec![];

    for byte in content.split_whitespace() {
        match u8::from_str_radix(byte, 16) {
            Ok(num) => buf.push(num),
            Err(e) => {
                eprintln!("Invalid hex value in program: {}", e);
                return None;
            }
        };
    }

    Some(buf)
}
