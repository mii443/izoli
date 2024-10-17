use std::str::FromStr;

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
