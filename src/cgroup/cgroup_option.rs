use super::cpu_limit::CpuLimit;

#[derive(Debug, Clone, Copy, Default)]
pub struct CGroupOption {
    pub cpu_max: Option<CpuLimit>,
}
