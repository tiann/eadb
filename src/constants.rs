use include_dir::{include_dir, Dir};

pub(crate) const DEFAULT_DEBIAN_DISTRO: &str = "bullseye";

pub(crate) const DEFAULT_DEBIAN_REPO: &str = "http://ftp.us.debian.org/debian/";

pub(crate) const DEFAULT_PREBUILT_ROOTFS_REPO: &str = "https://github.com/tiann/eadb/";

// eadb directory in device
pub(crate) const EADB_DIR: &str = "/data/eadb";

// the rootfs dir of eadb
const EADB_ROOTFS_DIR: &str = "/data/eadb/debian/";

// static assets we used.
pub(crate) static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub(crate) fn to_rootfs_dir(dir: &str) -> String {
    format!("{}{}", EADB_ROOTFS_DIR, dir)
}