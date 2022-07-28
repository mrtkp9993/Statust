use crate::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub enum ResultType {
    DescribeOne(DescribeResult),
    DescribeMany(HashMap<String, DescribeResult>),
}

pub struct FileWriter {
    pub fname: String,
    pub data: ResultType,
}

impl FileWriter {
    pub fn write_file(&self) {
        match &self.data {
            ResultType::DescribeOne(d) => {
                let mut file = File::create(&self.fname).unwrap();
                writeln!(file, "{}", d).unwrap();
            }
            ResultType::DescribeMany(d) => {
                let mut file = File::create(&self.fname).unwrap();
                for (_, v) in d {
                    writeln!(file, "{}", v).unwrap();
                }
            }
        }
    }
}
