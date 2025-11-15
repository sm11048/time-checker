// エラー型の定義
use std::fmt;

#[derive(Debug)]
pub enum TimeCheckerError {
    NoActiveTask,
    DataLoadError(String),
    DataSaveError(String),
    InvalidPeriod(String),
}

impl fmt::Display for TimeCheckerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeCheckerError::NoActiveTask => write!(f, "進行中のタスクがありません"),
            TimeCheckerError::DataLoadError(msg) => write!(f, "データの読み込みに失敗しました: {}", msg),
            TimeCheckerError::DataSaveError(msg) => write!(f, "データの保存に失敗しました: {}", msg),
            TimeCheckerError::InvalidPeriod(period) => write!(f, "無効な期間指定です: {}", period),
        }
    }
}

impl std::error::Error for TimeCheckerError {}
