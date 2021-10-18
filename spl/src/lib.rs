pub mod ast;
pub mod grammar;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn query() {
		assert_eq!(
			ast::Query(vec![
				ast::Expr::Filter {
					left: "index".to_owned(),
					operator: ast::Operator::Equal,
					right: "metric_name".to_owned(),
				},
				ast::Expr::Filter {
					left: "test_one".to_owned(),
					operator: ast::Operator::Equal,
					right: "one".to_owned(),
				},
				ast::Expr::Function(ast::Function::Earliest {
					value: 10,
					operator: ast::Operator::Equal,
					unit: ast::TimeUnit::Minute
				})
			]),
			grammar::QueryParser::new()
				.parse("index=\"metric_name\" | test_one=\"one\" | earliest=-10m")
				.unwrap()
		);
	}

	#[test]
	fn function_earliest() {
		assert_eq!(
			ast::Function::Earliest {
				value: 10,
				operator: ast::Operator::Equal,
				unit: ast::TimeUnit::Minute
			},
			grammar::FunctionParser::new()
				.parse("earliest=-10m")
				.unwrap()
		);
	}

	#[test]
	fn operator() {
		assert_eq!(
			ast::Operator::Equal,
			grammar::OperatorParser::new().parse("=").unwrap()
		);
	}

	#[test]
	fn time_unit() {
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
			ast::TimeUnit::Month,
			grammar::TimeUnitParser::new().parse("mon").unwrap()
		);
		assert_eq!(
			ast::TimeUnit::Quarter,
			grammar::TimeUnitParser::new().parse("q").unwrap()
		);
		assert_eq!(
			ast::TimeUnit::Year,
			grammar::TimeUnitParser::new().parse("y").unwrap()
		);
	}
}
