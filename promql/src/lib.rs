pub mod ast;
pub mod grammar;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn query() {
		assert_eq!(
			ast::Query {
				metric_name: Some("metric_name".to_owned()),
				label_matchers: vec![
					ast::LabelMatcher { name: "test_one".to_owned(), operator: ast::LabelMatcherOperator::Equal, value: "one".to_owned() },
					ast::LabelMatcher { name: "test_two".to_owned(), operator: ast::LabelMatcherOperator::Equal, value: "two".to_owned() },

				],
				range_vector: Some(ast::RangeVector {
					value: 200,
					unit: ast::TimeUnit::Millisecond,
				}),
			},
			grammar::QueryParser::new()
				.parse("metric_name{test_one=\"one\", test_two=\"two\"}[200ms]")
				.unwrap()
		);
	}

	#[test]
	fn range_vector() {
		assert_eq!(
			ast::RangeVector {
				value: 10,
				unit: ast::TimeUnit::Minute
			},
			grammar::RangeVectorParser::new()
				.parse("[10m]")
				.unwrap()
		);
	}

	#[test]
	fn label_matchers() {
		assert_eq!(
			vec![ast::LabelMatcher {
				name: "metric_name".to_owned(),
				operator: ast::LabelMatcherOperator::Equal,
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
			ast::LabelMatcherOperator::Equal,
			grammar::LabelMatcherOperatorParser::new()
				.parse("=")
				.unwrap()
		);
		assert_eq!(
			ast::LabelMatcherOperator::NotEqual,
			grammar::LabelMatcherOperatorParser::new()
				.parse("!=")
				.unwrap()
		);
		assert_eq!(
			ast::LabelMatcherOperator::Match,
			grammar::LabelMatcherOperatorParser::new()
				.parse("=~")
				.unwrap()
		);
		assert_eq!(
			ast::LabelMatcherOperator::NotMatch,
			grammar::LabelMatcherOperatorParser::new()
				.parse("!~")
				.unwrap()
		);
	}

	#[test]
	fn time_unit() {
		assert_eq!(
			ast::TimeUnit::Millisecond,
			grammar::TimeUnitParser::new().parse("ms").unwrap()
		);
		assert_eq!(
			ast::TimeUnit::Second,
			grammar::TimeUnitParser::new().parse("s").unwrap()
		);
		assert_eq!(
			ast::TimeUnit::Minute,
			grammar::TimeUnitParser::new().parse("m").unwrap()
		);
		assert_eq!(
			ast::TimeUnit::Hour,
			grammar::TimeUnitParser::new().parse("h").unwrap()
		);
		assert_eq!(
			ast::TimeUnit::Day,
			grammar::TimeUnitParser::new().parse("d").unwrap()
		);
		assert_eq!(
			ast::TimeUnit::Week,
			grammar::TimeUnitParser::new().parse("w").unwrap()
		);
		assert_eq!(
			ast::TimeUnit::Year,
			grammar::TimeUnitParser::new().parse("y").unwrap()
		);
	}
}
