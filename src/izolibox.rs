use nix::{
    libc::SIGCHLD,
    sched::{self, CloneCb, CloneFlags},
    unistd::Pid,
};

const STACK_SIZE: usize = 8192;

pub struct IzoliBox {}

impl IzoliBox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn enter(&self, callback: CloneCb<'_>) -> Result<Pid, nix::errno::Errno> {
        let mut stack = [0u8; STACK_SIZE];
        let flags = CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWPID;

        unsafe { sched::clone(callback, &mut stack, flags, Some(SIGCHLD)) }
    }
}
