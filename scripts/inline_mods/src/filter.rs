use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

use crate::use_parser::parse_use;
use crate::utils::{RustPathBuf, resolve_mod_path, RustPath};

pub fn record_used_mods(src_path: &Path) -> HashSet<RustPathBuf> {
    let mut used_mods = HashSet::new();
    process_file(src_path, &[], &mut used_mods);
    used_mods
}

fn process_file(src_path: &Path, cur_mod: &RustPath, used_mods: &mut HashSet<RustPathBuf>) {
    if used_mods.contains(cur_mod) {
        return;
    }
    if let Some(path) = resolve_mod_path(src_path, cur_mod) {
        eprintln!("Record used mods for {cur_mod:?} in {path:?}");
        used_mods.insert(cur_mod.iter().cloned().collect());
        let source_code = read_to_string(path).unwrap();
        for use_mod in parse_use(&source_code, cur_mod) {
            for len in 1..=use_mod.len() {
                process_file(src_path, &use_mod[..len], used_mods);
            }
        }
    } else {
        eprintln!("Ignore non-existing {cur_mod:?}");
    };
}