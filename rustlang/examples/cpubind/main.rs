use anyhow::{format_err, Result};
use hwloc::{Bitmap, ObjectType, Topology, TopologyObject, CPUBIND_THREAD};
use lazy_static::lazy_static;
use log::{debug, info, warn};
use std::sync::{Arc, Mutex, MutexGuard};

use std::thread;

lazy_static! {
    pub static ref TOPOLOGY: Mutex<Topology> = Mutex::new(Topology::new());

// cpu cores mutex
    pub static ref CPU_CORES: Vec<Mutex<CoreIndex>> = {
        let worker_cput_core_start_env =std::env::var("LOTUS_WORKER_CPU_CORE_START").unwrap();
        let worker_cput_core_start = worker_cput_core_start_env.parse::<usize>().unwrap();
        info!("worker_cput_core_start: {}", worker_cput_core_start);
        let worker_cput_core_end_env = std::env::var("LOTUS_WORKER_CPU_CORE_END").unwrap();
        let worker_cput_core_end = worker_cput_core_end_env.parse::<usize>().unwrap();
        info!("worker_cput_core_end: {}", worker_cput_core_end);

        let x = worker_cput_core_start..worker_cput_core_end;
        let ccores=  x.map(|i| CoreIndex(i)).collect::<Vec<_>>();
        ccores.iter().map(|i|Mutex::new(i.clone())).collect::<Vec<_>>()
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// `CoreIndex` is a simple wrapper type for indexes into the set of vixible cores. A `CoreIndex` should only ever be
/// created with a value known to be less than the number of visible cores.
pub struct CoreIndex(usize);

fn main() {
    flexi_logger::Logger::try_with_env().unwrap();

    println!("Hello, world!");

    let t1 = thread::spawn(work);
    let t2 = thread::spawn(work);

    for _ in 1..100 {
        checkout_cpu_core();
        thread::sleep(std::time::Duration::from_secs(1));
    }
    t1.join().unwrap();
    t2.join().unwrap();
}

fn work() {
    let _bind = bind_single_core();

    // let core_index_guard = Arc::new(checkout_cpu_core());
    // //

    // let core_index: &CoreIndex = match *core_index_guard {
    //     Some(ref x) => x,
    //     None => return,
    // };

    // let _dff = bind_core(*core_index);

    //  bind_single_core().unwrap();
    info!("work start");

    let mut ss = 0;
    loop {
        ss = ss + 1;
        if ss > 100000 {
            ss = 0;
        }
    }
    // println!("ss{}", ss);
}

pub fn checkout_cpu_core() -> Option<MutexGuard<'static, CoreIndex>> {
    for (i, ccore) in CPU_CORES.iter().enumerate() {
        match ccore.try_lock() {
            Ok(guard) => {
                info!("checkout_cpu_core {}", i);
                return Some(guard);
            }
            Err(_) => debug!("core {} locked, could not checkout", i),
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
pub type ThreadId = libc::pthread_t;

#[cfg(not(target_os = "windows"))]
fn get_thread_id() -> ThreadId {
    unsafe { libc::pthread_self() }
}

pub struct Cleanup {
    #[allow(dead_code)]
    tid: ThreadId,
    #[allow(dead_code)]
    prior_state: Option<Bitmap>,
}

pub fn bind_single_core() -> Result<Cleanup> {
    let core_index_guard = Arc::new(checkout_cpu_core());
    //

    let core_index: &CoreIndex = match *core_index_guard {
        Some(ref x) => x,
        None => return Err(format_err!("no gpu")),
    };

    bind_core(*core_index)
}

pub fn bind_core(core_index: CoreIndex) -> Result<Cleanup> {
    let child_topo = &TOPOLOGY;
    let tid = get_thread_id();
    let mut locked_topo = child_topo.lock().expect("poisoned lock");
    let core = get_core_by_index(&locked_topo, core_index)
        .map_err(|err| format_err!("failed to get core at index {}: {:?}", core_index.0, err))?;

    let cpuset = core
        .allowed_cpuset()
        .ok_or_else(|| format_err!("no allowed cpuset for core at index {}", core_index.0,))?;
    debug!("allowed cpuset: {:?}", cpuset);
    let mut bind_to = cpuset;

    // Get only one logical processor (in case the core is SMT/hyper-threaded).
    bind_to.singlify();

    // Thread binding before explicit set.
    let before = locked_topo.get_cpubind_for_thread(tid, CPUBIND_THREAD);

    info!("binding to {:?}, tid: {:?}", bind_to, tid);

    // debug!("binding to {:?}", bind_to);
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

fn get_core_by_index(topo: &Topology, index: CoreIndex) -> Result<&TopologyObject> {
    let idx = index.0;

    match topo.objects_with_type(&ObjectType::Core) {
        Ok(all_cores) if idx < all_cores.len() => Ok(all_cores[idx]),
        Ok(all_cores) => Err(format_err!(
            "idx ({}) out of range for {} cores",
            idx,
            all_cores.len()
        )),
        _e => Err(format_err!("failed to get core by index {}", idx,)),
    }
}
