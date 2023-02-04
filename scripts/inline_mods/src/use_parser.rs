use std::collections::HashSet;

use syn::visit::Visit;
use syn::UseTree;

use crate::utils::{RustPathBuf, RustPath, parent_rust_path};

struct UseVisitor<'a> {
    cur_mod: &'a RustPath,
    use_statements: HashSet<RustPathBuf>,
    stack: Vec<String>,
}

impl <'a> UseVisitor<'a> {
    fn record_use(&mut self, name: Option<String>) {
        let mut path = if let Some(first) = self.stack.first() {
            match first.as_str() {
                "crate" => vec![],
                "self" => self.cur_mod.to_vec(),
                "super" => parent_rust_path(self.cur_mod).unwrap().to_vec(),
                "std" => return,
                other => {
                    panic!("use should not start with {other:?}: {:?}", self.stack);
                }
            }
        } else {
            return;
        };
        for ident in self.stack.iter().skip(1) {
            if ident == "super" {
                path.pop();
            } else {
                path.push(ident.to_owned());
            }
        }
        if let Some(name) = name {
            path.push(name);
        }
        self.use_statements.insert(path);
    }
}

impl<'ast, 'a> Visit<'ast> for UseVisitor<'a> {
    fn visit_use_tree(&mut self, tree: &'ast syn::UseTree) {
        match tree {
            UseTree::Path(path) => {
                self.stack.push(path.ident.to_string());
                self.visit_use_tree(&path.tree);
                self.stack.pop();
            }
            UseTree::Group(group) => {
                for item in group.items.iter() {
                    self.visit_use_tree(item);
                }
            }
            UseTree::Name(use_name) => {
                self.record_use(Some(use_name.ident.to_string()));
            }
            UseTree::Rename(use_rename) => {
                self.record_use(Some(use_rename.ident.to_string()));
            }
            UseTree::Glob(_) => {
                self.record_use(None);
            }
        }
    }
}

pub fn parse_use(source_code: &str, cur_mod: &RustPath) -> HashSet<RustPathBuf> {
    let file = syn::parse_file(&source_code).expect("Unable to parse file");
    let mut visitor = UseVisitor {
        cur_mod,
        use_statements: HashSet::new(),
        stack: Vec::new(),
    };
    visitor.visit_file(&file);
    visitor.use_statements
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::utils::RustPathBuf;

    use super::parse_use;

    #[test]
    pub fn parse_use_basic() {
        check(
            "use crate::utils::io::{InputSource, OutputTarget};",
            "plat::leetcode::io",
            &["utils::io::InputSource", "utils::io::OutputTarget"],
        );
        check("use crate::utils::io;", "", &["utils::io"]);
        check(
            "use self::parser::LeetcodeValueNode;",
            "plat::leetcode::io",
            &["plat::leetcode::io::parser::LeetcodeValueNode"],
        );
        check(
            "use super::includes::*;",
            "plat::classic::checker",
            &["plat::classic::includes"],
        );

        check(
            "use std::collections::HashSet;",
            "plat::classic::solution",
            &[],
        );
        check(
            "pub use runner::run;",
            "plat::classic",
            &["plat::classic::runner::run"],
        );
    }

    fn check(src: &str, cur_mod: &str, expected: &[&str]) {
        let actual = parse_use(src, &split_mod(cur_mod));
        let expected: HashSet<_> = expected.into_iter().map(|&s| split_mod(s)).collect();
        assert_eq!(
            actual, expected,
            "current mod: '{cur_mod}', source: '{src}'"
        );
    }

    fn split_mod(s: &str) -> RustPathBuf {
        s.split("::").map(|p| p.to_owned()).collect()
    }
}
