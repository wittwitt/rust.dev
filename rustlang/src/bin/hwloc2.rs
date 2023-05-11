use std::sync::Mutex;

use anyhow::{format_err, Result};
use log::*;

use hwloc::ObjectType;
use hwloc::Topology;

use hwloc::{Bitmap, TopologyObject, CPUBIND_THREAD};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TOPOLOGY: Mutex<Topology> = Mutex::new(Topology::new());
}

fn main() {
    flexi_logger::Logger::try_with_env()
        .unwrap()
        .start()
        .unwrap();

    let _f = bind_core(3).unwrap();

    let mut i = 0;
    loop {
        i = i + 1;
        if i > 9999999 {
            i = 0;
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub type ThreadId = libc::pthread_t;

#[cfg(not(target_os = "windows"))]
fn get_thread_id() -> ThreadId {
    unsafe { libc::pthread_self() }
}

pub struct Cleanup {
    tid: ThreadId,
    prior_state: Option<Bitmap>,
}

pub fn bind_core(core_index: usize) -> Result<Cleanup> {
    let child_topo = &TOPOLOGY;
    let tid = get_thread_id();
    let mut locked_topo = child_topo.lock().expect("poisoned lock");
    let core = get_core_by_index(&locked_topo, core_index)
        .map_err(|err| format_err!("failed to get core at index {}: {:?}", core_index, err))?;

    let cpuset = core
        .allowed_cpuset()
        .ok_or_else(|| format_err!("no allowed cpuset for core at index {}", core_index,))?;
    debug!("allowed cpuset: {:?}", cpuset);
    let mut bind_to = cpuset;

    // Get only one logical processor (in case the core is SMT/hyper-threaded).
    bind_to.singlify();

    // Thread binding before explicit set.
    let before = locked_topo.get_cpubind_for_thread(tid, CPUBIND_THREAD);

    debug!("binding to {:?}", bind_to);
    // Set the binding.
    let result = locked_topo
        .set_cpubind_for_thread(tid, bind_to, CPUBIND_THREAD)
        .map_err(|err| format_err!("failed to bind CPU: {:?}", err));

    if result.is_err() {
        warn!("error in bind_core, {:?}", result);
    }

    Ok(Cleanup {
        tid,
        prior_state: before,
    })
}

fn get_core_by_index(topo: &Topology, index: usize) -> Result<&TopologyObject> {
    let idx = index; //index.0;

    match topo.objects_with_type(&ObjectType::NUMANode) {
        Ok(all_cores) if idx < all_cores.len() => Ok(all_cores[idx]),
        Ok(all_cores) => Err(format_err!(
            "idx ({}) out of range for {} cores",
            idx,
            all_cores.len()
        )),
        _e => Err(format_err!("failed to get core by index {}", idx,)),
    }
}

// hwloc-ls
// lstopo
