use std::str::FromStr;

#[derive(Debug)]
pub enum CGroupLimitValue<T>
where
    T: FromStr,
{
    Max,
    Value(T),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCGroupLimitValueError;

impl<T> FromStr for CGroupLimitValue<T>
where
    T: FromStr,
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
