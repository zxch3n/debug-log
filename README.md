<div align="center">
  <h1><code>debug-log</code></h2>
  <h3><a href="https://docs.rs/debug-log">Documentation</a></h3>
  <p></p>
</div>

Dead simple log utils for debug in Rust.

- 🦀 Enabled only in debug mode when `DEBUG` environment variable is set. You
  can change the `DEBUG` value in runtime as well by `set_debug`.
- 🔊 Only log in files whose paths match `DEBUG="filename"`. Match all by using
  `DEBUG=""`, or `DEBUG="*"`
- 📦 Group output with `debug_group`
- 📤 WASM support. It will use the console API

The output log is super easy to read on VS Code with sticky scroll enabled.

<img src="https://user-images.githubusercontent.com/18425020/202741062-0467b470-32ca-4a23-b280-73fa7d4c7868.gif" width="600"/>

# Example

```rust
use debug_log::{debug_dbg, debug_log, group, group_end};
fn main() {
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
```

Run with `DEBUG=* cargo run`

Output

```log
A Group {
    Sub A Group {
        [src/lib.rs:144] &arr = [
            0,
            1,
            2,
        ]
        Sub Sub A Group {
            [src/lib.rs:147] &arr = [
                0,
                1,
                2,
            ]
        }
        [src/lib.rs:150] Hi
        [src/lib.rs:151] &arr = [
            0,
            1,
            2,
        ]
    }
    B Group {
        [src/lib.rs:157] END
    }
}
```
