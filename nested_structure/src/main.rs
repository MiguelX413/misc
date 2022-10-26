use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Operation {
    Union,
    Intersection,
    Difference,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Union => {
                    "∪"
                }
                Self::Intersection => {
                    "∩"
                }
                Self::Difference => {
                    "-"
                }
            }
        )
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Part {
    Segment(i64, i64),
    Expression(Box<Part>, Operation, Box<Part>),
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Segment(a, b) => {
                write!(f, "[{}, {}]", a, b)
            }
            Self::Expression(a, b, c) => {
                let sub_part = |ff: &mut Formatter, p: &Part| {
                    match p {
                        Self::Segment(_, _) => {
                            write!(ff, "{}", p)?;
                        }
                        Self::Expression(_, _, _) => {
                            write!(ff, "({})", p)?;
                        }
                    }
                    Ok(())
                };

                sub_part(f, a)?;
                write!(f, " {} ", b)?;
                sub_part(f, c)?;
                Ok(())
            }
        }
    }
}

impl From<(i64, i64)> for Part {
    fn from(segment: (i64, i64)) -> Self {
        Self::Segment(segment.0, segment.1)
    }
}

impl<A, B> From<(A, Operation, B)> for Part
where
    A: Into<Part>,
    B: Into<Part>,
{
    fn from(expression: (A, Operation, B)) -> Self {
        Self::Expression(
            Box::new(expression.0.into()),
            expression.1,
            Box::new(expression.2.into()),
        )
    }
}

fn main() {
    println!(
        "{}",
        Part::from((
            Part::from((
                (10, 40),
                Operation::Union,
                Part::from(((100, 200), Operation::Intersection, (125, 175)))
            )),
            Operation::Difference,
            Part::from(((20, 40), Operation::Intersection, (10, 30)))
        ))
    );
}
