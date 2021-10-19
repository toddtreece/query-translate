use ast::time_series::{self, Operator, ScalarValue, Time};
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

impl From<time_series::Query> for Query {
    fn from(q: time_series::Query) -> Self {
        q.0.into_iter().fold(Query(vec![]), |mut r, e| {
            match e {
                time_series::Expr::TimeRange {
                    from: Some(time),
                    to: None,
                } => r.0.push(Expr::Function(Function::Earliest(time))),
                time_series::Expr::BinaryExpression {
                    left: box time_series::Expr::Label(name),
                    right:
                        box time_series::Expr::Literal(ScalarValue::Utf8(Some(
                            value,
                        ))),
                    op: operator,
                } => r.0.push(Expr::Filter {
                    left: name,
                    operator: operator,
                    right: value,
                }),
                _ => {}
            }
            r
        })
    }
}

impl From<Query> for ast::time_series::Query {
    fn from(q: Query) -> Self {
        let res = ast::time_series::Query(vec![]);
        q.0.into_iter().fold(res, |mut r, e| {
            match e {
                Expr::Function(Function::Earliest(time)) => {
                    r.0.push(time_series::Expr::TimeRange {
                        from: Some(time),
                        to: None,
                    })
                }
                Expr::Filter { left: name, operator, right: value } => {
                    r.0.push(time_series::Expr::BinaryExpression {
                        left: Box::new(time_series::Expr::Label(name)),
                        right: Box::new(time_series::Expr::Literal(
                            ScalarValue::Utf8(Some(value)),
                        )),
                        op: operator,
                    })
                }
            }
            r
        })
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "search ")?;
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

    #[test]
    fn display() {
        use super::{Expr, Function, Query};
        use ast::time_series::{Interval, Operator, Time, TimeUnit};

        assert_eq!(
            "search test_one=\"one\" | earliest=-10m",
            format!(
                "{}",
                Query(vec![
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

    #[test]
    fn from_ts() {
        use ast::time_series::{
            Expr, Interval, Operator, Query, ScalarValue, Time, TimeUnit,
        };
        assert_eq!(
            "search test_one=\"one\" | earliest=-50w",
            format!(
                "{}",
                super::Query::from(Query(vec![
                    Expr::Metric("metric_name".to_owned()),
                    Expr::BinaryExpression {
                        left: Box::new(Expr::Label("test_one".to_owned())),
                        op: Operator::Equal,
                        right: Box::new(Expr::Literal(ScalarValue::Utf8(
                            Some("one".to_owned())
                        )))
                    },
                    Expr::TimeRange {
                        from: Some(Time::Relative(Interval {
                            value: 50,
                            unit: TimeUnit::Week
                        })),
                        to: None
                    },
                ]))
            )
        );
    }

    #[test]
    fn to_ts() {
        use ast::time_series::{
            Expr, Interval, Operator, Query, ScalarValue, Time, TimeUnit,
        };
        let q = super::Query(vec![
            super::Expr::Filter {
                left: "test_one".to_owned(),
                operator: Operator::Equal,
                right: "one".to_owned(),
            },
            super::Expr::Function(super::Function::Earliest(Time::Relative(
                Interval { value: 50, unit: TimeUnit::Week },
            ))),
        ]);
        let ts = Query(vec![
            Expr::BinaryExpression {
                left: Box::new(Expr::Label("test_one".to_owned())),
                op: Operator::Equal,
                right: Box::new(Expr::Literal(ScalarValue::Utf8(Some(
                    "one".to_owned(),
                )))),
            },
            Expr::TimeRange {
                from: Some(Time::Relative(Interval {
                    value: 50,
                    unit: TimeUnit::Week,
                })),
                to: None,
            },
        ]);

        assert_eq!(ts, Query::from(q));
    }
}
