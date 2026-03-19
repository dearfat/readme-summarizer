use chrono::Local;
use std::fs;
use std::path::PathBuf;
use crate::error::{AppError, Result};
use crate::parser::ProjectSummary;

pub struct ProjectEntry {
    pub summary: ProjectSummary,
    pub source_path: PathBuf,
}

pub struct Generator {
    output_path: PathBuf,
}

impl Generator {
    pub fn new(output_path: PathBuf) -> Self {
        Self { output_path }
    }

    pub fn generate_table(&self, entries: &[ProjectEntry]) -> Result<()> {
        let content = self.format_table(entries);

        if let Some(parent) = self.output_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| AppError::GenerateError(format!("创建输出目录失败: {}", e)))?;
            }
        }

        fs::write(&self.output_path, content)
            .map_err(|e| AppError::GenerateError(format!("写入文件失败: {}", e)))?;

        Ok(())
    }

    fn format_table(&self, entries: &[ProjectEntry]) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        
        let mut content = String::new();
        content.push_str("# README 项目汇总\n\n");
        content.push_str(&format!("生成时间: {}\n\n", timestamp));
        content.push_str("| 项目名称 | 项目目录 | 项目功能 | 解决的问题 | 源文件路径 |\n");
        content.push_str("|---------|---------|---------|-----------|----------|\n");

        for entry in entries {
            let name = Self::escape_table_cell(&entry.summary.name);
            let project_dir = if let Some(parent) = entry.source_path.parent() {
                let abs_path = parent.canonicalize().unwrap_or_else(|_| parent.to_path_buf());
                format!("[📁]({}/)", abs_path.display())
            } else {
                "N/A".to_string()
            };
            let functionality = Self::format_multiline(&entry.summary.functionality);
            let problems = Self::format_multiline(&entry.summary.problems_solved);
            let source = Self::escape_table_cell(&entry.source_path.to_string_lossy());

            content.push_str(&format!("| {} | {} | {} | {} | {} |\n", name, project_dir, functionality, problems, source));
        }

        content.push_str("\n---\n");
        content.push_str("*此摘要由 README Summarizer 自动生成*\n");

        content
    }

    fn format_multiline(text: &str) -> String {
        let lines: Vec<&str> = text
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        if lines.is_empty() {
            return "无内容".to_string();
        }

        Self::escape_table_cell(&lines.join("<br>"))
    }

    fn escape_table_cell(text: &str) -> String {
        text.replace('|', "\\|").replace('\n', " ")
    }
}
