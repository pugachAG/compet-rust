pub enum ExecutionMode {
    Solution { multitest: bool },
    Checker,
}

/// # Examples
/// ```
/// exec_mode!(multitest = true);
/// exec_mode!(multitest = false);
/// exec_mode!(checker);
/// ```
#[macro_export]
macro_rules! exec_mode {
    (multitest = $mt:literal) => {
        pub const EXECUTION_MODE: $crate::plat::classic::config::ExecutionMode =
            $crate::plat::classic::config::ExecutionMode::Solution { multitest: $mt };
    };
    (checker) => {
        pub const EXECUTION_MODE: $crate::plat::classic::config::ExecutionMode =
            $crate::plat::classic::config::ExecutionMode::Checker;
    };
}
