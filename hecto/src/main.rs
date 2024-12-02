use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    println!("Write away!");
    for byte in io::stdin().bytes() {
        match byte {
            Ok(byte) => {
                let c = byte as char;

                if c.is_control() {
                    println!("Binary: {0:08b} ASCII: {0:#03} \r", byte);
                } else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?} \r", byte, c);
                }

                if c == 'q' {
                    disable_raw_mode().unwrap();
                    break;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
