---
title: Rust 生命周期详解
date: 2026-03-04
summary: 深入理解 Rust 生命周期，掌握借用检查器的核心概念。
tags: Rust, Lifetime
---

## 什么是生命周期？

生命周期是 Rust 编译器的工具，用于确保引用在使用期间始终有效。

### 生命周期语法

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 关键规则

- 每个引用都有自己的生命周期
- 生命周期参数描述引用的关系
- 编译器自动推导大部分生命周期

理解生命周期是掌握 Rust 的关键一步。
