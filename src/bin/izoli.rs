use std::ffi::CString;

use izolilib::{
    cgroup::{
        cgroup::CGroup, cgroup_option::CGroupOption, cpu_limit::CpuLimit,
        limit_value::CGroupLimitValue,
    },
    izolibox::{IzoliBox, IzoliBoxOptions},
};
use nix::{sys::wait::waitpid, unistd::execvp};
use tracing::Level;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let cgroup = CGroup::new("izoli").unwrap();

    cgroup
        .add_subtree_control(cgroup.get_controllers().unwrap())
        .unwrap();

    let izolibox = IzoliBox::new(
        1,
        IzoliBoxOptions {
            cgroup_option: Some(CGroupOption {
                cpu_max: Some(CpuLimit {
                    max: CGroupLimitValue::Max,
                    period: 100000,
                }),
                memory_max: Some(CGroupLimitValue::Value(1024 * 1024 * 2)),
                ..Default::default()
            }),
            new_net: true,
        },
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
