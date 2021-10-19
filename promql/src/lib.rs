#![feature(box_patterns)]

pub mod ast;
pub mod grammar;

#[cfg(test)]
mod tests {
    use super::ast::{LabelMatcher, LabelMatcherOperator, Query};
    use super::grammar;
    use ast::time_series::{Interval, Time, TimeUnit};

    #[test]
    fn query() {
        assert_eq!(
            Query {
                metric_name: Some("metric_name".to_owned()),
                label_matchers: vec![
                    LabelMatcher {
                        name: "test_one".to_owned(),
                        operator: LabelMatcherOperator::Equal,
                        value: "one".to_owned()
                    },
                    LabelMatcher {
                        name: "test_two".to_owned(),
                        operator: LabelMatcherOperator::Equal,
                        value: "two".to_owned()
                    },
                ],
                range_vector: Some(Time::Relative(Interval {
                    value: 200,
                    unit: TimeUnit::Millisecond,
                })),
            },
            grammar::QueryParser::new()
                .parse(
                    "metric_name{test_one=\"one\", test_two=\"two\"}[200ms]"
                )
                .unwrap()
        );
    }

    #[test]
    fn range_vector() {
        assert_eq!(
            Time::Relative(Interval { value: 10, unit: TimeUnit::Minute }),
            grammar::RangeVectorParser::new().parse("[10m]").unwrap()
        );
    }

    #[test]
    fn label_matchers() {
        assert_eq!(
            vec![LabelMatcher {
                name: "metric_name".to_owned(),
                operator: LabelMatcherOperator::Equal,
                value: "value".to_owned(),
            }],
            grammar::LabelMatchersParser::new()
                .parse("{metric_name=\"value\"}")
                .unwrap()
        );
    }

    #[test]
    fn label_matcher_operator() {
        assert_eq!(
            LabelMatcherOperator::Equal,
            grammar::LabelMatcherOperatorParser::new().parse("=").unwrap()
        );
        assert_eq!(
            LabelMatcherOperator::NotEqual,
            grammar::LabelMatcherOperatorParser::new().parse("!=").unwrap()
        );
        assert_eq!(
            LabelMatcherOperator::Match,
            grammar::LabelMatcherOperatorParser::new().parse("=~").unwrap()
        );
        assert_eq!(
            LabelMatcherOperator::NotMatch,
            grammar::LabelMatcherOperatorParser::new().parse("!~").unwrap()
        );
    }

    #[test]
    fn time_unit() {
        assert_eq!(
            TimeUnit::Millisecond,
            grammar::TimeUnitParser::new().parse("ms").unwrap()
        );
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
            TimeUnit::Year,
            grammar::TimeUnitParser::new().parse("y").unwrap()
        );
    }
}
