use std::boxed::Box;

#[derive(Debug, Clone)]
pub enum Expr {
	Namespace(String),
	Column(String),
	Literal(ScalarValue),
	BinaryExpression { left: Box<Expr>, op: Operator, right: Box<Expr> },
}

impl PartialEq for Expr {
	fn eq(&self, other: &Self) -> bool {
		use Expr::*;
		match (self, other) {
			(Namespace(a), Namespace(b)) => a == b,
			(Column(a), Column(b)) => a == b,
			(Literal(a), Literal(b)) => a == b,
			(
				BinaryExpression {
					left: left_a,
					op: op_a,
					right: right_a,
				},
				BinaryExpression {
					left: left_b,
					op: op_b,
					right: right_b,
				},
			) => {
				left_a == left_b
					&& op_a == op_b && right_a == right_b
			}
			_ => false,
		}
	}
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
	Equal,
	NotEqual,
	And,
	Or,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ScalarValue {
	Boolean(Option<bool>),
	Float32(Option<f32>),
	Float64(Option<f64>),
	Int8(Option<i8>),
	Int16(Option<i16>),
	Int32(Option<i32>),
	Int64(Option<i64>),
	UInt8(Option<u8>),
	UInt16(Option<u16>),
	UInt32(Option<u32>),
	UInt64(Option<u64>),
	Utf8(Option<String>),
	LargeUtf8(Option<String>),
	Binary(Option<Vec<u8>>),
	LargeBinary(Option<Vec<u8>>),
	Date32(Option<i32>),
	Date64(Option<i64>),
	TimestampSecond(Option<i64>),
	TimestampMillisecond(Option<i64>),
	TimestampMicrosecond(Option<i64>),
	TimestampNanosecond(Option<i64>),
	IntervalYearMonth(Option<i32>),
	IntervalDayTime(Option<i64>),
}
