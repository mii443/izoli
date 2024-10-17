use std::{fmt, str::FromStr};

#[derive(Debug)]
pub enum CGroupLimitValue<T>
where
    T: FromStr + std::fmt::Display,
{
    Max,
    Value(T),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCGroupLimitValueError;

impl<T> FromStr for CGroupLimitValue<T>
where
    T: FromStr + std::fmt::Display,
{
    type Err = ParseCGroupLimitValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim() == "max" {
            Ok(Self::Max)
        } else {
            if let Ok(value) = T::from_str(s) {
                Ok(Self::Value(value))
            } else {
                Err(ParseCGroupLimitValueError)
            }
        }
    }
}

impl<T> fmt::Display for CGroupLimitValue<T>
where
    T: FromStr + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CGroupLimitValue::Max => write!(f, "max"),
            CGroupLimitValue::Value(value) => write!(f, "{value}"),
        }
    }
}
