//!
//! Dead simple log utils for debug in Rust.
//!
//! - 🦀 Enabled only in debug mode when DEBUG environment variable is set
//! - 🔊 Only perform log in files whose paths match `DEBUG="filename"`. Match all by
//!   using `DEBUG=""`, or `DEBUG="*"`
//! - 📦 Group output with `debug_group`
//! - 📤 WASM support. It will use the console API.
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

#[cfg(all(debug_assertions))]
mod debug {
    use std::sync::Mutex;

    use once_cell::sync::Lazy;

    static DEBUG: Lazy<Mutex<Option<String>>> =
        Lazy::new(|| Mutex::new(std::option_env!("DEBUG").map(|x| x.to_owned())));
    static LEVELS: Mutex<Vec<String>> = Mutex::new(Vec::new());

    /// Change the DEBUG value to filter tests
    pub fn set_debug(s: &str) {
        *DEBUG.lock().unwrap() = Some(s.to_owned());
    }

    #[cfg(feature = "wasm")]
    pub mod console {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        extern "C" {
            // Use `js_namespace` here to bind `console.log(..)` instead of just
            // `log(..)`
            #[wasm_bindgen(js_namespace = console)]
            pub fn log(s: &str);

            #[wasm_bindgen(js_namespace = console)]
            pub fn group(s: &str);

            #[wasm_bindgen(js_namespace = console)]
            pub fn groupEnd();
        }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! inner_println {
        ($($arg:tt)+) => {{
            if $crate::should_log(&file!()) {
                #[cfg(not(feature="wasm"))]
                {
                    eprintln!($($arg)*);
                }
                #[cfg(feature="wasm")]
                {
                    let s = format!($($arg)+);
                    $crate::console::log(&s);
                }
            }
        }};
        () => {
            if $crate::should_log(&file!()) {
                #[cfg(not(feature="wasm"))]
                {
                    eprintln!();
                }
                #[cfg(feature="wasm")]
                {
                    $crate::console::log("");
                }
            }
        };
    }

    #[doc(hidden)]
    pub fn get_level() -> usize {
        LEVELS.lock().unwrap().len()
    }

    #[doc(hidden)]
    pub fn indent(name: &str) {
        let space = format!("{}", "    ".repeat(get_level()));
        inner_println!("{}{} {{", space, name);
        LEVELS.lock().unwrap().push(name.to_string())
    }

    #[doc(hidden)]
    pub fn outdent() {
        LEVELS.lock().unwrap().pop();
        let space = format!("{}", "    ".repeat(get_level()));
        inner_println!("{}}}", space);
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

        if ans.ends_with('\n') {
            ans.drain(ans.len() - 1..);
        }

        inner_println!("{}", ans);
    }

    #[doc(hidden)]
    pub fn prepend_indent(s: String) -> String {
        let mut ans = String::new();
        for (i, line) in s.split('\n').enumerate() {
            if i != 0 {
                ans.push_str(&"    ".repeat(get_level()));
            }
            ans.push_str(line);
            ans.push('\n')
        }
        ans
    }

    #[doc(hidden)]
    pub fn should_log(file: &str) -> bool {
        let lock = DEBUG.lock().unwrap();
        lock.as_ref()
            .map_or(false, |x| !x.is_empty() && (x == "*" || file.contains(x)))
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
                let space = format!("{}", "    ".repeat($crate::get_level()));
                $crate::inner_println!("{}[{}] ",space, line);
            }
        }
    }

    /// Use it like println!(). Except it can be filtered by DEBUG env and can only log on debug mode
    #[macro_export]
    macro_rules! debug_log {
        ($($arg:tt)*) => {{
            let line = format!("{}:{}", file!(), line!());
            if $crate::should_log(&line) {
                let prefix = format!("{}[{}] ", "    ".repeat($crate::get_level()), line);
                let s = format!($($arg)*);
                $crate::inner_println!("{}{}", prefix, $crate::prepend_indent(s));
            }
        }};
        () => {
            if $crate::should_log(&file!()) {
                $crate::inner_println();
            }
        };
    }
}

#[cfg(not(debug_assertions))]
mod debug {
    pub fn set_debug(s: &str) {}

    /// Group the following logs until [debug_log::group_end]
    #[macro_export]
    macro_rules! group {
        ($($arg:tt)*) => {};
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
