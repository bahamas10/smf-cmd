#![allow(dead_code)]
use std::fmt;
use std::ops::Deref;

use strip_ansi_escapes::strip_str;

pub struct ColorAwareString {
    input: String,
}

impl ColorAwareString {
    pub fn with_string(input: String) -> Self {
        Self { input }
    }

    pub fn full_len(&self) -> usize {
        self.input.len()
    }

    pub fn raw_string(&self) -> String {
        strip_str(&self.input)
    }

    pub fn raw_len(&self) -> usize {
        self.raw_string().len()
    }

    pub fn pad_end(&self, len: usize) -> String {
        let mut out_string = self.to_string();

        let mut i = self.raw_len();
        while i < len {
            out_string.push_str(" ");
            i += 1;
        }

        out_string
    }

    pub fn pad_start(&self, len: usize) -> String {
        let out_string = self.to_string();
        let mut pad = String::new();

        let mut i = self.raw_len();
        while i < len {
            pad.push_str(" ");
            i += 1;
        }

        format!("{}{}", pad, out_string)
    }
}

impl Deref for ColorAwareString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.input
    }
}

impl fmt::Display for ColorAwareString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use colored::*;

    #[test]
    fn basic_test() {
        let s =
            format!("{} {} {}", "red".red(), "blue".blue(), "green".green(),);
        let color_string = ColorAwareString::with_string(s);

        assert_eq!(color_string.raw_string(), "red blue green");
        assert_eq!(color_string.raw_len(), "red blue green".len());
    }

    #[test]
    fn pad_end() {
        let s = format!("green: {}", "green starting now".green());
        let color_string = ColorAwareString::with_string(s);

        let output = color_string.pad_end(28);
        assert_eq!(
            output,
            format!("green: {}   ", "green starting now".green())
        );

        let output = color_string.pad_start(28);
        assert_eq!(
            output,
            format!("   green: {}", "green starting now".green())
        );
    }
}
