use rand::Rng as _;

/// 用户意图分类
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    // —— 文件操作 ——
    FindFiles { keyword: String, count: usize },
    SearchContent { keyword: String },
    CreateDir { name: String },
    CreateFile { name: String },
    ListFiles,
    DeleteFile { name: String },
    // —— 系统信息 ——
    ShowTime,
    ShowDate,
    ShowDisk,
    ShowMemory,
    // —— 计算器 ——
    Calculate(String),
    // —— 游戏 ——
    PlayGuessNumber,
    PlayRPS,
    PlayTyping,
    PlayDice,
    // —— 定时提醒 ——
    SetTimer { minutes: u64, message: String },
    // —— 待办事项 ——
    TodoAdd { content: String },
    TodoList,
    TodoDone { index: usize },
    TodoClear,
    // —— Shell 控制 ——
    Clear,
    Exit,
    Help,
    // —— 趣味 ——
    TellJoke,
    Compliment,
    AsciiArt,
    FlipCoin,
    // —— 彩蛋 ——
    EasterEgg(String),
    // —— 兜底 ——
    SystemCommand(String),
    // —— 未知 ——
    Unknown(String),
}

/// 意图匹配器 trait —— 可扩展的意图识别框架
/// 通过实现此 trait，可以向小壳添加新的意图识别器
pub trait Matcher {
    /// 检查输入是否匹配此意图
    fn matches(&self, input: &str) -> bool;
    /// 匹配优先级，数值越大越优先（默认 0）
    fn priority(&self) -> i32 {
        0
    }
    /// 返回对应的意图（如果实现者不产生意图则返回 None）
    fn to_intent(&self) -> Option<Intent> {
        None
    }
}

/// 基于关键词的意图匹配器
/// 当输入包含任意指定关键词时即匹配
pub struct KeywordMatcher {
    pub keywords: Vec<String>,
    pub priority_val: i32,
    pub intent: Intent,
}

impl Matcher for KeywordMatcher {
    fn matches(&self, input: &str) -> bool {
        let lower = input.to_lowercase();
        self.keywords
            .iter()
            .any(|k| lower.contains(&k.to_lowercase()))
    }

    fn priority(&self) -> i32 {
        self.priority_val
    }

    fn to_intent(&self) -> Option<Intent> {
        Some(self.intent.clone())
    }
}

/// 使用泛型 + trait bound 查找优先级最高的匹配项
/// 生命周期 'a 确保返回引用与输入 lifetimes 一致
pub fn best_match<'a, M: Matcher>(matchers: &'a [M], input: &str) -> Option<&'a M> {
    matchers
        .iter()
        .filter(|m| m.matches(input))
        .max_by_key(|m| m.priority())
}

/// 使用 trait object 数组查找优先级最高的匹配项，返回索引
pub fn find_best_matcher(matchers: &[Box<dyn Matcher>], input: &str) -> Option<usize> {
    matchers
        .iter()
        .enumerate()
        .filter(|(_, m)| m.matches(input))
        .max_by_key(|(_, m)| m.priority())
        .map(|(i, _)| i)
}

/// 解析用户输入，返回意图
pub fn parse_intent(input: &str) -> Intent {
    let input = input.trim();
    let lower = input.to_lowercase();

    // === 彩蛋检测（优先） ===
    if let Some(egg) = detect_easter_egg(input) {
        return egg;
    }

    // === 定时提醒 ===
    if let Some(intent) = parse_timer(input) {
        return intent;
    }

    // === 待办事项 ===
    if let Some(intent) = parse_todo(input) {
        return intent;
    }

    // === 游戏（使用泛型 + trait 匹配架构） ===
    let game_matchers = [
        KeywordMatcher {
            keywords: vec!["猜数字".to_string(), "猜数".to_string()],
            priority_val: 0,
            intent: Intent::PlayGuessNumber,
        },
        KeywordMatcher {
            keywords: vec!["石头剪刀布".to_string(), "猜拳".to_string()],
            priority_val: 0,
            intent: Intent::PlayRPS,
        },
        KeywordMatcher {
            keywords: vec![
                "打字".to_string(),
                "打字测试".to_string(),
                "打字练习".to_string(),
                "测手速".to_string(),
            ],
            priority_val: 0,
            intent: Intent::PlayTyping,
        },
        KeywordMatcher {
            keywords: vec![
                "骰子".to_string(),
                "掷骰子".to_string(),
                "掷骰".to_string(),
                "roll".to_string(),
                "dice".to_string(),
            ],
            priority_val: 0,
            intent: Intent::PlayDice,
        },
    ];
    if let Some(m) = best_match(&game_matchers, &lower) {
        return m.intent.clone();
    }
    if contains_any(&lower, &["玩游戏", "来玩", "玩个游戏", "有什么游戏"]) {
        let game_list = ["猜数字", "石头剪刀布", "打字测试", "掷骰子"];
        let pick = game_list[rand::thread_rng().gen_range(0..game_list.len())];
        return Intent::Unknown(format!(
            "小壳这里有这些游戏：🎮 猜数字、✊ 石头剪刀布、⌨️ 打字测试、🎲 掷骰子\n想玩哪个？跟我说「{}」就行~",
            pick
        ));
    }

    // === Shell 控制（使用 trait object 匹配器） ===
    let shell_matchers: Vec<Box<dyn Matcher>> = vec![
        Box::new(KeywordMatcher {
            keywords: vec![
                "退出".to_string(),
                "拜拜".to_string(),
                "再见".to_string(),
                "exit".to_string(),
                "quit".to_string(),
                "离开".to_string(),
                "关闭".to_string(),
                "关机".to_string(),
            ],
            priority_val: 100,
            intent: Intent::Exit,
        }),
        Box::new(KeywordMatcher {
            keywords: vec![
                "清屏".to_string(),
                "清理屏幕".to_string(),
                "clear".to_string(),
                "干净点".to_string(),
            ],
            priority_val: 0,
            intent: Intent::Clear,
        }),
        Box::new(KeywordMatcher {
            keywords: vec![
                "帮助".to_string(),
                "help".to_string(),
                "你会什么".to_string(),
                "你能干嘛".to_string(),
                "功能".to_string(),
                "怎么用".to_string(),
            ],
            priority_val: 0,
            intent: Intent::Help,
        }),
    ];
    if let Some(idx) = find_best_matcher(&shell_matchers, &lower)
        && let Some(intent) = shell_matchers[idx].to_intent()
    {
        return intent;
    }

    // === 文件操作 ===
    // 搜索文件内容
    if contains_any(
        &lower,
        &["搜索内容", "搜索文件内容", "查找内容", "grep", "文件中找"],
    ) || (lower.contains("搜索") && lower.contains("内容"))
        || (lower.contains("找") && lower.contains("包含") && lower.contains("文件"))
    {
        if let Some(kw) =
            extract_after(input, &["搜索内容 ", "搜索文件内容 ", "查找内容 ", "grep "])
        {
            return Intent::SearchContent { keyword: kw };
        }
        return Intent::Unknown("要搜索什么内容呢？比如「搜索内容 TODO」".to_string());
    }

    // 查找文件
    if contains_any(
        &lower,
        &["找文件", "搜索文件", "查找文件", "帮我找", "找一下"],
    ) || (lower.contains("最近") && lower.contains("修改"))
        || (lower.contains("找") && lower.contains("文件"))
    {
        let keyword = extract_search_keyword(input);
        let count = extract_count(input).unwrap_or(5);
        return Intent::FindFiles { keyword, count };
    }

    // 创建文件夹
    if contains_any(
        &lower,
        &[
            "创建文件夹",
            "新建文件夹",
            "建文件夹",
            "mkdir",
            "建个文件夹",
        ],
    ) || (lower.contains("创建") && lower.contains("文件夹"))
        || (lower.contains("新建") && lower.contains("文件夹"))
    {
        if let Some(name) = extract_filename(
            input,
            &["创建文件夹", "新建文件夹", "建文件夹", "建个文件夹"],
        ) {
            return Intent::CreateDir { name };
        }
        return Intent::Unknown("要创建的文件夹叫什么名字呢？比如「创建文件夹 test」".to_string());
    }

    // 创建文件
    if contains_any(
        &lower,
        &["创建文件", "新建文件", "建文件", "touch", "建个文件"],
    ) || (lower.contains("创建") && lower.contains("文件"))
        || (lower.contains("新建") && lower.contains("文件"))
    {
        if let Some(name) = extract_filename(input, &["创建文件", "新建文件", "建文件", "建个文件"])
        {
            return Intent::CreateFile { name };
        }
        return Intent::Unknown(
            "要创建的文件叫什么名字呢？比如「创建文件 readme.txt」".to_string(),
        );
    }

    // 删除文件
    if contains_any(&lower, &["删除文件", "删文件", "rm", "del", "删除"])
        || lower.starts_with("删掉")
    {
        if let Some(name) = extract_filename(input, &["删除文件", "删文件", "删掉", "删除"])
        {
            return Intent::DeleteFile { name };
        }
        return Intent::Unknown("要删除哪个文件呢？比如「删除文件 test.txt」".to_string());
    }

    // 列出文件
    if contains_any(
        &lower,
        &[
            "列出文件",
            "看看文件",
            "ls",
            "dir",
            "显示文件",
            "文件列表",
            "有什么文件",
            "有哪些文件",
            "看看有什么",
            "看看目录",
        ],
    ) {
        return Intent::ListFiles;
    }

    // === 系统信息 ===
    if contains_any(
        &lower,
        &["几点", "时间", "现在时间", "当前时间", "什么时候"],
    ) && !lower.contains("日期")
    {
        return Intent::ShowTime;
    }
    if contains_any(&lower, &["日期", "几号", "今天几号", "今天日期", "日历"]) {
        return Intent::ShowDate;
    }
    if contains_any(
        &lower,
        &["磁盘", "硬盘", "空间", "容量", "还剩多少", "存储"],
    ) {
        return Intent::ShowDisk;
    }
    if contains_any(&lower, &["内存", "ram", "运存"]) {
        return Intent::ShowMemory;
    }

    // === 计算器 ===
    if contains_any(&lower, &["算", "计算", "帮我算", "等于多少"]) || looks_like_math(&lower)
    {
        let expr = extract_math_expr(input);
        return Intent::Calculate(expr);
    }

    // === 趣味 ===
    if contains_any(
        &lower,
        &["笑话", "讲个笑话", "来段笑话", "joke", "幽默一下", "段子"],
    ) {
        return Intent::TellJoke;
    }
    if contains_any(
        &lower,
        &["夸我", "彩虹屁", "夸夸", "表扬我", "赞美", "鼓励我", "求夸"],
    ) {
        return Intent::Compliment;
    }
    if contains_any(&lower, &["ascii", "字符画", "画个", "打印图案"]) {
        return Intent::AsciiArt;
    }
    if contains_any(&lower, &["硬币", "抛硬币", "掷硬币", "flip"]) {
        return Intent::FlipCoin;
    }

    // === 兜底：系统命令 ===
    if looks_like_system_command(input) {
        return Intent::SystemCommand(input.to_string());
    }

    // === 完全不知道 ===
    let preview = if input.len() > 30 {
        format!("{}...", &input[..30])
    } else {
        input.to_string()
    };
    Intent::Unknown(format!(
        "小壳不太明白「{}」的意思呢……\n你可以试试：\n  • 问我「几点了」\n  • 让我「找文件 xxx」\n  • 说「猜数字」来玩游戏\n  • 说「5分钟后提醒我 开会」\n  • 输入「帮助」看全部功能",
        preview
    ))
}

// ============ 辅助解析函数 ============

/// 从字符串中提取两个标记之间的子串
/// 生命周期 'a 确保返回值借用自输入
fn extract_between<'a>(input: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let start_pos = input.find(start)?;
    let after_start = &input[start_pos + start.len()..];
    let end_pos = after_start.find(end)?;
    Some(&after_start[..end_pos])
}

fn contains_any(input: &str, keywords: &[&str]) -> bool {
    keywords.iter().any(|k| input.contains(k))
}

fn extract_after(input: &str, prefixes: &[&str]) -> Option<String> {
    for prefix in prefixes {
        if let Some(pos) = input.find(prefix) {
            let after = input[pos + prefix.len()..].trim();
            if !after.is_empty() {
                return Some(after.to_string());
            }
        }
    }
    None
}

fn extract_filename(input: &str, prefixes: &[&str]) -> Option<String> {
    if let Some(name) = extract_after(input, prefixes) {
        return Some(name.split_whitespace().next()?.to_string());
    }
    input.split_whitespace().last().map(|s| s.to_string())
}

fn extract_search_keyword(input: &str) -> String {
    // 支持双引号精确匹配（使用生命周期注解的 extract_between）
    if let Some(quoted) = extract_between(input, "\"", "\"") {
        return quoted.to_string();
    }
    let patterns = [
        "找文件 ",
        "搜索文件 ",
        "查找文件 ",
        "帮我找 ",
        "找一下 ",
        "找 ",
    ];
    for p in patterns {
        if let Some(pos) = input.find(p) {
            let after = input[pos + p.len()..].trim();
            let kw: String = after
                .split_whitespace()
                .take_while(|w| !w.starts_with(|c: char| c.is_ascii_digit()))
                .collect::<Vec<_>>()
                .join(" ");
            if !kw.is_empty() {
                return kw;
            }
        }
    }
    String::new()
}

fn extract_count(input: &str) -> Option<usize> {
    // 先尝试提取阿拉伯数字
    for word in input.split_whitespace() {
        let digits: String = word.chars().filter(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() {
            return digits.parse().ok();
        }
    }
    // 中文数字
    let cn_nums = [
        ("一", 1),
        ("两", 2),
        ("三", 3),
        ("四", 4),
        ("五", 5),
        ("六", 6),
        ("七", 7),
        ("八", 8),
        ("九", 9),
        ("十", 10),
    ];
    for (cn, n) in cn_nums {
        if input.contains(cn) {
            return Some(n);
        }
    }
    None
}

fn extract_math_expr(input: &str) -> String {
    let prefixes = ["算 ", "计算 ", "帮我算 ", "等于多少 "];
    for p in prefixes {
        if let Some(pos) = input.find(p) {
            return input[pos + p.len()..].trim().to_string();
        }
    }
    input
        .trim_start_matches(|c: char| !c.is_ascii_digit() && c != '(' && c != '-')
        .to_string()
}

fn looks_like_math(input: &str) -> bool {
    let s = input.trim();
    let math_chars: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
    if math_chars.is_empty() {
        return false;
    }
    let math_ratio = math_chars
        .iter()
        .filter(|c| c.is_ascii_digit() || "+-*/()^%.√".contains(**c))
        .count() as f64
        / math_chars.len() as f64;
    math_ratio > 0.5 && math_chars.iter().any(|c| c.is_ascii_digit())
}

fn looks_like_system_command(input: &str) -> bool {
    let s = input.trim();
    let first_word = s.split_whitespace().next().unwrap_or("");
    first_word
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.')
        && first_word.len() <= 20
        && !first_word.is_empty()
        && first_word.chars().any(|c| c.is_ascii_alphabetic())
}

// ============ 定时提醒解析 ============

fn parse_timer(input: &str) -> Option<Intent> {
    let lower = input.to_lowercase();

    if !(contains_any(
        &lower,
        &["分钟后提醒", "分钟后叫我", "分钟后通知", "分钟后告诉"],
    ) || (lower.contains("分钟") && lower.contains("提醒"))
        || (lower.contains("定个") && lower.contains("闹钟")))
    {
        return None;
    }

    // 提取分钟数
    let minutes = extract_minutes(input)?;
    if minutes == 0 || minutes > 1440 {
        return Some(Intent::Unknown(
            "提醒时间需要在 1 到 1440 分钟（24小时）之间哦~".to_string(),
        ));
    }

    // 提取提醒内容
    let message = extract_timer_message(input).unwrap_or_else(|| "时间到！".to_string());

    Some(Intent::SetTimer { minutes, message })
}

fn extract_minutes(input: &str) -> Option<u64> {
    // 找 "N分钟" 模式
    for word in input.split_whitespace() {
        if word.contains("分钟") {
            let digits: String = word.chars().filter(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = digits.parse() {
                return Some(n);
            }
        }
    }
    // 中文数字
    let cn_map = [
        ("一", 1),
        ("两", 2),
        ("三", 3),
        ("四", 4),
        ("五", 5),
        ("六", 6),
        ("七", 7),
        ("八", 8),
        ("九", 9),
        ("十", 10),
        ("十五", 15),
        ("二十", 20),
        ("三十", 30),
        ("半", 30),
    ];
    for (cn, n) in cn_map {
        if input.contains(&format!("{cn}分钟")) || input.contains(&format!("{cn}小时")) {
            return Some(n);
        }
    }
    // 尝试从数字中提取
    for word in input.split_whitespace() {
        let digits: String = word.chars().filter(|c| c.is_ascii_digit()).collect();
        if let Ok(n) = digits.parse() {
            return Some(n);
        }
    }
    None
}

fn extract_timer_message(input: &str) -> Option<String> {
    let patterns = ["提醒我 ", "叫我 ", "通知我 ", "告诉我 ", "提醒 "];
    for p in patterns {
        if let Some(pos) = input.find(p) {
            let msg = input[pos + p.len()..].trim();
            if !msg.is_empty() {
                return Some(msg.to_string());
            }
        }
    }
    None
}

// ============ 待办事项解析 ============

fn parse_todo(input: &str) -> Option<Intent> {
    let lower = input.to_lowercase();

    if contains_any(
        &lower,
        &["添加待办", "新增待办", "加待办", "add todo", "待办添加"],
    ) || (lower.contains("添加") && lower.contains("待办"))
    {
        let content = extract_after(
            input,
            &["添加待办 ", "新增待办 ", "加待办 ", "add todo ", "添加 "],
        )
        .unwrap_or_default();
        if content.is_empty() {
            return Some(Intent::Unknown(
                "要添加什么待办事项呢？比如「添加待办 写完大作业」".to_string(),
            ));
        }
        return Some(Intent::TodoAdd { content });
    }

    if contains_any(
        &lower,
        &["查看待办", "待办列表", "我的待办", "显示待办", "todo"],
    ) && !lower.contains("添加")
    {
        return Some(Intent::TodoList);
    }

    if contains_any(
        &lower,
        &["完成待办", "做完待办", "done", "搞定待办", "勾掉待办"],
    ) || (lower.contains("完成") && lower.contains("待办"))
        || (lower.contains("做完") && lower.contains("待办"))
    {
        let idx = extract_after(input, &["完成待办 ", "做完待办 ", "done "]);
        if let Some(s) = idx
            && let Ok(n) = s.split_whitespace().next().unwrap_or("0").parse()
            && n > 0
        {
            return Some(Intent::TodoDone { index: n });
        }
        return Some(Intent::Unknown(
            "要完成哪个待办？比如「完成待办 1」".to_string(),
        ));
    }

    if contains_any(&lower, &["清空待办", "清除待办", "删光待办", "clear todo"]) {
        return Some(Intent::TodoClear);
    }

    None
}

// ============ 彩蛋检测 ============

fn detect_easter_egg(input: &str) -> Option<Intent> {
    let input = input.trim().to_lowercase();

    if input == "hello world" || input == "hello, world" {
        return Some(Intent::EasterEgg(
            "🎉 恭喜你发现了彩蛋！\n   'Hello World' — 每个程序员梦开始的地方。\n   愿你写的每一行代码，都如初见般美好。"
                .to_string(),
        ));
    }

    if input == "whoami" {
        return Some(Intent::EasterEgg(
            "🦊 我是小壳，一个会卖萌的自定义 Shell！\n   用 Rust 打造，诞生于 2026 年夏天。\n   我的使命是让命令行不再枯燥~"
                .to_string(),
        ));
    }

    if input.contains("小壳") && input.contains("可爱") {
        return Some(Intent::EasterEgg(
            "😳 哎呀，被夸了！谢谢谢谢~ 你也很可爱！".to_string(),
        ));
    }

    if input == "42" {
        return Some(Intent::EasterEgg(
            "🐬 42 — 生命、宇宙以及任何事情的终极答案。\n   《银河系漫游指南》粉确认！".to_string(),
        ));
    }

    if input.contains("rust") && (input.contains("爱") || input.contains("喜欢")) {
        return Some(Intent::EasterEgg(
            "🦀 Rust 是真的香！零成本抽象、内存安全、\n   编译即正确……我们都是 Rustacean！"
                .to_string(),
        ));
    }

    if input == "sudo" || input.contains("sudo rm -rf") {
        return Some(Intent::EasterEgg(
            "🚨 危险操作拦截！\n   小壳不会让你伤害自己的电脑的！\n   来，喝杯茶冷静一下 🍵"
                .to_string(),
        ));
    }

    if input == "42" {
        return Some(Intent::EasterEgg(
            "🐬 42 — 生命、宇宙以及任何事情的终极答案。\n   《银河系漫游指南》粉确认！".to_string(),
        ));
    }

    if input == "上上下下左右左右ba" || input == "上上下下左右左右巴" {
        return Some(Intent::EasterEgg(
            "🎮 魂斗罗 30 条命！……可惜这里用不上。\n   不过你有一颗怀旧的心，小壳喜欢！"
                .to_string(),
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_exit() {
        assert_eq!(parse_intent("退出"), Intent::Exit);
        assert_eq!(parse_intent("拜拜"), Intent::Exit);
        assert_eq!(parse_intent("exit"), Intent::Exit);
    }

    #[test]
    fn test_parse_help() {
        assert_eq!(parse_intent("帮助"), Intent::Help);
        assert_eq!(parse_intent("help"), Intent::Help);
        assert_eq!(parse_intent("你能干嘛"), Intent::Help);
    }

    #[test]
    fn test_parse_clear() {
        assert_eq!(parse_intent("清屏"), Intent::Clear);
        assert_eq!(parse_intent("clear"), Intent::Clear);
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_intent("几点了"), Intent::ShowTime);
        assert_eq!(parse_intent("现在时间"), Intent::ShowTime);
    }

    #[test]
    fn test_parse_date() {
        assert_eq!(parse_intent("今天几号"), Intent::ShowDate);
        assert_eq!(parse_intent("日期"), Intent::ShowDate);
    }

    #[test]
    fn test_parse_find_files() {
        let intent = parse_intent("找文件 main");
        match intent {
            Intent::FindFiles { keyword, .. } => assert_eq!(keyword, "main"),
            _ => panic!("Expected FindFiles"),
        }
    }

    #[test]
    fn test_parse_calculate() {
        let intent = parse_intent("算 1+2");
        match intent {
            Intent::Calculate(expr) => assert_eq!(expr, "1+2"),
            _ => panic!("Expected Calculate"),
        }
    }

    #[test]
    fn test_parse_auto_math() {
        let intent = parse_intent("1+2*3");
        match intent {
            Intent::Calculate(_) => {}
            _ => panic!("Expected Calculate for math expression"),
        }
    }

    #[test]
    fn test_parse_joke() {
        assert_eq!(parse_intent("讲个笑话"), Intent::TellJoke);
        assert_eq!(parse_intent("笑话"), Intent::TellJoke);
    }

    #[test]
    fn test_parse_compliment() {
        assert_eq!(parse_intent("夸我"), Intent::Compliment);
        assert_eq!(parse_intent("彩虹屁"), Intent::Compliment);
    }

    #[test]
    fn test_parse_games() {
        assert_eq!(parse_intent("猜数字"), Intent::PlayGuessNumber);
        assert_eq!(parse_intent("石头剪刀布"), Intent::PlayRPS);
        assert_eq!(parse_intent("打字测试"), Intent::PlayTyping);
        assert_eq!(parse_intent("掷骰子"), Intent::PlayDice);
    }

    #[test]
    fn test_parse_create_dir() {
        let intent = parse_intent("创建文件夹 test");
        match intent {
            Intent::CreateDir { name } => assert_eq!(name, "test"),
            _ => panic!("Expected CreateDir"),
        }
    }

    #[test]
    fn test_parse_create_file() {
        let intent = parse_intent("创建文件 readme.txt");
        match intent {
            Intent::CreateFile { name } => assert_eq!(name, "readme.txt"),
            _ => panic!("Expected CreateFile"),
        }
    }

    #[test]
    fn test_parse_timer() {
        let intent = parse_intent("5分钟后提醒我 开会");
        match intent {
            Intent::SetTimer {
                minutes,
                ref message,
            } => {
                assert_eq!(minutes, 5);
                assert!(message.contains("开会"));
            }
            _ => panic!("Expected SetTimer"),
        }
    }

    #[test]
    fn test_parse_todo_add() {
        let intent = parse_intent("添加待办 写完大作业");
        match intent {
            Intent::TodoAdd { ref content } => {
                assert!(content.contains("写完大作业"));
            }
            _ => panic!("Expected TodoAdd, got {:?}", intent),
        }
    }

    #[test]
    fn test_parse_todo_list() {
        assert_eq!(parse_intent("查看待办"), Intent::TodoList);
    }

    #[test]
    fn test_parse_todo_done() {
        let intent = parse_intent("完成待办 1");
        match intent {
            Intent::TodoDone { index } => assert_eq!(index, 1),
            _ => panic!("Expected TodoDone"),
        }
    }

    #[test]
    fn test_parse_system_command() {
        // 看起来像系统命令的输入
        let intent = parse_intent("git status");
        match intent {
            Intent::SystemCommand(cmd) => assert!(cmd.contains("git")),
            _ => panic!("Expected SystemCommand"),
        }
    }

    #[test]
    fn test_easter_egg_hello_world() {
        match parse_intent("hello world") {
            Intent::EasterEgg(_) => {}
            _ => panic!("Expected EasterEgg"),
        }
    }

    #[test]
    fn test_easter_egg_42() {
        match parse_intent("42") {
            Intent::EasterEgg(_) => {}
            _ => panic!("Expected EasterEgg"),
        }
    }

    #[test]
    fn test_easter_egg_whoami() {
        match parse_intent("whoami") {
            Intent::EasterEgg(_) => {}
            _ => panic!("Expected EasterEgg"),
        }
    }

    #[test]
    fn test_easter_egg_sudo() {
        match parse_intent("sudo rm -rf /") {
            Intent::EasterEgg(_) => {}
            _ => panic!("Expected EasterEgg"),
        }
    }

    #[test]
    fn test_contains_any() {
        assert!(contains_any("hello world", &["hello"]));
        assert!(!contains_any("hello world", &["xyz"]));
        assert!(contains_any("你好世界", &["你好"]));
    }

    #[test]
    fn test_extract_count_arabic() {
        assert_eq!(extract_count("找 5 个文件"), Some(5));
        assert_eq!(extract_count("找文件 main 10"), Some(10));
    }

    #[test]
    fn test_extract_count_chinese() {
        assert_eq!(extract_count("找三个文件"), Some(3));
        assert_eq!(extract_count("最近五个修改"), Some(5));
    }

    #[test]
    fn test_extract_timer_minutes() {
        assert_eq!(extract_minutes("5分钟后提醒我"), Some(5));
        assert_eq!(extract_minutes("十分钟后提醒我"), Some(10));
        assert_eq!(extract_minutes("半小时后提醒我"), Some(30));
    }

    #[test]
    fn test_extract_between() {
        assert_eq!(
            extract_between("hello [world] end", "[", "]"),
            Some("world")
        );
        assert_eq!(extract_between("no brackets here", "[", "]"), None);
        assert_eq!(extract_between("empty [] brackets", "[", "]"), Some(""));
    }

    #[test]
    fn test_keyword_matcher() {
        let matcher = KeywordMatcher {
            keywords: vec!["hello".to_string(), "你好".to_string()],
            priority_val: 50,
            intent: Intent::Help,
        };
        assert!(matcher.matches("hello world"));
        assert!(matcher.matches("你好世界"));
        assert!(!matcher.matches("goodbye"));
        assert_eq!(matcher.priority(), 50);
    }

    #[test]
    fn test_find_best_matcher() {
        let matchers: Vec<Box<dyn Matcher>> = vec![
            Box::new(KeywordMatcher {
                keywords: vec!["退出".to_string()],
                priority_val: 100,
                intent: Intent::Exit,
            }),
            Box::new(KeywordMatcher {
                keywords: vec!["帮助".to_string()],
                priority_val: 0,
                intent: Intent::Help,
            }),
        ];
        // "退出" 优先级更高
        let idx = find_best_matcher(&matchers, "我想退出程序");
        assert_eq!(idx, Some(0));
        // "帮助" 匹配但优先级低
        let idx = find_best_matcher(&matchers, "请帮助我");
        assert_eq!(idx, Some(1));
        // 都不匹配
        let idx = find_best_matcher(&matchers, "玩游戏");
        assert_eq!(idx, None);
    }
}
