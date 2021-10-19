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
                    left: "index".to_owned(),
                    operator: Operator::Equal,
                    right: "metric_name".to_owned(),
                },
                Expr::Filter {
                    left: "test_one".to_owned(),
                    operator: Operator::Equal,
                    right: "one".to_owned(),
                },
                Expr::Function(Function::Earliest(Time::Relative(Interval {
                    value: 10,
                    unit: TimeUnit::Minute
                })))
            ]),
            grammar::QueryParser::new()
                .parse(
                    "index=\"metric_name\" | test_one=\"one\" | earliest=-10m"
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
