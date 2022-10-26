use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Op {
    Union,
    Intersection,
    Difference,
    SymmetricDifference,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Union => "∪",
                Self::Intersection => "∩",
                Self::Difference => "-",
                Self::SymmetricDifference => "⊖",
            }
        )
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Interval {
    Segment(i64, i64),
    Expression(Box<Interval>, Op, Box<Interval>),
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Segment(a, b) => {
                write!(f, "[{}, {}]", a, b)
            }
            Self::Expression(a, b, c) => {
                let sub_part = |ff: &mut Formatter, p: &Interval| {
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

impl From<(i64, i64)> for Interval {
    fn from(segment: (i64, i64)) -> Self {
        Self::Segment(segment.0, segment.1)
    }
}

impl<A, B> From<(A, Op, B)> for Interval
where
    A: Into<Interval>,
    B: Into<Interval>,
{
    fn from(expression: (A, Op, B)) -> Self {
        Self::Expression(
            Box::new(expression.0.into()),
            expression.1,
            Box::new(expression.2.into()),
        )
    }
}

impl Interval {
    pub fn contains(&self, num: &i64) -> bool {
        match self {
            Self::Segment(a, b) => (a <= num) & (num <= b),
            Self::Expression(a, op, b) => match op {
                Op::Union => match a as &Interval {
                    Self::Segment(_, _) => a.contains(num) || b.contains(num),
                    _ => b.contains(num) || a.contains(num),
                },
                Op::Intersection => a.contains(num) && b.contains(num),
                Op::Difference => a.contains(num) && !b.contains(num),
                Op::SymmetricDifference => a.contains(num) ^ b.contains(num),
            },
        }
    }
}

fn main() {
    let interval = Interval::from((
        Interval::from((
            (10, 40),
            Op::Union,
            Interval::from(((100, 200), Op::Intersection, (125, 175))),
        )),
        Op::Difference,
        Interval::from(((20, 40), Op::Intersection, (10, 30))),
    ));
    println!("{}", interval,);
    println!("{}", interval.contains(&25),)
}
