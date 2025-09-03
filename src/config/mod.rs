//! 配置管理模块：支持多格式配置文件（YAML/TOML/JSON）、环境变量覆盖（可自定义前缀与分隔符）、多文件合并与默认值。
//! 参见项目根目录的 CONFIG.md 获取更完整的使用指南与示例。
use crate::error::{ClamberError, Result};
use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

/// 配置文件格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    /// YAML 格式
    Yaml,
    /// TOML 格式
    Toml,
    /// JSON 格式
    Json,
}

impl ConfigFormat {
    /// 从文件扩展名推断配置格式
    pub fn from_extension(path: &Path) -> Option<Self> {
        match path.extension()?.to_str()? {
            "yaml" | "yml" => Some(ConfigFormat::Yaml),
            "toml" => Some(ConfigFormat::Toml),
            "json" => Some(ConfigFormat::Json),
            _ => None,
        }
    }

    /// 转换为 config crate 的 FileFormat
    fn to_file_format(self) -> FileFormat {
        match self {
            ConfigFormat::Yaml => FileFormat::Yaml,
            ConfigFormat::Toml => FileFormat::Toml,
            ConfigFormat::Json => FileFormat::Json,
        }
    }
}

/// 配置构建器
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    /// 配置文件路径列表
    files: Vec<(PathBuf, Option<ConfigFormat>)>,
    /// 环境变量前缀
    env_prefix: Option<String>,
    /// 环境变量分隔符
    env_separator: String,
    /// 是否忽略缺失的配置文件
    ignore_missing: bool,
    /// 默认值
    defaults: HashMap<String, config::Value>,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            env_prefix: None,
            env_separator: "__".to_string(),
            ignore_missing: false,
            defaults: HashMap::new(),
        }
    }
}

impl ConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加配置文件
    ///
    /// # 参数
    /// * `path` - 配置文件路径
    /// * `format` - 可选的文件格式，如果不指定则从文件扩展名推断
    pub fn add_file<P: AsRef<Path>>(mut self, path: P, format: Option<ConfigFormat>) -> Self {
        self.files.push((path.as_ref().to_path_buf(), format));
        self
    }

    /// 添加 YAML 配置文件
    pub fn add_yaml_file<P: AsRef<Path>>(self, path: P) -> Self {
        self.add_file(path, Some(ConfigFormat::Yaml))
    }

    /// 添加 TOML 配置文件
    pub fn add_toml_file<P: AsRef<Path>>(self, path: P) -> Self {
        self.add_file(path, Some(ConfigFormat::Toml))
    }

    /// 添加 JSON 配置文件
    pub fn add_json_file<P: AsRef<Path>>(self, path: P) -> Self {
        self.add_file(path, Some(ConfigFormat::Json))
    }

    /// 设置环境变量前缀
    ///
    /// # 参数
    /// * `prefix` - 环境变量前缀，例如 "APP"
    pub fn with_env_prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.env_prefix = Some(prefix.into());
        self
    }

    /// 设置环境变量分隔符
    ///
    /// # 参数
    /// * `separator` - 分隔符，默认为 "__"
    pub fn with_env_separator<S: Into<String>>(mut self, separator: S) -> Self {
        self.env_separator = separator.into();
        self
    }

    /// 设置是否忽略缺失的配置文件
    pub fn ignore_missing_files(mut self, ignore: bool) -> Self {
        self.ignore_missing = ignore;
        self
    }

    /// 添加默认值
    ///
    /// # 参数
    /// * `key` - 配置键
    /// * `value` - 默认值
    pub fn with_default<K, V>(mut self, key: K, value: V) -> Result<Self>
    where
        K: Into<String>,
        V: Into<config::Value>,
    {
        self.defaults.insert(key.into(), value.into());
        Ok(self)
    }

    /// 构建配置并反序列化为指定类型
    ///
    /// # 返回值
    /// 返回反序列化后的配置对象
    pub fn build<T>(self) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut config_builder = Config::builder();

        // 添加默认值
        for (key, value) in self.defaults {
            config_builder = config_builder.set_default(&key, value).map_err(|e| {
                ClamberError::ConfigLoadError {
                    details: format!("设置默认值失败: {}", e),
                }
            })?;
        }

        // 添加配置文件
        for (path, format) in self.files {
            let format = format
                .or_else(|| ConfigFormat::from_extension(&path))
                .ok_or_else(|| ClamberError::ConfigLoadError {
                    details: format!("无法推断配置文件格式: {:?}", path),
                })?;

            let file_config = File::from(path.clone())
                .format(format.to_file_format())
                .required(!self.ignore_missing);

            config_builder = config_builder.add_source(file_config);
        }

        // 添加环境变量
        if let Some(prefix) = self.env_prefix {
            let env_config = Environment::with_prefix(&prefix)
                .separator(&self.env_separator)
                .try_parsing(true)
                .ignore_empty(true);
            config_builder = config_builder.add_source(env_config);
        }

        // 构建配置
        let config = config_builder
            .build()
            .map_err(|e| ClamberError::ConfigLoadError {
                details: e.to_string(),
            })?;

        // 反序列化
        config
            .try_deserialize::<T>()
            .map_err(|e| ClamberError::ConfigParseError {
                details: e.to_string(),
            })
    }

    /// 构建配置并返回原始 Config 对象
    pub fn build_raw(self) -> Result<Config> {
        let mut config_builder = Config::builder();

        // 添加默认值
        for (key, value) in self.defaults {
            config_builder = config_builder.set_default(&key, value).map_err(|e| {
                ClamberError::ConfigLoadError {
                    details: format!("设置默认值失败: {}", e),
                }
            })?;
        }

        // 添加配置文件
        for (path, format) in self.files {
            let format = format
                .or_else(|| ConfigFormat::from_extension(&path))
                .ok_or_else(|| ClamberError::ConfigLoadError {
                    details: format!("无法推断配置文件格式: {:?}", path),
                })?;

            let file_config = File::from(path.clone())
                .format(format.to_file_format())
                .required(!self.ignore_missing);

            config_builder = config_builder.add_source(file_config);
        }

        // 添加环境变量
        if let Some(prefix) = self.env_prefix {
            let env_config = Environment::with_prefix(&prefix)
                .separator(&self.env_separator)
                .try_parsing(true)
                .ignore_empty(true);
            config_builder = config_builder.add_source(env_config);
        }

        // 构建配置
        config_builder
            .build()
            .map_err(|e| ClamberError::ConfigLoadError {
                details: e.to_string(),
            })
    }
}

/// 配置管理器
pub struct ConfigManager;

impl ConfigManager {
    /// 从单个配置文件加载配置
    ///
    /// # 参数
    /// * `path` - 配置文件路径
    ///
    /// # 返回值
    /// 返回反序列化后的配置对象
    pub fn load_from_file<T, P>(path: P) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        P: AsRef<Path>,
    {
        ConfigBuilder::new().add_file(path, None).build()
    }

    /// 从配置文件和环境变量加载配置
    ///
    /// # 参数
    /// * `config_path` - 配置文件路径
    /// * `env_prefix` - 环境变量前缀
    ///
    /// # 返回值
    /// 返回反序列化后的配置对象
    pub fn load_with_env<T, P, S>(config_path: P, env_prefix: S) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        P: AsRef<Path>,
        S: Into<String>,
    {
        ConfigBuilder::new()
            .add_file(config_path, None)
            .with_env_prefix(env_prefix)
            .build()
    }

    /// 加载多个配置文件，支持环境变量覆盖
    ///
    /// # 参数
    /// * `config_paths` - 配置文件路径列表（按优先级顺序）
    /// * `env_prefix` - 可选的环境变量前缀
    ///
    /// # 返回值
    /// 返回反序列化后的配置对象
    pub fn load_multiple<T, P, S>(config_paths: Vec<P>, env_prefix: Option<S>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        P: AsRef<Path>,
        S: Into<String>,
    {
        let mut builder = ConfigBuilder::new().ignore_missing_files(true);

        for path in config_paths {
            builder = builder.add_file(path, None);
        }

        if let Some(prefix) = env_prefix {
            builder = builder.with_env_prefix(prefix);
        }

        builder.build()
    }

    /// 创建配置构建器
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

/// 便利函数：从配置文件加载配置
pub fn load_config<T, P>(path: P) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path>,
{
    ConfigManager::load_from_file(path)
}

/// 便利函数：从配置文件和环境变量加载配置
pub fn load_config_with_env<T, P, S>(config_path: P, env_prefix: S) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
    P: AsRef<Path>,
    S: Into<String>,
{
    ConfigManager::load_with_env(config_path, env_prefix)
}

/// 便利函数：获取当前工作目录下的配置文件路径
pub fn get_config_paths(name: &str) -> Vec<PathBuf> {
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    vec![
        current_dir.join(format!("{}.yaml", name)),
        current_dir.join(format!("{}.yml", name)),
        current_dir.join(format!("{}.toml", name)),
        current_dir.join(format!("{}.json", name)),
        current_dir.join("config").join(format!("{}.yaml", name)),
        current_dir.join("config").join(format!("{}.yml", name)),
        current_dir.join("config").join(format!("{}.toml", name)),
        current_dir.join("config").join(format!("{}.json", name)),
    ]
}

/// 便利函数：自动发现并加载配置文件
pub fn auto_load_config<T>(name: &str, env_prefix: Option<&str>) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let config_paths = get_config_paths(name);
    ConfigManager::load_multiple(config_paths, env_prefix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use tempfile::tempdir;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestConfig {
        name: String,
        port: u16,
        debug: bool,
        database: DatabaseConfig,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct DatabaseConfig {
        host: String,
        port: u16,
        username: String,
        password: String,
    }

    impl Default for TestConfig {
        fn default() -> Self {
            Self {
                name: "test-app".to_string(),
                port: 8080,
                debug: false,
                database: DatabaseConfig {
                    host: "localhost".to_string(),
                    port: 5432,
                    username: "user".to_string(),
                    password: "password".to_string(),
                },
            }
        }
    }

    #[test]
    fn test_config_format_from_extension() {
        assert_eq!(
            ConfigFormat::from_extension(Path::new("config.yaml")),
            Some(ConfigFormat::Yaml)
        );
        assert_eq!(
            ConfigFormat::from_extension(Path::new("config.yml")),
            Some(ConfigFormat::Yaml)
        );
        assert_eq!(
            ConfigFormat::from_extension(Path::new("config.toml")),
            Some(ConfigFormat::Toml)
        );
        assert_eq!(
            ConfigFormat::from_extension(Path::new("config.json")),
            Some(ConfigFormat::Json)
        );
        assert_eq!(ConfigFormat::from_extension(Path::new("config.txt")), None);
    }

    #[test]
    fn test_load_yaml_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.yaml");

        let yaml_content = r#"
name: "test-service"
port: 3000
debug: true
database:
  host: "db.example.com"
  port: 5432
  username: "testuser"
  password: "testpass"
"#;

        fs::write(&config_path, yaml_content).unwrap();

        let config: TestConfig = ConfigManager::load_from_file(&config_path).unwrap();

        assert_eq!(config.name, "test-service");
        assert_eq!(config.port, 3000);
        assert_eq!(config.debug, true);
        assert_eq!(config.database.host, "db.example.com");
    }

    #[test]
    fn test_load_toml_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");

        let toml_content = r#"
name = "test-service"
port = 3000
debug = true

[database]
host = "db.example.com"
port = 5432
username = "testuser"
password = "testpass"
"#;

        fs::write(&config_path, toml_content).unwrap();

        let config: TestConfig = ConfigManager::load_from_file(&config_path).unwrap();

        assert_eq!(config.name, "test-service");
        assert_eq!(config.port, 3000);
        assert_eq!(config.debug, true);
        assert_eq!(config.database.host, "db.example.com");
    }

    #[test]
    fn test_load_json_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");

        let json_content = r#"{
  "name": "test-service",
  "port": 3000,
  "debug": true,
  "database": {
    "host": "db.example.com",
    "port": 5432,
    "username": "testuser",
    "password": "testpass"
  }
}"#;

        fs::write(&config_path, json_content).unwrap();

        let config: TestConfig = ConfigManager::load_from_file(&config_path).unwrap();

        assert_eq!(config.name, "test-service");
        assert_eq!(config.port, 3000);
        assert_eq!(config.debug, true);
        assert_eq!(config.database.host, "db.example.com");
    }

    #[test]
    fn test_config_builder_with_defaults() {
        let config: TestConfig = ConfigBuilder::new()
            .with_default("name", "default-app")
            .unwrap()
            .with_default("port", 9000)
            .unwrap()
            .with_default("debug", false)
            .unwrap()
            .with_default("database.host", "default-host")
            .unwrap()
            .with_default("database.port", 3306)
            .unwrap()
            .with_default("database.username", "default-user")
            .unwrap()
            .with_default("database.password", "default-pass")
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(config.name, "default-app");
        assert_eq!(config.port, 9000);
        assert_eq!(config.debug, false);
        assert_eq!(config.database.host, "default-host");
        assert_eq!(config.database.port, 3306);
    }

    #[test]
    fn test_config_with_env_override() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.yaml");

        let yaml_content = r#"
name: "test-service"
port: 3000
debug: false
database:
  host: "localhost"
  port: 5432
  username: "user"
  password: "password"
"#;

        fs::write(&config_path, yaml_content).unwrap();

        // 设置环境变量
        unsafe {
            env::set_var("TEST_PORT", "8080");
            env::set_var("TEST_DEBUG", "true");
            env::set_var("TEST_DATABASE__HOST", "env-db-host");
        }

        let config: TestConfig = ConfigManager::load_with_env(&config_path, "TEST").unwrap();

        assert_eq!(config.name, "test-service"); // 从文件
        assert_eq!(config.port, 8080); // 从环境变量覆盖
        assert_eq!(config.debug, true); // 从环境变量覆盖
        assert_eq!(config.database.host, "env-db-host"); // 从环境变量覆盖

        // 清理环境变量
        unsafe {
            env::remove_var("TEST_PORT");
            env::remove_var("TEST_DEBUG");
            env::remove_var("TEST_DATABASE__HOST");
        }
    }

    #[test]
    fn test_load_multiple_configs() {
        let dir = tempdir().unwrap();

        // 创建基础配置文件
        let base_config_path = dir.path().join("base.yaml");
        let base_content = r#"
name: "base-service"
port: 8000
debug: false
database:
  host: "base-host"
  port: 5432
  username: "base-user"
  password: "base-pass"
"#;
        fs::write(&base_config_path, base_content).unwrap();

        // 创建覆盖配置文件
        let override_config_path = dir.path().join("override.yaml");
        let override_content = r#"
port: 9000
debug: true
database:
  host: "override-host"
"#;
        fs::write(&override_config_path, override_content).unwrap();

        let config: TestConfig = ConfigManager::load_multiple(
            vec![&base_config_path, &override_config_path],
            None::<&str>,
        )
        .unwrap();

        assert_eq!(config.name, "base-service"); // 从基础配置
        assert_eq!(config.port, 9000); // 被覆盖
        assert_eq!(config.debug, true); // 被覆盖
        assert_eq!(config.database.host, "override-host"); // 被覆盖
        assert_eq!(config.database.username, "base-user"); // 从基础配置
    }

    #[test]
    fn test_get_config_paths() {
        let paths = get_config_paths("myapp");

        assert!(
            paths
                .iter()
                .any(|p| p.to_string_lossy().ends_with("myapp.yaml"))
        );
        assert!(
            paths
                .iter()
                .any(|p| p.to_string_lossy().ends_with("myapp.yml"))
        );
        assert!(
            paths
                .iter()
                .any(|p| p.to_string_lossy().ends_with("myapp.toml"))
        );
        assert!(
            paths
                .iter()
                .any(|p| p.to_string_lossy().ends_with("myapp.json"))
        );
        assert!(paths.iter().any(|p| p.to_string_lossy().contains("config")
            && p.to_string_lossy().ends_with("myapp.yaml")));
    }

    #[test]
    fn test_ignore_missing_files() {
        let dir = tempdir().unwrap();
        let existing_config = dir.path().join("existing.yaml");
        let missing_config = dir.path().join("missing.yaml");

        let yaml_content = r#"
name: "test-service"
port: 3000
debug: true
database:
  host: "localhost"
  port: 5432
  username: "user"
  password: "password"
"#;

        fs::write(&existing_config, yaml_content).unwrap();

        // 不忽略缺失文件时应该失败
        let result: Result<TestConfig> = ConfigBuilder::new()
            .add_file(&existing_config, None)
            .add_file(&missing_config, None)
            .ignore_missing_files(false)
            .build();
        assert!(result.is_err());

        // 忽略缺失文件时应该成功
        let config: TestConfig = ConfigBuilder::new()
            .add_file(&existing_config, None)
            .add_file(&missing_config, None)
            .ignore_missing_files(true)
            .build()
            .unwrap();

        assert_eq!(config.name, "test-service");
    }
}
