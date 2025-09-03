//! Snowflake 模块：线程安全的分布式唯一 ID 生成与解析，支持自定义纪元与批量生成。
//! 详见根目录 SNOWFLAKE.md 获取更完整说明与示例。
use crate::error::{ClamberError, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use twitter_snowflake::Snowflake;

/// Snowflake配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnowflakeConfig {
    /// 工作者ID (0-1023)
    pub worker_id: u64,
    /// 自定义纪元时间戳（毫秒，可选）
    pub epoch: Option<u64>,
}

impl Default for SnowflakeConfig {
    fn default() -> Self {
        Self {
            worker_id: 1,
            epoch: None, // 使用默认纪元
        }
    }
}

impl SnowflakeConfig {
    /// 创建新的Snowflake配置
    pub fn new(worker_id: u64) -> Result<Self> {
        Self::validate_worker_id(worker_id)?;

        Ok(Self {
            worker_id,
            epoch: None,
        })
    }

    /// 创建带自定义纪元的配置
    pub fn with_epoch(worker_id: u64, epoch: u64) -> Result<Self> {
        Self::validate_worker_id(worker_id)?;

        Ok(Self {
            worker_id,
            epoch: Some(epoch),
        })
    }

    /// 设置工作者ID
    pub fn worker_id(mut self, worker_id: u64) -> Result<Self> {
        Self::validate_worker_id(worker_id)?;
        self.worker_id = worker_id;
        Ok(self)
    }

    /// 设置自定义纪元
    pub fn epoch(mut self, epoch: u64) -> Self {
        self.epoch = Some(epoch);
        self
    }

    /// 验证工作者ID有效性
    fn validate_worker_id(worker_id: u64) -> Result<()> {
        if worker_id > 1023 {
            return Err(ClamberError::SnowflakeConfigError {
                details: format!("工作者ID必须在0-1023范围内，当前值: {}", worker_id),
            });
        }
        Ok(())
    }
}

/// Snowflake ID生成器封装
pub struct SnowflakeManager {
    generator: Mutex<Snowflake>,
    config: SnowflakeConfig,
}

impl SnowflakeManager {
    /// 使用自定义配置创建Snowflake管理器
    pub fn new(config: SnowflakeConfig) -> Result<Self> {
        let generator = if let Some(epoch) = config.epoch {
            Snowflake::builder()
                .with_worker_id(config.worker_id)
                .with_epoch(epoch)
                .build()
        } else {
            Snowflake::new(config.worker_id)
        };

        let generator = generator.map_err(|e| ClamberError::SnowflakeInitError {
            details: format!("初始化Snowflake生成器失败: {:?}", e),
        })?;

        Ok(Self {
            generator: Mutex::new(generator),
            config,
        })
    }

    /// 使用默认配置创建Snowflake管理器
    pub fn default() -> Result<Self> {
        Self::new(SnowflakeConfig::default())
    }

    /// 生成新的Snowflake ID
    pub fn generate_id(&self) -> Result<u64> {
        let mut generator =
            self.generator
                .lock()
                .map_err(|e| ClamberError::SnowflakeGenerateError {
                    details: format!("获取生成器锁失败: {}", e),
                })?;

        generator
            .generate()
            .map_err(|e| ClamberError::SnowflakeGenerateError {
                details: format!("生成ID失败: {:?}", e),
            })
    }

    /// 生成多个ID
    pub fn generate_ids(&self, count: usize) -> Result<Vec<u64>> {
        let mut ids = Vec::with_capacity(count);
        for _ in 0..count {
            ids.push(self.generate_id()?);
        }
        Ok(ids)
    }

    /// 获取当前配置
    pub fn get_config(&self) -> &SnowflakeConfig {
        &self.config
    }

    /// 获取工作者ID
    pub fn worker_id(&self) -> u64 {
        self.config.worker_id
    }

    /// 解析Snowflake ID的各个组成部分
    pub fn parse_id(&self, id: u64) -> SnowflakeIdInfo {
        // Twitter Snowflake ID结构：1位符号位 + 41位时间戳 + 10位工作者ID + 12位序列号
        let timestamp = (id >> 22) & 0x1FFFFFFFFFF; // 41位时间戳
        let worker_id = (id >> 12) & 0x3FF; // 10位工作者ID
        let sequence = id & 0xFFF; // 12位序列号

        SnowflakeIdInfo {
            id,
            timestamp,
            worker_id,
            sequence: sequence as u16,
        }
    }
}

/// Snowflake ID信息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnowflakeIdInfo {
    /// 原始ID
    pub id: u64,
    /// 时间戳部分
    pub timestamp: u64,
    /// 工作者ID部分
    pub worker_id: u64,
    /// 序列号部分
    pub sequence: u16,
}

impl SnowflakeIdInfo {
    /// 获取生成时间（毫秒时间戳）
    pub fn generation_time(&self, epoch: Option<u64>) -> u64 {
        let epoch = epoch.unwrap_or(1288834974657); // Twitter纪元 (2010-11-04T01:42:54.657Z)
        self.timestamp + epoch
    }

    /// 获取可读的时间字符串
    pub fn generation_time_string(&self, epoch: Option<u64>) -> String {
        let timestamp_ms = self.generation_time(epoch);
        let datetime = chrono::DateTime::from_timestamp_millis(timestamp_ms as i64);
        match datetime {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            None => "Invalid timestamp".to_string(),
        }
    }

    /// 转换为字符串格式的ID
    pub fn to_string(&self) -> String {
        self.id.to_string()
    }

    /// 从字符串解析ID
    pub fn from_string(id_str: &str) -> Result<u64> {
        id_str
            .parse::<u64>()
            .map_err(|e| ClamberError::SnowflakeConfigError {
                details: format!("无法解析ID字符串: {}", e),
            })
    }
}

// 便利函数：使用默认配置
use once_cell::sync::Lazy;

static DEFAULT_MANAGER: Lazy<Result<SnowflakeManager>> = Lazy::new(|| SnowflakeManager::default());

/// 获取或创建默认的Snowflake管理器
fn get_default_manager() -> Result<&'static SnowflakeManager> {
    match &*DEFAULT_MANAGER {
        Ok(manager) => Ok(manager),
        Err(e) => Err(ClamberError::SnowflakeInitError {
            details: format!("无法初始化默认Snowflake管理器: {}", e),
        }),
    }
}

/// 使用默认配置生成ID
pub fn generate_id() -> Result<u64> {
    get_default_manager()?.generate_id()
}

/// 使用默认配置生成多个ID
pub fn generate_ids(count: usize) -> Result<Vec<u64>> {
    get_default_manager()?.generate_ids(count)
}

/// 解析ID信息
pub fn parse_id(id: u64) -> Result<SnowflakeIdInfo> {
    Ok(get_default_manager()?.parse_id(id))
}

/// 生成字符串格式的ID
pub fn generate_string_id() -> Result<String> {
    Ok(generate_id()?.to_string())
}

/// 从字符串解析并获取ID信息
pub fn parse_string_id(id_str: &str) -> Result<SnowflakeIdInfo> {
    let id = SnowflakeIdInfo::from_string(id_str)?;
    parse_id(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_snowflake_config_creation() {
        // 测试正常创建
        let config = SnowflakeConfig::new(1).unwrap();
        assert_eq!(config.worker_id, 1);
        assert!(config.epoch.is_none());

        // 测试带纪元的创建
        let config_with_epoch = SnowflakeConfig::with_epoch(3, 1609459200000).unwrap();
        assert_eq!(config_with_epoch.worker_id, 3);
        assert_eq!(config_with_epoch.epoch, Some(1609459200000));

        // 测试无效工作者ID
        assert!(SnowflakeConfig::new(1024).is_err());
    }

    #[test]
    fn test_snowflake_manager_creation() {
        let config = SnowflakeConfig::new(1).unwrap();
        let manager = SnowflakeManager::new(config).unwrap();
        assert_eq!(manager.worker_id(), 1);
    }

    #[test]
    fn test_id_generation() {
        let config = SnowflakeConfig::new(1).unwrap();
        let manager = SnowflakeManager::new(config).unwrap();

        // 生成单个ID
        let id = manager.generate_id().unwrap();
        assert!(id > 0);

        // 生成多个ID并检查唯一性
        let ids = manager.generate_ids(100).unwrap();
        assert_eq!(ids.len(), 100);

        let unique_ids: HashSet<u64> = ids.into_iter().collect();
        assert_eq!(unique_ids.len(), 100); // 所有ID应该是唯一的
    }

    #[test]
    fn test_id_parsing() {
        let config = SnowflakeConfig::new(1).unwrap();
        let manager = SnowflakeManager::new(config).unwrap();

        let id = manager.generate_id().unwrap();
        let info = manager.parse_id(id);

        assert_eq!(info.id, id);
        assert!(info.timestamp > 0);
    }

    #[test]
    fn test_convenience_functions() {
        // 测试便利函数
        let id = generate_id().unwrap();
        assert!(id > 0);

        let ids = generate_ids(10).unwrap();
        assert_eq!(ids.len(), 10);

        let info = parse_id(id).unwrap();
        assert_eq!(info.id, id);
    }

    #[test]
    fn test_string_functions() {
        let id = generate_string_id().unwrap();
        assert!(!id.is_empty());

        let info = parse_string_id(&id).unwrap();
        assert_eq!(info.to_string(), id);
    }

    #[test]
    fn test_id_info_time_functions() {
        let id = generate_id().unwrap();
        let info = parse_id(id).unwrap();

        let gen_time = info.generation_time(None);
        assert!(gen_time > 0);

        let time_str = info.generation_time_string(None);
        assert!(!time_str.is_empty());
        assert!(time_str.contains("-")); // 应该包含日期格式
    }
}
