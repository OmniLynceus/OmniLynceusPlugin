use crate::template::TEMPLATE_DIR; // 引入刚才定义的静态变量

use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::{Context, Result, anyhow};

pub fn run(name: Option<&str>) -> Result<()> {
    // 1. 确定项目名称
    let project_name = name.unwrap_or("my-olplugin");
    println!("🚀 正在准备插件项目: {}...", project_name);

    // 2. 调用 cargo new 创建基础项目
    let status = Command::new("cargo")
        .arg("new")
        .arg("--bin")
        .arg(project_name)
        .status()
        .with_context(|| "执行 'cargo new' 失败，请检查是否安装了 Rust 环境")?;

    if !status.success() {
        return Err(anyhow!("'cargo new' 执行返回错误代码"));
    }

    let project_path = std::env::current_dir()?.join(project_name);

    // 3. 自动加载必要依赖
    println!("📦 正在添加 SDK 依赖...");
    let _ = Command::new("cargo")
        .current_dir(&project_path)
        .args(["add", "omnilynceus-lib"])
        .status();

    // 4. 寻找并应用模板
    apply_templates(&project_path, project_name)?;

    println!("\n✅ 插件项目创建成功！");
    println!("📍 目录: {}", project_path.display());
    
    Ok(())
}

fn traverse_dir(
    dir: &include_dir::Dir<'_>,
    project_root: &Path,
    name: &str,
) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::File(file) => {
                let file_path = file.path();
                
                // 只处理以 .tmpl 结尾的文件
                if file_path.extension().and_then(|s| s.to_str()) == Some("tmpl") {
                    // 直接用 file.path() 的相对路径，去掉 .tmpl 后缀
                    let target_file_path = project_root.join(file_path.with_extension(""));

                    // 自动创建目录
                    if let Some(parent) = target_file_path.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    // 读取内容并替换占位符
                    let content = file.contents_utf8()
                        .ok_or_else(|| anyhow!("无法解析模板文件内容: {:?}", file_path))?
                        .replace("{{name}}", name);
                    
                    fs::write(&target_file_path, content)?;
                    println!("   - 已生成: {}", target_file_path.strip_prefix(project_root).unwrap_or(file_path).display());
                }
            }
            include_dir::DirEntry::Dir(subdir) => {
                // 递归处理子目录
                traverse_dir(subdir, project_root, name)?;
            }
        }
    }
    Ok(())
}

fn apply_templates(project_root: &Path, name: &str) -> Result<()> {
    println!("🎨 正在应用嵌入式模板...");

    traverse_dir(&TEMPLATE_DIR, project_root, name)?;
    
    Ok(())
}