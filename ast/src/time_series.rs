use std::boxed::Box;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Query(pub Vec<Expr>);

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Metric(String),
    Label(String),
    Literal(ScalarValue),
    TimeRange { to: Option<Time>, from: Option<Time> },
    BinaryExpression { left: Box<Expr>, op: Operator, right: Box<Expr> },
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Equal,
    NotEqual,
    Match,
    NotMatch,
    And,
    Or,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Equal => write!(f, "="),
            Self::NotEqual => write!(f, "!="),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeUnit {
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Millisecond => write!(f, "ms"),
            Self::Second => write!(f, "s"),
            Self::Minute => write!(f, "m"),
            Self::Hour => write!(f, "h"),
            Self::Day => write!(f, "d"),
            Self::Week => write!(f, "w"),
            Self::Month => write!(f, "mon"),
            Self::Quarter => write!(f, "q"),
            Self::Year => write!(f, "y"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Time {
    Relative(Interval),
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Relative(t) => write!(f, "{}", t),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub value: u64,
    pub unit: TimeUnit,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

#[derive(Debug, Clone, PartialEq)]
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
