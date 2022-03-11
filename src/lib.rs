/*
What do we want to accomplish?
    1. Print recieved text to any source (stdout, stderr, etc)
    2. Control color of text
What do we need to accomplish this?
    1. buffer to take in string and later print it
    2. way to change color of string
        1. to change color we can use ascii characters
            This means appending ascii color codes to end of buffer
        2. need to a hashmap of ascii codes so we dont have to look them up
*/
#![allow(dead_code)]
use std::io::{self, Write};

pub enum SupportedColors {
    Red,
    Green,
    Blue,
    Yellow,
    White,
    Magenta,
}

pub enum StandardStreamTypes {
    Stdout,
    Stderr,
}

pub enum StandardStream {
    Stdout(io::Stdout),
    Stderr(io::Stderr),
}

impl io::Write for StandardStream {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        match self {
            StandardStream::Stderr(ref mut s) => s.write(buffer),
            StandardStream::Stdout(ref mut s) => s.write(buffer),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            StandardStream::Stderr(ref mut s) => s.flush(),
            StandardStream::Stdout(ref mut s) => s.flush(),
        }
    }
}

pub struct ColorWriter<'a> {
    writer: StandardStream,
    choice: &'a str,
}

impl<'a> ColorWriter<'a> {
    pub fn new(stream_type: StandardStreamTypes, choice: Option<SupportedColors>) -> ColorWriter<'a> {
        if let Some(choice) = choice {
            match stream_type {
                StandardStreamTypes::Stdout => ColorWriter {
                    writer: StandardStream::Stdout(io::stdout()),
                    choice: get_color(choice),
                },
                StandardStreamTypes::Stderr => ColorWriter {
                    writer: StandardStream::Stderr(io::stderr()),
                    choice: get_color(choice),
                },
            }
        } else {
            let choice = ""; 
            match stream_type {
                StandardStreamTypes::Stdout => ColorWriter {
                    writer: StandardStream::Stdout(io::stdout()),
                    choice: choice,
                },
                StandardStreamTypes::Stderr => ColorWriter {
                    writer: StandardStream::Stderr(io::stderr()),
                    choice: choice,
                },
            }
        }
    }

    pub fn print(&mut self, str: &[u8]) -> io::Result<usize> {
        let formatted_str = self.format_str(str, self.choice.as_bytes());
        self.writer.write(&formatted_str)
    }

    pub fn reset(mut self) {
        let choice = "";
        self.choice = choice;
    }

    pub fn set_color(mut self, color: SupportedColors) {
        let color_code = get_color(color);
        self.choice = color_code;
    }

    fn format_str(&mut self, str: &[u8], color_code: &[u8]) -> Vec<u8> {
        let start  = b"\x1b[";
        let end = b"\x1b[0m";
        let choice = [color_code, b"m"].concat();
        
        [start, choice.as_slice(), str, end].concat()
    }
}

fn get_color<'a>(color: SupportedColors) -> &'a str {
    match color {
        SupportedColors::Red => "31",
        SupportedColors::Green => "32",
        SupportedColors::Blue => "34",
        SupportedColors::Yellow => "33",
        SupportedColors::White => "37",
        SupportedColors::Magenta => "35",
        _ => "39", // default
    }
}
