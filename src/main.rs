use clap::{Parser, Subcommand};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    done: bool,
    priority: Option<String>,        // low, medium, high
    due_date: Option<NaiveDate>,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
    file_path: PathBuf,
}

impl TodoList {
    fn new() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| std::env::temp_dir())
            .join("rust-todo");
        fs::create_dir_all(&data_dir).ok();

        let file_path = data_dir.join("todos.json");

        let (tasks, next_id) = if file_path.exists() {
            if let Ok(content) = fs::read_to_string(&file_path) {
                if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&content) {
                    let max_id = tasks.iter().map(|t| t.id).max().unwrap_or(0);
                    (tasks, max_id + 1)
                } else {
                    (vec![], 1)
                }
            } else {
                (vec![], 1)
            }
        } else {
            (vec![], 1)
        };

        Self {
            tasks,
            next_id,
            file_path,
        }
    }

    fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.tasks) {
            let _ = fs::write(&self.file_path, json);
        }
    }

    fn add(&mut self, description: String, priority: Option<String>, due: Option<NaiveDate>) {
        let task = Task {
            id: self.next_id,
            description,
            done: false,
            priority,
            due_date: due,
        };
        self.tasks.push(task);
        self.next_id += 1;
        self.save();
        println!("✅ 任务添加成功！");
    }

    fn list(&self, filter: Option<bool>) {
        let filtered: Vec<&Task> = self.tasks
            .iter()
            .filter(|t| match filter {
                Some(done) => t.done == done,
                None => true,
            })
            .collect();

        if filtered.is_empty() {
            println!("📭 当前没有任务！");
            return;
        }

        println!("{:<4} {:<6} {:<40} {:<10} {:<12}", "ID", "状态", "任务描述", "优先级", "截止日期");
        println!("{}", "-".repeat(85));

        for task in filtered {
            let status = if task.done { "✅" } else { "⏳" };
            let pri = task.priority.as_deref().unwrap_or("-");
            let due = task.due_date.map_or_else(|| "-".to_string(), |d| d.to_string());
            println!("{:<4} {:<6} {:<40} {:<10} {:<12}", task.id, status, task.description, pri, due);
        }
    }

    fn done(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
            self.save();
            println!("✅ 任务 {} 已标记为完成！", id);
        } else {
            println!("❌ 未找到 ID 为 {} 的任务", id);
        }
    }

    fn remove(&mut self, id: u32) {
        let len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        if self.tasks.len() != len {
            self.save();
            println!("🗑️  任务 {} 已删除！", id);
        } else {
            println!("❌ 未找到 ID 为 {} 的任务", id);
        }
    }
}

#[derive(Parser)]
#[command(author, version, about = "Rust Todo CLI - 强大且易用的待办事项工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加新任务
    Add {
        #[arg(required = true)]
        description: String,

        #[arg(short, long)]
        priority: Option<String>,

        #[arg(short, long)]
        due: Option<String>,   // 格式: 2026-04-20
    },

    /// 列出任务 (默认全部)
    List {
        #[arg(short, long, default_value = "all")]
        status: String,   // all / todo / done
    },

    /// 标记任务完成
    Done { id: u32 },

    /// 删除任务
    Remove { id: u32 },
}

fn main() {
    let cli = Cli::parse();
    let mut todo = TodoList::new();

    match cli.command {
        Commands::Add { description, priority, due } => {
            let due_date = due.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
            todo.add(description, priority, due_date);
        }
        Commands::List { status } => {
            let filter = match status.as_str() {
                "todo" => Some(false),
                "done" => Some(true),
                _ => None,
            };
            todo.list(filter);
        }
        Commands::Done { id } => todo.done(id),
        Commands::Remove { id } => todo.remove(id),
    }
}