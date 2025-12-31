# 💻 LeetCode 算法题解

此工作空间包含使用 Rust 实现的 LeetCode 算法题解。

## 📁 结构说明
- 每道题的解答是一个独立的二进制程序，位于 `src/problems/` 目录下
- 题目文件名使用 LeetCode 题目的标识符命名
- 公共工具函数可以添加到 `src/lib.rs` 中

## 🛠️ 添加新题目解答
要添加新题目的解答：
1. 在 `src/problems/{problem_name}.rs` 创建新文件
2. 在 `Cargo.toml` 中添加新的 `[[bin]]` 条目指向该文件
3. 使用 `cargo run --bin {problem_name}` 运行

## 📝 示例
运行两数之和问题：
```bash
cargo run --bin two_sum
```

运行题目测试：
```bash
cargo test --bin two_sum
```
