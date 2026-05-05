use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::{Result, anyhow, Context};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct PluginInfo {
    name: String,
    version: String,
    target: String,
    platform: String,
}

pub fn run() -> Result<()> {
    println!("🔨 正在构建插件...");

    // 1. 读取 plugin.toml
    let manifest_path = Path::new("plugin.toml");
    if !manifest_path.exists() {
        return Err(anyhow!(
            "未找到 plugin.toml，请确保在插件项目根目录下执行此命令"
        ));
    }

    let manifest_content = fs::read_to_string(manifest_path)?;
    let plugin_info: PluginInfo = toml::from_str(&manifest_content)
        .context("解析 plugin.toml 失败")?;

    // 验证内容
    validate_manifest(&plugin_info)?;

    let plugin_name = &plugin_info.name;

    // 2. 执行 cargo build --release
    println!("📦 编译中 (release 模式)...");
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .status()
        .context("执行 'cargo build --release' 失败")?;

    if !status.success() {
        return Err(anyhow!("编译失败"));
    }

    // 3. 找到可执行文件
    let exe_name = if cfg!(windows) {
        format!("{}.exe", plugin_name)
    } else {
        plugin_name.to_string()
    };

    let exe_path = Path::new("target/release").join(&exe_name);
    if !exe_path.exists() {
        return Err(anyhow!(
            "未找到编译后的可执行文件: {}",
            exe_path.display()
        ));
    }

    // 4. 创建 .olp (ZIP) 文件
    let olp_path = Path::new("target").join(format!("{}.olp", plugin_name));
    println!("📦 打包为 .olp 文件...");

    package(&olp_path, manifest_path, &exe_path)?;

    println!("✅ 构建完成！");
    println!("📍 输出位置: {}", olp_path.display());

    Ok(())
}

fn package(
    olp_path: &Path,
    manifest_path: &Path,
    exe_path: &Path,
) -> Result<()> {
    use zip::ZipWriter;
    use std::io::Write;

    let file = fs::File::create(olp_path)?;
    let mut zip = ZipWriter::new(file);
    let options: zip::write::FileOptions<()> = zip::write::FileOptions::default();

    // 添加 plugin.toml 到根目录
    zip.start_file("plugin.toml", options.clone())?;
    let content = fs::read(manifest_path)?;
    zip.write_all(&content)?;

    // 添加可执行文件到 bin/ 目录
    let exe_name = exe_path.file_name().unwrap().to_string_lossy();
    let bin_path = format!("bin/{}", exe_name);
    zip.start_file(bin_path.as_str(), options.clone())?;
    let exe_content = fs::read(exe_path)?;
    zip.write_all(&exe_content)?;

    // 添加 assets/ 目录中的文件（如果存在）
    let assets_dir = Path::new("assets");
    if assets_dir.exists() && assets_dir.is_dir() {
        for entry in walkdir::WalkDir::new(assets_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                let relative_path = path.strip_prefix(".")?;
                let path_str = relative_path.to_string_lossy();
                zip.start_file(path_str.as_ref(), options.clone())?;
                let content = fs::read(path)?;
                zip.write_all(&content)?;
            }
        }
    }

    zip.finish()?;
    Ok(())
}

// 更新验证函数签名
fn validate_manifest(plugin: &PluginInfo) -> Result<()> {
    if plugin.name.is_empty() {
        return Err(anyhow!("plugin.toml: name 不能为空"));
    }
    if plugin.version.is_empty() {
        return Err(anyhow!("plugin.toml: version 不能为空"));
    }
    if plugin.target.is_empty() {
        return Err(anyhow!("plugin.toml: target 不能为空"));
    }
    if plugin.platform.is_empty() {
        return Err(anyhow!("plugin.toml: platform 不能为空"));
    }
    
    println!("✓ plugin.toml 验证通过");
    println!("  名称: {}", plugin.name);
    println!("  版本: {}", plugin.version);
    println!("  目标: {}", plugin.target);
    println!("  平台: {}", plugin.platform);
    
    Ok(())
}