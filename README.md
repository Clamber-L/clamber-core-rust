# Clamber Core (Rust)

一个面向 Rust 项目的通用核心工具库，提供配置管理、JWT、分布式 ID（Snowflake）、结构化日志与错误处理等常用能力，开箱即用、类型安全、易扩展。

- 配置管理：多格式文件 + 环境变量覆盖 + 多文件合并（详见 CONFIG.md）
- JWT：简洁易用的 Token 生成与校验，支持自定义配置（详见 JWT_README.md）
- Snowflake：线程安全的分布式唯一 ID 生成（详见 SNOWFLAKE.md）
- 日志与追踪：tracing 生态，支持文件/控制台、按日滚动、环境过滤
- 错误处理：基于 thiserror 的统一错误类型 ClamberError（详见 ERROR_HANDLING_UPGRADE.md）

## 安装

在你的 Cargo.toml 中添加依赖：

```toml
[dependencies]
clamber-core = "0.1.3"
serde = { version = "1.0", features = ["derive"] }
```

最低 Rust 版本：与本库 edition 2024 兼容的稳定版 Rust（建议使用最新 stable）。

## 快速开始

下面展示各核心模块的最小可用示例。更多进阶用法请查看对应的模块文档。

### 1) 配置管理

- 从单个文件加载并支持环境变量覆盖：

```rust
use clamber_core::{load_config_with_env};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig { name: String, port: u16, debug: bool }

fn main() -> clamber_core::Result<()> {
    // 自动解析 YAML/TOML/JSON，支持环境变量前缀 APP_
    let config: AppConfig = load_config_with_env("config.yaml", "APP")?;
    println!("{:?}", config);
    Ok(())
}
```

- 构建器加载多个文件并设置默认值：

```rust
use clamber_core::ConfigBuilder;

let config: AppConfig = ConfigBuilder::new()
    .add_yaml_file("base.yaml")
    .add_yaml_file("production.yaml")
    .with_default("port", 8080)?
    .with_env_prefix("APP")
    .with_env_separator("__")
    .ignore_missing_files(true)
    .build()?;
```

更多内容请见 CONFIG.md（格式、环境变量命名规则、自动发现等）。

### 2) JWT

- 便利函数：

```rust
use clamber_core::token::{generate_token, verify_token, is_valid_token, JwtConfig};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User { id: String, username: String, role: String }

fn main() -> clamber_core::Result<()> {
    let user = User { id: "1".into(), username: "alice".into(), role: "admin".into() };
    let token = generate_token(&user, JwtConfig::default())?;
    assert!(is_valid_token(&token));
    let decoded: User = verify_token(&token)?;
    assert_eq!(user, decoded);
    Ok(())
}
```

- 自定义配置：

```rust
use clamber_core::token::{JwtConfig, JwtManager};
let manager = JwtManager::new(JwtConfig::new("my_secret", 30));
```

更多内容请见 JWT_README.md（配置说明、API、错误示例等），以及 examples/jwt_usage.rs。

### 3) Snowflake 分布式 ID

```rust
use clamber_core::snowflake_utils;

let id = snowflake_utils::generate_id()?;
let string_id = snowflake_utils::generate_string_id()?;
let info = snowflake_utils::parse_id(id)?;
```

更多内容请见 SNOWFLAKE.md（ID 结构、配置、纪元自定义等），以及 examples/snowflake_example.rs。

### 4) 日志与追踪（tracing）

库内提供基于 tracing 的日志初始化与文件滚动方案，支持：
- 控制台与文件输出
- 按天滚动与压缩
- 环境变量过滤（RUST_LOG）

典型做法：

```rust
// 伪代码示例，实际 API 以 src/tracing_logs 为准
let (_guard_stdout, _guard_file) = clamber_core::tracing_logs::logger_start("logs")?;
tracing::info!("service started");
```

### 5) 统一错误处理

- 公开类型：
  - Result<T> = std::result::Result<T, ClamberError>
  - ClamberError：覆盖 JWT、IO、配置、日志等常见错误

示例与迁移说明见 ERROR_HANDLING_UPGRADE.md。

## 运行示例

项目内包含多份可运行示例，使用 Cargo 运行：

```bash
# JWT 示例
cargo run --example jwt_usage

# 错误处理示例
cargo run --example error_handling

# 自定义日志示例
cargo run --example beautiful_logs_custom

# Snowflake 示例
cargo run --example snowflake_example
```

在 Windows PowerShell 下亦可同样执行以上命令。

## 目录结构（节选）

- src/config 配置模块实现
- src/token JWT 模块实现
- src/snowflake 雪花算法模块实现
- src/tracing_logs 日志与追踪初始化
- examples 示例程序
- CONFIG.md、JWT_README.md、SNOWFLAKE.md、ERROR_HANDLING_UPGRADE.md 详细文档

## 版本与许可证

- crate: clamber-core = 0.1.3
- 许可证：MIT OR Apache-2.0
- 仓库：https://github.com/Clamber-L/clamber-core-rust

如有问题或建议，欢迎提交 Issue/PR。
