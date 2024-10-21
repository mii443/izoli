use nix::{
    libc::SIGCHLD,
    sched::{self, CloneCb, CloneFlags},
    unistd::Pid,
};

use crate::cgroup::cgroup::CGroup;

const STACK_SIZE: usize = 8192;

pub struct CGroupOption {}

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

        if let Some(_cgroup_option) = &self.cgroup_option {
            let cgroup = CGroup::new(&format!("izoli/box_{}", self.id)).unwrap();
            cgroup.enter().unwrap();
        }

        unsafe { sched::clone(callback, &mut stack, flags, Some(SIGCHLD)) }
    }
}
