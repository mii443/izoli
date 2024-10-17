use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    str::FromStr,
};

use super::controller::Controller;

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

    pub fn read(&self, name: &str) -> Result<String, std::io::Error> {
        let path = self.get_file_path(name);
        let mut file = File::open(path)?;
        let mut buf = String::default();
        file.read_to_string(&mut buf)?;

        Ok(buf)
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

    pub fn get_controllers(&self) -> Result<Vec<Controller>, std::io::Error> {
        let controllers = self
            .read("cgroup.controllers")?
            .trim()
            .split(" ")
            .map(|controller| Controller::from_str(controller).unwrap_or(Controller::Unknown))
            .collect();

        Ok(controllers)
    }

    pub fn get_procs(&self) -> Result<Vec<u32>, std::io::Error> {
        let procs = self
            .read("cgroup.procs")?
            .lines()
            .map(|proc| u32::from_str(proc.trim()).unwrap())
            .collect();

        Ok(procs)
    }
}
