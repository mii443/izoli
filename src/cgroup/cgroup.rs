use std::{
    fmt,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

use tracing::info;

use super::{
    cgroup_option::CGroupOption, cgroup_stat::CGroupStat, controller::Controller,
    cpu_limit::CpuLimit, limit_value::CGroupLimitValue,
};

pub struct CGroup {
    pub path: PathBuf,
}

impl Drop for CGroup {
    fn drop(&mut self) {
        let root = self.get_root_path();
        let _ = fs::remove_dir(root);
    }
}

impl CGroup {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        info!("creating new cgroup");
        let cgroup = CGroup {
            path: PathBuf::from(path),
        };

        if !cgroup.check_status() {
            info!("cgroup not exists. creating");
            cgroup.create()?;
        }

        Ok(cgroup)
    }

    pub fn get_self_cgroup() -> Result<String, std::io::Error> {
        let mut file = std::fs::File::open("/proc/self/cgroup")?;
        let mut buf = String::default();
        file.read_to_string(&mut buf)?;
        info!("self cgroup: {}", buf);

        Ok(buf.trim().to_string())
    }

    fn create(&self) -> Result<(), std::io::Error> {
        let root = self.get_root_path();
        fs::create_dir_all(root)
    }

    pub fn apply_options(&self, option: &CGroupOption) -> Result<(), std::io::Error> {
        info!("applying cgroup options");
        if let Some(cpu_max) = &option.cpu_max {
            info!("setting cpu.max");
            self.set_cpu_max(cpu_max)?;
        }

        if let Some(memory_max) = &option.memory_max {
            info!("setting memory.max");
            self.set_memory_max(memory_max)?;
        }

        if let Some(pids_max) = &option.pids_max {
            info!("setting pids.max");
            self.set_pids_max(pids_max)?;
        }

        if let Some(cpus) = &option.cpus {
            info!("setting cpuset.cpus");
            self.set_cpuset_cpus(cpus)?;
        }

        Ok(())
    }

    pub fn enter(&self) -> Result<(), std::io::Error> {
        let pid = std::process::id();
        info!("cgroup enter: {}", pid);

        self.add_procs(vec![pid])
    }

    pub fn read(&self, name: &str) -> Result<String, std::io::Error> {
        info!("reading {}", name);
        let path = self.get_file_path(name);
        let mut file = File::open(path)?;
        let mut buf = String::default();
        file.read_to_string(&mut buf)?;

        Ok(buf)
    }

    pub fn write(&self, name: &str, data: &str) -> Result<(), std::io::Error> {
        info!("writing {} to {}", data, name);
        let path = self.get_file_path(name);
        let mut file = File::options().append(true).open(path)?;
        file.write_all(data.as_bytes())?;

        Ok(())
    }

    pub fn check_status(&self) -> bool {
        let root = self.get_root_path();

        root.exists() && root.is_dir()
    }

    pub fn get_file_path(&self, name: &str) -> PathBuf {
        let root = self.get_root_path();

        root.join(name)
    }

    pub fn get_root_path(&self) -> PathBuf {
        let cgroup_root = PathBuf::from("/sys/fs/cgroup/");

        cgroup_root.join(&self.path)
    }

    // cgroup files read

    pub fn get_controllers(&self) -> Result<Vec<Controller>, std::io::Error> {
        self.inner_get_controllers("cgroup.controllers")
    }

    pub fn get_subtree_control(&self) -> Result<Vec<Controller>, std::io::Error> {
        self.inner_get_controllers("cgroup.subtree_control")
    }

    fn inner_get_controllers(&self, name: &str) -> Result<Vec<Controller>, std::io::Error> {
        let controllers = self
            .read(name)?
            .trim()
            .split(" ")
            .map(|controller| Controller::from_str(controller).unwrap_or(Controller::Unknown))
            .collect();

        Ok(controllers)
    }

    pub fn get_procs(&self) -> Result<Vec<u32>, std::io::Error> {
        self.get_u32_list("cgroup.procs")
    }

    pub fn get_threads(&self) -> Result<Vec<u32>, std::io::Error> {
        self.get_u32_list("cgroup.threads")
    }

    pub fn get_stat(&self) -> Result<CGroupStat, std::io::Error> {
        let stat = self.read("cgroup.stat")?;

        Ok(CGroupStat::from_str(&stat).unwrap())
    }

    pub fn get_max_depth(&self) -> Result<CGroupLimitValue<u64>, std::io::Error> {
        self.get_limit_value("cgroup.max.depth")
    }

    pub fn get_max_descendants(&self) -> Result<CGroupLimitValue<u64>, std::io::Error> {
        self.get_limit_value("cgroup.max.descendants")
    }

    // cgroup files write

    pub fn add_subtree_control(&self, controllers: Vec<Controller>) -> Result<(), std::io::Error> {
        let to_write = controllers
            .iter()
            .map(|controller| format!("+{}", controller))
            .collect::<Vec<String>>()
            .join(" ");

        self.write("cgroup.subtree_control", &to_write)?;

        Ok(())
    }

    pub fn remove_subtree_control(
        &self,
        controllers: Vec<Controller>,
    ) -> Result<(), std::io::Error> {
        let to_write = controllers
            .iter()
            .map(|controller| format!("-{}", controller))
            .collect::<Vec<String>>()
            .join(" ");

        self.write("cgroup.subtree_control", &to_write)?;

        Ok(())
    }

    pub fn set_max_depth(&self, max: CGroupLimitValue<u64>) -> Result<(), std::io::Error> {
        self.write_value("cgroup.max.depth", max)
    }

    pub fn set_max_descendants(&self, max: CGroupLimitValue<u64>) -> Result<(), std::io::Error> {
        self.write_value("cgroup.max.descendants", max)
    }

    pub fn add_procs(&self, procs: Vec<u32>) -> Result<(), std::io::Error> {
        self.write_list("cgroup.procs", procs)
    }

    pub fn add_threads(&self, threads: Vec<u32>) -> Result<(), std::io::Error> {
        self.write_list("cgroup.threads", threads)
    }

    // cpu read

    pub fn get_cpu_max(&self) -> Result<CpuLimit, std::io::Error> {
        let max = self.read("cpu.max")?;

        Ok(CpuLimit::from_str(&max).unwrap())
    }

    // cpu write

    pub fn set_cpu_max(&self, cpu_limit: &CpuLimit) -> Result<(), std::io::Error> {
        let to_write = cpu_limit.to_string();

        self.write("cpu.max", &to_write)
    }

    // memory read

    pub fn get_memory_max(&self) -> Result<CGroupLimitValue<u32>, std::io::Error> {
        let max = self.read("memory.max")?;

        Ok(CGroupLimitValue::from_str(&max).unwrap())
    }

    // memory write

    pub fn set_memory_max(
        &self,
        memory_limit: &CGroupLimitValue<u32>,
    ) -> Result<(), std::io::Error> {
        let to_write = memory_limit.to_string();

        self.write("memory.max", &to_write)
    }

    // pids read

    pub fn get_pids_max(&self) -> Result<CGroupLimitValue<u32>, std::io::Error> {
        let max = self.read("pids.max")?;

        Ok(CGroupLimitValue::from_str(&max).unwrap())
    }

    // pids write

    pub fn set_pids_max(&self, pids_limit: &CGroupLimitValue<u32>) -> Result<(), std::io::Error> {
        let to_write = pids_limit.to_string();

        self.write("pids.max", &to_write)
    }

    // cpuset read

    pub fn get_cpuset_cpus(&self) -> Result<Vec<u32>, std::io::Error> {
        self.get_u32_list("cpuset.cpus")
    }

    // cpuset write

    pub fn set_cpuset_cpus(&self, cpus: &Vec<u32>) -> Result<(), std::io::Error> {
        self.write_list("cpuset.cpus", cpus.clone())
    }

    fn write_value<T>(&self, name: &str, value: T) -> Result<(), std::io::Error>
    where
        T: fmt::Display,
    {
        self.write(name, &value.to_string())?;
        Ok(())
    }

    fn write_list<T>(&self, name: &str, value: Vec<T>) -> Result<(), std::io::Error>
    where
        T: fmt::Display,
    {
        let to_write = value
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        self.write(name, &to_write)?;

        Ok(())
    }

    fn get_u32_list(&self, name: &str) -> Result<Vec<u32>, std::io::Error> {
        let procs = self
            .read(name)?
            .lines()
            .map(|proc| u32::from_str(proc.trim()).unwrap())
            .collect();

        Ok(procs)
    }

    fn get_limit_value<T>(&self, name: &str) -> Result<CGroupLimitValue<T>, std::io::Error>
    where
        T: FromStr + fmt::Display,
    {
        let value = self.read(name)?;

        Ok(CGroupLimitValue::from_str(&value).unwrap())
    }
}
