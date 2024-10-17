use std::{
    fs,
    path::{Path, PathBuf},
};

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

    pub fn check_status(&self) -> bool {
        let root = self.get_root_path();

        root.exists() && root.is_dir()
    }

    pub fn get_root_path(&self) -> PathBuf {
        let cgroup_root = PathBuf::from("/sys/fs/cgroup/");
        let root = cgroup_root.join(&self.path);

        root
    }
}
