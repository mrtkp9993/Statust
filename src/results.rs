use crate::*;
use std::fmt;
use std::fmt::{Display, Formatter};

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
    pub name: String,
    pub dtype: DataType,
    pub null_count: i32,
    pub min: f32,
    pub max: f32,
    pub mean: f32,
    pub std: f32,
}

impl Display for NumericDescribeResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "==============================================================="
        )
        .ok();
        write!(
            f,
            "{}:\n\
            \tNull Count: {}\n\
            \tMin: {}\n\
            \tMax: {}\n\
            \tMean: {}\n\
            \tStd: {}",
            self.name, self.null_count, self.min, self.max, self.mean, self.std
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
    pub name: String,
    pub dtype: DataType,
    pub null_count: i32,
    pub unique_count: i32,
    pub unique_values: Vec<String>,
    pub most_freq_value: String,
    pub most_freq_count: i32,
}

impl Display for CategoricalDescribeResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "==============================================================="
        )
        .ok();
        write!(
            f,
            "{}:\n\
            \tNull Count: {}\n\
            \tUnique Count: {}\n\
            \tUnique Values: {:?}\n\
            \tMost Freq Value: {}\n\
            \tMost Freq Count: {}",
            self.name,
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
    pub name: String,
    pub dtype: DataType,
    pub null_count: i32,
    pub true_count: i32,
    pub false_count: i32,
}

impl Display for BooleanDescribeResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "==============================================================="
        )
        .ok();
        write!(
            f,
            "{}:\n\
            \tNull Count: {}\n\
            \tTrue Count: {}\n\
            \tFalse Count: {}",
            self.name, self.null_count, self.true_count, self.false_count
        )
    }
}

#[cfg(test)]
mod tests {

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
