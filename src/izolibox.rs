use std::{env::set_current_dir, fs, os::unix::fs::chroot, path::Path};

use nix::{
    libc::SIGCHLD,
    mount::{mount, umount, MsFlags},
    sched::{self, CloneCb, CloneFlags},
    unistd::{sethostname, Pid},
};
use tracing::info;

use crate::cgroup::{cgroup::CGroup, cgroup_option::CGroupOption};

const STACK_SIZE: usize = 8192;

pub struct IzoliBox {
    pub id: usize,
    pub options: IzoliBoxOptions,
}

#[derive(Debug, Clone, Default)]
pub struct IzoliBoxOptions {
    pub cgroup_option: Option<CGroupOption>,
    pub new_net: bool,
}

impl IzoliBox {
    pub fn new(id: usize, options: IzoliBoxOptions) -> Self {
        Self { id, options }
    }

    pub fn enter(&self, callback: CloneCb<'_>) -> Result<Pid, nix::errno::Errno> {
        info!("box enter");
        let mut stack = [0u8; STACK_SIZE];
        let mut flags = CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWPID;

        if self.options.new_net {
            flags = flags | CloneFlags::CLONE_NEWNET;
        }

        if let Some(cgroup_option) = &self.options.cgroup_option {
            let cgroup = CGroup::new(&format!("izoli/box_{}", self.id)).unwrap();
            cgroup.apply_options(cgroup_option).unwrap();
            cgroup.enter().unwrap();
        }

        let mut callback = callback;
        let new_callback = Box::new(|| {
            IzoliBox::prelude(self.id).unwrap();

            callback();

            127
        });

        unsafe { sched::clone(new_callback, &mut stack, flags, Some(SIGCHLD)) }
    }

    pub fn prelude(id: usize) -> Result<(), Box<dyn std::error::Error>> {
        info!("box prelude");
        let root = format!("/var/local/lib/izoli/{}", id);
        fs::create_dir_all(Path::new(&root))?;

        Self::umount_mount(
            Some("none"),
            "/",
            None::<&str>,
            MsFlags::MS_REC | MsFlags::MS_PRIVATE,
            None::<&str>,
        )?;

        let mounts = [
            ("tmp", Some("tmpfs"), MsFlags::empty()),
            ("proc", Some("proc"), MsFlags::empty()),
        ];

        for (target, source, flags) in mounts.iter() {
            info!("mounting {} {:?} {:?}", target, source, flags);
            fs::create_dir_all(format!("{}/{}", root, target))?;
            let full_target = format!("{}/{}", root, target);
            Self::umount_mount(
                source.clone(),
                &full_target,
                source.clone(),
                *flags,
                None::<&str>,
            )?;
        }

        // readonly monut
        let mounts = [
            ("bin", "/bin"),
            ("usr/bin", "/usr/bin"),
            ("usr/lib", "/usr/lib"),
            ("lib", "/usr/lib"),
            ("lib64", "/usr/lib64"),
        ];

        for (target, source) in mounts.iter() {
            let target: &str = &format!("{}/{}", root, target);
            info!("mounting {} to {} readonly", source, target);
            fs::create_dir_all(target)?;
            mount(
                Some(*source),
                target,
                Some("none"),
                MsFlags::MS_BIND | MsFlags::MS_REC,
                None::<&str>,
            )?;

            mount(
                None::<&str>,
                target,
                None::<&str>,
                MsFlags::MS_BIND | MsFlags::MS_REMOUNT | MsFlags::MS_RDONLY | MsFlags::MS_REC,
                None::<&str>,
            )?;
        }

        info!("chroot to {}", root);
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
