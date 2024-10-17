use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Controller {
    Cpu,
    Cpuset,
    Memory,
    Io,
    Hugetlb,
    Misc,
    Pids,
    Rdma,
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseControllerError;

impl FromStr for Controller {
    type Err = ParseControllerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s {
            "cpu" => Ok(Self::Cpu),
            "cpuset" => Ok(Self::Cpuset),
            "memory" => Ok(Self::Memory),
            "io" => Ok(Self::Io),
            "hugetlb" => Ok(Self::Hugetlb),
            "misc" => Ok(Self::Misc),
            "pids" => Ok(Self::Pids),
            "rdma" => Ok(Self::Rdma),
            _ => Err(ParseControllerError),
        }
    }
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Controller::Cpu => write!(f, "cpu"),
            Controller::Cpuset => write!(f, "cpuset"),
            Controller::Memory => write!(f, "memory"),
            Controller::Io => write!(f, "io"),
            Controller::Hugetlb => write!(f, "hugetlb"),
            Controller::Misc => write!(f, "misc"),
            Controller::Pids => write!(f, "pids"),
            Controller::Rdma => write!(f, "rdma"),
            Controller::Unknown => write!(f, "unknown"),
        }
    }
}
