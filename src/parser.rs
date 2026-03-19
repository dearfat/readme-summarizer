use regex::Regex;
use std::fs;
use std::path::Path;
use crate::error::{AppError, Result};

#[derive(Debug, Clone)]
pub struct ProjectSummary {
    pub name: String,
    pub functionality: String,
    pub problems_solved: String,
}

pub struct Parser;

impl Parser {
    pub fn parse<P: AsRef<Path>>(readme_path: P, fallback_name: &str) -> Result<ProjectSummary> {
        let content = fs::read_to_string(readme_path.as_ref())
            .map_err(|e| AppError::ParseError(format!("读取文件失败: {}", e)))?;

        let name = Self::extract_name(&content).unwrap_or_else(|| fallback_name.to_string());
        let functionality = Self::extract_functionality(&content);
        let problems_solved = Self::extract_problems(&content);

        Ok(ProjectSummary {
            name,
            functionality,
            problems_solved,
        })
    }

    fn extract_name(content: &str) -> Option<String> {
        let heading_re = Regex::new(r"^#\s+(.+)$").unwrap();
        
        for line in content.lines() {
            if let Some(caps) = heading_re.captures(line) {
                return Some(caps[1].trim().to_string());
            }
        }
        None
    }

    fn extract_functionality(content: &str) -> String {
        let keywords = [
            "## Features",
            "## 功能",
            "## What",
            "## Description",
            "## 简介",
            "## 描述",
            "## Overview",
            "## About",
            "## 关于",
        ];

        if let Some(section) = Self::extract_section(content, &keywords) {
            return section;
        }

        Self::extract_first_paragraph(content)
            .unwrap_or_else(|| "未找到功能描述".to_string())
    }

    fn extract_problems(content: &str) -> String {
        let keywords = [
            "## Problem",
            "## 问题",
            "## 解决的问题",
            "## Why",
            "## Motivation",
            "## 背景",
            "## Background",
            "## 动机",
            "## Purpose",
            "## 目的",
        ];

        Self::extract_section(content, &keywords)
            .unwrap_or_else(|| "未明确说明".to_string())
    }

    fn extract_section(content: &str, keywords: &[&str]) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            for keyword in keywords {
                if line.trim().starts_with(keyword) {
                    return Some(Self::collect_section_content(&lines, i + 1));
                }
            }
        }
        None
    }

    fn collect_section_content(lines: &[&str], start_idx: usize) -> String {
        let mut content = Vec::new();
        let heading_re = Regex::new(r"^#{1,6}\s+").unwrap();

        for line in lines.iter().skip(start_idx) {
            if heading_re.is_match(line) {
                break;
            }
            
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                content.push(trimmed.to_string());
            }
        }

        if content.is_empty() {
            return "无内容".to_string();
        }

        content.join("\n")
    }

    fn extract_first_paragraph(content: &str) -> Option<String> {
        let heading_re = Regex::new(r"^#{1,6}\s+").unwrap();
        let mut paragraph = Vec::new();
        let mut found_content = false;

        for line in content.lines() {
            let trimmed = line.trim();
            
            if heading_re.is_match(trimmed) {
                if found_content {
                    break;
                }
                continue;
            }

            if trimmed.is_empty() {
                if found_content {
                    break;
                }
                continue;
            }

            paragraph.push(trimmed.to_string());
            found_content = true;
        }

        if paragraph.is_empty() {
            None
        } else {
            Some(paragraph.join(" "))
        }
    }
}
