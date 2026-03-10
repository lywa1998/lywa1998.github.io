---
title: Hello, Dioxus!
date: 2026-03-05
summary: 开始使用 Dioxus 0.7 构建现代 Web 应用的入门指南。
tags: Rust, Dioxus, Web
---

## Dioxus 是什么？

Dioxus 是一个用 Rust 编写的类 React 风格的 UI 库。它跨平台、高性能且语法优雅。

### 为什么选择 Dioxus？

- **优雅**：使用声明式语法构建 UI
- **高性能**：受益于 Rust 的速度和安全性
- **跨平台**：支持 Web、桌面、移动端

### 快速开始

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { "Hello, World!" }
}
```

这就是开始所需的一切！
