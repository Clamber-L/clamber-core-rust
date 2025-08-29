use crate::error::{ClamberError, Result};
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Serialize, de::DeserializeOwned};
use sha2::Sha256;
use std::collections::BTreeMap;

/// JWT配置结构
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// JWT密钥
    pub secret: String,
    /// 过期时间（天数）
    pub expire_days: i64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "default_jwt_secret".to_string(),
            expire_days: 7,
        }
    }
}

impl JwtConfig {
    /// 创建新的JWT配置
    pub fn new(secret: impl Into<String>, expire_days: i64) -> Self {
        Self {
            secret: secret.into(),
            expire_days,
        }
    }
}

/// JWT管理器
pub struct JwtManager {
    config: JwtConfig,
}

impl JwtManager {
    /// 创建新的JWT管理器
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建JWT管理器
    pub fn default() -> Self {
        Self {
            config: JwtConfig::default(),
        }
    }

    /// 生成JWT token
    pub fn generate_token<T>(&self, payload: &T) -> Result<String>
    where
        T: Serialize,
    {
        let expire_time = Utc::now() + Duration::days(self.config.expire_days);

        // 将payload序列化为JSON字符串
        let payload_json = serde_json::to_string(payload)?;

        let mut claims = BTreeMap::new();
        claims.insert("payload".to_string(), payload_json);
        claims.insert("exp".to_string(), expire_time.timestamp().to_string());
        claims.insert("createAt".to_string(), Utc::now().timestamp().to_string());

        let key: Hmac<Sha256> =
            Hmac::new_from_slice(self.config.secret.as_bytes()).map_err(|e| {
                ClamberError::JwtKeyError {
                    details: e.to_string(),
                }
            })?;

        claims
            .sign_with_key(&key)
            .map_err(|e| ClamberError::JwtSignError {
                details: e.to_string(),
            })
    }

    /// 验证并解析JWT token
    pub fn verify_token<T>(&self, token: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(self.config.secret.as_bytes()).map_err(|e| {
                ClamberError::JwtKeyError {
                    details: e.to_string(),
                }
            })?;

        let claims: BTreeMap<String, String> =
            token
                .verify_with_key(&key)
                .map_err(|e| ClamberError::JwtVerifyError {
                    details: e.to_string(),
                })?;

        // 检查过期时间
        if let Some(exp_str) = claims.get("exp") {
            let exp_timestamp = exp_str.parse::<i64>().map_err(|_| ClamberError::JwtError {
                message: "无效的过期时间格式".to_string(),
            })?;

            if exp_timestamp <= Utc::now().timestamp() {
                return Err(ClamberError::JwtExpiredError);
            }
        } else {
            return Err(ClamberError::JwtMissingFieldError {
                field: "exp".to_string(),
            });
        }

        // 获取payload并反序列化
        if let Some(payload_str) = claims.get("payload") {
            serde_json::from_str::<T>(payload_str).map_err(|e| ClamberError::DeserializationError {
                details: e.to_string(),
            })
        } else {
            Err(ClamberError::JwtMissingFieldError {
                field: "payload".to_string(),
            })
        }
    }

    /// 检查token是否有效（不解析payload）
    pub fn is_valid_token(&self, token: &str) -> bool {
        let key = match Hmac::<Sha256>::new_from_slice(self.config.secret.as_bytes()) {
            Ok(key) => key,
            Err(_) => return false,
        };

        if let Ok(claims) = token.verify_with_key(&key) {
            let claims: BTreeMap<String, String> = claims;
            if let Some(exp_str) = claims.get("exp") {
                if let Ok(exp_timestamp) = exp_str.parse::<i64>() {
                    return exp_timestamp > Utc::now().timestamp();
                }
            }
        }
        false
    }
}

// 便利函数：使用默认配置
pub fn generate_token<T>(payload: &T, config: JwtConfig) -> Result<String>
where
    T: Serialize,
{
    let manager = JwtManager::new(config);
    manager.generate_token(payload)
}

pub fn verify_token<T>(token: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let manager = JwtManager::default();
    manager.verify_token(token)
}

pub fn is_valid_token(token: &str) -> bool {
    let manager = JwtManager::default();
    manager.is_valid_token(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestUser {
        pub id: String,
        pub name: String,
        pub role: String,
    }

    #[test]
    fn test_jwt_generate_and_verify() {
        let config = JwtConfig::new("test_secret", 1);
        let manager = JwtManager::new(config);

        let user = TestUser {
            id: "123".to_string(),
            name: "John Doe".to_string(),
            role: "admin".to_string(),
        };

        // 生成token
        let token = manager.generate_token(&user).unwrap();
        assert!(!token.is_empty());

        // 验证token
        let decoded_user: TestUser = manager.verify_token(&token).unwrap();
        assert_eq!(user, decoded_user);

        // 检查token有效性
        assert!(manager.is_valid_token(&token));
    }

    #[test]
    fn test_convenience_functions() {
        let user = TestUser {
            id: "456".to_string(),
            name: "Jane Doe".to_string(),
            role: "user".to_string(),
        };

        let token = generate_token(&user, JwtConfig::default()).unwrap();
        let decoded_user: TestUser = verify_token(&token).unwrap();

        assert_eq!(user, decoded_user);
        assert!(is_valid_token(&token));
    }

    #[test]
    fn test_invalid_token() {
        let manager = JwtManager::default();

        // 测试无效token
        assert!(!manager.is_valid_token("invalid_token"));

        // 测试错误密钥
        let config1 = JwtConfig::new("secret1", 1);
        let config2 = JwtConfig::new("secret2", 1);
        let manager1 = JwtManager::new(config1);
        let manager2 = JwtManager::new(config2);

        let user = TestUser {
            id: "789".to_string(),
            name: "Test User".to_string(),
            role: "test".to_string(),
        };

        let token = manager1.generate_token(&user).unwrap();
        assert!(manager2.verify_token::<TestUser>(&token).is_err());
    }
}
