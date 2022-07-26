use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, PartialEq)]
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
        DataType::String(s.replace('\"', ""))
    }
}

#[derive(Debug, PartialEq)]
pub enum DescribeResult {
    Numeric(NumericDescribeResult),
    Categorical(CategoricalDescribeResult),
    Boolean(BooleanDescribeResult),
}

impl Display for DescribeResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DescribeResult::Numeric(n) => write!(f, "{}", n),
            DescribeResult::Categorical(c) => write!(f, "{}", c),
            DescribeResult::Boolean(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug)]
pub struct NumericDescribeResult {
    name: String,
    dtype: DataType,
    null_count: i32,
    min: f32,
    max: f32,
    mean: f32,
    std: f32,
}

impl Display for NumericDescribeResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}:\n\
            \tType: {}\n\
            \tNull Count: {}\n\
            \tMin: {}\n\
            \tMax: {}\n\
            \tMean: {}\n\
            \tStd: {}",
            self.name, self.dtype, self.null_count, self.min, self.max, self.mean, self.std
        )
    }
}

impl PartialEq for NumericDescribeResult {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.dtype == other.dtype
            && self.null_count == other.null_count
            && self.min == other.min
            && self.max == other.max
            && (self.mean - other.mean < 0.01)
            && (self.std - other.std < 0.01)
    }
}

#[derive(Debug, PartialEq)]
pub struct CategoricalDescribeResult {
    name: String,
    dtype: DataType,
    null_count: i32,
    unique_count: i32,
    unique_values: Vec<String>,
    most_freq_value: String,
    most_freq_count: i32,
}

impl Display for CategoricalDescribeResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}:\n\
            \tType: {}\n\
            \tNull Count: {}\n\
            \tUnique Count: {}\n\
            \tUnique Values: {:?}\n\
            \tMost Freq Value: {}\n\
            \tMost Freq Count: {}",
            self.name,
            self.dtype,
            self.null_count,
            self.unique_count,
            self.unique_values,
            self.most_freq_value,
            self.most_freq_count
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct BooleanDescribeResult {
    name: String,
    dtype: DataType,
    null_count: i32,
    true_count: i32,
    false_count: i32,
}

impl Display for BooleanDescribeResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}:\n\
            \tType: {}\n\
            \tNull Count: {}\n\
            \tTrue Count: {}\n\
            \tFalse Count: {}",
            self.name, self.dtype, self.null_count, self.true_count, self.false_count
        )
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
                header = line
                    .unwrap()
                    .split(',')
                    .map(|s| s.trim().to_lowercase().replace('\"', ""))
                    .collect();
            } else {
                let line = line.unwrap();
                let mut row = Vec::new();
                for s in line.split(',') {
                    row.push(predict_type(s));
                }
                data.push(row);
            }
        }
        Ok(DataFrame { header, data })
    }

    pub fn describe(&self) -> HashMap<String, DescribeResult> {
        let mut result = HashMap::new();

        for (i, col) in self.header.iter().enumerate() {
            let col_data = &self.get_col(i).unwrap();
            let dtype = &col_data[0];
            match dtype {
                DataType::None => (),
                DataType::Bool(_) => {
                    let mut bool_result = BooleanDescribeResult {
                        name: col.clone(),
                        dtype: dtype.clone(),
                        null_count: 0,
                        true_count: 0,
                        false_count: 0,
                    };
                    for row in col_data {
                        match row {
                            DataType::Bool(b) => {
                                if *b {
                                    bool_result.true_count += 1;
                                } else {
                                    bool_result.false_count += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    result.insert(col.clone(), DescribeResult::Boolean(bool_result));
                }
                DataType::Int(_) | DataType::Float(_) => {
                    let mut numeric_result = NumericDescribeResult {
                        name: col.clone(),
                        dtype: dtype.clone(),
                        null_count: 0,
                        min: f32::MAX,
                        max: f32::MIN,
                        mean: 0.0,
                        std: 0.0,
                    };
                    for row in col_data {
                        let val = match row {
                            DataType::Int(i) => *i as f32,
                            DataType::Float(f) => *f,
                            _ => f32::NAN,
                        };
                        numeric_result.std = 0.0;
                        if val.is_nan() {
                            numeric_result.null_count += 1;
                        } else {
                            numeric_result.min = f32::min(numeric_result.min, val);
                            numeric_result.max = f32::max(numeric_result.max, val);
                            numeric_result.mean += val;
                        }
                    }
                    numeric_result.mean /=
                        (col_data.len() - (numeric_result.null_count as usize)) as f32;
                    for row in col_data {
                        let val = match row {
                            DataType::Int(i) => *i as f32,
                            DataType::Float(f) => *f,
                            _ => f32::NAN,
                        };
                        if !val.is_nan() {
                            numeric_result.std += (val - numeric_result.mean).powi(2);
                        }
                    }
                    numeric_result.std /=
                        col_data.len() as f32 - (numeric_result.null_count as f32);
                    numeric_result.std = numeric_result.std.sqrt();

                    result.insert(col.clone(), DescribeResult::Numeric(numeric_result));
                }

                DataType::String(_) => {
                    let mut categorical_result = CategoricalDescribeResult {
                        name: col.clone(),
                        dtype: dtype.clone(),
                        null_count: 0,
                        unique_count: 0,
                        unique_values: Vec::new(),
                        most_freq_value: String::new(),
                        most_freq_count: 0,
                    };
                    for row in col_data {
                        match row {
                            DataType::String(s) => {
                                if s.is_empty() {
                                    categorical_result.null_count += 1;
                                } else {
                                    if !categorical_result.unique_values.contains(&s) {
                                        categorical_result.unique_values.push(s.clone());
                                        categorical_result.unique_count += 1;
                                    }
                                    if categorical_result.most_freq_count < 1 {
                                        categorical_result.most_freq_value = s.clone();
                                        categorical_result.most_freq_count = 1;
                                    } else if categorical_result.most_freq_value.eq(s) {
                                        categorical_result.most_freq_count += 1;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    result.insert(col.clone(), DescribeResult::Categorical(categorical_result));
                }
            }
        }
        result
    }

    pub fn get_row(&self, row: usize) -> Option<Vec<DataType>> {
        if row < self.data.len() {
            Some(self.data[row].clone())
        } else {
            None
        }
    }

    pub fn get_col(&self, col: usize) -> Option<Vec<DataType>> {
        if col < self.header.len() {
            Some(self.data.iter().map(|row| row[col].clone()).collect())
        } else {
            None
        }
    }

    pub fn print(&self) -> () {
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10}",
            &self.header[0]
                .chars()
                .into_iter()
                .take(8)
                .collect::<String>(),
            &self.header[1]
                .chars()
                .into_iter()
                .take(8)
                .collect::<String>(),
            &self.header[2]
                .chars()
                .into_iter()
                .take(8)
                .collect::<String>(),
            &self.header[3]
                .chars()
                .into_iter()
                .take(8)
                .collect::<String>(),
            &self.header[self.header.len() - 1]
                .chars()
                .into_iter()
                .take(8)
                .collect::<String>()
        );
        for row in self.data.iter().take(5) {
            println!(
                "{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10}",
                &row[0]
                    .to_string()
                    .chars()
                    .into_iter()
                    .take(8)
                    .collect::<String>(),
                &row[1]
                    .to_string()
                    .chars()
                    .into_iter()
                    .take(8)
                    .collect::<String>(),
                &row[2]
                    .to_string()
                    .chars()
                    .into_iter()
                    .take(8)
                    .collect::<String>(),
                &row[3]
                    .to_string()
                    .chars()
                    .into_iter()
                    .take(8)
                    .collect::<String>(),
                &row[row.len() - 1]
                    .to_string()
                    .chars()
                    .into_iter()
                    .take(8)
                    .collect::<String>()
            );
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

    #[test]
    fn test_describe() {
        let fname = "./examples/iris.csv";
        let df = super::DataFrame::read(fname).unwrap();
        let describe_result = df.describe();
        assert_eq!(describe_result.len(), 5);
        assert_eq!(
            describe_result["sepal.length"],
            super::DescribeResult::Numeric(super::NumericDescribeResult {
                name: "sepal.length".to_string(),
                dtype: super::DataType::Float(5.1),
                null_count: 0,
                mean: 5.8433,
                std: 0.8281,
                min: 4.3,
                max: 7.9,
            })
        );
        assert_eq!(
            describe_result["sepal.width"],
            super::DescribeResult::Numeric(super::NumericDescribeResult {
                name: "sepal.width".to_string(),
                dtype: super::DataType::Float(3.5),
                null_count: 0,
                mean: 3.0573,
                std: 0.4359,
                min: 2.0,
                max: 4.4,
            })
        );
        assert_eq!(
            describe_result["petal.length"],
            super::DescribeResult::Numeric(super::NumericDescribeResult {
                name: "petal.length".to_string(),
                dtype: super::DataType::Float(1.4),
                null_count: 0,
                mean: 3.758,
                std: 1.765,
                min: 1.0,
                max: 6.9,
            })
        );
        assert_eq!(
            describe_result["petal.width"],
            super::DescribeResult::Numeric(super::NumericDescribeResult {
                name: "petal.width".to_string(),
                dtype: super::DataType::Float(0.2),
                null_count: 0,
                mean: 1.1993,
                std: 0.7622,
                min: 0.1,
                max: 2.5,
            })
        );
        assert_eq!(
            describe_result["variety"],
            super::DescribeResult::Categorical(super::CategoricalDescribeResult {
                name: "variety".to_string(),
                dtype: super::DataType::String("Setosa".to_string()),
                null_count: 0,
                most_freq_count: 50,
                most_freq_value: "Setosa".to_string(),
                unique_count: 3,
                unique_values: vec![
                    "Setosa".to_string(),
                    "Versicolor".to_string(),
                    "Virginica".to_string()
                ],
            })
        );
    }
}
