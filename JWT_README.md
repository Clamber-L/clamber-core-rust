# Clamber Core JWT 模块

这是一个通用的、高度可配置的 JWT (JSON Web Token) 库，基于 Rust 构建，可以轻松集成到任何项目中。

## 功能特性

- 🔐 **安全可靠**：使用 HMAC-SHA256 签名算法
- 🛠️ **高度可配置**：支持自定义密钥和过期时间
- 📦 **类型安全**：支持任何实现了 `Serialize`/`Deserialize` 的数据类型作为 payload
- 🚀 **易于使用**：提供便利函数和灵活的配置选项
- ✅ **全面测试**：包含完整的单元测试和示例

## 快速开始

### 1. 添加依赖

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
clamber-core = \"0.1.0\"
serde = { version = \"1.0\", features = [\"derive\"] }
anyhow = \"1.0\"
```

### 2. 基本使用

```rust
use clamber_core::token::{generate_token, verify_token, is_valid_token};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    id: String,
    username: String,
    role: String,
}

fn main() -> Result<()> {
    let user = User {
        id: \"123\".to_string(),
        username: \"john_doe\".to_string(),
        role: \"admin\".to_string(),
    };

    // 生成 JWT token
    let token = generate_token(&user)?;
    println!(\"Token: {}\", token);

    // 检查 token 有效性
    if is_valid_token(&token) {
        println!(\"Token 有效\");
    }

    // 验证并解析 token
    let decoded_user: User = verify_token(&token)?;
    println!(\"解析的用户: {:?}\", decoded_user);

    assert_eq!(user, decoded_user);
    Ok(())
}
```

### 3. 自定义配置

```rust
use clamber_core::token::{JwtConfig, JwtManager};

// 创建自定义配置
let config = JwtConfig::new(\"my_secret_key\", 30); // 30天过期
let jwt_manager = JwtManager::new(config);

// 使用自定义配置
let token = jwt_manager.generate_token(&user)?;
let decoded_user: User = jwt_manager.verify_token(&token)?;
```

## API 文档

### `JwtConfig`

JWT 配置结构体，用于设置密钥和过期时间。

```rust
impl JwtConfig {
    // 创建新的配置
    pub fn new(secret: impl Into<String>, expire_days: i64) -> Self;
    
    // 使用默认配置（密钥：\"default_jwt_secret\"，过期时间：7天）
    pub fn default() -> Self;
}
```

### `JwtManager`

JWT 管理器，提供核心的 token 生成和验证功能。

```rust
impl JwtManager {
    // 使用指定配置创建管理器
    pub fn new(config: JwtConfig) -> Self;
    
    // 使用默认配置创建管理器
    pub fn default() -> Self;
    
    // 生成 JWT token
    pub fn generate_token<T: Serialize>(&self, payload: &T) -> Result<String>;
    
    // 验证并解析 JWT token
    pub fn verify_token<T: DeserializeOwned>(&self, token: &str) -> Result<T>;
    
    // 检查 token 是否有效（不解析 payload）
    pub fn is_valid_token(&self, token: &str) -> bool;
}
```

### 便利函数

使用默认配置的快捷函数：

```rust
// 生成 token
pub fn generate_token<T: Serialize>(payload: &T) -> Result<String>;

// 验证 token
pub fn verify_token<T: DeserializeOwned>(token: &str) -> Result<T>;

// 检查 token 有效性
pub fn is_valid_token(token: &str) -> bool;
```

## 支持的数据类型

这个 JWT 库支持任何实现了 `Serialize` 和 `Deserialize` trait 的数据类型作为 payload：

- 基本类型：`String`, `i32`, `f64`, `bool` 等
- 集合类型：`Vec<T>`, `HashMap<K, V>` 等
- 自定义结构体和枚举
- 嵌套的复杂数据结构

### 示例

```rust
// 字符串
let token = generate_token(&\"Hello World\")?;
let message: String = verify_token(&token)?;

// 数字
let token = generate_token(&42i32)?;
let number: i32 = verify_token(&token)?;

// 复杂结构体
#[derive(Serialize, Deserialize)]
struct ComplexData {
    name: String,
    values: Vec<i32>,
    metadata: HashMap<String, String>,
}

let data = ComplexData { /* ... */ };
let token = generate_token(&data)?;
let decoded: ComplexData = verify_token(&token)?;
```

## 错误处理

所有可能失败的操作都返回 `anyhow::Result<T>`，包含详细的错误信息：

```rust
match verify_token::<User>(&invalid_token) {
    Ok(user) => println!(\"用户: {:?}\", user),
    Err(e) => println!(\"验证失败: {}\", e),
}
```

常见错误类型：
- `Invalid secret key`: 密钥格式错误
- `Failed to verify JWT`: Token 签名验证失败
- `Token has expired`: Token 已过期
- `Token missing expiration time`: Token 缺少过期时间
- `Failed to deserialize payload`: Payload 反序列化失败

## 安全注意事项

1. **密钥管理**：
   - 使用足够强度的密钥（建议至少32个字符）
   - 不要在代码中硬编码密钥
   - 定期轮换密钥

2. **过期时间**：
   - 根据应用场景设置合理的过期时间
   - 敏感操作使用较短的过期时间
   - 考虑实现 token 刷新机制

3. **传输安全**：
   - 在生产环境中使用 HTTPS
   - 避免在 URL 参数中传输 token
   - 使用适当的 HTTP headers

## 运行示例

克隆项目后，你可以运行内置示例来查看各种使用方式：

```bash
cargo run --example jwt_usage
```

## 测试

运行所有测试：

```bash
cargo test
```

运行特定模块测试：

```bash
cargo test token
```

## 许可证

MIT 或 Apache-2.0

## 贡献

欢迎提交 issues 和 pull requests！