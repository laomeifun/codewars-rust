# Codewars Rust 练习项目

## 项目结构

```
codewars-rust/
├── Cargo.toml
├── CLAUDE.md
└── src/
    ├── main.rs
    └── bin/
        ├── new_kata.rs              # 工具：自动生成 kata 文件
        └── YYYY-MM-DD_kyuN_name.rs  # 练习题文件
```

## 命名规则

- 文件名格式：`日期_kyu难度_题目名.rs`
- 示例：`2026-01-11_kyu8_multiply.rs`
- 难度：kyu8 最简单，kyu1 最难

## 常用命令

```bash
# 创建新题目（从 Codewars URL 或 kata ID）
cargo run --bin new_kata -- <url_or_id>

# 运行指定题目
cargo run --bin 2026-01-11_kyu8_multiply

# 测试指定题目
cargo test --bin 2026-01-11_kyu8_multiply

# 测试所有题目
cargo test
```

## 文件模板

每个 kata 文件包含：
- 题目注释（名称 + 链接）
- `solution()` 函数
- `main()` 用于本地调试
- `tests` 模块用于测试

## 工作流程

1. 在 Codewars 找到想做的题目
2. 运行 `cargo run --bin new_kata -- <kata_url>`
3. 编辑生成的文件，实现 solution
4. 用 `cargo test --bin <name>` 验证
5. 提交到 Codewars
