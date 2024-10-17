use izolilib::cgroup::{cgroup::CGroup, controller::Controller};

fn main() {
    let cgroup = CGroup::new("test").unwrap();
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
}
