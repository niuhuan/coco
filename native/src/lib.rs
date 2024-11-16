use std::future::Future;
use std::sync::Mutex;

pub use anyhow::Result;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use tokio::runtime;

use local::join_paths;

use crate::database::init_database;
use crate::local::create_dir_if_not_exists;

mod api;
mod bridge_generated;
mod database;

mod local;
mod utils;

mod download;
mod entities;

#[cfg(test)]
mod tests;

lazy_static! {
    pub(crate) static ref RUNTIME: runtime::Runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_keep_alive(tokio::time::Duration::new(60, 0))
        .max_blocking_threads(30)
        .worker_threads(30)
        .build()
        .unwrap();
    static ref INIT_ED: Mutex<bool> = Mutex::new(false);
}

pub(crate) fn block_on<T>(f: impl Future<Output = T>) -> T {
    RUNTIME.block_on(f)
}

static ROOT: OnceCell<String> = OnceCell::new();
static DOWNLOADS_DIR: OnceCell<String> = OnceCell::new();
static IMAGE_CACHE_DIR: OnceCell<String> = OnceCell::new();
static DATABASE_DIR: OnceCell<String> = OnceCell::new();

pub fn init_root(root: &str, downloads_to: &str) {
    let mut lock = INIT_ED.lock().unwrap();
    if *lock {
        return;
    }
    *lock = true;
    println!(
        "Init application with root : {}, downloads_to : {}",
        root, downloads_to,
    );
    ROOT.set(root.to_owned()).unwrap();
    DOWNLOADS_DIR.set(downloads_to.to_owned()).unwrap();
    IMAGE_CACHE_DIR
        .set(join_paths(vec![root, "image_cache"]))
        .unwrap();
    DATABASE_DIR
        .set(join_paths(vec![root, "database"]))
        .unwrap();
    create_dir_if_not_exists(ROOT.get().unwrap());
    create_dir_if_not_exists(DOWNLOADS_DIR.get().unwrap());
    create_dir_if_not_exists(IMAGE_CACHE_DIR.get().unwrap());
    create_dir_if_not_exists(DATABASE_DIR.get().unwrap());
    RUNTIME.block_on(init_database());
    let _ = RUNTIME.spawn(download::download_demon());
}

#[allow(dead_code)]
pub(crate) fn get_root() -> &'static String {
    ROOT.get().unwrap()
}

pub(crate) fn get_downloads_dir() -> &'static String {
    DOWNLOADS_DIR.get().unwrap()
}

pub(crate) fn get_image_cache_dir() -> &'static String {
    IMAGE_CACHE_DIR.get().unwrap()
}

pub(crate) fn get_database_dir() -> &'static String {
    DATABASE_DIR.get().unwrap()
}
