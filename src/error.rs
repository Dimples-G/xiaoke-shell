use std::fmt;

/// 小壳 Shell 的自定义错误类型
#[derive(Debug)]
pub enum ShellError {
    /// IO 错误
    Io(std::io::Error),
    /// 解析错误
    Parse(String),
    /// 表达式求值错误
    Eval(String),
    /// 资源未找到（预留）
    #[allow(dead_code)]
    NotFound(String),
    /// 用户取消操作
    Canceled,
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Io(e) => write!(f, "IO 错误: {}", e),
            ShellError::Parse(msg) => write!(f, "解析错误: {}", msg),
            ShellError::Eval(msg) => write!(f, "计算错误: {}", msg),
            ShellError::NotFound(msg) => write!(f, "未找到: {}", msg),
            ShellError::Canceled => write!(f, "操作已取消"),
        }
    }
}

impl std::error::Error for ShellError {}

impl From<std::io::Error> for ShellError {
    fn from(e: std::io::Error) -> Self {
        ShellError::Io(e)
    }
}

impl From<std::num::ParseFloatError> for ShellError {
    fn from(e: std::num::ParseFloatError) -> Self {
        ShellError::Eval(format!("数字解析失败: {}", e))
    }
}

impl From<std::num::ParseIntError> for ShellError {
    fn from(e: std::num::ParseIntError) -> Self {
        ShellError::Parse(format!("整数解析失败: {}", e))
    }
}

/// Shell 操作结果别名
pub type ShellResult<T> = Result<T, ShellError>;
