use include_dir::{include_dir, Dir};

// eadb directory in device
pub(crate) const EADB_DIR: &str = "/data/eadb";

// the rootfs dir of eadb
const EADB_ROOTFS_DIR: &str = "/data/eadb/debian";

// static assets we used.
pub(crate) static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub(crate) fn to_rootfs_dir(dir: &str) -> String {
    format!("{}{}", EADB_ROOTFS_DIR, dir)
}