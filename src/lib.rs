#[cfg(debug_assertions)]
mod debug {
    const DEBUG: Option<&'static str> = std::option_env!("DEBUG");
    static mut LEVELS: Vec<String> = vec![];

    #[doc(hidden)]
    pub fn get_level() -> usize {
        unsafe { LEVELS.len() }
    }

    #[doc(hidden)]
    pub fn indent(name: &str) {
        eprint!("{}", "    ".repeat(get_level()));
        eprintln!("{} {{", name);
        unsafe { LEVELS.push(name.to_string()) }
    }

    #[doc(hidden)]
    pub fn outdent() {
        unsafe {
            LEVELS.pop();
        }
        eprint!("{}", "    ".repeat(get_level()));
        eprintln!("}}");
    }

    #[doc(hidden)]
    pub fn dbg<T: std::fmt::Debug>(value: T, line: &str) {
        let s = format!("{:#?}", value);
        let mut ans = String::new();
        ans.push_str(&"    ".repeat(get_level()));
        ans.push('[');
        ans.push_str(line);
        ans.push(']');
        for line in s.split('\n') {
            ans.push_str(&"    ".repeat(get_level()));
            ans.push_str(line);
            ans.push('\n')
        }

        eprint!("{}", ans);
    }

    #[doc(hidden)]
    pub fn should_log(file: &str) -> bool {
        DEBUG.map_or(false, |x| x.is_empty() || x == "*" || file.contains(x))
    }

    #[macro_export]
    macro_rules! group {
        ($arg:tt) => {
            if $crate::should_log(&file!()) {
                $crate::indent($arg);
            }
        };
        () => {
            if $crate::should_log(&file!()) {
                $crate::indent("".to_string());
            }
        };
    }

    #[macro_export]
    macro_rules! group_end {
        () => {
            if $crate::should_log(&file!()) {
                $crate::outdent();
            }
        };
    }

    #[macro_export]
    macro_rules! debug_dbg {
        ($($val:expr),+ $(,)?) => {
            let line = format!("{}:{}", file!(), line!());
            if $crate::should_log(&line) {
                ($($crate::dbg($val, &line)),+,);
            }
        };
        () => {
            let line = format!("{}:{}", file!(), line!());
            if $crate::should_log(&line) {
                eprint!("{}", "    ".repeat($crate::get_level()));
                eprint!("[{}] ", line);
            }
        }
    }

    #[macro_export]
    macro_rules! debug_log {
        ($($arg:tt)*) => {{
            let line = format!("{}:{}", file!(), line!());
            if $crate::should_log(&line) {
                eprint!("{}", "    ".repeat($crate::get_level()));
                eprint!("[{}] ", line);
                eprintln!($($arg)*);
            }
        }};
        () => {
            if $crate::should_log(&file!()) {
                $crate::eprint!("\n")
            }
        };
    }
}

#[cfg(not(debug_assertions))]
mod debug {
    #[macro_export]
    macro_rules! group {
        ($arg:tt) => {};
    }

    #[macro_export]
    macro_rules! debug_log {
        ($($arg:tt)*) => {{}};
        () => {};
    }

    #[macro_export]
    macro_rules! group {
        ($arg:tt) => {};
        () => {};
    }

    #[macro_export]
    macro_rules! group_end {
        () => {};
    }
}

pub use debug::*;

#[cfg(test)]
mod tests {
    use crate::{debug_dbg, debug_log, group, group_end};

    #[test]
    fn it_works() {
        group!("A");
        let arr: Vec<_> = (0..100).collect();
        debug_log!("Hello, world! {:?}", arr);
        debug_dbg!(&arr);
        debug_dbg!();
        debug_log!("Hello, world!");
        group!("B");
        debug_log!("Hello, world!");
        group_end!();
        group_end!();
    }
}
