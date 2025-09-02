# Snowflake ID 生成器

这个模块封装了 Snowflake ID 生成器，提供了线程安全的分布式唯一ID生成功能。

## 特性

- 支持自定义配置和默认配置
- 线程安全的ID生成
- 支持批量生成ID
- 提供ID解析功能
- 支持自定义纪元时间
- 集成 thiserror 错误处理
- 提供便利函数简化使用

## 使用方法

### 1. 使用默认配置（便利函数）

```rust
use clamber_core::snowflake_utils;

// 生成单个ID
let id = snowflake_utils::generate_id()?;
println!("ID: {}", id);

// 生成多个ID
let ids = snowflake_utils::generate_ids(10)?;

// 解析ID信息
let info = snowflake_utils::parse_id(id)?;
println!("时间戳: {}, 工作者ID: {}, 序列号: {}", 
         info.timestamp, info.worker_id, info.sequence);
```

### 2. 使用自定义配置

```rust
use clamber_core::{SnowflakeConfig, SnowflakeManager};

// 创建自定义配置
let config = SnowflakeConfig::new(5)?; // 工作者ID = 5
let manager = SnowflakeManager::new(config)?;

// 生成ID
let id = manager.generate_id()?;
```

### 3. 使用自定义纪元

```rust
let custom_epoch = 1609459200000; // 2021-01-01 00:00:00 UTC
let config = SnowflakeConfig::with_epoch(10, custom_epoch)?;
let manager = SnowflakeManager::new(config)?;
```

### 4. 字符串格式ID

```rust
// 生成字符串ID
let string_id = snowflake_utils::generate_string_id()?;

// 从字符串解析ID
let info = snowflake_utils::parse_string_id(&string_id)?;
```

## 配置说明

- `worker_id`: 工作者ID，范围 0-1023
- `epoch`: 自定义纪元时间戳（毫秒，可选）

## ID 结构

Snowflake ID 由以下部分组成：
- 1位符号位（始终为0）
- 41位时间戳
- 10位工作者ID
- 12位序列号

## 错误处理

所有函数都返回 `Result<T>` 类型，使用 `thiserror` 进行错误处理：

- `SnowflakeInitError`: 初始化错误
- `SnowflakeGenerateError`: 生成ID错误
- `SnowflakeConfigError`: 配置错误

## 示例

查看 `examples/snowflake_example.rs` 了解完整的使用示例。