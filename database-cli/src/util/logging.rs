/// The log level. This is used program-wide to specify what should
/// and what should not be logged. Default is [`Info`](LogLevel::Info). 
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl From<u8> for LogLevel {
    fn from(from: u8) -> Self {
        match from {
            0 => LogLevel::Trace,
            1 => LogLevel::Debug,
            3 => LogLevel::Warn,
            4 => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }
}
