<div align="center">
  <h1><code>debug-log</code></h2>
  <h3><a href="https://docs.rs/debug-log">Documentation</a></h3>
  <p></p>
</div>

Dead simple log utils for debug.

- 🦀 Enabled only in debug mode when DEBUG environment variable is set
- 🔊 Only perform log in files whose paths match DEBUG="filename". Match all by
  using DEBUG="", or DEBUG="\*"
- 📦 Group output with `debug_group`

The output log is super easy to read on VS Code with sticky scroll enabled.
