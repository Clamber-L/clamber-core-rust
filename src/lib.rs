pub mod config;
pub mod error;
pub mod snowflake;
pub mod token;
pub mod tracing_logs;

pub use error::{ClamberError, Result};
pub use tracing_logs::{LogConfig, logger_start_with_config};

// 导出 token 相关的类型和函数
pub use token::{JwtConfig, JwtManager, generate_token, is_valid_token, verify_token};

// 导出 snowflake 相关的类型和函数
pub use snowflake::{SnowflakeConfig, SnowflakeIdInfo, SnowflakeManager};

// 导出 config 相关的类型和函数
pub use config::{
    ConfigBuilder, ConfigFormat, ConfigManager, auto_load_config, get_config_paths, load_config,
    load_config_with_env,
};

// snowflake 便利函数（使用前缀避免命名冲突）
pub mod snowflake_utils {
    pub use crate::snowflake::{
        generate_id, generate_ids, generate_string_id, parse_id, parse_string_id,
    };
}
