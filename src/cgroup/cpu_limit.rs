use core::fmt;
use std::str::FromStr;

use super::limit_value::CGroupLimitValue;

#[derive(Debug, Clone, Copy)]
pub struct CpuLimit {
    pub max: CGroupLimitValue<u64>,
    pub period: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCpuLimitError;

impl FromStr for CpuLimit {
    type Err = ParseCpuLimitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.trim().split(" ");

        let max = if let Some(max) = spl.next() {
            if let Ok(max) = CGroupLimitValue::from_str(max) {
                max
            } else {
                return Err(ParseCpuLimitError);
            }
        } else {
            return Err(ParseCpuLimitError);
        };

        let period = if let Some(period) = spl.next() {
            if let Ok(period) = u64::from_str(period) {
                period
            } else {
                return Err(ParseCpuLimitError);
            }
        } else {
            return Err(ParseCpuLimitError);
        };

        Ok(Self { max, period })
    }
}

impl fmt::Display for CpuLimit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.max, self.period)
    }
}
