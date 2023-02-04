use std::path::{Path, PathBuf};

pub type RustPathBuf = Vec<String>;
pub type RustPath = [String];

pub fn join_rust_paths(path: &RustPath, tail: String) -> RustPathBuf {
    let mut buf = path.to_vec();
    buf.push(tail);
    buf
}

pub fn parent_rust_path(path: &RustPath) -> Option<&RustPath> {
    path.len().checked_sub(1).map(|n| &path[0..n])
}

pub fn resolve_mod_path(src_path: &Path, mod_path: &RustPath) -> Option<PathBuf> {
    let cands = if let Some(last) = mod_path.last() {
        let mut buf = src_path.to_owned();
        for v in mod_path.iter().rev().skip(1).rev() {
            buf.push(v);
        }

        let mut mod_buf = buf.clone();
        mod_buf.push(last.to_owned());
        mod_buf.push("mod.rs");
        buf.push(last.to_owned() + ".rs");
        vec![mod_buf, buf]
    } else {
        vec![src_path.join("main.rs").to_owned()]
    };
    cands.into_iter().filter(|path| path.exists()).next()
}
