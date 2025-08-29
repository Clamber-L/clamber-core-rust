use clamber_core::Result;
use clamber_core::token::{JwtConfig, JwtManager, generate_token, is_valid_token, verify_token};
use serde::{Deserialize, Serialize};

// 定义你的用户结构体
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    id: String,
    username: String,
    email: String,
    role: String,
}

// 也可以是任何其他结构体
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SessionData {
    user_id: String,
    permissions: Vec<String>,
    login_time: i64,
}

fn main() -> Result<()> {
    println!("=== Clamber Core JWT 使用示例 ===\n");

    // 示例1：使用默认配置的便利函数
    example_convenience_functions()?;

    // 示例2：使用自定义配置
    example_custom_config()?;

    // 示例3：不同类型的payload
    example_different_payloads()?;

    // 示例4：错误处理
    example_error_handling()?;

    Ok(())
}

fn example_convenience_functions() -> Result<()> {
    println!("1. 使用默认配置的便利函数:");

    let user = User {
        id: "user123".to_string(),
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
        role: "admin".to_string(),
    };

    // 生成token
    let token = generate_token(&user, JwtConfig::default())?;
    println!("   生成的token: {}", token);

    // 检查token是否有效
    let is_valid = is_valid_token(&token);
    println!("   Token有效性: {}", is_valid);

    // 验证并解析token
    let decoded_user: User = verify_token(&token)?;
    println!("   解析出的用户: {:?}", decoded_user);

    // 验证数据一致性
    assert_eq!(user, decoded_user);
    println!("   ✓ 数据一致性验证通过\n");

    Ok(())
}

fn example_custom_config() -> Result<()> {
    println!("2. 使用自定义配置:");

    // 创建自定义配置
    let config = JwtConfig::new("my_super_secret_key_2024", 30); // 30天过期
    let jwt_manager = JwtManager::new(config);

    let session_data = SessionData {
        user_id: "user456".to_string(),
        permissions: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
        login_time: chrono::Utc::now().timestamp(),
    };

    // 使用自定义配置生成token
    let token = jwt_manager.generate_token(&session_data)?;
    println!("   自定义配置生成的token: {}...", &token[..50]);

    // 使用同样的管理器验证token
    let decoded_session: SessionData = jwt_manager.verify_token(&token)?;
    println!("   解析出的会话数据: {:?}", decoded_session);

    assert_eq!(session_data, decoded_session);
    println!("   ✓ 自定义配置验证通过\n");

    Ok(())
}

fn example_different_payloads() -> Result<()> {
    println!("3. 不同类型的payload支持:");

    // 字符串payload
    let simple_data = "Hello JWT World!";
    let token1 = generate_token(&simple_data, JwtConfig::default())?;
    let decoded_string: String = verify_token(&token1)?;
    println!("   字符串payload: {} -> {}", simple_data, decoded_string);

    // 数字payload
    let number_data = 42i32;
    let token2 = generate_token(&number_data, JwtConfig::default())?;
    let decoded_number: i32 = verify_token(&token2)?;
    println!("   数字payload: {} -> {}", number_data, decoded_number);

    // 复杂结构体
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct ComplexData {
        name: String,
        values: Vec<i32>,
        metadata: std::collections::HashMap<String, String>,
    }

    let mut metadata = std::collections::HashMap::new();
    metadata.insert("version".to_string(), "1.0".to_string());
    metadata.insert("env".to_string(), "production".to_string());

    let complex_data = ComplexData {
        name: "test_data".to_string(),
        values: vec![1, 2, 3, 4, 5],
        metadata,
    };

    let token3 = generate_token(&complex_data, JwtConfig::default())?;
    let decoded_complex: ComplexData = verify_token(&token3)?;
    println!("   复杂结构体payload: {:?}", decoded_complex);

    assert_eq!(complex_data, decoded_complex);
    println!("   ✓ 不同类型payload验证通过\n");

    Ok(())
}

fn example_error_handling() -> Result<()> {
    println!("4. 错误处理示例:");

    // 无效的token
    let invalid_token = "invalid.jwt.token";
    match verify_token::<User>(invalid_token) {
        Ok(_) => println!("   意外：无效token被接受了"),
        Err(e) => println!("   ✓ 无效token正确被拒绝: {}", e),
    }

    // 不同密钥的token
    let config1 = JwtConfig::new("secret1", 7);
    let config2 = JwtConfig::new("secret2", 7);
    let manager1 = JwtManager::new(config1);
    let manager2 = JwtManager::new(config2);

    let user = User {
        id: "test".to_string(),
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        role: "user".to_string(),
    };

    let token = manager1.generate_token(&user)?;
    match manager2.verify_token::<User>(&token) {
        Ok(_) => println!("   意外：不同密钥的token被接受了"),
        Err(e) => println!("   ✓ 不同密钥的token正确被拒绝: {}", e),
    }

    println!("   ✓ 错误处理验证完成\n");

    Ok(())
}
