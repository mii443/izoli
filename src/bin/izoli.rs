use izolilib::{
    cgroup::cgroup::CGroup,
    izolibox::{CGroupOption, IzoliBox},
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

    let izolibox = IzoliBox::new(1, Some(CGroupOption {}));
    let pid = izolibox
        .enter(Box::new(|| {
            println!("Isolated process: {}", std::process::id());
            println!("cgroup: {:?}", CGroup::get_self_cgroup());
            127
        }))
        .unwrap();
    println!("Box real PID: {:?}", pid);
}
