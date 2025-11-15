use chrono::{Local, TimeZone};
use time_checker::data::{TimeEntry, DataStore};
use tempfile::tempdir;

#[test]
fn test_timeentry_serialization_with_all_fields() {
    // テストデータの準備
    let start_time = Local.with_ymd_and_hms(2025, 11, 14, 9, 0, 0).unwrap();
    let end_time = Local.with_ymd_and_hms(2025, 11, 14, 10, 30, 0).unwrap();

    let entry = TimeEntry {
        task: "プログラミング".to_string(),
        start: start_time,
        end: Some(end_time),
        note: Some("Rust実装".to_string()),
    };

    // JSONにシリアライズ
    let json = serde_json::to_string(&entry).expect("シリアライズに失敗");

    // JSONに必要なフィールドが含まれているか確認
    assert!(json.contains("\"task\":\"プログラミング\""));
    assert!(json.contains("\"note\":\"Rust実装\""));
}

#[test]
fn test_timeentry_serialization_without_optional_fields() {
    // 進行中のタスク（end がない）
    let start_time = Local.with_ymd_and_hms(2025, 11, 14, 9, 0, 0).unwrap();

    let entry = TimeEntry {
        task: "会議".to_string(),
        start: start_time,
        end: None,
        note: None,
    };

    let json = serde_json::to_string(&entry).expect("シリアライズに失敗");

    // noteフィールドは省略されているはず
    assert!(!json.contains("\"note\""));
    assert!(json.contains("\"task\":\"会議\""));
}

#[test]
fn test_timeentry_deserialization() {
    let json = r#"{
        "task": "テスト作業",
        "start": "2025-11-14T09:00:00+09:00",
        "end": "2025-11-14T10:00:00+09:00"
    }"#;

    let entry: TimeEntry = serde_json::from_str(json).expect("デシリアライズに失敗");

    assert_eq!(entry.task, "テスト作業");
    assert!(entry.end.is_some());
    assert!(entry.note.is_none());
}

#[test]
fn test_timeentry_roundtrip() {
    // シリアライズ -> デシリアライズで元に戻るか
    let start_time = Local.with_ymd_and_hms(2025, 11, 14, 14, 0, 0).unwrap();
    let end_time = Local.with_ymd_and_hms(2025, 11, 14, 15, 30, 0).unwrap();

    let original = TimeEntry {
        task: "ドキュメント作成".to_string(),
        start: start_time,
        end: Some(end_time),
        note: Some("設計書更新".to_string()),
    };

    let json = serde_json::to_string(&original).expect("シリアライズに失敗");
    let deserialized: TimeEntry = serde_json::from_str(&json).expect("デシリアライズに失敗");

    assert_eq!(original, deserialized);
}

// DataStore のテスト

#[test]
fn test_datastore_save_and_load() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("data.json");
    let store = DataStore::new(data_file);

    // テストデータの作成
    let start_time = Local.with_ymd_and_hms(2025, 11, 14, 9, 0, 0).unwrap();
    let end_time = Local.with_ymd_and_hms(2025, 11, 14, 10, 0, 0).unwrap();

    let entries = vec![
        TimeEntry {
            task: "タスク1".to_string(),
            start: start_time,
            end: Some(end_time),
            note: None,
        },
    ];

    // 保存
    store.save(&entries).expect("保存に失敗");

    // 読み込み
    let loaded = store.load().expect("読み込みに失敗");

    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].task, "タスク1");
}

#[test]
fn test_datastore_load_empty_file() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("empty.json");
    let store = DataStore::new(data_file);

    // 空のファイルを作成
    std::fs::write(store.data_file(), "").expect("ファイル作成に失敗");

    // 空のベクタが返るはず
    let loaded = store.load().expect("読み込みに失敗");
    assert_eq!(loaded.len(), 0);
}

#[test]
fn test_datastore_load_nonexistent_file() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("nonexistent.json");
    let store = DataStore::new(data_file);

    // 存在しないファイルからの読み込みは空のベクタを返すはず
    let loaded = store.load().expect("読み込みに失敗");
    assert_eq!(loaded.len(), 0);
}

#[test]
fn test_datastore_get_current_task() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("current.json");
    let store = DataStore::new(data_file);

    let start_time1 = Local.with_ymd_and_hms(2025, 11, 14, 9, 0, 0).unwrap();
    let end_time1 = Local.with_ymd_and_hms(2025, 11, 14, 10, 0, 0).unwrap();
    let start_time2 = Local.with_ymd_and_hms(2025, 11, 14, 10, 0, 0).unwrap();

    let entries = vec![
        TimeEntry {
            task: "完了タスク".to_string(),
            start: start_time1,
            end: Some(end_time1),
            note: None,
        },
        TimeEntry {
            task: "進行中タスク".to_string(),
            start: start_time2,
            end: None,
            note: None,
        },
    ];

    store.save(&entries).expect("保存に失敗");

    // 現在のタスク（end が None のもの）を取得
    let current = store.get_current_task().expect("取得に失敗");

    assert!(current.is_some());
    assert_eq!(current.unwrap().task, "進行中タスク");
}

#[test]
fn test_datastore_get_current_task_none() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("no_current.json");
    let store = DataStore::new(data_file);

    let start_time = Local.with_ymd_and_hms(2025, 11, 14, 9, 0, 0).unwrap();
    let end_time = Local.with_ymd_and_hms(2025, 11, 14, 10, 0, 0).unwrap();

    let entries = vec![
        TimeEntry {
            task: "完了タスク".to_string(),
            start: start_time,
            end: Some(end_time),
            note: None,
        },
    ];

    store.save(&entries).expect("保存に失敗");

    // 全てのタスクが完了している場合は None
    let current = store.get_current_task().expect("取得に失敗");
    assert!(current.is_none());
}

#[test]
fn test_datastore_get_today_entries() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("today.json");
    let store = DataStore::new(data_file);

    let now = Local::now();
    let today_start = now.date_naive().and_hms_opt(9, 0, 0).unwrap();
    let today_end = now.date_naive().and_hms_opt(10, 0, 0).unwrap();

    // 昨日のタスク
    let yesterday = now.date_naive().pred_opt().unwrap().and_hms_opt(9, 0, 0).unwrap();

    let entries = vec![
        TimeEntry {
            task: "昨日のタスク".to_string(),
            start: Local.from_local_datetime(&yesterday).unwrap(),
            end: Some(Local.from_local_datetime(&yesterday.checked_add_signed(chrono::Duration::hours(1)).unwrap()).unwrap()),
            note: None,
        },
        TimeEntry {
            task: "今日のタスク".to_string(),
            start: Local.from_local_datetime(&today_start).unwrap(),
            end: Some(Local.from_local_datetime(&today_end).unwrap()),
            note: None,
        },
    ];

    store.save(&entries).expect("保存に失敗");

    // 今日のエントリのみ取得
    let today_entries = store.get_today_entries().expect("取得に失敗");

    assert_eq!(today_entries.len(), 1);
    assert_eq!(today_entries[0].task, "今日のタスク");
}
