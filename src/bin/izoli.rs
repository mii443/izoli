use std::ffi::CString;

use izolilib::{
    cgroup::{cgroup::CGroup, cgroup_option::CGroupOption, cpu_limit::CpuLimit},
    izolibox::IzoliBox,
};
use nix::{sys::wait::waitpid, unistd::execvp};

fn main() {
    let cgroup = CGroup::new("izoli").unwrap();

    cgroup
        .add_subtree_control(cgroup.get_controllers().unwrap())
        .unwrap();

    let izolibox = IzoliBox::new(
        1,
        Some(CGroupOption {
            cpu_max: Some(CpuLimit {
                max: izolilib::cgroup::limit_value::CGroupLimitValue::Max,
                period: 100000,
            }),
        }),
    );
    let pid = izolibox
        .enter(Box::new(|| {
            IzoliBox::prelude(1).unwrap();
            println!("Isolated process: {}", std::process::id());

            let cmd = CString::new("/usr/bin/bash").unwrap();
            let args: Vec<CString> = vec![];
            if let Err(e) = execvp(&cmd, &args.as_ref()) {
                eprintln!("execvp failed: {:?}", e);
                return 127;
            }

            127
        }))
        .unwrap();

    if let Ok(status) = waitpid(pid, None) {
        println!("{:?}", status);
    }

    println!("Box real PID: {:?}", pid);
}
