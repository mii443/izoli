use std::{env::set_current_dir, fs, os::unix::fs::chroot, path::Path};

use nix::{
    libc::SIGCHLD,
    mount::{mount, umount, MsFlags},
    sched::{self, CloneCb, CloneFlags},
    unistd::{sethostname, Pid},
};
use tracing::{info, trace};

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
    pub mounts: Vec<Mount>,
}

#[derive(Debug, Clone, Default)]
pub struct Mount {
    pub target: String,
    pub source: String,
    pub readonly: bool,
    pub no_exec: bool,
}

impl Mount {
    pub fn new(target: &str, source: &str, readonly: bool, no_exec: bool) -> Self {
        Self {
            target: target.to_string(),
            source: source.to_string(),
            readonly,
            no_exec,
        }
    }
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
            self.prelude().unwrap();

            info!("running user code");
            callback();

            127
        });

        unsafe { sched::clone(new_callback, &mut stack, flags, Some(SIGCHLD)) }
    }

    fn prelude(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("box prelude");
        let root = self.get_root();
        let _ = fs::remove_dir(Path::new(&root));
        fs::create_dir_all(Path::new(&root))?;

        self.prelude_mount()?;

        info!("chroot to {}", root);
        chroot(&root)?;
        set_current_dir("/")?;

        sethostname(format!("IzoliBox"))?;
        Ok(())
    }

    fn get_root(&self) -> String {
        format!("/var/local/lib/izoli/{}", self.id)
    }

    fn prelude_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = self.get_root();

        Self::umount_mount(
            Some("none"),
            "/",
            None::<&str>,
            MsFlags::MS_REC | MsFlags::MS_PRIVATE,
            None::<&str>,
        )?;

        let mounts = [
            ("tmp", Some("tmpfs"), MsFlags::empty()),
            ("proc", Some("proc"), MsFlags::MS_RDONLY),
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

        for Mount {
            target,
            source,
            readonly,
            no_exec,
        } in self.options.mounts.clone()
        {
            let target: &str = &target;
            let source: &str = &source;
            let flag_noexec = if no_exec {
                MsFlags::MS_NOEXEC
            } else {
                MsFlags::empty()
            };

            let flag_rdonly = if readonly {
                MsFlags::MS_RDONLY
            } else {
                MsFlags::empty()
            };

            let full_target: &str = &format!("{}/{}", root, target.trim_start_matches("/"));
            info!("mounting {} to {}", source, full_target);
            fs::create_dir_all(full_target)?;

            mount(
                Some(source),
                full_target,
                Some("none"),
                MsFlags::MS_BIND | MsFlags::MS_REC,
                None::<&str>,
            )?;

            mount(
                None::<&str>,
                full_target,
                None::<&str>,
                MsFlags::MS_BIND
                    | MsFlags::MS_REMOUNT
                    | MsFlags::MS_REC
                    | flag_rdonly
                    | flag_noexec,
                None::<&str>,
            )?;
            trace!(
                "{:?}",
                MsFlags::MS_BIND
                    | MsFlags::MS_REMOUNT
                    | MsFlags::MS_REC
                    | flag_rdonly
                    | flag_noexec
            );
        }

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
