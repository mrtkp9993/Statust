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
                for v in d.values() {
                    writeln!(file, "{}", v).unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path};

    #[test]
    fn test_write_file() {
        let fname = "./examples/iris.csv";
        let df = super::DataFrame::read(fname).unwrap();
        let res = df.describe();
        let res_fname = "./iris_describe.txt";
        let file_writer = super::FileWriter {
            fname: res_fname.to_string(),
            data: super::ResultType::DescribeMany(res),
        };
        file_writer.write_file();
        let res_read = fs::read_to_string(path::Path::new(&res_fname)).unwrap();
        assert!(res_read.contains("sepal.length"));
        assert!(res_read.contains("sepal.width"));
        assert!(res_read.contains("petal.length"));
        assert!(res_read.contains("petal.width"));
        assert!(res_read.contains("variety"));
        assert!(res_read.contains("Unique Count: 3"));
        assert!(res_read.contains("Unique Values: [\"Setosa\", \"Versicolor\", \"Virginica\"]"));
        assert!(res_read.contains("Most Freq Value: Setosa"));
        assert!(res_read.contains("Most Freq Count: 50"));
        assert!(res_read.contains("Null Count: 0"));
        assert!(res_read.contains("Min: 0.1"));
        assert!(res_read.contains("Max: 7.9"));
        assert!(res_read.contains("Mean: 3.7580001"));
    }
}
