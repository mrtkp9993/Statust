use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt::Display, fs};

#[derive(Debug, PartialEq)]
pub enum DataType {
    None,
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataType::None => write!(f, "None"),
            DataType::Bool(b) => write!(f, "{}", b),
            DataType::Int(i) => write!(f, "{}", i),
            DataType::Float(fl) => write!(f, "{}", fl),
            DataType::String(s) => write!(f, "{}", s),
        }
    }
}

fn predict_type(s: &str) -> DataType {
    lazy_static! {
        static ref BOOL_RE: Regex = Regex::new(r"^(true|false)$").unwrap();
        static ref INT_RE: Regex = Regex::new(r"^-?\d+$").unwrap();
        static ref FLOAT_RE: Regex = Regex::new(r"^-?\d+\.\d+$").unwrap();
    }

    if BOOL_RE.is_match(s) {
        DataType::Bool(s.to_lowercase() == "true")
    } else if INT_RE.is_match(s) {
        DataType::Int(s.parse::<i32>().unwrap())
    } else if FLOAT_RE.is_match(s) {
        DataType::Float(s.parse::<f32>().unwrap())
    } else {
        DataType::String(s.to_string())
    }
}

pub struct DataFrame<'a> {
    header: Vec<&'a str>,
    data: Vec<Vec<DataType>>,
}

impl DataFrame<'_> {
    fn read(fname: &str) -> DataFrame<'_> {
        // let mut header = Vec::new();
        // let mut data = Vec::new();
        // let file_contents = fs::read(fname).unwrap();

        // DataFrame { header, data }
        todo!()
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
