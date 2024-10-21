use std::{env::set_current_dir, fs, os::unix::fs::chroot, path::Path};

use nix::{
    errno::Errno,
    libc::SIGCHLD,
    mount::{mount, umount, MsFlags},
    sched::{self, CloneCb, CloneFlags},
    unistd::{sethostname, Pid},
};

use crate::cgroup::{cgroup::CGroup, cgroup_option::CGroupOption};

const STACK_SIZE: usize = 8192;

pub struct IzoliBox {
    pub id: usize,
    pub cgroup_option: Option<CGroupOption>,
}

impl IzoliBox {
    pub fn new(id: usize, cgroup_option: Option<CGroupOption>) -> Self {
        Self { id, cgroup_option }
    }

    pub fn enter(&self, callback: CloneCb<'_>) -> Result<Pid, nix::errno::Errno> {
        let mut stack = [0u8; STACK_SIZE];
        let flags = CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWPID;

        if let Some(cgroup_option) = &self.cgroup_option {
            let cgroup = CGroup::new(&format!("izoli/box_{}", self.id)).unwrap();
            cgroup.apply_options(cgroup_option).unwrap();
            cgroup.enter().unwrap();
        }

        unsafe { sched::clone(callback, &mut stack, flags, Some(SIGCHLD)) }
    }

    pub fn prelude(id: usize) -> Result<(), Box<dyn std::error::Error>> {
        let root = format!("/var/local/lib/izoli/{}", id);
        fs::create_dir_all(Path::new(&root))?;

        Self::umount_mount(
            Some("none"),
            "/",
            None::<&str>,
            MsFlags::MS_REC | MsFlags::MS_PRIVATE,
            None::<&str>,
        )?;

        for dir in &[
            "/proc", "/dev", "/tmp", "/lib", "/usr", "/bin", "/lib64", "/usr/lib", "/usr/bin",
        ] {
            fs::create_dir_all(format!("{}{}", root, dir))?;
        }

        let mounts = [
            ("tmp", "tmpfs", MsFlags::empty()),
            ("proc", "proc", MsFlags::empty()),
            ("dev", "devtmpfs", MsFlags::empty()),
            ("lib", "/lib", MsFlags::MS_BIND | MsFlags::MS_REC),
            ("usr/lib", "/usr/lib", MsFlags::MS_BIND | MsFlags::MS_REC),
            ("usr/bin", "/usr/bin", MsFlags::MS_BIND | MsFlags::MS_REC),
            ("bin", "/bin", MsFlags::MS_BIND | MsFlags::MS_REC),
            ("lib64", "/lib64", MsFlags::MS_BIND | MsFlags::MS_REC),
        ];

        for (target, source, flags) in mounts.iter() {
            let full_target = format!("{}/{}", root, target);
            Self::umount_mount(
                Some(source),
                &full_target,
                Some(source),
                *flags,
                None::<&str>,
            )?;
        }

        chroot(&root)?;
        set_current_dir("/")?;

        sethostname(format!("IzoliBox"))?;
        Ok(())
    }

    fn umount_mount<P: AsRef<Path>>(
        source: Option<&str>,
        target: P,
        fstype: Option<&str>,
        flags: MsFlags,
        data: Option<&str>,
    ) -> Result<(), nix::Error> {
        let target_path = target.as_ref();

        let _ = umount(target_path);

        mount(source, target_path, fstype, flags, data)
    }
}
