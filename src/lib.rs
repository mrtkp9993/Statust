#![feature(iter_collect_into)]

use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub enum DataType {
    None,
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataType::None => write!(f, "None"),
            DataType::Bool(b) => write!(f, "{}", b),
            DataType::Int(i) => write!(f, "{}", i),
            DataType::Float(fl) => write!(f, "{}", fl),
            DataType::String(s) => write!(f, "{}", s),
        }
    }
}

pub fn predict_type(s: &str) -> DataType {
    lazy_static! {
        static ref BOOL_RE: Regex = Regex::new(r"^(true|false)$").unwrap();
        static ref INT_RE: Regex = Regex::new(r"^-?\d+$").unwrap();
        static ref FLOAT_RE: Regex = Regex::new(r"^[-+]?\d*\.\d+$").unwrap();
    }

    if BOOL_RE.is_match(s.to_lowercase().as_str()) {
        DataType::Bool(s.to_lowercase() == "true")
    } else if INT_RE.is_match(s) {
        DataType::Int(s.parse::<i32>().unwrap())
    } else if FLOAT_RE.is_match(s) {
        DataType::Float(s.parse::<f32>().unwrap())
    } else {
        DataType::String(s.to_string())
    }
}

pub struct DataFrame {
    header: Vec<String>,
    data: Vec<Vec<DataType>>,
}

impl DataFrame {
    pub fn read(fname: &str) -> io::Result<DataFrame> {
        let mut header = Vec::new();
        let mut data = Vec::new();
        let file = File::open(fname)?;
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().into_iter().enumerate() {
            if i == 0 {
                line.unwrap()
                    .split(",")
                    .map(|s| s.trim().to_lowercase())
                    .collect_into(&mut header);
            } else {
                let line = line.unwrap();
                let mut row = Vec::new();
                for s in line.split(",") {
                    row.push(predict_type(s));
                }
                data.push(row);
            }
        }

        Ok(DataFrame { header, data })
    }

    pub fn print(&self) {
        println!("{:?}", self.header);
        for row in self.data.iter().take(5) {
            println!("{:?}", row);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_predict_type() {
        assert_eq!(super::predict_type("true"), super::DataType::Bool(true));
        assert_eq!(super::predict_type("false"), super::DataType::Bool(false));
        assert_eq!(super::predict_type("-1"), super::DataType::Int(-1));
        assert_eq!(super::predict_type("1"), super::DataType::Int(1));
        assert_eq!(super::predict_type("1.0"), super::DataType::Float(1.0));
        assert_eq!(super::predict_type("-1.0"), super::DataType::Float(-1.0));
        assert_eq!(
            super::predict_type("hello"),
            super::DataType::String("hello".to_string())
        );
    }
}
