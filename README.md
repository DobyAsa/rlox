# RLox

RLox 是一个用 Rust 实现的 Lox 语言解释器。

## 项目简介

这个项目是 Lox 编程语言的 Rust 实现版本。Lox 是一个小巧但功能完整的编程语言，最初由 Robert Nystrom 在其著作 [Crafting Interpreters](http://craftinginterpreters.com/) 中设计。

## 功能特性

目前实现的功能包括：
- 词法分析器 (Scanner)
- 基本的词法单元 (Token) 处理

## 项目结构

```plaintext
src/
├── lox.rs       - Lox 解释器的核心实现
├── main.rs      - 程序入口
├── scanner.rs   - 词法分析器
└── token/       - 词法单元相关定义
    └── mod.rs
    └── types.rs
```

## 开始使用

### 环境要求
- Rust 1.75 或更高版本
- Cargo 包管理器

### 安装

```bash
git clone https://github.com/yourusername/rlox.git
cd rlox
cargo build
```

### 运行

```bash
cargo run -- [文件路径]  # 执行 Lox 源文件
cargo run               # 启动交互式 REPL
```

## 依赖项

- clap (v4.5.27) - 命令行参数解析

## 致谢

- 感谢 Robert Nystrom 的 [Crafting Interpreters](http://craftinginterpreters.com/) 提供的优秀教程和设计思路。
```
