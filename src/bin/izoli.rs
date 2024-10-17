use izolilib::cgroup::cgroup::CGroup;

fn main() {
    let cgroup = CGroup::new("test").unwrap();
    println!("{:?}", cgroup.get_root_path());
    println!("{}", cgroup.check_status());
    println!("{:?}", cgroup.read("cgroup.type"));
    println!("{:?}", cgroup.get_controllers());
    println!("{:?}", cgroup.get_procs());
    println!("{:?}", cgroup.get_stat());
}
