use std::fmt;
use std::path::PathBuf;

pub struct Violation {
    pub path: PathBuf,
    pub line: usize,
    pub rule: &'static str,
    pub message: String,
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: [{}] {}",
            self.path.display(),
            self.line,
            self.rule,
            self.message
        )
    }
}
