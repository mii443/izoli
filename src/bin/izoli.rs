use std::ffi::CString;

use izolilib::{
    cgroup::{
        cgroup::CGroup, cgroup_option::CGroupOption, cpu_limit::CpuLimit,
        limit_value::CGroupLimitValue,
    },
    izolibox::{IzoliBox, IzoliBoxOptions, Mount},
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
                memory_max: Some(CGroupLimitValue::Value(1024 * 1024 * 1024)),
                pids_max: Some(CGroupLimitValue::Value(10)),
                cpus: Some(vec![0]),
                ..Default::default()
            }),
            new_net: false,
            mounts: vec![
                Mount::new("/bin", "/bin", true, false),
                Mount::new("/usr/bin", "/usr/bin", true, false),
                Mount::new("/lib", "/lib", true, false),
                Mount::new("/lib64", "/lib64", true, false),
                Mount::new("/usr/lib", "/usr/lib", true, false),
                Mount::new("/usr/lib64", "/usr/lib64", true, false),
                Mount::new("/etc", "/etc", true, true),
            ],
        },
    );

    let pid = izolibox
        .enter(Box::new(|| {
            let cmd = CString::new("/usr/bin/bash").unwrap();
            let args: Vec<CString> = vec![];

            #[allow(irrefutable_let_patterns)]
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
