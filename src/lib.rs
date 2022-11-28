//!
//! Dead simple log utils for debug in Rust.
//!
//! - 🦀 Enabled only in debug mode when DEBUG environment variable is set
//! - 🔊 Only perform log in files whose paths match `DEBUG="filename"`. Match all by
//!   using `DEBUG=""`, or `DEBUG="*"`
//! - 📦 Group output with `debug_group`
//!
//! The output log is super easy to read on VS Code with sticky scroll enabled.
//!
//! <img src="https://user-images.githubusercontent.com/18425020/202741062-0467b470-32ca-4a23-b280-73fa7d4c7868.gif" width="600"/>
//!
//! # Example
//!
//! ```rust
//! use debug_log::{debug_dbg, debug_log, group, group_end};
//! group!("A Group");
//! {
//!     group!("Sub A Group");
//!     let arr: Vec<_> = (0..3).collect();
//!     debug_dbg!(&arr);
//!     {
//!         group!("Sub Sub A Group");
//!         debug_dbg!(&arr);
//!         group_end!();
//!     }
//!     debug_log!("Hi");
//!     debug_dbg!(&arr);
//!     group_end!();
//! }
//!
//! {
//!     group!("B Group");
//!     debug_log!("END");
//!     group_end!();
//! }
//! group_end!();
//! ```
//!
//! Run with `DEBUG=* cargo run`
//!
//! Output
//!
//! ```log
//! A Group {
//!     Sub A Group {
//!         [src/lib.rs:144] &arr = [
//!             0,
//!             1,
//!             2,
//!         ]
//!         Sub Sub A Group {
//!             [src/lib.rs:147] &arr = [
//!                 0,
//!                 1,
//!                 2,
//!             ]
//!         }
//!         [src/lib.rs:150] Hi
//!         [src/lib.rs:151] &arr = [
//!             0,
//!             1,
//!             2,
//!         ]
//!     }
//!     B Group {
//!         [src/lib.rs:157] END
//!     }
//! }
//! ```

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
    pub fn dbg<T: std::fmt::Debug>(value: T, name: &str, line: &str) {
        let s = format!("{:#?}", value);
        let mut ans = String::new();
        ans.push_str(&"    ".repeat(get_level()));
        ans.push_str(format!("[{}] {} = ", line, name).as_str());
        for (i, line) in s.split('\n').enumerate() {
            if i != 0 {
                ans.push_str(&"    ".repeat(get_level()));
            }
            ans.push_str(line);
            ans.push('\n')
        }

        eprint!("{}", ans);
    }

    #[doc(hidden)]
    pub fn should_log(file: &str) -> bool {
        DEBUG.map_or(false, |x| x.is_empty() || x == "*" || file.contains(x))
    }

    /// Group the following logs until group_end!()
    #[macro_export]
    macro_rules! group {
        ($($arg:tt)*) => {{
            let line = format!("{}:{}", file!(), line!());
            if $crate::should_log(&line) {
                $crate::indent(&format!($($arg)*));
            }
        }};
        () => {
            if $crate::should_log(&file!()) {
                $crate::indent("".to_string());
            }
        };
    }

    /// End the previous group
    #[macro_export]
    macro_rules! group_end {
        () => {
            if $crate::should_log(&file!()) {
                $crate::outdent();
            }
        };
    }

    /// It can be filtered by DEBUG env and can only log on debug mode
    #[macro_export]
    macro_rules! debug_dbg {
        ($($val:expr),+ $(,)?) => {
            let line = format!("{}:{}", file!(), line!());
            if $crate::should_log(&line) {
                ($($crate::dbg($val, stringify!($val), &line)),+,);
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

    /// Use it like println!(). Except it can be filtered by DEBUG env and can only log on debug mode
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
    /// Group the following logs until [debug_log::group_end]
    #[macro_export]
    macro_rules! group {
        ($arg:tt) => {};
        () => {};
    }

    /// End the previous group
    #[macro_export]
    macro_rules! group_end {
        () => {};
    }

    /// Use it like println!(). Except it can be filtered by DEBUG env and can only log on debug mode
    #[macro_export]
    macro_rules! debug_log {
        ($($arg:tt)*) => {{}};
        () => {};
    }

    /// It's just dbg!() with indent and can be filtered by DEBUG env
    #[macro_export]
    macro_rules! debug_dbg {
        ($($val:expr),+ $(,)?) => {};
        () => {};
    }
}

pub use debug::*;

#[cfg(test)]
mod tests {
    use crate::{debug_dbg, debug_log, group, group_end};

    #[test]
    /// Run this test with
    /// DEBUG=* cargo test -- --nocapture &> data.log
    fn it_works() {
        group!("A Group");
        {
            group!("Sub A Group");
            let arr: Vec<_> = (0..3).collect();
            debug_dbg!(&arr);
            {
                group!("Sub Sub A Group");
                debug_dbg!(&arr);
                group_end!();
            }
            debug_log!("Hi");
            debug_dbg!(&arr);
            group_end!();
        }

        {
            group!("B Group");
            debug_log!("END");
            group_end!();
        }
        group_end!();
    }
}
