# Rust Todo CLI + TUI

一个功能完整、代码清晰的 Rust 待办事项工具，同时支持 **命令行（CLI）** 和 **终端图形界面（TUI）**。

![Rust Todo TUI](https://github.com/Tim5500/rust-todo/releases/download/v0.3.0/rust-todo-tui-screenshot.png)  
*(TUI 界面截图 - 可后续补充)*

## ✨ 功能亮点

- **CLI 命令行模式**：快速添加、查看、编辑、删除任务
- **TUI 图形界面**：支持键盘操作（上下键、空格切换完成、删除等），美观易用
- 支持任务字段：标题、开始日期、截止日期、优先级、遇到的困难、解决方法、备注
- 支持导出 / 导入 JSON 备份
- 支持 Git 自动同步（todos.json）
- 数据自动持久化保存（完全离线，本地使用）
- Windows 原生 exe，绿色免安装

## 📥 下载

**最新版本 v0.3.0**（2026-04-16）

- [rust-todo.exe](https://github.com/Tim5500/rust-todo/releases/download/v0.3.0/rust-todo.exe) （CLI 命令行版）
- [rust-todo-tui.exe](https://github.com/Tim5500/rust-todo/releases/download/v0.3.0/rust-todo-tui.exe) （TUI 图形界面版）

直接下载后双击运行即可使用，无需安装。

## 🚀 使用方法

### CLI 命令行版

```powershell
# 查看所有任务
.\rust-todo.exe list

# 添加任务（支持空格）
.\rust-todo.exe add "学习 Rust 所有权与借用" -p high -s 2026-04-16 -d 2026-04-30

# 查看待办任务
.\rust-todo.exe list -t todo

# 搜索任务
.\rust-todo.exe list -q rust

# 进入 TUI 图形界面
.\rust-todo-tui.exe

TUI 图形界面版（推荐）

运行 rust-todo-tui.exe 后：

    ↑ ↓ 或 j k：上下移动选择
    空格：切换完成 / 未完成状态
    r：删除选中任务
    s：保存并退出
    q：退出程序
    h：显示帮助

📁 项目结构

    src/main.rs → CLI 主程序
    src/main_tui.rs → TUI 图形界面
    数据文件默认保存在：%APPDATA%\rust-todo\todos.json

🛠️ 开发与构建

# 编译 CLI
cargo build --release

# 编译 TUI
cargo build --release --bin rust-todo-tui

生成的 exe 文件位于 target\x86_64-pc-windows-gnu\release\
🤝 欢迎贡献

欢迎提交 Issue、Pull Request 或反馈建议！

    Star 这个项目表示支持
    有任何功能需求（如标签系统、提醒、暗黑模式等）欢迎告诉我

Made with ❤️ using Rust