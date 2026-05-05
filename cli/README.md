# OmniLynceus CLI

OmniLynceus 插件开发工具包，用于快速创建、开发和打包游戏插件。

## 功能特性

- 🚀 **快速初始化** - 一键生成标准化的插件项目模板
- 📦 **自动打包** - 编译并打包为 `.olp` 格式的插件文件
- ✅ **配置验证** - 自动检测和验证 `plugin.toml` 配置
- 🎯 **资源打包** - 支持自动打包 assets 目录和二进制文件

## 安装

从 crates.io 安装（推荐）：

```bash
cargo install omnilynceus_cli
```

或者从源代码编译：

```bash
git clone https://github.com/OmniLynceus/OmniLynceusPlugin.git
cd OmniLynceusPlugin/cli
cargo install --path .
```

## 快速开始

### 1. 创建新插件项目

```bash
omnilynceus new my-awesome-plugin
cd my-awesome-plugin
```

这会生成以下结构：

```
my-awesome-plugin/
├── plugin.toml              # 插件配置文件
├── src/
│   └── main.rs             # 插件代码
├── assets/                 # 资源文件目录（可选）
└── Cargo.toml              # Rust 项目配置
```

### 2. 编辑配置文件 (plugin.toml)

```toml
name = "my-awesome-plugin"
version = "0.1.0"
target = "具体游戏名称"
platform = "windows"
```

**配置说明：**
- `name` - 插件名称（必填）
- `version` - 版本号，遵循语义版本（必填）
- `target` - 目标游戏名称（必填）
- `platform` - 目标平台，如 `windows`、`linux`（必填）

### 3. 开发插件代码

在 `src/main.rs` 中编写插件逻辑：

```rust
fn main() {
    println!("Hello from my awesome plugin!");
}
```

### 4. 打包插件

在项目根目录执行：

```bash
omnilynceus build
```

输出：

```
🔨 正在构建插件...
✓ plugin.toml 验证通过
  名称: my-awesome-plugin
  版本: 0.1.0
  目标: 具体游戏名称
  平台: windows
📦 编译中 (release 模式)...
📦 打包为 .olp 文件...
✅ 构建完成！
📍 输出位置: target/my-awesome-plugin.olp
```

生成的 `.olp` 文件就可以分发使用了！

## 命令用法

### 获取帮助

```bash
# 总体帮助
omnilynceus --help

# 子命令帮助
omnilynceus new --help
omnilynceus build --help
```

### new 命令

创建新的插件项目：

```bash
# 使用自定义名称
omnilynceus new my-plugin

# 使用默认名称 (my-olplugin)
omnilynceus new
```

### build 命令

编译并打包插件：

```bash
omnilynceus build
```

**要求：**
- 必须在插件项目根目录（包含 `plugin.toml` 和 `Cargo.toml`）
- `plugin.toml` 中的所有字段必须有效
- 项目必须能正确编译

## .olp 文件格式

`.olp` 文件是一个 ZIP 包，包含以下结构：

```
plugin.olp
├── plugin.toml          # 插件配置
├── bin/
│   └── plugin.exe       # 编译后的可执行文件
└── assets/              # 资源文件（可选）
    └── ...
```

## 常见问题

### Q: 如何添加资源文件？
A: 在项目根目录创建 `assets/` 目录，将所需文件放入。运行 `omnilynceus build` 时会自动打包。

### Q: 编译失败怎么办？
A: 检查：
1. `plugin.toml` 配置是否正确
2. Rust 代码是否有语法错误
3. 依赖是否正确安装

### Q: 能否修改生成的模板？
A: 当前版本使用内置模板。如需自定义，可手动编辑生成后的项目。

### Q: 支持哪些平台？
A: 当前支持在 `plugin.toml` 中指定任意平台名称。实际支持取决于游戏客户端。

## 开发

### 项目结构

```
cli/
├── src/
│   ├── main.rs          # CLI 入口
│   ├── commands.rs      # 命令模块导出
│   ├── template.rs      # 模板管理
│   └── commands/
│       ├── new.rs       # new 命令实现
│       └── build.rs     # build 命令实现
├── templates/           # 项目模板
│   └── default/
│       └── plugin.toml.tmpl
└── Cargo.toml
```

### 构建开发版本

```bash
cargo build
./target/debug/omnilynceus --help
```

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 贡献

欢迎提交 Issue 和 Pull Request！

## 联系方式

- GitHub: [OmniLynceus/OmniLynceusPlugin](https://github.com/OmniLynceus/OmniLynceusPlugin)
- Issues: [GitHub Issues](https://github.com/OmniLynceus/OmniLynceusPlugin/issues)
