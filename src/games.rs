use colored::*;
use rand::Rng as _;
use std::io::{self, Write};
use std::time::Instant;

// ============ 猜数字游戏 ============

pub fn guess_number() {
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    let reader = io::stdin();

    println!();
    println!("{}", "╔════════════════════════════════╗".bright_cyan());
    println!(
        "{}",
        "║     🎯 猜数字游戏 (1-100)     ║".bright_cyan().bold()
    );
    println!("{}", "╚════════════════════════════════╝".bright_cyan());
    println!(
        "{} 小壳心里想了一个 1~100 之间的数字~",
        "🤔".bright_yellow()
    );
    println!("💡 来猜猜看吧！输入「不玩了」可以退出\n");

    loop {
        print!("{} 你的猜测：", "👉".bright_green());
        io::stdout().flush().ok();

        let mut input = String::new();
        reader.read_line(&mut input).ok();
        let input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }

        if contains_any(&input, &["不玩了", "退出", "放弃", "quit"]) {
            println!(
                "{} 答案是 {}，下次加油哦~",
                "😢".bright_yellow(),
                secret.to_string().bright_green().bold()
            );
            break;
        }

        let guess: i32 = match input.parse() {
            Ok(n) if (1..=100).contains(&n) => n,
            _ => {
                println!("{} 请输入 1~100 之间的整数哦！", "😅".bright_red());
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret) {
            std::cmp::Ordering::Equal => {
                let msg = match attempts {
                    1 => "一发入魂！你就是传说中的欧皇？？👑".bright_yellow().bold(),
                    2..=3 => format!("{} 次就猜中了！天才！🌟", attempts)
                        .bright_green()
                        .bold(),
                    4..=6 => format!("{} 次猜中，不错不错~ 👍", attempts).bright_cyan(),
                    7..=9 => format!("{} 次，还可以啦~ 🙂", attempts).bright_yellow(),
                    _ => format!("{} 次……终于猜到了 😅", attempts).bright_red(),
                };
                println!("\n{}", "🎉🎉🎉 猜对了！ 🎉🎉🎉".bright_green().bold());
                println!("{}", msg);
                println!();
                break;
            }
            std::cmp::Ordering::Less => {
                let hint = if secret - guess > 20 {
                    "太小了！！！📉".bright_blue()
                } else if secret - guess > 10 {
                    "有点小哦 📉".bright_cyan()
                } else {
                    "稍微小了那么一点点~ 📉".bright_yellow()
                };
                println!("{}", hint);
            }
            std::cmp::Ordering::Greater => {
                let hint = if guess - secret > 20 {
                    "太大了！！！📈".bright_red()
                } else if guess - secret > 10 {
                    "有点大哦 📈".bright_magenta()
                } else {
                    "稍微大了那么一点点~ 📈".bright_yellow()
                };
                println!("{}", hint);
            }
        }
    }
}

// ============ 石头剪刀布 ============

pub fn rock_paper_scissors() {
    let choices = ["石头", "剪刀", "布"];
    let emojis = ["🪨", "✂️", "📄"];
    let mut player_score = 0;
    let mut shell_score = 0;
    let reader = io::stdin();

    println!();
    println!("{}", "╔════════════════════════════════╗".bright_cyan());
    println!(
        "{}",
        "║   ✊ 石头剪刀布 — 三局两胜   ║".bright_cyan().bold()
    );
    println!("{}", "╚════════════════════════════════╝".bright_cyan());
    println!(
        "{} 输入「石头」「剪刀」「布」来对战！",
        "🎮".bright_yellow()
    );
    println!("💡 输入「不玩了」退出\n");

    loop {
        if player_score >= 2 || shell_score >= 2 {
            break;
        }

        println!(
            "{} 比分 — 你: {}  小壳: {}",
            "📊".bright_white(),
            player_score.to_string().bright_green(),
            shell_score.to_string().bright_red()
        );
        print!("{} 你出什么？", "👉".bright_green());
        io::stdout().flush().ok();

        let mut input = String::new();
        reader.read_line(&mut input).ok();
        let input = input.trim().to_string();

        if contains_any(&input, &["不玩了", "退出", "放弃", "quit"]) {
            println!("{} 下次再来玩哦~", "👋".bright_yellow());
            println!();
            return;
        }

        let player_choice = match find_choice(&input) {
            Some(idx) => idx,
            None => {
                println!("{} 出「石头」「剪刀」或「布」哦！", "😅".bright_red());
                continue;
            }
        };

        let shell_choice = rand::thread_rng().gen_range(0..3);

        println!(
            "  你出了 {} {}    小壳出了 {} {}",
            emojis[player_choice],
            choices[player_choice].bright_green(),
            emojis[shell_choice],
            choices[shell_choice].bright_red()
        );

        // 0=石头, 1=剪刀, 2=布
        // 石头(0) > 剪刀(1), 剪刀(1) > 布(2), 布(2) > 石头(0)
        match (player_choice, shell_choice) {
            (p, s) if p == s => {
                println!("{} 平局！再来~", "🤝".bright_yellow());
            }
            (0, 1) | (1, 2) | (2, 0) => {
                println!("{} 你赢了这局！", "🎉".bright_green());
                player_score += 1;
            }
            _ => {
                println!("{} 小壳赢了这局~", "😝".bright_red());
                shell_score += 1;
            }
        }
        println!();
    }

    println!();
    if player_score >= 2 {
        println!("{} 你赢了！太厉害了！🏆", "🎉🎉🎉".bright_green().bold());
        println!("{} 小壳甘拜下风~", "🙇".bright_yellow());
    } else {
        println!("{} 小壳赢了！承让承让~ 😝", "🦊".bright_red().bold());
        println!("{} 再来一局你一定能赢！", "💪".bright_green());
    }
    println!();
}

fn find_choice(input: &str) -> Option<usize> {
    if input.contains("石头") {
        Some(0)
    } else if input.contains("剪刀") {
        Some(1)
    } else if input.contains("布") {
        Some(2)
    } else {
        None
    }
}

// ============ 打字速度测试 ============

pub fn typing_test() {
    let sentences = [
        "小壳是一个可爱的自定义Shell程序",
        "Rust是一门系统级编程语言",
        "今天天气真好适合写代码",
        "命令行也可以很有趣很好玩",
        "每一个程序员都是从HelloWorld开始的",
        "代码就像魔法一样可以创造世界",
        "人生苦短要用Rust写代码",
    ];

    let target = sentences[rand::thread_rng().gen_range(0..sentences.len())];
    let reader = io::stdin();

    println!();
    println!("{}", "╔════════════════════════════════╗".bright_cyan());
    println!(
        "{}",
        "║       ⌨️  打字速度测试          ║".bright_cyan().bold()
    );
    println!("{}", "╚════════════════════════════════╝".bright_cyan());
    println!("{} 请准确输入下面这句话：", "📝".bright_yellow());
    println!();
    println!("  {}", target.bright_white().bold());
    println!();

    print!("{} 准备好了按 Enter 开始计时！", "⏳".bright_green());
    io::stdout().flush().ok();
    let mut ready = String::new();
    reader.read_line(&mut ready).ok();

    let start = Instant::now();

    print!("{} 输入：", "⌨️ ".bright_green());
    io::stdout().flush().ok();
    let mut input = String::new();
    reader.read_line(&mut input).ok();
    let input = input.trim().to_string();

    let elapsed = start.elapsed();
    let seconds = elapsed.as_secs_f64();

    // 计算准确率
    let target_chars: Vec<char> = target.chars().collect();
    let input_chars: Vec<char> = input.chars().collect();
    let total = target_chars.len().max(input_chars.len());
    let correct = target_chars
        .iter()
        .zip(input_chars.iter())
        .filter(|(a, b)| a == b)
        .count();
    let accuracy = if total > 0 {
        correct as f64 / total as f64 * 100.0
    } else {
        0.0
    };

    // 字数（按中文字符算）
    let char_count = input.chars().count();
    let cpm = char_count as f64 / seconds * 60.0; // 字/分钟

    println!();
    println!("{} 结果：", "📊".bright_cyan());
    println!("  输入内容：{}", input.bright_white());
    println!("  正确内容：{}", target.bright_green());
    println!("  用时：{} 秒", format!("{:.1}", seconds).bright_yellow());
    println!("  准确率：{}", format!("{:.1}%", accuracy).bright_magenta());
    println!("  速度：{} 字/分钟", format!("{:.0}", cpm).bright_blue());

    let rating = if accuracy > 95.0 && cpm > 60.0 {
        "🏆 打字大神！".bright_yellow().bold()
    } else if accuracy > 90.0 && cpm > 40.0 {
        "👍 打字高手！".bright_green()
    } else if accuracy > 80.0 {
        "🙂 还不错，继续加油！".bright_cyan()
    } else if cpm > 20.0 {
        "🐢 打字小萌新，多多练习~".bright_yellow()
    } else {
        "💤 你是在用脚打字吗……".bright_red()
    };
    println!("  评级：{}", rating);
    println!();
}

pub fn dice_roll() {
    let reader = io::stdin();

    println!();
    println!("{}", "╔════════════════════════════════╗".bright_cyan());
    println!(
        "{}",
        "║       🎲 掷骰子游戏            ║".bright_cyan().bold()
    );
    println!("{}", "╚════════════════════════════════╝".bright_cyan());
    println!(
        "{} 输入「roll」掷骰子，「不玩了」退出",
        "🎮".bright_yellow()
    );
    println!();

    loop {
        print!("{} ", "👉".bright_green());
        io::stdout().flush().ok();

        let mut input = String::new();
        reader.read_line(&mut input).ok();
        let input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }

        if contains_any(&input, &["不玩了", "退出", "放弃", "quit"]) {
            println!("{} 下次再来玩哦~", "👋".bright_yellow());
            println!();
            break;
        }

        if contains_any(&input, &["roll", "掷", "扔", "丢", "摇"]) {
            let d1 = rand::thread_rng().gen_range(1..=6);
            let d2 = rand::thread_rng().gen_range(1..=6);
            let total = d1 + d2;
            let dice_faces = ["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"];

            println!(
                "  {} {} + {} {} = {}",
                dice_faces[d1 - 1],
                d1.to_string().bright_white(),
                dice_faces[d2 - 1],
                d2.to_string().bright_white(),
                total.to_string().bright_green().bold()
            );

            match total {
                2 => println!("  {} 最小点数……人品有待提高 😅", "💀".bright_red()),
                3 | 4 => println!("  {} 运气不太好呢~", "😐".bright_yellow()),
                5..=7 => println!("  {} 一般般~", "🙂".bright_cyan()),
                8..=10 => println!("  {} 不错哦！", "😊".bright_green()),
                11 => println!("  {} 运气很好！", "🌟".bright_yellow()),
                12 => println!("  {} 天选之人！最大点数！！！", "👑".bright_yellow()),
                _ => {}
            }
        } else {
            println!("{} 输入「roll」来掷骰子吧~", "💡".bright_yellow());
        }
    }
}

fn contains_any(input: &str, keywords: &[&str]) -> bool {
    keywords.iter().any(|k| input.contains(k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_any() {
        assert!(contains_any("hello world", &["hello"]));
        assert!(!contains_any("hello world", &["xyz"]));
        assert!(contains_any("猜数字游戏", &["猜数字"]));
    }
}
