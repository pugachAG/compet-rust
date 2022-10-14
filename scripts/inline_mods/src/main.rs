use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::{env, fs};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3, "Expected 2 args, got {:?}", &args[1..]);
    let main_path = Path::new(&args[1]);
    let res = process_file(&main_path);
    fs::write(&args[2], res.join("\n")).unwrap()
}

fn process_file(path: &Path) -> Vec<String> {
    lazy_static! {
        static ref MOD_DECL_RE: Regex = Regex::new(r"((?:pub )?mod )(\S+);").unwrap();
    }
    eprintln!("Processing {:?}", path);
    let mut res = Vec::<String>::new();
    let file = File::open(path).unwrap();
    for line in BufReader::new(file).lines().map(|r| r.unwrap()) {
        match MOD_DECL_RE.captures(&line) {
            Some(caps) => {
                let mod_name = &caps[2];
                res.push(String::from(&caps[1]) + &format!("{mod_name} {{"));
                let mod_path = resolve_mod_path(path, mod_name);
                for mut mod_line in process_file(&mod_path) {
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

fn resolve_mod_path(decl_path: &Path, mod_name: &str) -> PathBuf {
    let dir_path = decl_path.parent().unwrap();
    [
        dir_path.join(String::from(mod_name) + ".rs"),
        dir_path.join(mod_name).join("mod.rs"),
    ]
    .into_iter()
    .filter(|path| path.exists())
    .next()
    .unwrap_or_else(|| {
        panic!(
            "mod {:?} does not exist relative to {:?}",
            mod_name, decl_path
        );
    })
}
