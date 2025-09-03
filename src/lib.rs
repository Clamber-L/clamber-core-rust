//!
//! Clamber Core 是一个面向 Rust 项目的通用核心工具库，提供：
//! - 配置管理（YAML/TOML/JSON + 环境变量覆盖 + 多文件合并）
//! - JWT 令牌生成与校验
//! - 分布式唯一 ID（Snowflake）
//! - 基于 tracing 的结构化日志初始化
//! - 统一错误处理（thiserror）
//!
//! 快速开始示例：
//!
//! ```no_run
//! use serde::{Serialize, Deserialize};
//! use clamber_core::token::{JwtConfig, generate_token, verify_token};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct User { id: String, username: String, role: String }
//!
//! fn main() -> clamber_core::Result<()> {
//!     let user = User { id: "1".into(), username: "alice".into(), role: "admin".into() };
//!     let token = generate_token(&user, JwtConfig::default())?;
//!     let decoded: User = verify_token(&token)?;
//!     assert_eq!(user, decoded);
//!     Ok(())
//! }
//! ```
//!
//! 更多示例请查看项目 README 以及 examples 目录。

pub mod config;
pub mod error;
pub mod snowflake;
pub mod token;
pub mod tracing_logs;

pub use error::{ClamberError, Result};
pub use tracing_logs::{LogConfig, logger_start_with_config};

/// re-export: token 模块的主要类型与函数
pub use token::{JwtConfig, JwtManager, generate_token, is_valid_token, verify_token};

/// re-export: snowflake 模块的主要类型
pub use snowflake::{SnowflakeConfig, SnowflakeIdInfo, SnowflakeManager};

/// re-export: config 模块的主要类型与函数
pub use config::{
    ConfigBuilder, ConfigFormat, ConfigManager, auto_load_config, get_config_paths, load_config,
    load_config_with_env,
};

/// snowflake 便利函数（使用前缀避免命名冲突）
pub mod snowflake_utils {
    //! 便利函数：Snowflake ID 相关的快捷 API。
    pub use crate::snowflake::{
        generate_id, generate_ids, generate_string_id, parse_id, parse_string_id,
    };
}
