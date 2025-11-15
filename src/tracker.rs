// ビジネスロジック（Tracker）

use chrono::Local;
use std::collections::HashMap;
use std::time::Duration;
use crate::data::{DataStore, TimeEntry};
use crate::error::TimeCheckerError;

/// ビジネスロジックを担当する構造体
pub struct Tracker {
    store: DataStore,
}

impl Tracker {
    /// 新しいTrackerインスタンスを作成
    pub fn new(store: DataStore) -> Self {
        Self { store }
    }

    /// DataStoreへの参照を取得
    pub fn store(&self) -> &DataStore {
        &self.store
    }

    /// 新しいタスクを開始
    /// 進行中のタスクがあれば自動的に終了する
    pub fn start_task(&self, task: String, note: Option<String>) -> Result<(), TimeCheckerError> {
        let mut entries = self.store.load()?;
        let now = Local::now();

        // 進行中のタスクがあれば終了する
        if let Some(current) = entries.iter_mut().rev().find(|e| e.end.is_none()) {
            current.end = Some(now);
        }

        // 新しいタスクを追加
        let new_entry = TimeEntry {
            task,
            start: now,
            end: None,
            note,
        };

        entries.push(new_entry);
        self.store.save(&entries)?;

        Ok(())
    }

    /// 現在のタスクを停止
    pub fn stop_task(&self) -> Result<(), TimeCheckerError> {
        let mut entries = self.store.load()?;
        let now = Local::now();

        // 進行中のタスクを見つけて終了
        let found = entries.iter_mut().rev().find(|e| e.end.is_none());

        match found {
            Some(entry) => {
                entry.end = Some(now);
                self.store.save(&entries)?;
                Ok(())
            }
            None => Err(TimeCheckerError::NoActiveTask),
        }
    }

    /// 今日のタスクのサマリーを取得（タスク名ごとに集計）
    pub fn get_today_summary(&self) -> Result<HashMap<String, Duration>, TimeCheckerError> {
        let entries = self.store.get_today_entries()?;
        let mut summary: HashMap<String, Duration> = HashMap::new();

        for entry in entries {
            // 終了時刻がない場合は現在時刻を使用
            let end = entry.end.unwrap_or_else(Local::now);
            let duration = end.signed_duration_since(entry.start);

            if duration.num_milliseconds() >= 0 {
                let std_duration = Duration::from_millis(duration.num_milliseconds() as u64);
                summary
                    .entry(entry.task.clone())
                    .and_modify(|d| *d += std_duration)
                    .or_insert(std_duration);
            }
        }

        Ok(summary)
    }
}
