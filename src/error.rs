use thiserror::Error;

/// Clamber Core 库的统一错误类型
#[derive(Error, Debug)]
pub enum ClamberError {
    /// 日志相关错误
    #[error("日志系统错误: {message}")]
    LoggingError { message: String },

    /// 目录创建错误
    #[error("创建目录失败: {path}")]
    DirectoryCreationError { path: String },

    /// JWT相关错误
    #[error("JWT错误: {message}")]
    JwtError { message: String },

    /// JWT密钥错误
    #[error("JWT密钥无效: {details}")]
    JwtKeyError { details: String },

    /// JWT签名错误
    #[error("JWT签名失败: {details}")]
    JwtSignError { details: String },

    /// JWT验证错误
    #[error("JWT验证失败: {details}")]
    JwtVerifyError { details: String },

    /// JWT过期错误
    #[error("JWT已过期")]
    JwtExpiredError,

    /// JWT缺少必要字段错误
    #[error("JWT缺少必要字段: {field}")]
    JwtMissingFieldError { field: String },

    /// Snowflake相关错误
    #[error("Snowflake初始化错误: {details}")]
    SnowflakeInitError { details: String },

    /// Snowflake生成ID错误
    #[error("Snowflake生成ID失败: {details}")]
    SnowflakeGenerateError { details: String },

    /// Snowflake配置错误
    #[error("Snowflake配置无效: {details}")]
    SnowflakeConfigError { details: String },

    /// 配置管理相关错误
    #[error("配置加载错误: {details}")]
    ConfigLoadError { details: String },

    /// 配置文件不存在错误
    #[error("配置文件不存在: {path}")]
    ConfigFileNotFoundError { path: String },

    /// 配置解析错误
    #[error("配置解析失败: {details}")]
    ConfigParseError { details: String },

    /// 配置验证错误
    #[error("配置验证失败: {details}")]
    ConfigValidationError { details: String },

    /// 环境变量解析错误
    #[error("环境变量解析错误: {details}")]
    EnvVarParseError { details: String },

    /// 序列化错误
    #[error("序列化错误: {details}")]
    SerializationError { details: String },

    /// 反序列化错误
    #[error("反序列化错误: {details}")]
    DeserializationError { details: String },

    /// IO错误
    #[error("IO错误: {details}")]
    IoError { details: String },

    /// 其他错误
    #[error("未知错误: {message}")]
    Other { message: String },
}

impl From<std::io::Error> for ClamberError {
    fn from(err: std::io::Error) -> Self {
        ClamberError::IoError {
            details: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for ClamberError {
    fn from(err: serde_json::Error) -> Self {
        ClamberError::SerializationError {
            details: err.to_string(),
        }
    }
}

impl From<config::ConfigError> for ClamberError {
    fn from(err: config::ConfigError) -> Self {
        match err {
            config::ConfigError::NotFound(_) => ClamberError::ConfigFileNotFoundError {
                path: err.to_string(),
            },
            _ => ClamberError::ConfigLoadError {
                details: err.to_string(),
            },
        }
    }
}

impl From<toml::de::Error> for ClamberError {
    fn from(err: toml::de::Error) -> Self {
        ClamberError::ConfigParseError {
            details: err.to_string(),
        }
    }
}

impl From<serde_yaml::Error> for ClamberError {
    fn from(err: serde_yaml::Error) -> Self {
        ClamberError::ConfigParseError {
            details: err.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, ClamberError>;
