use super::{cpu_limit::CpuLimit, limit_value::CGroupLimitValue};

#[derive(Debug, Clone, Copy, Default)]
pub struct CGroupOption {
    pub cpu_max: Option<CpuLimit>,
    pub memory_max: Option<CGroupLimitValue<u32>>,
    pub pids_max: Option<CGroupLimitValue<u32>>,
}
