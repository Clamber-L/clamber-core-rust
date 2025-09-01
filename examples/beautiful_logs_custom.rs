use clamber_core::{LogConfig, Result, logger_start_with_config};
use tracing::metadata::LevelFilter;
use tracing::{debug, error, info, trace, warn};

fn main() -> Result<()> {
    println!("=== Clamber Core 自定义美化日志示例 ===\n");

    // 使用自定义配置
    example_custom_logging()?;

    Ok(())
}

fn example_custom_logging() -> Result<()> {
    println!("使用自定义美化配置:");

    // 创建自定义配置
    let config = LogConfig::new()
        .time_format("%Y-%m-%d %H:%M:%S%.3f") // 包含毫秒
        .ansi(true) // 启用颜色
        .target(true) // 显示模块路径
        .thread_ids(false) // 不显示线程ID
        .compact(false) // 使用完整格式
        .console_level(LevelFilter::DEBUG) // 控制台显示DEBUG级别
        .file_level(LevelFilter::INFO); // 文件只记录INFO及以上

    let _guards = logger_start_with_config("custom_demo", None, config)?;

    println!("📝 开始记录不同级别的美化日志...\n");

    // 记录各种级别的日志
    trace!("🔍 这是TRACE级别日志 - 通常用于非常详细的调试信息");
    debug!("🐛 这是DEBUG级别日志 - 用于调试信息");
    info!("ℹ️  这是INFO级别日志 - 用于一般信息");
    warn!("⚠️  这是WARN级别日志 - 用于警告信息");
    error!("❌ 这是ERROR级别日志 - 用于错误信息");

    println!();

    // 记录结构化数据
    info!(
        event = "user_registration",
        user_id = 12345,
        email = "user@example.com",
        registration_time = "2024-01-15T10:30:00Z",
        source = "web_app",
        "🎉 新用户注册成功"
    );

    info!(
        event = "payment_processed",
        user_id = 67890,
        amount = 299.99,
        currency = "CNY",
        payment_method = "credit_card",
        transaction_id = "TXN-ABC123",
        merchant_id = "MERCHANT_001",
        "💳 支付处理完成"
    );

    // 记录系统状态
    debug!(
        memory_usage = "256MB",
        cpu_usage = "15%",
        active_connections = 42,
        response_time_ms = 120,
        "📊 系统状态检查"
    );

    // 记录警告信息
    warn!(
        error_code = "AUTH_FAILED",
        attempts = 3,
        max_attempts = 5,
        ip_address = "203.0.113.42",
        user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
        blocked_duration = "5min",
        "🚫 认证失败，剩余尝试次数: {}",
        2
    );

    // 记录错误信息
    error!(
        error_type = "database_connection",
        error_message = "Connection timeout after 30s",
        database_host = "db.example.com",
        port = 5432,
        retry_count = 3,
        "🔌 数据库连接失败"
    );

    // 记录业务逻辑
    info!(
        order_id = "ORD-2024-001",
        customer_id = "CUST-9876",
        items_count = 3,
        total_amount = 599.97,
        shipping_address = "北京市朝阳区xxx街道",
        estimated_delivery = "2024-01-18",
        "📦 订单创建成功"
    );

    // 记录API调用
    debug!(
        api_endpoint = "/api/v1/users/profile",
        method = "GET",
        status_code = 200,
        response_time_ms = 85,
        request_id = "req-uuid-12345",
        user_id = 98765,
        "🌐 API调用成功"
    );

    println!("\n✅ 自定义配置日志记录完成！");
    println!("📁 日志文件保存在: ./logs/");
    println!("🎨 配置特性:");
    println!("   • 时间格式: yyyy-MM-dd HH:mm:ss.SSS (包含毫秒)");
    println!("   • 控制台: 彩色输出，显示DEBUG级别及以上");
    println!("   • 文件: 无颜色，记录INFO级别及以上");
    println!("   • 格式: 完整格式，显示模块路径");
    println!("   • 线程: 不显示线程ID");

    Ok(())
}
