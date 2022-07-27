use crate::*;
use std::collections::HashMap;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

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
                        if let DataType::Bool(b) = row {
                            if *b {
                                bool_result.true_count += 1;
                            } else {
                                bool_result.false_count += 1;
                            }
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
                        if let DataType::String(s) = row {
                            if s.is_empty() {
                                categorical_result.null_count += 1;
                            } else {
                                if !categorical_result.unique_values.contains(s) {
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
                    }
                    result.insert(col.clone(), DescribeResult::Categorical(categorical_result));
                }
            }
        }
        result
    }

    pub fn print_describe(&self) {
        let result = self.describe();
        for (_, describe_result) in result {
            match describe_result {
                DescribeResult::Boolean(bool_result) => println!("{}", bool_result),
                DescribeResult::Numeric(numeric_result) => println!("{}", numeric_result),
                DescribeResult::Categorical(categorical_result) => {
                    println!("{}", categorical_result)
                }
            }
        }
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

    pub fn print(&self) {
        println!("===============================================================");
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
