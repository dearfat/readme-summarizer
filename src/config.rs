use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use crate::error::{AppError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub source_directory: String,
    pub readme_filename: String,
    pub output_filename: String,
    #[serde(default)]
    pub obsidian_vault_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            source_directory: String::new(),
            readme_filename: "README.md".to_string(),
            output_filename: "README-SUMMARY.md".to_string(),
            obsidian_vault_path: String::new(),
        }
    }
}

impl Config {
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::ConfigError("无法获取配置目录".to_string()))?
            .join("readme-summarizer");
        
        Ok(config_dir.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| AppError::ConfigError(format!("读取配置文件失败: {}", e)))?;
            
            match toml::from_str(&content) {
                Ok(config) => Ok(config),
                Err(e) => {
                    eprintln!("⚠️  配置文件损坏: {}，使用默认配置", e);
                    Ok(Config::default())
                }
            }
        } else {
            Ok(Config::default())
        }
    }

    pub fn prompt_for_source() -> Result<String> {
        println!("📁 欢迎使用 README Summarizer!\n");
        
        loop {
            print!("请输入要扫描的源目录路径: ");
            io::stdout().flush()
                .map_err(|e| AppError::ConfigError(format!("输出错误: {}", e)))?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .map_err(|e| AppError::ConfigError(format!("读取输入失败: {}", e)))?;
            
            let path = input.trim().to_string();
            
            if path.is_empty() {
                eprintln!("❌ 路径不能为空，请重新输入");
                continue;
            }
            
            let expanded = Self::expand_path(&path);
            if !expanded.exists() {
                eprintln!("❌ 路径不存在: {}，请重新输入", expanded.display());
                continue;
            }
            if !expanded.is_dir() {
                eprintln!("❌ 路径不是目录: {}，请重新输入", expanded.display());
                continue;
            }
            
            println!("✓ 配置已保存\n");
            return Ok(path);
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| AppError::ConfigError(format!("创建配置目录失败: {}", e)))?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| AppError::ConfigError(format!("序列化配置失败: {}", e)))?;
        
        fs::write(&config_path, content)
            .map_err(|e| AppError::ConfigError(format!("写入配置文件失败: {}", e)))?;

        Ok(())
    }

    pub fn expand_path(path: &str) -> PathBuf {
        if path.starts_with('~') {
            if let Some(home) = dirs::home_dir() {
                return home.join(path.trim_start_matches("~/"));
            }
        }
        PathBuf::from(path)
    }

    pub fn validate(&self) -> Result<()> {
        let source = Self::expand_path(&self.source_directory);
        if !source.exists() {
            return Err(AppError::ConfigError(format!(
                "源目录不存在: {}",
                source.display()
            )));
        }
        if !source.is_dir() {
            return Err(AppError::ConfigError(format!(
                "源路径不是目录: {}",
                source.display()
            )));
        }
        Ok(())
    }
}
