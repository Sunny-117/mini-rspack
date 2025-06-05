使用 `nodemon` 的 `--exec` 选项来执行 `cargo run` 并监听 Rust 文件的变化。下面是一个完整的命令和配置说明：

---

### ✅ 一次性命令

```bash
nodemon --exec "cargo run" -e rs
```

解释：

* `--exec "cargo run"`：每次文件变动时执行 `cargo run`
* `-e rs`：只监听 `.rs` 文件变化（Rust 文件）

---

### ✅ 推荐写法：使用配置文件 `nodemon.json`

你可以在项目根目录创建一个 `nodemon.json`，内容如下：

```json
{
  "exec": "cargo run",
  "ext": "rs",
  "watch": [
    "src"
  ],
  "delay": "100ms"
}
```

然后在终端中运行：

```bash
nodemon
```

这样更可维护，适合团队协作。

---

### 🔁 如果你用的是 `workspace`，或 `cargo` 执行路径不在默认位置，可以这样写：

```json
{
  "exec": "cargo run --manifest-path path/to/Cargo.toml",
  "ext": "rs",
  "watch": ["src", "other-crate/src"]
}
```

---

### ⚠️ 注意事项

* `cargo run` 每次会重新编译整个 crate，较慢。可以考虑用 [cargo-watch](https://github.com/watchexec/cargo-watch) 替代（专为 Rust 优化）。
* `nodemon` 是 Node.js 工具，不了解 Rust 编译缓存，所以效率不如原生工具。

---

### 🔄 替代方案（推荐）

Rust 社区推荐用：

```bash
cargo install cargo-watch
```

然后使用：

```bash
cargo watch -x run
```

这比 `nodemon` 更适合 Rust 项目，性能更好。

---