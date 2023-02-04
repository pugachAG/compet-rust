use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use regex::Regex;

use crate::filter::record_used_mods;
use crate::utils::{join_rust_paths, resolve_mod_path, RustPath, RustPathBuf};

pub fn inline_main(src_path: &Path, dest_path: &Path) {
    let used_mods = record_used_mods(src_path);
    eprintln!("Filter mods: {used_mods:#?}");
    let lines = process_file(src_path, &[], &used_mods);
    fs::write(dest_path, lines.join("\n")).unwrap()
}

fn process_file(
    src_path: &Path,
    cur_mod: &RustPath,
    used_mods: &HashSet<RustPathBuf>,
) -> Vec<String> {
    let path = resolve_mod_path(src_path, cur_mod).unwrap();
    eprintln!("Inline {cur_mod:?} from {path:?}");
    let mod_decl_re: Regex = Regex::new(r"((?:pub )?mod )(\S+);").unwrap();
    let mut res = Vec::<String>::new();
    let file = File::open(path).unwrap();
    for line in BufReader::new(file).lines().map(|r| r.unwrap()) {
        if line.trim() == "#[cfg(test)]" {
            continue;
        }
        match mod_decl_re.captures(&line) {
            Some(caps) => {
                let mod_name = &caps[2];
                let mod_path = join_rust_paths(cur_mod, mod_name.to_owned());
                if !used_mods.contains(&mod_path) {
                    eprintln!("Skip {mod_path:?}");
                    continue;
                }
                res.push(String::from(&caps[1]) + &format!("{mod_name} {{"));
                for mut mod_line in process_file(&src_path, &mod_path, used_mods) {
                    mod_line.insert_str(0, "    ");
                    res.push(mod_line);
                }
                res.extend_from_slice(&[String::from("}"), String::from("")]);
            }
            None => res.push(line),
        }
    }
    res
}
