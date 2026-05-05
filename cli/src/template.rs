use include_dir::{include_dir, Dir};

// 这里的路径是相对于 cli/Cargo.toml 的路径
pub static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/default");