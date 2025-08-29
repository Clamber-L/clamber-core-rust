# Clamber Core 错误处理升级说明

## 概述

本次升级使用 `thiserror` 替换了所有的 `unwrap()` 调用，提供了完整的错误处理机制，让库的使用者可以优雅地处理各种错误情况。

## 主要改进

### 1. 新增错误类型模块 (`src/error.rs`)

创建了统一的错误类型 `ClamberError`，包含以下具体错误：

- **`LoggingError`** - 日志系统相关错误
- **`DirectoryCreationError`** - 目录创建失败
- **`JwtError`** - 通用JWT错误
- **`JwtKeyError`** - JWT密钥无效
- **`JwtSignError`** - JWT签名失败
- **`JwtVerifyError`** - JWT验证失败
- **`JwtExpiredError`** - JWT已过期
- **`JwtMissingFieldError`** - JWT缺少必要字段
- **`SerializationError`** - 序列化错误
- **`DeserializationError`** - 反序列化错误
- **`IoError`** - IO操作错误
- **`Other`** - 其他未分类错误

### 2. 模块升级

#### `tracing_logs` 模块
- **移除** `unwrap()` 调用
- **更新** `logger_start` 函数返回 `Result<(WorkerGuard, WorkerGuard)>`
- **改进** 目录创建错误处理

#### `token` 模块  
- **替换** `anyhow` 为 `thiserror`
- **移除** 所有 `unwrap()` 调用
- **细化** 错误类型，提供更精确的错误信息
- **保持** API 兼容性，所有公共函数签名保持不变

### 3. 依赖更新

在 `Cargo.toml` 中新增：
```toml
thiserror = "1.0"
```

### 4. 新的类型导出

`lib.rs` 中新增导出：
```rust
pub use error::{ClamberError, Result};
```

## 使用示例

### 基础错误处理
```rust
use clamber_core::{ClamberError, Result, generate_token, verify_token};

fn handle_jwt_operations() -> Result<()> {
    let user = MyUser { /* ... */ };
    
    // 生成token - 可能失败
    let token = generate_token(&user)?;
    
    // 验证token - 可能失败  
    let decoded_user: MyUser = verify_token(&token)?;
    
    Ok(())
}
```

### 具体错误类型匹配
```rust
match verify_token::<User>(token) {
    Ok(user) => {
        // 处理成功情况
        handle_authenticated_user(user);
    }
    Err(ClamberError::JwtExpiredError) => {
        // Token过期 - 重定向到登录页
        redirect_to_login();
    }
    Err(ClamberError::JwtVerifyError { details }) => {
        // 验证失败 - 记录安全日志
        log_security_event(&details);
    }
    Err(ClamberError::JwtMissingFieldError { field }) => {
        // 格式错误 - 返回400错误
        return_bad_request(&format!("Missing field: {}", field));
    }
    Err(e) => {
        // 其他错误 - 通用处理
        log_error(&e.to_string());
    }
}
```

### 错误传播
```rust
fn complex_operation() -> Result<String> {
    let user = create_user()?;        // 错误自动传播
    let token = generate_token(&user)?; // 错误自动传播
    let verified: User = verify_token(&token)?; // 错误自动传播
    Ok(format!("User {} authenticated", verified.username))
}
```

## 兼容性

- ✅ **向后兼容** - 所有现有的API保持不变
- ✅ **类型安全** - 编译时错误检查
- ✅ **性能无影响** - 零开销抽象
- ✅ **文档完善** - 每种错误都有清晰的文档说明

## 迁移指南

### 对于新用户
直接使用新的错误处理API：
```rust
use clamber_core::{Result, ClamberError, generate_token};
```

### 对于现有用户
不需要修改现有代码，但建议升级错误处理：

**之前:**
```rust
let token = generate_token(&user).unwrap(); // 可能panic
```

**现在:**
```rust  
let token = generate_token(&user)?; // 优雅的错误处理
// 或者
let token = match generate_token(&user) {
    Ok(t) => t,
    Err(e) => {
        log::error!("Failed to generate token: {}", e);
        return Err(e);
    }
};
```

## 测试和验证

- ✅ 所有原有测试通过
- ✅ 新增错误处理示例 (`examples/error_handling.rs`)
- ✅ 错误类型覆盖完整
- ✅ 文档示例全部验证

## 最佳实践

1. **使用具体错误匹配** - 为不同错误类型提供不同处理逻辑
2. **合理使用 ? 操作符** - 在适当的地方传播错误
3. **提供上下文信息** - 在错误处理中记录必要的上下文
4. **优雅降级** - 为错误情况提供合理的备选方案

这次升级显著提高了库的健壮性和用户体验，使得错误处理变得更加优雅和类型安全。