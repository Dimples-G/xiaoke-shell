use colored::*;
use rand::Rng as _;

/// 计算字符串的终端显示宽度
/// CJK 字符、全角符号、emoji 占 2 列，ASCII 占 1 列
pub fn display_width(s: &str) -> usize {
    s.chars().map(char_width).sum()
}

/// 计算单个字符的终端显示宽度
pub fn char_width(c: char) -> usize {
    if c.is_ascii() {
        return 1;
    }
    // 非 ASCII 字符（含中文、日韩文、全角符号、emoji）在等宽终端中通常占 2 列
    2
}

/// 生成一条对齐的框线内容
/// 自动计算视觉宽度并填充空格，使右边界对齐
pub fn box_line(content: &str, target_width: usize) -> String {
    let dw = display_width(content);
    let pad = if dw < target_width {
        target_width - dw
    } else {
        1
    };
    format!("║  {}{} ║", content, " ".repeat(pad))
}

/// 生成空框线
pub fn empty_box_line(target_width: usize) -> String {
    box_line("", target_width)
}

/// 生成水平分隔线
pub fn hr(left: &str, fill: &str, right: &str, target_width: usize) -> String {
    let fill_w = char_width(fill.chars().next().unwrap_or('═'));
    let n = target_width / fill_w + 2;
    format!("{}{}{}", left, fill.repeat(n), right)
}

/// 生成操作成功消息
pub fn success_msg(msg: &str) {
    println!("{} {}", "✅".bright_green(), msg.bright_white());
}

/// 生成操作失败消息
pub fn error_msg(msg: &str) {
    println!("{} {}", "❌".bright_red(), msg.red());
}

/// 生成信息提示消息
pub fn info_msg(msg: &str) {
    println!("{} {}", "💡".bright_yellow(), msg.dimmed());
}

/// 显示小壳的个性欢迎界面
pub fn show_welcome() {
    let greetings = [
        "小壳睡醒啦！今天有什么可以帮你的？☀️",
        "嗨！小壳已上线~ 想玩点什么？🎮",
        "又见面了！小壳好开心~ 💖",
        "叮咚！你的智能助手小壳已就绪 🦊",
    ];
    let greeting = greetings[rand::thread_rng().gen_range(0..greetings.len())];

    let w = 48;

    println!();
    println!("{}", hr("╔", "══", "╗", w).bright_cyan());
    println!(
        "{}",
        box_line("🦊 小壳 Shell — 你的智能命令行伙伴", w)
            .bright_cyan()
            .bold()
    );
    println!("{}", hr("╚", "══", "╝", w).bright_cyan());
    println!();
    println!("{} {}", "💬".bright_yellow(), greeting.bright_white());
    info_msg("试试对我说：");
    println!(
        "     {} {}",
        "•".dimmed(),
        "「几点了」「猜数字」「找文件 main」".dimmed()
    );
    println!(
        "     {} {}",
        "•".dimmed(),
        format!("输入「{}」查看全部功能", "帮助".bright_green()).dimmed()
    );
    println!();
}

/// 显示小壳的随机告别语
pub fn show_goodbye() {
    let exits = [
        "小壳先睡啦，拜拜~ 😴💤",
        "再见！记得想小壳哦 👋🦊",
        "关机睡觉！下次再玩~ 🌙✨",
        "拜拜！小壳会想你的 💖",
        "退出成功！你的代码今天也很棒呢~ ✨",
    ];
    let msg = exits[rand::thread_rng().gen_range(0..exits.len())];
    println!("{}", msg.bright_magenta());
}

/// 显示帮助菜单（使用精确对齐的框线）
pub fn show_help() {
    const W: usize = 52;

    println!();
    println!("{}", hr("╔", "══", "╗", W).bright_cyan());
    println!(
        "{}",
        box_line("🦊 小壳 Shell — 智能助手使用指南", W)
            .bright_cyan()
            .bold()
    );
    println!("{}", hr("╠", "══", "╣", W).bright_cyan());
    println!("{}", empty_box_line(W).bright_cyan());

    println!("{}", box_line("📂 文件操作", W).bright_white());
    println!(
        "{}",
        box_line("   找文件 <关键词>          搜索内容 <关键词>", W).bright_white()
    );
    println!(
        "{}",
        box_line("   创建文件 <名字>           创建文件夹 <名字>", W).bright_white()
    );
    println!(
        "{}",
        box_line("   删除文件 <名字>           列出文件 / 看看有什么", W).bright_white()
    );
    println!("{}", empty_box_line(W).bright_cyan());

    println!("{}", box_line("🔧 系统信息", W).bright_white());
    println!(
        "{}",
        box_line("   几点了 / 时间              今天几号 / 日期", W).bright_white()
    );
    println!(
        "{}",
        box_line("   磁盘空间 / 硬盘            内存 / 运存", W).bright_white()
    );
    println!("{}", empty_box_line(W).bright_cyan());

    println!("{}", box_line("🧮 计算器", W).bright_white());
    println!(
        "{}",
        box_line("   算 <表达式>                1 + 2 * 3", W).bright_white()
    );
    println!("{}", empty_box_line(W).bright_cyan());

    println!("{}", box_line("🎮 小游戏", W).bright_white());
    println!(
        "{}",
        box_line("   猜数字                      石头剪刀布", W).bright_white()
    );
    println!(
        "{}",
        box_line("   打字测试                     骰子 / 掷骰子", W).bright_white()
    );
    println!("{}", empty_box_line(W).bright_cyan());

    println!("{}", box_line("⏰ 定时提醒", W).bright_white());
    println!(
        "{}",
        box_line("   N分钟后提醒我 <消息>        例如：5分钟后提醒我 开会", W).bright_white()
    );
    println!("{}", empty_box_line(W).bright_cyan());

    println!("{}", box_line("📋 待办事项", W).bright_white());
    println!(
        "{}",
        box_line("   添加待办 <内容>             查看待办", W).bright_white()
    );
    println!(
        "{}",
        box_line("   完成待办 <编号>             清空待办", W).bright_white()
    );
    println!("{}", empty_box_line(W).bright_cyan());

    println!("{}", box_line("🎉 趣味", W).bright_white());
    println!(
        "{}",
        box_line("   笑话 / 讲个笑话            夸我 / 彩虹屁", W).bright_white()
    );
    println!(
        "{}",
        box_line("   ASCII / 字符画              掷硬币 / 抛硬币", W).bright_white()
    );
    println!("{}", empty_box_line(W).bright_cyan());

    println!("{}", box_line("⚙️  其他", W).bright_white());
    println!(
        "{}",
        box_line("   清屏 / 清理屏幕            帮助 / help", W).bright_white()
    );
    println!(
        "{}",
        box_line("   退出 / 拜拜                 也可以直接输入系统命令", W).bright_white()
    );
    println!("{}", empty_box_line(W).bright_cyan());

    println!("{}", box_line("🔍 彩蛋等你发现……", W).bright_yellow());
    println!("{}", hr("╚", "══", "╝", W).bright_cyan());
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
        assert_eq!(display_width(""), 0);
        assert_eq!(display_width("abc123"), 6);
    }

    #[test]
    fn test_display_width_chinese() {
        // 每个中文字符占 2 列
        assert_eq!(display_width("你好"), 4);
        assert_eq!(display_width("小壳"), 4);
    }

    #[test]
    fn test_display_width_mixed() {
        // 中英文混合
        assert_eq!(display_width("hello你好"), 9); // 5 + 4
        assert_eq!(display_width("test测试123"), 11); // 4 + 4 + 3
    }

    #[test]
    fn test_display_width_emoji() {
        assert_eq!(display_width("🦊"), 2);
        assert_eq!(display_width("🎮📂"), 4);
    }

    #[test]
    fn test_char_width() {
        assert_eq!(char_width('a'), 1);
        assert_eq!(char_width('1'), 1);
        assert_eq!(char_width(' '), 1);
        assert_eq!(char_width('中'), 2);
        assert_eq!(char_width('🦊'), 2);
        assert_eq!(char_width('╔'), 2);
    }

    #[test]
    fn test_box_line_output() {
        let line = box_line("test", 10);
        // 应该包含 ║ 边框
        assert!(line.starts_with('║'));
        assert!(line.ends_with('║'));
    }
}
