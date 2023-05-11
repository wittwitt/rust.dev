use std::sync::Mutex;

use hwloc::ObjectType;
use hwloc::Topology;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TOPOLOGY: Mutex<Topology> = Mutex::new(Topology::new());
}

fn main() {
    // let topo = Topology::new();

    let topo = TOPOLOGY.lock().expect("poisoned lock");

    for i in 0..topo.depth() {
        println!("*** Objects at level {}", i);

        for (idx, object) in topo.objects_at_depth(i).iter().enumerate() {
            println!(
                "{}: {},{},{:?}",
                idx,
                object,
                object.depth(),
                object.object_type()
            );
        }
    }

    let core_depth = match topo.depth_or_below_for_type(&ObjectType::Core) {
        Ok(depth) => depth,
        Err(_) => 1234567,
    };

    println!("core_depth:{}", core_depth)
}
