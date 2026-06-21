use crate::display;
use crate::error::{ShellError, ShellResult};
use crate::games;
use crate::intent::Intent;

use colored::*;
use rand::Rng as _;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command as SysCommand;
use std::sync::mpsc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ============ 全局状态 ============

/// 待办事项列表（在 main.rs 中维护，通过参数传入）
pub type TodoList = Vec<TodoItem>;

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub content: String,
    pub done: bool,
}

/// 执行意图，返回 false 表示退出 shell
pub fn execute_intent(intent: &Intent, todo_list: &mut TodoList) -> bool {
    match intent {
        Intent::Clear => {
            print!("\x1b[2J\x1b[H");
            io::stdout().flush().ok();
            true
        }
        Intent::Exit => {
            display::show_goodbye();
            false
        }
        Intent::Help => {
            display::show_help();
            true
        }
        Intent::ShowTime => {
            show_time();
            true
        }
        Intent::ShowDate => {
            show_date();
            true
        }
        Intent::ShowDisk => {
            show_disk();
            true
        }
        Intent::ShowMemory => {
            show_memory();
            true
        }
        Intent::FindFiles { keyword, count } => {
            let _ = find_files(keyword, *count);
            true
        }
        Intent::SearchContent { keyword } => {
            let _ = search_content(keyword);
            true
        }
        Intent::CreateDir { name } => {
            let _ = create_dir(name);
            true
        }
        Intent::CreateFile { name } => {
            let _ = create_file(name);
            true
        }
        Intent::DeleteFile { name } => {
            let _ = delete_file(name);
            true
        }
        Intent::ListFiles => {
            let _ = list_files();
            true
        }
        Intent::Calculate(expr) => {
            let _ = calculate(expr);
            true
        }
        Intent::PlayGuessNumber => {
            games::guess_number();
            true
        }
        Intent::PlayRPS => {
            games::rock_paper_scissors();
            true
        }
        Intent::PlayTyping => {
            games::typing_test();
            true
        }
        Intent::PlayDice => {
            games::dice_roll();
            true
        }
        Intent::SetTimer { minutes, message } => {
            set_timer(*minutes, message.clone());
            true
        }
        Intent::TodoAdd { content } => {
            todo_list.push(TodoItem {
                content: content.clone(),
                done: false,
            });
            display::success_msg(&format!("待办已添加：{}", content));
            true
        }
        Intent::TodoList => {
            show_todo_list(todo_list);
            true
        }
        Intent::TodoDone { index } => {
            if *index > 0 && *index <= todo_list.len() {
                todo_list[*index - 1].done = true;
                display::success_msg(&format!("已完成：{}", todo_list[*index - 1].content));
            } else {
                display::error_msg("待办编号无效");
            }
            true
        }
        Intent::TodoClear => {
            todo_list.clear();
            display::success_msg("待办列表已清空");
            true
        }
        Intent::TellJoke => {
            tell_joke();
            true
        }
        Intent::Compliment => {
            compliment();
            true
        }
        Intent::AsciiArt => {
            show_ascii_art();
            true
        }
        Intent::FlipCoin => {
            flip_coin();
            true
        }
        Intent::EasterEgg(msg) => {
            println!("{}", msg.bright_magenta().bold());
            true
        }
        Intent::SystemCommand(cmd) => {
            let _ = run_system_command(cmd);
            true
        }
        Intent::Unknown(msg) => {
            println!("{}", msg.bright_yellow());
            true
        }
    }
}

// ============ 系统信息 ============

fn show_time() {
    let now = chrono_local();
    println!(
        "{} 现在是 {}",
        "🕐".bright_yellow(),
        now.bright_cyan().bold()
    );
}

fn show_date() {
    let now = chrono_local();
    println!(
        "{} 今天是 {}",
        "📅".bright_yellow(),
        now.bright_cyan().bold()
    );
}

fn chrono_local() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => {
            let secs = dur.as_secs();
            // 北京时间 UTC+8
            let total_secs = secs + 8 * 3600;
            let days = total_secs / 86400;
            let time_secs = total_secs % 86400;
            let hours = time_secs / 3600;
            let minutes = (time_secs % 3600) / 60;
            let seconds = time_secs % 60;
            let (y, m, d) = days_to_date(days as i64);
            format!("{y}年{m:02}月{d:02}日 {hours:02}:{minutes:02}:{seconds:02}")
        }
        Err(_) => "未知时间".to_string(),
    }
}

fn days_to_date(mut days: i64) -> (i64, i64, i64) {
    let mut year = 1970i64;
    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }
    let month_days = if is_leap(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut month = 1i64;
    for &md in month_days.iter() {
        if days < md {
            break;
        }
        days -= md;
        month += 1;
    }
    (year, month, days + 1)
}

fn is_leap(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn show_disk() {
    println!("{} 正在查看磁盘空间...", "💾".bright_yellow());
    match SysCommand::new("cmd")
        .args(["/c", "wmic logicaldisk get size,freespace,caption"])
        .output()
    {
        Ok(output) => {
            let s = String::from_utf8_lossy(&output.stdout);
            println!("{}", s.trim().bright_white());
        }
        Err(e) => {
            display::error_msg(&format!("无法获取磁盘信息: {}", e));
        }
    }
}

fn show_memory() {
    println!("{} 正在查看内存信息...", "🧠".bright_yellow());

    let result: ShellResult<()> = (|| {
        let output = SysCommand::new("cmd")
            .args([
                "/c",
                "wmic OS get TotalVisibleMemorySize,FreePhysicalMemory /Value",
            ])
            .output()?;

        let s = String::from_utf8_lossy(&output.stdout);
        for line in s.lines() {
            let line = line.trim();
            if line.starts_with("TotalVisibleMemorySize=") {
                let kb: f64 = line
                    .trim_start_matches("TotalVisibleMemorySize=")
                    .parse()
                    .map_err(|e| ShellError::Parse(format!("解析总内存失败: {}", e)))?;
                println!("  总内存：{:.1} GB", kb / 1024.0 / 1024.0);
            }
            if line.starts_with("FreePhysicalMemory=") {
                let kb: f64 = line
                    .trim_start_matches("FreePhysicalMemory=")
                    .parse()
                    .map_err(|e| ShellError::Parse(format!("解析可用内存失败: {}", e)))?;
                println!("  可用内存：{:.1} GB", kb / 1024.0 / 1024.0);
            }
        }
        Ok(())
    })();

    if let Err(e) = result {
        display::error_msg(&format!("无法获取内存信息: {}", e));
    }
}

// ============ 文件操作 ============

fn find_files(keyword: &str, count: usize) -> ShellResult<()> {
    if keyword.is_empty() {
        println!("{} 请输入要搜索的关键词哦~", "🔍".bright_yellow());
        return Ok(());
    }
    println!(
        "{} 正在搜索包含「{}」的文件（最多 {} 个）...",
        "🔍".bright_yellow(),
        keyword.bright_green(),
        count
    );

    let files = search_files_recursive(".", keyword, count)?;

    if files.is_empty() {
        println!("😔 没有找到包含「{}」的文件呢", keyword);
    } else {
        for (i, f) in files.iter().enumerate() {
            let size = fs::metadata(f).map(|m| m.len()).unwrap_or(0);
            println!(
                "  {}. {} {}",
                (i + 1).to_string().bright_blue(),
                f.bright_white(),
                format_size(size).dimmed()
            );
        }
        display::success_msg(&format!("共找到 {} 个文件", files.len()));
    }
    Ok(())
}

/// 递归搜索文件（泛型参数 P: AsRef<Path> 接受多种路径类型）
fn search_files_recursive<P: AsRef<Path>>(
    dir: P,
    keyword: &str,
    max: usize,
) -> ShellResult<Vec<String>> {
    let mut results = Vec::new();
    let keyword_lower = keyword.to_lowercase();

    let entries = match fs::read_dir(dir.as_ref()) {
        Ok(e) => e,
        Err(_) => return Ok(results),
    };

    for entry in entries.flatten() {
        if results.len() >= max {
            break;
        }
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        if name.to_lowercase().contains(&keyword_lower) {
            results.push(path.display().to_string());
        }
        if path.is_dir() && results.len() < max {
            let sub = path.display().to_string();
            if let Ok(mut sub_results) = search_files_recursive(&sub, keyword, max - results.len())
            {
                results.append(&mut sub_results);
            }
        }
    }
    Ok(results)
}

/// 搜索文件内容（使用线程 + channel 实现并发）
fn search_content(keyword: &str) -> ShellResult<()> {
    if keyword.is_empty() {
        println!("{} 请输入要搜索的关键词哦~", "🔍".bright_yellow());
        return Ok(());
    }

    println!(
        "{} 正在搜索文件内容「{}」...",
        "🔍".bright_yellow(),
        keyword.bright_green()
    );

    let (tx, rx) = mpsc::channel();
    let kw = keyword.to_string();

    // 在独立线程中搜索文件内容
    let handle = std::thread::spawn(move || {
        let mut found_count = 0;
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file()
                    && let Ok(content) = fs::read_to_string(&path)
                {
                    let line_matches: Vec<usize> = content
                        .lines()
                        .enumerate()
                        .filter(|(_, line)| line.to_lowercase().contains(&kw))
                        .map(|(i, _)| i + 1)
                        .collect();

                    if !line_matches.is_empty() {
                        found_count += 1;
                        if tx.send((path.display().to_string(), line_matches)).is_err() {
                            break;
                        }
                    }
                }
            }
        }
        found_count
    });

    let mut total = 0;
    // 接收搜索结果（带超时避免无限等待）
    for (path, lines) in rx {
        total += 1;
        println!("  {} {}", "📄".bright_white(), path.bright_cyan());
        println!(
            "     行 {}",
            lines
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ")
                .dimmed()
        );
    }

    let found = handle.join().unwrap_or(0);
    if total == 0 {
        println!("😔 没有找到包含「{}」的文件", keyword);
    } else {
        display::success_msg(&format!("在 {} 个文件中找到匹配", found));
    }
    Ok(())
}

fn create_dir(name: &str) -> ShellResult<()> {
    let path = Path::new(name);
    fs::create_dir(path).map_err(|e| {
        if e.kind() == io::ErrorKind::AlreadyExists {
            println!("📁 文件夹「{}」已经存在啦~", name.bright_yellow());
            ShellError::Canceled
        } else {
            display::error_msg(&format!("创建失败: {}", e));
            ShellError::from(e)
        }
    })?;
    display::success_msg(&format!("文件夹「{}」创建成功！", name.bright_cyan()));
    Ok(())
}

fn create_file(name: &str) -> ShellResult<()> {
    fs::File::create(name)?;
    display::success_msg(&format!("文件「{}」创建成功！", name.bright_cyan()));
    Ok(())
}

fn delete_file(name: &str) -> ShellResult<()> {
    let path = Path::new(name);
    if !path.exists() {
        println!("🤔「{}」不存在哦", name.bright_yellow());
        return Ok(());
    }
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    display::success_msg(&format!("「{}」已删除", name.bright_cyan()));
    Ok(())
}

fn list_files() -> ShellResult<()> {
    let current = std::env::current_dir()?;
    println!(
        "{} 当前目录：{}",
        "📂".bright_yellow(),
        current.display().to_string().bright_cyan()
    );

    let entries = fs::read_dir(&current)?;
    let mut items: Vec<_> = entries.flatten().collect();
    items.sort_by_key(|e| e.file_name());

    let mut dir_count = 0u32;
    let mut file_count = 0u32;

    for entry in &items {
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        if is_dir {
            println!("  {} {}/", "📁".bright_blue(), name.bright_blue().bold());
            dir_count += 1;
        } else {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            println!(
                "  {} {} {}",
                "📄".dimmed(),
                name.bright_white(),
                format_size(size).dimmed()
            );
            file_count += 1;
        }
    }

    println!();
    println!(
        "{} 共 {} 个文件夹，{} 个文件",
        "📊".bright_green(),
        dir_count.to_string().bright_blue(),
        file_count.to_string().bright_white()
    );
    Ok(())
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

// ============ 计算器 ============

fn calculate(expr: &str) -> ShellResult<()> {
    let expr = expr.trim();
    if expr.is_empty() {
        println!(
            "{} 请输入要计算的表达式哦，比如「1 + 2 * 3」",
            "🧮".bright_yellow()
        );
        return Ok(());
    }
    match eval_expr(expr) {
        Ok(result) => println!(
            "{} {} = {}",
            "🧮".bright_yellow(),
            expr.bright_white(),
            result.to_string().bright_green().bold()
        ),
        Err(e) => display::error_msg(&format!("计算失败: {}", e)),
    }
    Ok(())
}

fn eval_expr(expr: &str) -> ShellResult<f64> {
    let tokens = tokenize_expr(expr)?;
    let (result, _) = parse_expr(&tokens, 0)?;
    Ok(result)
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(f64),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    LParen,
    RParen,
}

fn tokenize_expr(expr: &str) -> ShellResult<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' | '.' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n = num
                    .parse::<f64>()
                    .map_err(|_| ShellError::Eval(format!("无效数字: {num}")))?;
                tokens.push(Token::Num(n));
            }
            '+' => {
                tokens.push(Token::Add);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Sub);
                chars.next();
            }
            '*' | '×' | 'x' | 'X' => {
                tokens.push(Token::Mul);
                chars.next();
            }
            '/' | '÷' => {
                tokens.push(Token::Div);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Pow);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ' ' | '\t' => {
                chars.next();
            }
            _ => {
                return Err(ShellError::Eval(format!("不支持的字符: '{ch}'")));
            }
        }
    }
    Ok(tokens)
}

fn parse_expr(tokens: &[Token], pos: usize) -> ShellResult<(f64, usize)> {
    let (mut left, mut pos) = parse_term(tokens, pos)?;

    while pos < tokens.len() {
        match tokens[pos] {
            Token::Add => {
                let (right, new_pos) = parse_term(tokens, pos + 1)?;
                left += right;
                pos = new_pos;
            }
            Token::Sub => {
                let (right, new_pos) = parse_term(tokens, pos + 1)?;
                left -= right;
                pos = new_pos;
            }
            _ => break,
        }
    }
    Ok((left, pos))
}

fn parse_term(tokens: &[Token], pos: usize) -> ShellResult<(f64, usize)> {
    let (mut left, mut pos) = parse_factor(tokens, pos)?;

    while pos < tokens.len() {
        match tokens[pos] {
            Token::Mul => {
                let (right, new_pos) = parse_factor(tokens, pos + 1)?;
                left *= right;
                pos = new_pos;
            }
            Token::Div => {
                let (right, new_pos) = parse_factor(tokens, pos + 1)?;
                if right == 0.0 {
                    return Err(ShellError::Eval("除数不能为0".to_string()));
                }
                left /= right;
                pos = new_pos;
            }
            _ => break,
        }
    }
    Ok((left, pos))
}

fn parse_factor(tokens: &[Token], pos: usize) -> ShellResult<(f64, usize)> {
    if pos >= tokens.len() {
        return Err(ShellError::Eval("表达式不完整".to_string()));
    }

    match tokens[pos] {
        Token::Num(n) => Ok((n, pos + 1)),
        Token::Sub => {
            let (val, new_pos) = parse_factor(tokens, pos + 1)?;
            Ok((-val, new_pos))
        }
        Token::LParen => {
            let (val, new_pos) = parse_expr(tokens, pos + 1)?;
            if new_pos < tokens.len() && tokens[new_pos] == Token::RParen {
                Ok((val, new_pos + 1))
            } else {
                Err(ShellError::Eval("缺少右括号 ')'".to_string()))
            }
        }
        _ => Err(ShellError::Eval(format!("意外的符号: {:?}", tokens[pos]))),
    }
}

// ============ 系统命令 ============

fn run_system_command(raw_cmd: &str) -> ShellResult<()> {
    // 去掉 # 注释（# 前面有空格或在行首时生效）
    let cmd_str = strip_comments(raw_cmd);
    let parts: Vec<&str> = cmd_str.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    let mut cmd = SysCommand::new(parts[0]);
    if parts.len() > 1 {
        cmd.args(&parts[1..]);
    }

    match cmd.spawn() {
        Ok(mut child) => {
            let _ = child.wait();
            Ok(())
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // cmd /c 回退
            let mut fallback = SysCommand::new("cmd");
            fallback.arg("/c").arg(cmd_str);
            match fallback.spawn() {
                Ok(mut child) => {
                    let _ = child.wait();
                    Ok(())
                }
                Err(e2) => {
                    display::error_msg(&format!("命令执行失败: {}", e2));
                    Err(ShellError::from(e2))
                }
            }
        }
        Err(e) => {
            display::error_msg(&format!("命令执行失败: {}", e));
            Err(ShellError::from(e))
        }
    }
}

// ============ 定时提醒（并发） ============

fn set_timer(minutes: u64, message: String) {
    let duration = Duration::from_secs(minutes * 60);
    let msg = message.clone();

    println!(
        "{} 已设置定时器，{} 分钟后提醒「{}」",
        "⏰".bright_yellow(),
        minutes.to_string().bright_cyan(),
        message.bright_white()
    );

    // 使用独立线程等待并提醒
    std::thread::spawn(move || {
        std::thread::sleep(duration);
        // 打印醒目的提醒消息
        println!();
        println!(
            "{}",
            "╔══════════════════════════════════════════╗"
                .bright_red()
                .bold()
        );
        println!(
            "{}",
            format!(
                "║  ⏰ 定时提醒：{}",
                pad_right(
                    &msg,
                    if display::display_width(&msg) > 36 {
                        36
                    } else {
                        36 - display::display_width(&msg)
                    }
                )
            )
            .bright_red()
            .bold()
        );
        println!(
            "{}",
            "║  时间到了！该行动啦~                    ║"
                .bright_red()
                .bold()
        );
        println!(
            "{}",
            "╚══════════════════════════════════════════╝"
                .bright_red()
                .bold()
        );
        println!();
        // 重新显示提示符的视觉提示
        print!("{} ", "🦊 小壳 >".bright_cyan().bold());
        io::stdout().flush().ok();
    });
}

/// 去掉命令行中的 # 注释（# 前有空格或位于行首时）
/// 生命周期 'a 确保返回值与输入 cmd 的生命周期一致
#[allow(clippy::needless_lifetimes)]
fn strip_comments<'a>(cmd: &'a str) -> &'a str {
    let bytes = cmd.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'#' && (i == 0 || bytes[i - 1] == b' ') {
            return cmd[..i].trim_end();
        }
    }
    cmd
}

fn pad_right(s: &str, target_width: usize) -> String {
    let dw = display::display_width(s);
    if dw >= target_width {
        s.to_string()
    } else {
        format!("{s}{}", " ".repeat(target_width - dw))
    }
}

// ============ 待办事项 ============

fn show_todo_list(todo_list: &TodoList) {
    if todo_list.is_empty() {
        println!(
            "{} 待办列表是空的~ 用「添加待办 <内容>」来添加吧！",
            "📋".bright_yellow()
        );
        return;
    }

    println!("{} 待办列表：", "📋".bright_yellow());
    for (i, item) in todo_list.iter().enumerate() {
        let status = if item.done {
            "✓".bright_green()
        } else {
            "○".dimmed()
        };
        let content = if item.done {
            item.content.strikethrough().dimmed()
        } else {
            item.content.bright_white()
        };
        println!(
            "  {} {} {}",
            (i + 1).to_string().bright_blue(),
            status,
            content
        );
    }

    let done_count = todo_list.iter().filter(|t| t.done).count();
    let total = todo_list.len();
    println!();
    println!(
        "{} 进度：{}/{} 已完成",
        "📊".bright_green(),
        done_count.to_string().bright_green(),
        total.to_string().bright_white()
    );
}

// ============ 趣味功能 ============

fn tell_joke() {
    let jokes = [
        "程序员最讨厌哪种程序员？\n  …别人的代码。",
        "为什么程序员总是搞混圣诞节和万圣节？\n  因为 Oct 31 == Dec 25！",
        "一个 SQL 语句走进酒吧，看到两张桌子，\n  它问：「我可以 JOIN 你们吗？」",
        "程序员的老婆说：「你去超市买一袋牛奶，\n  如果看到鸡蛋，买六个。」\n  程序员回来后带了六袋牛奶。\n  老婆：「为什么？！」\n  程序员：「因为我看到了鸡蛋。」",
        "bug 和 feature 的区别是什么？\n  文档。",
        "「这段代码看不懂，谁来解释一下？」\n  「这是我三年前写的，我也看不懂。」",
        "键盘上最悲伤的键是什么？\n  Ctrl+Z……因为无论怎么撤销，有些事都回不去了。",
        "如果有人问你会几种语言，\n  记得把 HTML 也算上——\n  反正他们也不知道那是不是编程语言。",
        "程序员的四大谎言：\n  1. 这个bug很简单\n  2. 明天一定做完\n  3. 我写的代码绝对没问题\n  4. 这个不需要文档",
    ];
    let joke = jokes[rand::thread_rng().gen_range(0..jokes.len())];
    println!("\n{} {}\n", "😂".bright_yellow(), joke.bright_white());
}

fn compliment() {
    let compliments = [
        "你的代码一定写得特别优雅，小壳能感觉到！✨",
        "今天也是元气满满的一天，你超棒的！💪",
        "遇到你是小壳最开心的事~ 🌟",
        "你这么聪明，一定什么bug都能解决！🐛✨",
        "你就是传说中的 10x 程序员吧？太强了！🚀",
        "世界上最好的debug方式是什么？\n  就是让你来写代码！😎",
        "你敲键盘的样子一定很帅~ ⌨️💖",
        "代码如诗，你就是诗人！📝✨",
    ];
    let c = compliments[rand::thread_rng().gen_range(0..compliments.len())];
    println!("\n{} {}\n", "💖".bright_magenta(), c.bright_yellow());
}

fn show_ascii_art() {
    let arts = [
        r#"
  /\_/\
 ( o.o )
  > ^ <
  小壳来啦！
"#,
        r#"
    *
   ***
  *****
 *******
  *****
   ***
    *
  星星送你！
"#,
        r#"
  ╔═══╗
  ║╔═╗║
  ║╚═╝║
  ║╔═╗║
  ║╚═╝║
  ╚═══╝
 保持可爱！
"#,
        r#"
   __
  / _)
 ."^"|
 |   |
 J L J
 小壳永远爱你~
"#,
    ];
    let art = arts[rand::thread_rng().gen_range(0..arts.len())];
    println!("{}", art.bright_cyan());
}

fn flip_coin() {
    let result = if rand::thread_rng().gen_bool(0.5) {
        "🪙 正面！"
    } else {
        "🪙 反面！"
    };
    println!("{} {}", "🎲".bright_yellow(), result.bright_white().bold());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(1023), "1023 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
    }

    #[test]
    fn test_is_leap() {
        assert!(is_leap(2024));
        assert!(!is_leap(2023));
        assert!(is_leap(2000));
        assert!(!is_leap(1900));
    }

    #[test]
    fn test_eval_simple() {
        assert!((eval_expr("1+2").unwrap() - 3.0).abs() < 0.001);
        assert!((eval_expr("2*3").unwrap() - 6.0).abs() < 0.001);
        assert!((eval_expr("10/2").unwrap() - 5.0).abs() < 0.001);
        assert!((eval_expr("10-3").unwrap() - 7.0).abs() < 0.001);
    }

    #[test]
    fn test_eval_precedence() {
        assert!((eval_expr("1+2*3").unwrap() - 7.0).abs() < 0.001);
        assert!((eval_expr("(1+2)*3").unwrap() - 9.0).abs() < 0.001);
        assert!((eval_expr("10-2*3").unwrap() - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_eval_negative() {
        assert!((eval_expr("-5+3").unwrap() - (-2.0)).abs() < 0.001);
        assert!((eval_expr("2*-3").unwrap() - (-6.0)).abs() < 0.001);
    }

    #[test]
    fn test_eval_division_by_zero() {
        assert!(eval_expr("1/0").is_err());
    }

    #[test]
    fn test_tokenize() {
        let tokens = tokenize_expr("1+2*3").unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Num(1.0));
        assert_eq!(tokens[1], Token::Add);
        assert_eq!(tokens[2], Token::Num(2.0));
        assert_eq!(tokens[3], Token::Mul);
        assert_eq!(tokens[4], Token::Num(3.0));
    }

    #[test]
    fn test_tokenize_parens() {
        let tokens = tokenize_expr("(1+2)*3").unwrap();
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0], Token::LParen);
        assert_eq!(tokens[6], Token::Num(3.0));
    }

    #[test]
    fn test_tokenize_error() {
        assert!(tokenize_expr("1+2&3").is_err());
    }

    #[test]
    fn test_todo_operations() {
        let mut list: TodoList = Vec::new();
        list.push(TodoItem {
            content: "test".to_string(),
            done: false,
        });
        assert_eq!(list.len(), 1);
        list[0].done = true;
        assert!(list[0].done);
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_strip_comments() {
        assert_eq!(strip_comments("cargo test # 43"), "cargo test");
        assert_eq!(strip_comments("cargo fmt    # 格式检查"), "cargo fmt");
        assert_eq!(strip_comments("echo hello#world"), "echo hello#world"); // # 前无空格不过滤
        assert_eq!(strip_comments("# 这是注释"), "");
        assert_eq!(strip_comments("ls -la"), "ls -la");
    }

    #[test]
    fn test_search_files_recursive() {
        // 搜索当前目录中包含 "main" 的文件
        let results = search_files_recursive(".", "main", 10).unwrap();
        assert!(!results.is_empty(), "应该找到包含 main 的文件");
        assert!(results.iter().any(|f| f.contains("main")));
    }
}
