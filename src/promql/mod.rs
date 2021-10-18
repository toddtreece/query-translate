use pest::{error::Error, iterators::Pairs, Parser};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
	Equals,
	NotEquals,
	Matches,
	NotMatches,
}

impl TryFrom<Rule> for Operator {
	type Error = Error<Rule>;
	fn try_from(rule: Rule) -> Result<Self, Self::Error> {
		let operator = match rule {
			Rule::equals => Operator::Equals,
			Rule::not_equals => Operator::NotEquals,
			Rule::matches => Operator::Matches,
			Rule::not_matches => Operator::NotMatches,
			_ => Operator::Equals,
		};
		Ok(operator)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Duration {
	MilliSeconds,
	Seconds,
	Minutes,
	Hours,
	Days,
	Weeks,
	Years,
}
impl Default for Duration {
	fn default() -> Self {
		Self::MilliSeconds
	}
}

impl TryFrom<Rule> for Duration {
	type Error = Error<Rule>;
	fn try_from(rule: Rule) -> Result<Self, Self::Error> {
		let operator = match rule {
			Rule::milliseconds => Duration::MilliSeconds,
			Rule::seconds => Duration::Seconds,
			Rule::minutes => Duration::Minutes,
			Rule::hours => Duration::Hours,
			Rule::days => Duration::Days,
			Rule::weeks => Duration::Weeks,
			Rule::years => Duration::Years,
			_ => Duration::MilliSeconds,
		};
		Ok(operator)
	}
}

#[derive(Debug, Clone, Default)]
pub struct RangeVector {
	time: u64,
	duration: Duration,
}

impl TryFrom<Pairs<'_, Rule>> for RangeVector {
	type Error = Error<Rule>;
	fn try_from(pairs: Pairs<'_, Rule>) -> Result<Self, Self::Error> {
		let rv = pairs.fold(RangeVector::default(), |mut p, r| match r.as_rule() {
			Rule::time => {
				p.time = r.as_str().parse().unwrap();
				p
			}
			Rule::duration => {
				p.duration = Duration::try_from(
					r.into_inner().next().unwrap().as_rule(),
				)
				.unwrap();
				p
			}
			_ => p,
		});
		Ok(rv)
	}
}

#[derive(Debug, Clone)]
pub struct LabelMatcher<'a> {
	name: &'a str,
	operator: Operator,
	value: &'a str,
}

impl Default for LabelMatcher<'_> {
	fn default() -> Self {
		Self {
			name: "",
			operator: Operator::Equals,
			value: "",
		}
	}
}

impl<'i, 'a> TryFrom<Pairs<'i, Rule>> for LabelMatcher<'i> {
	type Error = Error<Rule>;
	fn try_from(pairs: Pairs<'i, Rule>) -> Result<Self, Self::Error> {
		let matcher = pairs.fold(LabelMatcher::default(), |mut p, r| match r.as_rule() {
			Rule::name => {
				p.name = r.as_str();
				p
			}
			Rule::operator => {
				p.operator = Operator::try_from(
					r.into_inner().next().unwrap().as_rule(),
				)
				.unwrap();
				p
			}
			Rule::value => {
				p.value = r.as_str();
				p
			}
			_ => p,
		});
		Ok(matcher)
	}
}

#[derive(Parser, Debug, Default, Clone)]
#[grammar = "promql/promql.pest"]
pub struct PromQL<'a> {
	metric_name: Option<&'a str>,
	label_matchers: Vec<LabelMatcher<'a>>,
	range_vector: Option<RangeVector>,
}

impl<'a> TryFrom<&'a str> for PromQL<'a> {
	type Error = Error<Rule>;

	fn try_from(raw: &'a str) -> Result<Self, Self::Error> {
		let query = PromQL::parse(Rule::promql, raw)?;
		let promql = query.fold(PromQL::default(), |mut p, r| {
			match r.as_rule() {
				Rule::metric_name => p.metric_name = Some(r.as_str()),
				Rule::label_matcher => {
					p.label_matchers
						.push(LabelMatcher::try_from(r.into_inner())
							.unwrap());
				}
				Rule::range_vector => {
					p.range_vector =
						Some(RangeVector::try_from(r.into_inner())
							.unwrap());
				}
				_ => {}
			}
			p
		});
		Ok(promql)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_query() {
		let query = "some_metric{foo=\"bar\"}[5m]";

		let promql = PromQL::try_from(query).unwrap();
		assert_eq!(promql.metric_name, Some("some_metric"));

		let matcher = promql.label_matchers[0].clone();
		assert_eq!(matcher.name, "foo");
		assert_eq!(matcher.value, "bar");
		assert_eq!(matcher.operator, Operator::Equals);

		let rv = promql.range_vector.unwrap().clone();
		assert_eq!(rv.duration, Duration::Minutes);
		assert_eq!(rv.time, 5);
	}
}
