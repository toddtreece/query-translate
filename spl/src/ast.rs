#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
	Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeUnit {
	Second,
	Minute,
	Hour,
	Day,
	Week,
	Month,
	Quarter,
	Year,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
	Earliest { value: u64, operator: Operator, unit: TimeUnit },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
	Function(Function),
	Filter { left: String, operator: Operator, right: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query(pub Vec<Expr>);
