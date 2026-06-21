# 🦊 小壳 Shell — 智能命令行助手

一个用 Rust 编写的智能命令行助手，融合了**自然语言交互**与**趣味玩具**的自定义 Shell。

## 快速开始

```bash
# 构建
cargo build --release

# 运行
cargo run

# 测试
cargo test

# 代码检查
cargo fmt && cargo clippy
```

## 依赖

| crate | 版本 | 用途 |
|-------|------|------|
| `rustyline` | 15 | 行编辑、历史记录、Tab 补全 |
| `colored` | 2 | 终端彩色输出 |
| `rand` | 0.8 | 随机数（游戏、趣味功能） |

## 功能概览

### 📂 文件操作
- `找文件 <关键词>` — 递归搜索文件
- `搜索内容 <关键词>` — 并发搜索文件内容
- `创建文件/文件夹 <名字>` — 创建文件或目录
- `删除文件 <名字>` — 删除文件或目录
- `列出文件` / `看看有什么` — 显示当前目录

### 🔧 系统信息
- `几点了` / `时间` — 显示当前时间
- `今天几号` / `日期` — 显示当前日期
- `磁盘空间` / `硬盘` — 查看磁盘容量
- `内存` / `运存` — 查看内存信息

### 🧮 计算器
- `算 <表达式>` — 支持 + - * / () ^ 运算
- 自动识别数学表达式（如直接输入 `1+2*3`）

### 🎮 小游戏
- `猜数字` — 1~100 猜数，多级评价
- `石头剪刀布` — 三局两胜对战
- `打字测试` — 中文打字速度与准确率
- `骰子` / `掷骰子` — 双骰子掷点

### ⏰ 定时提醒
- `N分钟后提醒我 <消息>` — 后台线程定时提醒
- 支持中文数字：`五分钟后提醒我 开会`、`半小时后提醒我 休息`

### 📋 待办事项
- `添加待办 <内容>` — 添加待办
- `查看待办` — 显示列表（含进度）
- `完成待办 <编号>` — 标记完成
- `清空待办` — 清空列表

### 🎉 趣味
- `笑话` / `讲个笑话` — 程序员幽默
- `夸我` / `彩虹屁` — 随机赞美
- `ASCII` / `字符画` — 随机字符画
- `掷硬币` / `抛硬币` — 硬币正反

### 🔍 彩蛋
- `hello world`、`whoami`、`42`
- `sudo rm -rf` — 危险操作拦截
- `上上下下左右左右ba` — 魂斗罗致敬
- `小壳` + `可爱` — 害羞回应

### ⚙️ 其他
- `清屏` / `clear` — 清屏
- `帮助` / `help` — 显示帮助菜单
- `退出` / `拜拜` — 退出程序
- 直接输入系统命令也可以执行（如 `git status`）

## 模块架构

```
src/
├── main.rs       # 入口、REPL 循环、历史记录
├── intent.rs     # 自然语言意图解析（关键词匹配 + 正则提取）
├── actions.rs    # 意图执行（文件操作、系统信息、计算器、定时器、待办）
├── games.rs      # 小游戏（猜数字、石头剪刀布、打字测试、掷骰子）
├── display.rs    # UI 渲染（彩色框线、对齐算法、欢迎/帮助界面）
└── error.rs      # 错误处理（自定义 Error 类型、Result 别名）
```

### 数据流

```
用户输入 → intent::parse_intent() → Intent 枚举
         → actions::execute_intent() → 执行并输出
```

## Rust 技术点

### 自定义 Trait
```rust
pub trait Matcher {
    fn matches(&self, input: &str) -> bool;
    fn priority(&self) -> i32 { 0 }         // 默认实现
    fn to_intent(&self) -> Option<Intent> { None }
}
```
`KeywordMatcher` 实现该 trait，`best_match()` 通过泛型 + trait bound 调用，
`find_best_matcher()` 通过 `Box<dyn Matcher>` trait object 调用。

### 泛型
```rust
fn best_match<'a, M: Matcher>(matchers: &'a [M], input: &str) -> Option<&'a M>
fn search_files_recursive<P: AsRef<Path>>(dir: P, keyword: &str, max: usize) -> ShellResult<Vec<String>>
```

### 生命周期注解
```rust
fn strip_comments<'a>(cmd: &'a str) -> &'a str
fn extract_between<'a>(input: &'a str, start: &str, end: &str) -> Option<&'a str>
fn best_match<'a, M: Matcher>(matchers: &'a [M], input: &str) -> Option<&'a M>
```

### 并发
- `std::thread::spawn` — 独立线程定时提醒
- `mpsc::channel` — 文件内容搜索的生产者-消费者模式
- 主线程发送搜索任务，工作线程通过 channel 返回匹配结果

### 所有权与借用
- 所有函数优先使用 `&str` / `&[T]` 借用而非获取所有权
- `Intent` 枚举中的 `String` 通过 `clone()` 显式复制
- `TodoList` 通过 `&mut` 引用在 REPL 中维护

### 错误处理
- `ShellError` 枚举实现 `Display` + `Error` + `From<T>` trait
- `ShellResult<T>` 类型别名用于统一错误传播
- 大量使用 `?` 操作符，避免 `unwrap()` / `expect()`

### 结构体与枚举
- `Intent` 枚举：28 个变体覆盖全部功能
- `Token` 枚举：计算器词法分析的 7 种 Token
- `ShellError` 枚举：5 种错误变体
- `TodoItem` 结构体：待办事项数据模型
- `KeywordMatcher` 结构体：可扩展的意图匹配器

## 测试

```bash
cargo test                # 运行全部 47 个测试
cargo test --test '*'     # 运行集成测试
```

| 模块 | 测试数 | 覆盖内容 |
|------|--------|----------|
| intent | 25 | 意图解析、彩蛋、提取函数、Matcher trait |
| actions | 11 | 文件大小格式化、闰年判断、表达式计算、待办操作、注释处理 |
| display | 6 | 显示宽度计算（ASCII/CJK/Emoji）、框线生成 |
| games | 2 | 关键词匹配 |
