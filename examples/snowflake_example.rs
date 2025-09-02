use clamber_core::{Result, SnowflakeConfig, SnowflakeManager, snowflake_utils};

fn main() -> Result<()> {
    println!("=== Snowflake ID 生成器示例 ===\n");

    // 1. 使用默认配置（便利函数）
    println!("1. 使用默认配置生成ID:");
    let id1 = snowflake_utils::generate_id()?;
    println!("   生成的ID: {}", id1);

    // 解析ID信息
    let info = snowflake_utils::parse_id(id1)?;
    println!("   时间戳: {}", info.timestamp);
    println!("   工作者ID: {}", info.worker_id);
    println!("   序列号: {}", info.sequence);
    println!("   生成时间: {}", info.generation_time_string(None));

    // 2. 使用自定义配置
    println!("\n2. 使用自定义配置:");
    let config = SnowflakeConfig::new(5)?;
    let manager = SnowflakeManager::new(config)?;

    let id2 = manager.generate_id()?;
    println!("   生成的ID: {}", id2);
    println!("   工作者ID: {}", manager.worker_id());

    // 3. 批量生成ID
    println!("\n3. 批量生成ID:");
    let ids = manager.generate_ids(5)?;
    for (i, id) in ids.iter().enumerate() {
        println!("   ID {}: {}", i + 1, id);
    }

    // 4. 字符串格式的ID
    println!("\n4. 字符串格式的ID:");
    let string_id = snowflake_utils::generate_string_id()?;
    println!("   字符串ID: {}", string_id);

    let parsed_info = snowflake_utils::parse_string_id(&string_id)?;
    println!("   解析结果: {:?}", parsed_info);

    // 5. 使用自定义纪元
    println!("\n5. 使用自定义纪元的配置:");
    let custom_epoch = 1609459200000; // 2021-01-01 00:00:00 UTC
    let config_with_epoch = SnowflakeConfig::with_epoch(10, custom_epoch)?;
    let manager_with_epoch = SnowflakeManager::new(config_with_epoch)?;

    let id3 = manager_with_epoch.generate_id()?;
    let info3 = manager_with_epoch.parse_id(id3);
    println!("   生成的ID: {}", id3);
    println!(
        "   生成时间（自定义纪元）: {}",
        info3.generation_time_string(Some(custom_epoch))
    );

    println!("\n=== 示例完成 ===");

    Ok(())
}
