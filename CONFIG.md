# 配置管理模块 (Config)

这个模块提供了统一的配置管理功能，支持多种配置文件格式和环境变量覆盖。

## 特性

- 🗂️ **多格式支持**: 支持 YAML、TOML、JSON 配置文件
- 🌍 **环境变量覆盖**: 支持通过环境变量覆盖配置值
- 🔄 **多文件合并**: 支持加载和合并多个配置文件
- 🎯 **自动发现**: 自动发现应用配置文件
- ⚙️ **默认值支持**: 支持设置配置默认值
- 🔧 **灵活构建**: 提供灵活的配置构建器模式
- 📁 **忽略缺失**: 可选择忽略缺失的配置文件
- 🛡️ **类型安全**: 基于 Serde 的强类型配置反序列化

## 基本使用

### 1. 定义配置结构

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    name: String,
    port: u16,
    debug: bool,
    database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
}
```

### 2. 从单个配置文件加载

```rust
use clamber_core::load_config;

// 从 YAML 文件加载
let config: AppConfig = load_config(\"config.yaml\")?;

// 从 TOML 文件加载  
let config: AppConfig = load_config(\"config.toml\")?;

// 从 JSON 文件加载
let config: AppConfig = load_config(\"config.json\")?;
```

### 3. 支持环境变量覆盖

```rust
use clamber_core::load_config_with_env;

// 加载配置文件并支持 APP_ 前缀的环境变量覆盖
let config: AppConfig = load_config_with_env(\"config.yaml\", \"APP\")?;

// 环境变量示例:
// APP_PORT=8080
// APP_DEBUG=true  
// APP_DATABASE__HOST=localhost
// APP_DATABASE__PORT=5432
```

### 4. 使用配置构建器

```rust
use clamber_core::ConfigBuilder;

let config: AppConfig = ConfigBuilder::new()
    // 添加多个配置文件（按优先级顺序）
    .add_yaml_file(\"base.yaml\")
    .add_yaml_file(\"production.yaml\")
    // 设置默认值
    .with_default(\"name\", \"my-app\")?
    .with_default(\"port\", 8080)?
    // 启用环境变量覆盖
    .with_env_prefix(\"APP\")
    .with_env_separator(\"__\")
    // 忽略缺失的配置文件
    .ignore_missing_files(true)
    .build()?;
```

### 5. 自动发现配置

```rust
use clamber_core::auto_load_config;

// 自动查找 myapp.{yaml,yml,toml,json} 配置文件
let config: AppConfig = auto_load_config(\"myapp\", Some(\"APP\"))?;
```

## 配置文件格式

### YAML 格式 (`config.yaml`)

```yaml
name: \"my-application\"
port: 8080
debug: false
database:
  host: \"localhost\"
  port: 5432
  username: \"postgres\"
  password: \"password\"
```

### TOML 格式 (`config.toml`)

```toml
name = \"my-application\"
port = 8080
debug = false

[database]
host = \"localhost\"
port = 5432
username = \"postgres\"
password = \"password\"
```

### JSON 格式 (`config.json`)

```json
{
  \"name\": \"my-application\",
  \"port\": 8080,
  \"debug\": false,
  \"database\": {
    \"host\": \"localhost\",
    \"port\": 5432,
    \"username\": \"postgres\",
    \"password\": \"password\"
  }
}
```

## 环境变量规则

环境变量遵循以下命名规则：

- 前缀：通过 `with_env_prefix()` 设置（如 `APP`）
- 分隔符：通过 `with_env_separator()` 设置（默认 `__`）
- 嵌套：使用分隔符表示嵌套结构

例如，对于配置：
```yaml
port: 8080
database:
  host: \"localhost\"
  port: 5432
```

对应的环境变量为：
```bash
APP_PORT=9000
APP_DATABASE__HOST=db.example.com
APP_DATABASE__PORT=3306
```

## 配置优先级

配置值的优先级从高到低：

1. 🌍 **环境变量** - 最高优先级
2. 📄 **后加载的配置文件** - 覆盖先加载的文件
3. 📄 **先加载的配置文件**
4. ⚙️ **默认值** - 最低优先级

## 错误处理

配置模块使用统一的错误类型：

- `ConfigLoadError` - 配置加载错误
- `ConfigFileNotFoundError` - 配置文件不存在
- `ConfigParseError` - 配置解析错误
- `ConfigValidationError` - 配置验证错误
- `EnvVarParseError` - 环境变量解析错误

## 高级用法

### 配置验证

```rust
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    #[serde(deserialize_with = \"validate_port\")]
    port: u16,
    // 其他字段...
}

fn validate_port<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let port: u16 = u16::deserialize(deserializer)?;
    if port < 1024 {
        return Err(serde::de::Error::custom(\"端口号必须大于等于1024\"));
    }
    Ok(port)
}
```

### 可选配置项

```rust
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    name: String,
    port: u16,
    // 可选的 Redis 配置
    redis: Option<RedisConfig>,
    // 带默认值的配置
    #[serde(default = \"default_timeout\")]
    timeout: u64,
}

fn default_timeout() -> u64 {
    30
}
```

### 自定义配置路径

```rust
use clamber_core::get_config_paths;

// 获取应用 \"myapp\" 的所有可能配置路径
let paths = get_config_paths(\"myapp\");
for path in paths {
    println!(\"检查配置文件: {:?}\", path);
}
```

## 示例

查看 `examples/config_example.rs` 了解完整的使用示例，包括：

- 基础配置加载
- 环境变量覆盖
- 多文件合并
- 自动发现
- 不同格式支持

运行示例：
```bash
cargo run --example config_example
```