#![feature(box_patterns)]

pub mod ast;
pub mod grammar;

#[cfg(test)]
mod tests {
    use super::ast::{Expr, Function, Query};
    use crate::grammar;
    use ast::time_series::{Interval, Operator, Time, TimeUnit};

    #[test]
    fn query() {
        assert_eq!(
            Query(vec![
                Expr::Filter {
                    left: "example".to_owned(),
                    operator: Operator::Equal,
                    right: "test".to_owned(),
                },
                Expr::Filter {
                    left: "example_two".to_owned(),
                    operator: Operator::Equal,
                    right: "test2".to_owned(),
                },
                Expr::Function(Function::Earliest(Time::Relative(Interval {
                    value: 5,
                    unit: TimeUnit::Minute
                })))
            ]),
            grammar::QueryParser::new()
                .parse(
                    "search example=\"test\" | example_two=\"test2\" | earliest=-5m"
                )
                .unwrap()
        );
    }

    #[test]
    fn function_earliest() {
        assert_eq!(
            Function::Earliest(Time::Relative(Interval {
                value: 10,
                unit: TimeUnit::Minute
            })),
            grammar::FunctionParser::new().parse("earliest=-10m").unwrap()
        );
    }

    #[test]
    fn operator() {
        assert_eq!(
            Operator::Equal,
            grammar::OperatorParser::new().parse("=").unwrap()
        );
    }

    #[test]
    fn time_unit() {
        assert_eq!(
            TimeUnit::Second,
            grammar::TimeUnitParser::new().parse("s").unwrap()
        );
        assert_eq!(
            TimeUnit::Minute,
            grammar::TimeUnitParser::new().parse("m").unwrap()
        );
        assert_eq!(
            TimeUnit::Hour,
            grammar::TimeUnitParser::new().parse("h").unwrap()
        );
        assert_eq!(
            TimeUnit::Day,
            grammar::TimeUnitParser::new().parse("d").unwrap()
        );
        assert_eq!(
            TimeUnit::Week,
            grammar::TimeUnitParser::new().parse("w").unwrap()
        );
        assert_eq!(
            TimeUnit::Month,
            grammar::TimeUnitParser::new().parse("mon").unwrap()
        );
        assert_eq!(
            TimeUnit::Quarter,
            grammar::TimeUnitParser::new().parse("q").unwrap()
        );
        assert_eq!(
            TimeUnit::Year,
            grammar::TimeUnitParser::new().parse("y").unwrap()
        );
    }
}
