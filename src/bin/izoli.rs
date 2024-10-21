use std::{ffi::CString, process::Command};

use izolilib::{
    cgroup::{cgroup::CGroup, cgroup_option::CGroupOption, cpu_limit::CpuLimit},
    izolibox::IzoliBox,
};
use nix::{
    sys::wait::waitpid,
    unistd::{execvp, sethostname},
};

fn main() {
    let cgroup = CGroup::new("izoli").unwrap();
    println!("{:?}", cgroup.get_root_path());
    println!("{}", cgroup.check_status());
    println!("{:?}", cgroup.read("cgroup.type"));
    println!("{:?}", cgroup.get_controllers());
    println!("{:?}", cgroup.get_subtree_control());
    println!("{:?}", cgroup.get_procs());
    println!("{:?}", cgroup.get_threads());
    println!("{:?}", cgroup.get_stat());
    println!("{:?}", cgroup.get_max_depth());
    println!("{:?}", cgroup.get_max_descendants());
    println!("{:?}", cgroup.get_cpu_max());

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
            sethostname(format!("IzoliBox")).unwrap();
            println!("Isolated process: {}", std::process::id());
            println!("cgroup: {:?}", CGroup::get_self_cgroup());

            let cmd = CString::new("bash").unwrap();
            let args = vec![
                CString::new("containered bash").unwrap(),
                CString::new("-l").unwrap(),
            ];
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
