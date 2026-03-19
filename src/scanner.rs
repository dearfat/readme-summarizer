use std::fs;
use std::path::PathBuf;
use crate::error::{AppError, Result};

#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub project_name: String,
    pub readme_path: PathBuf,
    #[allow(dead_code)]
    pub project_dir: PathBuf,
}

pub struct Scanner {
    source_dir: PathBuf,
    readme_filename: String,
}

impl Scanner {
    pub fn new(source_dir: PathBuf, readme_filename: String) -> Self {
        Self {
            source_dir,
            readme_filename,
        }
    }

    pub fn scan(&self) -> Result<Vec<ProjectInfo>> {
        let mut projects = Vec::new();

        let entries = fs::read_dir(&self.source_dir)
            .map_err(|e| AppError::ScanError(format!("无法读取目录 {}: {}", self.source_dir.display(), e)))?;

        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    
                    if !path.is_dir() {
                        continue;
                    }

                    let project_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let readme_path = path.join(&self.readme_filename);

                    if readme_path.exists() && readme_path.is_file() {
                        projects.push(ProjectInfo {
                            project_name,
                            readme_path,
                            project_dir: path,
                        });
                    }
                }
                Err(e) => {
                    eprintln!("⚠️  跳过无法访问的条目: {}", e);
                }
            }
        }

        Ok(projects)
    }
}
