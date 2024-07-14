use std::io::Read;

pub fn read_binary(name: &str) -> Option<Vec<u32>> {
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
        let mut reversed = String::with_capacity(byte.len());

        for ch in byte.chars().rev() {
            reversed.push(ch);
        }

        match u32::from_str_radix(&reversed, 16) {
            Ok(num) => buf.push(num),
            Err(e) => {
                eprintln!("Invalid hex value in program: {}", e);
                return None;
            }
        };
    }
    Some(buf)
}
