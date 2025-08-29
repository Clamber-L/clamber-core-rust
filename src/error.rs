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

pub type Result<T> = std::result::Result<T, ClamberError>;
