#[derive(Debug, Clone, PartialEq)]
pub enum LabelMatcherOperator {
	Equal,
	NotEqual,
	Match,
	NotMatch,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeUnit {
	Millisecond,
	Second,
	Minute,
	Hour,
	Day,
	Week,
	Year,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeVector {
	pub value: u64,
	pub unit: TimeUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelMatcher {
	pub name: String,
	pub operator: LabelMatcherOperator,
	pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
	pub metric_name: Option<String>,
	pub label_matchers: Vec<LabelMatcher>,
	pub range_vector: Option<RangeVector>,
}
