mod config;
mod error;
mod scanner;
mod parser;
mod generator;

use clap::{Parser as ClapParser, Subcommand};
use config::Config;
use scanner::Scanner;
use parser::Parser;
use generator::Generator;
use error::Result;
use std::path::Path;
use std::time::Duration;
use notify_debouncer_mini::{new_debouncer, notify::*, DebounceEventResult};
use std::sync::mpsc::channel;

#[derive(ClapParser, Debug)]
#[command(name = "readme-sum")]
#[command(author, version, about = "扫描目录并生成 README 摘要", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, help = "扫描的源目录（覆盖配置文件）")]
    source: Option<String>,

    #[arg(short, long, help = "输出文件路径（默认: {源目录}/README-SUMMARY.md）")]
    output: Option<String>,

    #[arg(short, long, help = "README 文件名（默认: README.md）")]
    filename: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "监控源目录变化并自动重新生成摘要")]
    Watch {
        #[arg(short, long, help = "扫描的源目录（覆盖配置文件）")]
        source: Option<String>,
    },
    #[command(about = "同步摘要文件到 Obsidian vault")]
    SyncObsidian {
        #[arg(short, long, help = "Obsidian vault 路径（覆盖配置文件）")]
        vault: Option<String>,
    },
    #[command(about = "管理配置文件")]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    #[command(about = "查看当前配置")]
    View,
    #[command(about = "设置配置项")]
    Set {
        #[arg(help = "配置项名称 (source|readme-filename|output-filename|obsidian-vault)")]
        key: String,
        #[arg(help = "配置项的值")]
        value: String,
    },
    #[command(about = "在默认编辑器中打开配置文件")]
    Edit,
    #[command(about = "显示配置文件路径")]
    Path,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("❌ 错误: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Watch { source }) => {
            return run_watch(source.clone());
        }
        Some(Commands::SyncObsidian { vault }) => {
            return run_sync_obsidian(vault.clone());
        }
        Some(Commands::Config { action }) => {
            return run_config(action);
        }
        None => {
            // Continue with normal generation
        }
    }
    
    let mut config = Config::load()?;
    let mut config_updated = false;

    if let Some(source) = cli.source {
        config.source_directory = source;
        config_updated = true;
    } else if config.source_directory.is_empty() {
        config.source_directory = Config::prompt_for_source()?;
        config_updated = true;
    }

    if let Some(filename) = cli.filename {
        config.readme_filename = filename;
    }

    // Save config if it was updated (first run or source changed)
    if config_updated {
        config.save()?;
        if let Ok(config_path) = Config::config_path() {
            println!("✓ 配置已保存到 {}\n", config_path.display());
        }
    }

    config.validate()?;

    let source_dir = Config::expand_path(&config.source_directory);
    
    let output_path = if let Some(output) = cli.output {
        Config::expand_path(&output)
    } else {
        source_dir.join(&config.output_filename)
    };

    println!("📁 扫描目录: {}", source_dir.display());
    println!("📝 README 文件名: {}", config.readme_filename);
    println!("� 输出文件: {}", output_path.display());
    println!();

    let scanner = Scanner::new(source_dir, config.readme_filename);
    let projects = scanner.scan()?;

    if projects.is_empty() {
        println!("⚠️  未找到任何包含 README 文件的项目");
        return Ok(());
    }

    let mut entries = Vec::new();
    let mut success_count = 0;
    let mut skip_count = 0;

    for project in &projects {
        match Parser::parse(&project.readme_path, &project.project_name) {
            Ok(summary) => {
                println!("✓ 处理: {}", project.project_name);
                entries.push(generator::ProjectEntry {
                    summary,
                    source_path: project.readme_path.clone(),
                });
                success_count += 1;
            }
            Err(e) => {
                eprintln!("✗ 解析失败: {} - {}", project.project_name, e);
                skip_count += 1;
            }
        }
    }

    if !entries.is_empty() {
        let generator = Generator::new(output_path.to_path_buf());
        generator.generate_table(&entries)?;
    }

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    if skip_count > 0 {
        println!(
            "✅ 完成! 成功处理 {} 个项目，失败 {} 个",
            success_count, skip_count
        );
    } else {
        println!("✅ 完成! 成功处理 {} 个项目", success_count);
    }
    println!("📄 摘要文件已生成: {}", output_path.display());

    Ok(())
}

fn generate_summary(source_dir: &Path, readme_filename: &str, output_path: &Path) -> Result<()> {
    let scanner = Scanner::new(source_dir.to_path_buf(), readme_filename.to_string());
    let projects = scanner.scan()?;

    if projects.is_empty() {
        println!("⚠️  未找到任何包含 README 文件的项目");
        return Ok(());
    }

    let mut entries = Vec::new();
    let mut success_count = 0;
    let mut skip_count = 0;

    for project in &projects {
        match Parser::parse(&project.readme_path, &project.project_name) {
            Ok(summary) => {
                println!("✓ 处理: {}", project.project_name);
                entries.push(generator::ProjectEntry {
                    summary,
                    source_path: project.readme_path.clone(),
                });
                success_count += 1;
            }
            Err(e) => {
                eprintln!("✗ 解析失败: {} - {}", project.project_name, e);
                skip_count += 1;
            }
        }
    }

    if !entries.is_empty() {
        let generator = Generator::new(output_path.to_path_buf());
        generator.generate_table(&entries)?;
    }

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    if skip_count > 0 {
        println!(
            "✅ 完成! 成功处理 {} 个项目，失败 {} 个",
            success_count, skip_count
        );
    } else {
        println!("✅ 完成! 成功处理 {} 个项目", success_count);
    }
    println!("📄 摘要文件已生成: {}", output_path.display());

    Ok(())
}

fn run_watch(source_override: Option<String>) -> Result<()> {
    use error::AppError;
    
    let mut config = Config::load()?;
    
    if let Some(source) = source_override {
        config.source_directory = source;
    } else if config.source_directory.is_empty() {
        config.source_directory = Config::prompt_for_source()?;
        config.save()?;
    }
    
    config.validate()?;
    
    let source_dir = Config::expand_path(&config.source_directory);
    let output_path = source_dir.join(&config.output_filename);
    
    println!("👀 监控模式启动");
    println!("📁 监控目录: {}", source_dir.display());
    println!("📄 输出文件: {}", output_path.display());
    println!("按 Ctrl+C 停止监控\n");
    
    // Initial generation
    println!("🔄 初始生成...");
    generate_summary(&source_dir, &config.readme_filename, &output_path)?;
    println!();
    
    let (tx, rx) = channel();
    let tx_clone = tx.clone();
    
    let mut debouncer = new_debouncer(Duration::from_secs(2), move |result: DebounceEventResult| {
        match result {
            Ok(events) => {
                for event in events {
                    // Check if it's a README file change or new directory
                    let path_str = event.path.to_string_lossy();
                    if path_str.contains("README") && !path_str.contains("README-SUMMARY") {
                        let _ = tx_clone.send(());
                        break;
                    }
                }
            }
            Err(e) => eprintln!("监控错误: {:?}", e),
        }
    }).map_err(|e| AppError::ConfigError(format!("创建文件监控失败: {}", e)))?;
    
    debouncer.watcher().watch(&source_dir, RecursiveMode::Recursive)
        .map_err(|e| AppError::ConfigError(format!("监控目录失败: {}", e)))?;
    
    println!("✅ 监控已启动，等待文件变化...\n");
    
    for _ in rx {
        println!("🔄 检测到变化，重新生成摘要...");
        if let Err(e) = generate_summary(&source_dir, &config.readme_filename, &output_path) {
            eprintln!("❌ 生成失败: {}", e);
        }
        println!();
    }
    
    Ok(())
}

fn run_sync_obsidian(vault_override: Option<String>) -> Result<()> {
    use error::AppError;
    use std::fs;
    
    let mut config = Config::load()?;
    
    let vault_path = if let Some(vault) = vault_override {
        vault
    } else if !config.obsidian_vault_path.is_empty() {
        config.obsidian_vault_path.clone()
    } else {
        println!("📚 Obsidian Vault 同步\n");
        print!("请输入 Obsidian vault 路径: ");
        std::io::Write::flush(&mut std::io::stdout())
            .map_err(|e| AppError::ConfigError(format!("输出错误: {}", e)))?;
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .map_err(|e| AppError::ConfigError(format!("读取输入失败: {}", e)))?;
        
        let path = input.trim().to_string();
        
        // Save to config
        config.obsidian_vault_path = path.clone();
        config.save()?;
        println!("✓ Vault 路径已保存到配置\n");
        
        path
    };
    
    let vault_dir = Config::expand_path(&vault_path);
    
    if !vault_dir.exists() {
        return Err(AppError::ConfigError(format!(
            "Obsidian vault 路径不存在: {}",
            vault_dir.display()
        )));
    }
    
    if !vault_dir.is_dir() {
        return Err(AppError::ConfigError(format!(
            "路径不是目录: {}",
            vault_dir.display()
        )));
    }
    
    // Get source directory and output file
    if config.source_directory.is_empty() {
        return Err(AppError::ConfigError(
            "未配置源目录，请先运行一次 readme-sum 或使用 --source 参数".to_string()
        ));
    }
    
    let source_dir = Config::expand_path(&config.source_directory);
    let source_file = source_dir.join(&config.output_filename);
    
    if !source_file.exists() {
        return Err(AppError::ConfigError(format!(
            "摘要文件不存在: {}，请先运行 readme-sum 生成摘要",
            source_file.display()
        )));
    }
    
    let dest_file = vault_dir.join(&config.output_filename);
    
    println!("📚 同步到 Obsidian");
    println!("📄 源文件: {}", source_file.display());
    println!("📁 目标位置: {}", dest_file.display());
    
    fs::copy(&source_file, &dest_file)
        .map_err(|e| AppError::ConfigError(format!("复制文件失败: {}", e)))?;
    
    println!("\n✅ 同步完成!");
    
    Ok(())
}

fn run_config(action: &ConfigAction) -> Result<()> {
    use error::AppError;
    use std::process::Command;
    
    match action {
        ConfigAction::View => {
            let config = Config::load()?;
            let config_path = Config::config_path()?;
            
            println!("📋 当前配置");
            println!("配置文件: {}\n", config_path.display());
            println!("源目录:           {}", if config.source_directory.is_empty() { 
                "(未设置)".to_string() 
            } else { 
                config.source_directory.clone() 
            });
            println!("README 文件名:    {}", config.readme_filename);
            println!("输出文件名:       {}", config.output_filename);
            println!("Obsidian Vault:   {}", if config.obsidian_vault_path.is_empty() { 
                "(未设置)".to_string() 
            } else { 
                config.obsidian_vault_path.clone() 
            });
        }
        
        ConfigAction::Set { key, value } => {
            let mut config = Config::load()?;
            
            match key.as_str() {
                "source" | "source-directory" => {
                    let expanded = Config::expand_path(value);
                    if !expanded.exists() {
                        return Err(AppError::ConfigError(format!(
                            "路径不存在: {}",
                            expanded.display()
                        )));
                    }
                    if !expanded.is_dir() {
                        return Err(AppError::ConfigError(format!(
                            "路径不是目录: {}",
                            expanded.display()
                        )));
                    }
                    config.source_directory = value.clone();
                    println!("✓ 源目录已设置为: {}", value);
                }
                "readme-filename" | "readme" => {
                    config.readme_filename = value.clone();
                    println!("✓ README 文件名已设置为: {}", value);
                }
                "output-filename" | "output" => {
                    config.output_filename = value.clone();
                    println!("✓ 输出文件名已设置为: {}", value);
                }
                "obsidian-vault" | "obsidian" | "vault" => {
                    let expanded = Config::expand_path(value);
                    if !expanded.exists() {
                        return Err(AppError::ConfigError(format!(
                            "路径不存在: {}",
                            expanded.display()
                        )));
                    }
                    if !expanded.is_dir() {
                        return Err(AppError::ConfigError(format!(
                            "路径不是目录: {}",
                            expanded.display()
                        )));
                    }
                    config.obsidian_vault_path = value.clone();
                    println!("✓ Obsidian vault 路径已设置为: {}", value);
                }
                _ => {
                    return Err(AppError::ConfigError(format!(
                        "未知的配置项: {}。可用的配置项: source, readme-filename, output-filename, obsidian-vault",
                        key
                    )));
                }
            }
            
            config.save()?;
            println!("✓ 配置已保存");
        }
        
        ConfigAction::Edit => {
            let config_path = Config::config_path()?;
            
            // Ensure config file exists
            if !config_path.exists() {
                let config = Config::default();
                config.save()?;
                println!("✓ 配置文件已创建");
            }
            
            // Try to open with default editor
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
                if cfg!(target_os = "macos") {
                    "open".to_string()
                } else if cfg!(target_os = "windows") {
                    "notepad".to_string()
                } else {
                    "nano".to_string()
                }
            });
            
            println!("📝 使用 {} 打开配置文件...", editor);
            
            let status = if editor == "open" {
                Command::new(&editor)
                    .arg("-t")
                    .arg(&config_path)
                    .status()
            } else {
                Command::new(&editor)
                    .arg(&config_path)
                    .status()
            };
            
            match status {
                Ok(status) if status.success() => {
                    println!("✓ 配置文件已关闭");
                }
                Ok(_) => {
                    eprintln!("⚠️  编辑器退出时出现错误");
                }
                Err(e) => {
                    eprintln!("❌ 无法打开编辑器: {}", e);
                    println!("\n配置文件位置: {}", config_path.display());
                    println!("你可以手动编辑此文件");
                }
            }
        }
        
        ConfigAction::Path => {
            let config_path = Config::config_path()?;
            println!("{}", config_path.display());
        }
    }
    
    Ok(())
}
