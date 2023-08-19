//Topic: formatting
/*
Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:


RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
 */

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
    hex: &'static str,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "RGB: ({}, {}, {}) {}",
            self.red, self.green, self.blue, self.hex
        )
    }
}

fn main() {
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
            hex: "0x80FF5A",
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
            hex: "0x0003FE",
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
            hex: "0x000000",
        },
    ] {
        println!("{}", color);
    }
}
