use crate::error::{ClamberError, Result};
use std::fs;
use tracing::metadata::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::fmt::time::ChronoUtc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, fmt};

/// 日志配置结构
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// 时间格式字符串
    pub time_format: String,
    /// 是否启用ANSI颜色（控制台）
    pub enable_ansi: bool,
    /// 是否显示目标模块
    pub show_target: bool,
    /// 是否显示线程ID
    pub show_thread_ids: bool,
    /// 是否使用紧凑格式
    pub compact_format: bool,
    /// 控制台日志级别
    pub console_level: LevelFilter,
    /// 文件日志级别
    pub file_level: LevelFilter,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            time_format: "%Y-%m-%d %H:%M:%S".to_string(),
            enable_ansi: true,
            show_target: false,
            show_thread_ids: false,
            compact_format: true,
            console_level: LevelFilter::INFO,
            file_level: LevelFilter::INFO,
        }
    }
}

impl LogConfig {
    /// 创建新的日志配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置时间格式
    pub fn time_format(mut self, format: impl Into<String>) -> Self {
        self.time_format = format.into();
        self
    }

    /// 启用/禁用ANSI颜色
    pub fn ansi(mut self, enable: bool) -> Self {
        self.enable_ansi = enable;
        self
    }

    /// 显示/隐藏目标模块
    pub fn target(mut self, show: bool) -> Self {
        self.show_target = show;
        self
    }

    /// 显示/隐藏线程ID
    pub fn thread_ids(mut self, show: bool) -> Self {
        self.show_thread_ids = show;
        self
    }

    /// 设置紧凑格式
    pub fn compact(mut self, enable: bool) -> Self {
        self.compact_format = enable;
        self
    }

    /// 设置控制台日志级别
    pub fn console_level(mut self, level: LevelFilter) -> Self {
        self.console_level = level;
        self
    }

    /// 设置文件日志级别
    pub fn file_level(mut self, level: LevelFilter) -> Self {
        self.file_level = level;
        self
    }
}

// pub fn logger_start(
//     service_name: &str,
//     path: Option<String>,
// ) -> Result<(WorkerGuard, WorkerGuard)> {
//     let log_dir = match path {
//         Some(p) => format!("{}/logs", p),
//         None => format!("logs"),
//     };

//     fs::create_dir_all(&log_dir).map_err(|_| ClamberError::DirectoryCreationError {
//         path: log_dir.clone(),
//     })?;

//     let info_file = rolling::daily(&log_dir, format!("{}-info.log", service_name));
//     let error_file = rolling::daily(&log_dir, format!("{}-error.log", service_name));

//     let (info_writer, info_guard) = tracing_appender::non_blocking(info_file);
//     let (error_writer, error_guard) = tracing_appender::non_blocking(error_file);

//     // 创建自定义时间格式：yyyy-MM-dd HH:mm:ss
//     let timer = ChronoUtc::new("%Y-%m-%d %H:%M:%S".to_string());

//     let info_layer = fmt::layer()
//         .with_writer(info_writer)
//         .with_ansi(false)
//         .with_level(true)
//         .with_target(true)
//         .with_thread_ids(false)
//         .with_timer(timer.clone())
//         .with_filter(filter_fn(|metadata| {
//             metadata.level() == &tracing::Level::INFO
//         }));

//     let error_layer = fmt::layer()
//         .with_writer(error_writer)
//         .with_ansi(false)
//         .with_level(true)
//         .with_target(true)
//         .with_thread_ids(false)
//         .with_timer(timer.clone())
//         .with_filter(LevelFilter::ERROR);

//     let console_layer = fmt::layer()
//         .with_ansi(true)
//         .with_level(true)
//         .with_target(false) // 控制台不显示模块路径以保持简洁
//         .with_thread_ids(false)
//         .with_timer(timer)
//         .compact() // 使用紧凑格式
//         .with_filter(LevelFilter::INFO);

//     tracing_subscriber::registry()
//         .with(info_layer)
//         .with(error_layer)
//         .with(console_layer)
//         .init();

//     Ok((info_guard, error_guard))
// }

/// 使用自定义配置初始化日志系统
pub fn logger_start_with_config(
    service_name: &str,
    path: Option<String>,
    config: LogConfig,
) -> Result<(WorkerGuard, WorkerGuard)> {
    let log_dir = match path {
        Some(p) => format!("logs/{}", p),
        None => format!("logs"),
    };

    fs::create_dir_all(&log_dir).map_err(|_| ClamberError::DirectoryCreationError {
        path: log_dir.clone(),
    })?;

    let info_file = rolling::daily(&log_dir, format!("{}-info.log", service_name));
    let error_file = rolling::daily(&log_dir, format!("{}-error.log", service_name));

    let (info_writer, info_guard) = tracing_appender::non_blocking(info_file);
    let (error_writer, error_guard) = tracing_appender::non_blocking(error_file);

    // 使用用户配置的时间格式
    let timer = ChronoUtc::new(config.time_format.clone());

    // 根据配置选择格式类型
    if config.compact_format {
        // 使用紧凑格式
        let info_layer = fmt::layer()
            .compact()
            .with_writer(info_writer)
            .with_ansi(false)
            .with_level(true)
            .with_target(config.show_target)
            .with_thread_ids(config.show_thread_ids)
            .with_timer(timer.clone())
            .with_filter(filter_fn(move |metadata| {
                metadata.level() == &tracing::Level::INFO
            }));

        let error_layer = fmt::layer()
            .compact()
            .with_writer(error_writer)
            .with_ansi(false)
            .with_level(true)
            .with_target(config.show_target)
            .with_thread_ids(config.show_thread_ids)
            .with_timer(timer.clone())
            .with_filter(LevelFilter::ERROR);

        let console_layer = fmt::layer()
            .compact()
            .with_ansi(config.enable_ansi)
            .with_level(true)
            .with_target(config.show_target)
            .with_thread_ids(config.show_thread_ids)
            .with_timer(timer)
            .with_filter(config.console_level);

        tracing_subscriber::registry()
            .with(info_layer)
            .with(error_layer)
            .with(console_layer)
            .init();
    } else {
        // 使用完整格式
        let info_layer = fmt::layer()
            .with_writer(info_writer)
            .with_ansi(false)
            .with_level(true)
            .with_target(config.show_target)
            .with_thread_ids(config.show_thread_ids)
            .with_timer(timer.clone())
            .with_filter(filter_fn(move |metadata| {
                metadata.level() == &tracing::Level::INFO
            }));

        let error_layer = fmt::layer()
            .with_writer(error_writer)
            .with_ansi(false)
            .with_level(true)
            .with_target(config.show_target)
            .with_thread_ids(config.show_thread_ids)
            .with_timer(timer.clone())
            .with_filter(LevelFilter::ERROR);

        let console_layer = fmt::layer()
            .with_ansi(config.enable_ansi)
            .with_level(true)
            .with_target(config.show_target)
            .with_thread_ids(config.show_thread_ids)
            .with_timer(timer)
            .with_filter(config.console_level);

        tracing_subscriber::registry()
            .with(info_layer)
            .with(error_layer)
            .with(console_layer)
            .init();
    }

    Ok((info_guard, error_guard))
}
