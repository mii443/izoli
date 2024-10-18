use std::{
    fmt,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

use super::{cgroup_stat::CGroupStat, controller::Controller, limit_value::CGroupLimitValue};

pub struct CGroup {
    pub path: PathBuf,
}

impl CGroup {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let cgroup = CGroup {
            path: PathBuf::from(path),
        };

        if !cgroup.check_status() {
            cgroup.create()?;
        }

        Ok(cgroup)
    }

    fn create(&self) -> Result<(), std::io::Error> {
        let root = self.get_root_path();
        fs::create_dir_all(root)
    }

    pub fn enter(&self) -> Result<(), std::io::Error> {
        let pid = std::process::id();

        self.add_procs(vec![pid])
    }

    pub fn read(&self, name: &str) -> Result<String, std::io::Error> {
        let path = self.get_file_path(name);
        let mut file = File::open(path)?;
        let mut buf = String::default();
        file.read_to_string(&mut buf)?;

        Ok(buf)
    }

    pub fn write(&self, name: &str, data: &str) -> Result<(), std::io::Error> {
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
