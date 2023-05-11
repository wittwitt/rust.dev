use log::info;
use std::fs::{self, File};
use std::path::PathBuf;

pub enum WriteLayerDLock {
    NoLock,
    FileLock(PathBuf),
}

impl WriteLayerDLock {
    pub fn lock() -> WriteLayerDLock {
        let mut lock_path = PathBuf::new();
        if let Ok(path) = std::env::var("LOTUS_WRITE_LAYER_DLOCK_FILE") {
            lock_path.push(path);
        } else {
            info!("wite_layer no dlock");
            return WriteLayerDLock::NoLock;
        }

        info!("wite_layer dlock acquiring ...");
        File::create(lock_path.clone()).unwrap();
        info!("wite_layer dlock acquired!");
        WriteLayerDLock::FileLock(lock_path.clone())
    }
}

impl Drop for WriteLayerDLock {
    fn drop(&mut self) {
        if let WriteLayerDLock::FileLock(f) = self {
            info!("wite_layer del dlock: {:?}", f);
            fs::remove_file(f).unwrap();
            info!("wite_layer dlock released!");
        }
    }
}
