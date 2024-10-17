use izolilib::cgroup::cgroup::CGroup;

fn main() {
    println!("test");
    let cgroup = CGroup::new("test").unwrap();
    println!("{:?}", cgroup.get_root_path());
    println!("{}", cgroup.check_status());
    println!("{:?}", cgroup.read("cgroup.controllers"));
}
