use clamber_core::{
    ClamberError, JwtConfig, JwtManager, Result, generate_token, is_valid_token, verify_token,
};
use serde::{Deserialize, Serialize};

// 定义用户结构体
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    id: String,
    username: String,
    email: String,
    role: String,
}

fn main() -> Result<()> {
    println!("=== Clamber Core 错误处理示例 ===\n");

    // 示例1：JWT错误处理
    example_jwt_error_handling()?;

    // 示例2：成功案例
    example_success_case()?;

    println!("4. 错误处理最佳实践示例:");
    demonstrate_error_handling_patterns();

    Ok(())
}

fn example_jwt_error_handling() -> Result<()> {
    println!("1. JWT错误处理:");

    // 测试无效密钥
    let empty_config = JwtConfig::new("", 7); // 空密钥
    let manager = JwtManager::new(empty_config);

    let user = User {
        id: "123".to_string(),
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        role: "user".to_string(),
    };

    match manager.generate_token(&user) {
        Ok(_) => println!("   意外：空密钥应该失败"),
        Err(ClamberError::JwtKeyError { details }) => {
            println!("   ✓ 正确检测到密钥错误: {}", details);
        }
        Err(e) => println!("   ? 其他错误: {}", e),
    }

    // 测试无效token验证
    let valid_config = JwtConfig::new("valid_secret", 7);
    let valid_manager = JwtManager::new(valid_config);

    match valid_manager.verify_token::<User>("invalid.jwt.token") {
        Ok(_) => println!("   意外：无效token应该失败"),
        Err(ClamberError::JwtVerifyError { details }) => {
            println!("   ✓ 正确检测到JWT验证错误: {}", details);
        }
        Err(e) => println!("   ? 其他JWT错误: {}", e),
    }

    // 测试不同类型的错误处理
    let config1 = JwtConfig::new("secret1", 7);
    let config2 = JwtConfig::new("secret2", 7);
    let manager1 = JwtManager::new(config1);
    let manager2 = JwtManager::new(config2);

    let token = manager1.generate_token(&user)?;

    match manager2.verify_token::<User>(&token) {
        Ok(_) => println!("   意外：不同密钥应该失败"),
        Err(ClamberError::JwtVerifyError { details }) => {
            println!("   ✓ 正确检测到密钥不匹配: {}", details);
        }
        Err(e) => println!("   ? 其他错误: {}", e),
    }

    println!();
    Ok(())
}

fn example_success_case() -> Result<()> {
    println!("2. 成功案例:");

    let user = User {
        id: "456".to_string(),
        username: "success_user".to_string(),
        email: "success@example.com".to_string(),
        role: "admin".to_string(),
    };

    // 使用便利函数
    let token = generate_token(&user).map_err(|e| {
        println!("   生成token失败: {}", e);
        e
    })?;

    println!("   ✓ 成功生成token");

    // 检查有效性
    if is_valid_token(&token) {
        println!("   ✓ Token有效性验证通过");
    } else {
        println!("   ✗ Token无效");
    }

    // 验证并解析
    let decoded_user: User = verify_token(&token).map_err(|e| {
        println!("   验证token失败: {}", e);
        e
    })?;

    println!("   ✓ 成功验证并解析token");
    println!("   ✓ 用户数据: {:?}", decoded_user);

    assert_eq!(user, decoded_user);
    println!("   ✓ 数据一致性验证通过");

    println!();
    Ok(())
}

// 展示如何在实际应用中处理错误
fn demonstrate_error_handling_patterns() {
    println!("   演示不同的错误处理模式:");

    // 模式1：具体错误类型处理
    println!("\n   模式1：具体错误类型匹配");
    let invalid_token = "invalid.token";
    match verify_token::<User>(invalid_token) {
        Ok(user) => println!("   用户验证成功: {:?}", user),
        Err(ClamberError::JwtExpiredError) => {
            println!("   → Token已过期，请重新登录");
        }
        Err(ClamberError::JwtVerifyError { details }) => {
            println!("   → Token验证失败: {}", details);
        }
        Err(ClamberError::JwtMissingFieldError { field }) => {
            println!("   → Token格式错误，缺少字段: {}", field);
        }
        Err(ClamberError::DeserializationError { details }) => {
            println!("   → 数据格式错误: {}", details);
        }
        Err(e) => {
            println!("   → 未知错误: {}", e);
        }
    }

    // 模式2：错误传播
    println!("\n   模式2：错误传播（? 操作符）");
    let result = (|| -> Result<()> {
        let user = User {
            id: "test".to_string(),
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            role: "user".to_string(),
        };
        let token = generate_token(&user)?;
        let _decoded: User = verify_token(&token)?;
        Ok(())
    })();

    match result {
        Ok(()) => println!("   ✓ 操作序列成功完成"),
        Err(e) => println!("   ✗ 操作序列失败: {}", e),
    }

    // 模式3：错误转换和上下文
    println!("\n   模式3：错误上下文处理");
    let result = (|| -> Result<String> {
        let config = JwtConfig::new("test_secret", 1);
        let manager = JwtManager::new(config);

        let user = User {
            id: "context_test".to_string(),
            username: "context_user".to_string(),
            email: "context@example.com".to_string(),
            role: "user".to_string(),
        };

        manager.generate_token(&user)
    })();

    match result {
        Ok(token) => println!("   ✓ 生成token成功: {}...", &token[..20]),
        Err(e) => println!("   ✗ 生成token失败: {}", e),
    }

    println!("\n   ✓ 错误处理模式演示完成");
}
