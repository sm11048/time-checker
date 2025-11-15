// データ構造とDataStoreの実装

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use crate::error::TimeCheckerError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimeEntry {
    /// タスク名
    pub task: String,

    /// 開始時刻
    pub start: DateTime<Local>,

    /// 終了時刻（進行中の場合はNone）
    pub end: Option<DateTime<Local>>,

    /// 備考・メモ（オプション）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// データの永続化を担当する構造体
pub struct DataStore {
    data_file: PathBuf,
}

impl DataStore {
    /// 新しいDataStoreインスタンスを作成
    pub fn new(data_file: PathBuf) -> Self {
        Self { data_file }
    }

    /// データファイルのパスを取得
    pub fn data_file(&self) -> &Path {
        &self.data_file
    }

    /// エントリをファイルに保存
    pub fn save(&self, entries: &[TimeEntry]) -> Result<(), TimeCheckerError> {
        // 親ディレクトリが存在しない場合は作成
        if let Some(parent) = self.data_file.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                TimeCheckerError::DataSaveError(format!("ディレクトリの作成に失敗: {}", e))
            })?;
        }

        // JSONにシリアライズして保存
        let json = serde_json::to_string_pretty(entries).map_err(|e| {
            TimeCheckerError::DataSaveError(format!("シリアライズに失敗: {}", e))
        })?;

        fs::write(&self.data_file, json).map_err(|e| {
            TimeCheckerError::DataSaveError(format!("ファイルの書き込みに失敗: {}", e))
        })?;

        Ok(())
    }

    /// ファイルからエントリを読み込み
    pub fn load(&self) -> Result<Vec<TimeEntry>, TimeCheckerError> {
        // ファイルが存在しない場合は空のベクタを返す
        if !self.data_file.exists() {
            return Ok(Vec::new());
        }

        // ファイルを読み込み
        let content = fs::read_to_string(&self.data_file).map_err(|e| {
            TimeCheckerError::DataLoadError(format!("ファイルの読み込みに失敗: {}", e))
        })?;

        // 空のファイルの場合は空のベクタを返す
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        // JSONからデシリアライズ
        let entries: Vec<TimeEntry> = serde_json::from_str(&content).map_err(|e| {
            TimeCheckerError::DataLoadError(format!("デシリアライズに失敗: {}", e))
        })?;

        Ok(entries)
    }

    /// 現在進行中のタスクを取得（end が None のもの）
    pub fn get_current_task(&self) -> Result<Option<TimeEntry>, TimeCheckerError> {
        let entries = self.load()?;

        // 最後の end が None のエントリを探す
        Ok(entries.iter().rev().find(|e| e.end.is_none()).cloned())
    }

    /// 今日のエントリを取得
    pub fn get_today_entries(&self) -> Result<Vec<TimeEntry>, TimeCheckerError> {
        let entries = self.load()?;
        let today = Local::now().date_naive();

        // 今日の日付のエントリのみをフィルタ
        Ok(entries
            .into_iter()
            .filter(|e| e.start.date_naive() == today)
            .collect())
    }
}
