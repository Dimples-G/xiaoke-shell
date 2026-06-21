// 小壳 Shell — 智能命令行助手
// 模块声明
mod actions;
mod display;
mod error;
mod games;
mod intent;

use actions::TodoList;
use colored::*;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::env;
use std::path::PathBuf;

fn main() {
    // 初始化行编辑器（历史记录 + Tab 补全）
    let mut rl = DefaultEditor::new().expect("无法初始化行编辑器");

    // 加载历史记录
    let history_file = get_history_path();
    if history_file.exists() {
        let _ = rl.load_history(&history_file);
    }

    // 待办事项状态
    let mut todo_list: TodoList = Vec::new();

    // 显示欢迎界面
    display::show_welcome();

    // REPL 主循环
    loop {
        let prompt = format!("{} {} ", "🦊", "小壳 >".bright_cyan().bold());

        // 读取用户输入
        let line = match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim().to_string();
                if trimmed.is_empty() {
                    continue;
                }
                // 添加到历史
                rl.add_history_entry(&trimmed).ok();
                trimmed
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C：不退出，显示提示
                println!(
                    "\n{} 按 Ctrl+C 不会退出小壳哦~ 输入「退出」来告别吧",
                    "😼".bright_yellow()
                );
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D：退出
                println!();
                display::show_goodbye();
                break;
            }
            Err(err) => {
                eprintln!("读取错误: {:?}", err);
                break;
            }
        };

        // 解析意图
        let intent = intent::parse_intent(&line);

        // 执行意图（返回 false 表示退出）
        if !actions::execute_intent(&intent, &mut todo_list) {
            break;
        }
    }

    // 保存历史记录
    if let Some(parent) = history_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = rl.save_history(&history_file);
}

/// 获取历史记录文件路径
fn get_history_path() -> PathBuf {
    let home = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".xiaoke_history")
}
