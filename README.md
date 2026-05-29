# OpenCode 本地构建与运行（仅 macOS aarch64）

本文档仅覆盖 **macOS Apple Silicon（arm64 / aarch64）** 的本地构建与运行流程。

## 1. 环境要求

- macOS 12+
- CPU 架构：`arm64`（`uname -m` 应输出 `arm64`）
- 可访问外网（首次构建需要下载 Rust 依赖）

## 2. 安装依赖

### 2.1 安装 Xcode Command Line Tools

```bash
xcode-select --install
```

如果已经安装，命令会提示无需重复安装。

### 2.2 安装 Rust 工具链（rustup）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

### 2.3 配置 Rust 目标与常用组件

```bash
rustup default stable
rustup target add aarch64-apple-darwin
rustup component add rustfmt clippy
```

## 3. 构建程序

仓库根目录是 `opencode/`，Rust 工程目录在仓库根目录下：

```bash
cd /Users/lwl2/code/opencode
cd ./opencode-rs
```

### 3.1 Debug 构建（开发调试）

```bash
cargo build --manifest-path ./cli/Cargo.toml --bin opencode --target aarch64-apple-darwin
```

产物路径：

```text
target/aarch64-apple-darwin/debug/opencode
```

### 3.2 Release 构建（更接近发布表现）

```bash
cargo build --manifest-path ./cli/Cargo.toml --bin opencode --target aarch64-apple-darwin --profile release
```

产物路径：

```text
target/aarch64-apple-darwin/release/opencode
```


## 4. 运行程序

### 4.1 直接运行已构建二进制

```bash
# Debug
./target/aarch64-apple-darwin/debug/opencode --help

# Release
./target/aarch64-apple-darwin/release/opencode --help
```

> 说明：仓库内 crate 名仍可能保留历史命名（`codex-*`），但 CLI 二进制目标已使用 `opencode`。

## 5. 常见问题

### 5.1 `cargo: command not found`

执行：

```bash
source "$HOME/.cargo/env"
```

并重开终端后重试。

### 5.2 `xcode-select` / 编译器相关报错

先确认命令行工具已安装：

```bash
xcode-select -p
```

如果没有路径，重新执行：

```bash
xcode-select --install
```

### 5.3 首次构建很慢

首次会下载并编译依赖，时间较长属于正常现象。后续增量构建会明显加快。
