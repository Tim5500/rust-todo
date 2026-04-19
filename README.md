# Rust Todo CLI + TUI

A full-featured, clean-coded Rust to-do tool that supports both command line (CLI) and terminal graphical interface (TUI).**。

![Rust Todo TUI](https://github.com/Tim5500/rust-todo/releases/download/v0.3.0/rust-todo-tui-screenshot.png)  
*(TUI interface screenshot - can be added later)*

## ✨ Feature Highlights

- **CLI command line mode**: quickly add, view, edit, and delete tasks
- **TUI graphical interface**: supports keyboard operations (up and down keys, space switching completion, deletion, etc.), beautiful and easy to use
- Support task fields: title, start date, deadline, priority, difficulties encountered, solutions, notes
- Support export/import JSON backup
- Support Git automatic synchronization (todos.json)
- Automatic data persistence (completely offline, local use)
- Windows native exe, green and installation-free

## 📥 Download

**Latest version v0.3.0** (2026-04-16)

- [rust-todo.exe](https://github.com/Tim5500/rust-todo/releases/download/v0.3.0/rust-todo.exe) (CLI command line version)
- [rust-todo-tui.exe](https://github.com/Tim5500/rust-todo/releases/download/v0.3.0/rust-todo-tui.exe) (TUI graphical interface version)

After downloading directly, double-click to run it and you can use it without installation.

### CLI command line version

```powershell
# View all tasks
.\rust-todo.exe list

#Add task (supports spaces)
.\rust-todo.exe add "Learn Rust Ownership and Borrowing" -p high -s 2026-04-16 -d 2026-04-30

# View to-do tasks
.\rust-todo.exe list -t todo

# Search tasks
.\rust-todo.exe list -q rust

# Enter the TUI graphical interface
.\rust-todo-tui.exe

TUI graphical interface version (recommended)

After running rust-todo-tui.exe:

    ↑ ↓ or j k: Move selection up and down
    Space: switch completed/incomplete status
    r: Delete the selected task
    s: save and exit
    q: Exit the program
    h: show help

📁 Project structure

    src/main.rs → CLI main program
    src/main_tui.rs → TUI graphical interface
    The data file is saved in: %APPDATA%\rust-todo\todos.json by default

🛠️Develop and build

# Compile CLI
cargo build --release

# Compile TUI
cargo build --release --bin rust-todo-tui

The generated exe file is located at target\x86_64-pc-windows-gnu\release\
🤝 Contributions welcome

Welcome to submit Issues, Pull Requests or feedback suggestions!

    Star This project expresses support
    If you have any functional requirements (such as label system, reminder, dark mode, etc.), please let me know

Made with ❤️ using Rust
