mod commands;
mod template;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "omnilynceus",
    version,
    about = "OmniLynceus plugin toolkit - 用于创建和构建 OmniLynceus 插件",
    long_about = "OmniLynceus CLI 工具，帮助开发者快速创建和打包游戏插件。\n\n使用示例：\n  omnilynceus new my-plugin    # 创建新插件项目\n  omnilynceus build             # 编译并打包为 .olp 文件"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 创建新插件项目
    #[command(about = "初始化一个新的 OmniLynceus 插件项目")]
    New {
        /// 插件名称 (默认为 my-olplugin)
        #[arg(help = "项目名称，用于生成文件夹和配置")]
        name: Option<String>,
    },
    /// 编译并打包插件
    #[command(about = "将插件编译为 release 版本并打包成 .olp 文件")]
    Build {},
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::New { name } => commands::new::run(name.as_deref())?,
        Command::Build {} => commands::build::run()?
    }
    Ok(())
}