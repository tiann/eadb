use include_dir::{include_dir, Dir};

// eadb directory in device
pub(crate) const EADB_DIR: &str = "/data/eadb";

// static assets we used.
pub(crate) static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");