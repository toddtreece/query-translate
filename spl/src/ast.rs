use ast::time_series::{Operator, Time};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Earliest(Time),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Function(Function),
    Filter { left: String, operator: Operator, right: String },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Function(Function::Earliest(t)) => {
                write!(f, "earliest=-{}", t)
            }
            Self::Filter { left, operator, right } => {
                write!(f, "{}{}\"{}\"", left, operator, right)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query(pub Vec<Expr>);

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.len() > 0 {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|l| format!("{}", l))
                    .collect::<Vec<String>>()
                    .join(" | ")
            )?;
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod tests {

    use super::{Expr, Function, Query};
    use ast::time_series::{Interval, Operator, Time, TimeUnit};

    #[test]
    fn display() {
        assert_eq!(
            "index=\"metric_name\" | test_one=\"one\" | earliest=-10m",
            format!(
                "{}",
                Query(vec![
                    Expr::Filter {
                        left: "index".to_owned(),
                        operator: Operator::Equal,
                        right: "metric_name".to_owned(),
                    },
                    Expr::Filter {
                        left: "test_one".to_owned(),
                        operator: Operator::Equal,
                        right: "one".to_owned(),
                    },
                    Expr::Function(Function::Earliest(Time::Relative(
                        Interval { value: 10, unit: TimeUnit::Minute }
                    )))
                ])
            ),
        );
    }
}
