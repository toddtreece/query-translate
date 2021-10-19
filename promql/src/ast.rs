use ast::time_series::{Expr, Operator, ScalarValue, Time};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LabelMatcherOperator {
    Equal,
    NotEqual,
    Match,
    NotMatch,
}

impl fmt::Display for LabelMatcherOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Equal => write!(f, "="),
            Self::NotEqual => write!(f, "!="),
            Self::Match => write!(f, "=~"),
            Self::NotMatch => write!(f, "!~"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelMatcher {
    pub name: String,
    pub operator: LabelMatcherOperator,
    pub value: String,
}
impl fmt::Display for LabelMatcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}\"{}\"", self.name, self.operator, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Query {
    pub metric_name: Option<String>,
    pub label_matchers: Vec<LabelMatcher>,
    pub range_vector: Option<Time>,
}

impl From<ast::time_series::Query> for Query {
    fn from(q: ast::time_series::Query) -> Self {
        q.0.into_iter().fold(Query::default(), |mut r, e| {
            match e {
                Expr::Metric(n) => r.metric_name = Some(n),
                Expr::TimeRange { from: Some(time), to: None } => {
                    r.range_vector = Some(time)
                }
                Expr::BinaryExpression {
                    left: box Expr::Label(name),
                    right: box Expr::Literal(ScalarValue::Utf8(Some(value))),
                    ..
                } => r.label_matchers.push(LabelMatcher {
                    name: name.clone(),
                    value: value.clone(),
                    operator: LabelMatcherOperator::Equal,
                }),
                _ => {}
            }
            r
        })
    }
}

impl From<Query> for ast::time_series::Query {
    fn from(q: Query) -> Self {
        let mut r = ast::time_series::Query(vec![]);
        if let Some(metric) = q.metric_name.clone() {
            r.0.push(Expr::Metric(metric));
        }

        if q.label_matchers.len() > 0 {
            let mut labels = q
                .label_matchers
                .iter()
                .map(|l| Expr::BinaryExpression {
                    left: Box::new(Expr::Label(l.name.clone())),
                    op: Operator::Equal,
                    right: Box::new(Expr::Literal(ScalarValue::Utf8(Some(
                        l.value.clone(),
                    )))),
                })
                .collect::<Vec<Expr>>();
            r.0.append(&mut labels);
        }

        if let Some(_) = q.range_vector.clone() {
            r.0.push(Expr::TimeRange { from: q.range_vector, to: None });
        }

        r
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(metric) = self.metric_name.clone() {
            write!(f, "{}", metric)?;
        }

        if self.label_matchers.len() > 0 {
            write!(
                f,
                "{{{}}}",
                self.label_matchers
                    .iter()
                    .map(|l| format!("{}", l))
                    .collect::<Vec<String>>()
                    .join(",")
            )?;
        }

        if let Some(Time::Relative(t)) = self.range_vector.clone() {
            write!(f, "[{}]", t)?;
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod tests {

    use ast::time_series::{
        Expr, Interval, Operator, Query, ScalarValue, Time, TimeUnit,
    };

    #[test]
    fn range_vector() {
        assert_eq!(
            "metric_name{test_one=\"one\"}[50w]",
            format!(
                "{}",
                super::Query::from(Query(vec![
                    Expr::Metric("metric_name".to_owned()),
                    Expr::BinaryExpression {
                        left: Box::new(Expr::Label("test_one".to_owned())),
                        op: Operator::Equal,
                        right: Box::new(Expr::Literal(ScalarValue::Utf8(
                            Some("one".to_owned())
                        )))
                    },
                    Expr::TimeRange {
                        from: Some(Time::Relative(Interval {
                            value: 50,
                            unit: TimeUnit::Week
                        })),
                        to: None
                    },
                ]))
            )
        );
    }
}
