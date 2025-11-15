use chrono::{Local, TimeZone};
use tempfile::tempdir;
use time_checker::data::DataStore;
use time_checker::tracker::Tracker;

#[test]
fn test_tracker_start_new_task() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("tracker.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    // 新しいタスクを開始
    tracker.start_task("プログラミング".to_string(), None).expect("タスクの開始に失敗");

    // エントリが保存されているか確認
    let entries = tracker.store().load().expect("読み込みに失敗");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].task, "プログラミング");
    assert!(entries[0].end.is_none());
}

#[test]
fn test_tracker_start_task_auto_ends_previous() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("tracker_auto_end.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    // 最初のタスクを開始
    tracker.start_task("タスク1".to_string(), None).expect("タスク1の開始に失敗");

    // 少し待ってから次のタスクを開始
    std::thread::sleep(std::time::Duration::from_millis(10));
    tracker.start_task("タスク2".to_string(), None).expect("タスク2の開始に失敗");

    // エントリを確認
    let entries = tracker.store().load().expect("読み込みに失敗");
    assert_eq!(entries.len(), 2);

    // タスク1は終了しているはず
    assert_eq!(entries[0].task, "タスク1");
    assert!(entries[0].end.is_some());

    // タスク2は進行中のはず
    assert_eq!(entries[1].task, "タスク2");
    assert!(entries[1].end.is_none());
}

#[test]
fn test_tracker_stop_task() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("tracker_stop.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    // タスクを開始
    tracker.start_task("作業".to_string(), None).expect("タスクの開始に失敗");

    // タスクを停止
    tracker.stop_task().expect("タスクの停止に失敗");

    // 現在のタスクがないことを確認
    let current = tracker.store().get_current_task().expect("取得に失敗");
    assert!(current.is_none());
}

#[test]
fn test_tracker_stop_task_no_active_task() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("tracker_no_active.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    // アクティブなタスクがない状態で停止を試みる
    let result = tracker.stop_task();
    assert!(result.is_err());
}

#[test]
fn test_tracker_get_today_summary() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("tracker_summary.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    let now = Local::now();
    let today_start1 = now.date_naive().and_hms_opt(9, 0, 0).unwrap();
    let today_end1 = now.date_naive().and_hms_opt(10, 0, 0).unwrap();
    let today_start2 = now.date_naive().and_hms_opt(10, 0, 0).unwrap();
    let today_end2 = now.date_naive().and_hms_opt(11, 30, 0).unwrap();

    // 手動でテストデータを作成
    use time_checker::data::TimeEntry;
    let entries = vec![
        TimeEntry {
            task: "プログラミング".to_string(),
            start: Local.from_local_datetime(&today_start1).unwrap(),
            end: Some(Local.from_local_datetime(&today_end1).unwrap()),
            note: None,
        },
        TimeEntry {
            task: "会議".to_string(),
            start: Local.from_local_datetime(&today_start2).unwrap(),
            end: Some(Local.from_local_datetime(&today_end2).unwrap()),
            note: None,
        },
    ];

    tracker.store().save(&entries).expect("保存に失敗");

    // 今日のサマリーを取得
    let summary = tracker.get_today_summary().expect("サマリーの取得に失敗");

    assert_eq!(summary.len(), 2);
    assert_eq!(summary.get("プログラミング").unwrap().as_secs(), 3600); // 1時間
    assert_eq!(summary.get("会議").unwrap().as_secs(), 5400); // 1.5時間
}

#[test]
fn test_tracker_aggregate_same_task_names() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("tracker_aggregate.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    let now = Local::now();
    let start1 = now.date_naive().and_hms_opt(9, 0, 0).unwrap();
    let end1 = now.date_naive().and_hms_opt(10, 0, 0).unwrap();
    let start2 = now.date_naive().and_hms_opt(11, 0, 0).unwrap();
    let end2 = now.date_naive().and_hms_opt(12, 0, 0).unwrap();

    // 同じ名前のタスクを複数回実行
    use time_checker::data::TimeEntry;
    let entries = vec![
        TimeEntry {
            task: "プログラミング".to_string(),
            start: Local.from_local_datetime(&start1).unwrap(),
            end: Some(Local.from_local_datetime(&end1).unwrap()),
            note: None,
        },
        TimeEntry {
            task: "プログラミング".to_string(),
            start: Local.from_local_datetime(&start2).unwrap(),
            end: Some(Local.from_local_datetime(&end2).unwrap()),
            note: None,
        },
    ];

    tracker.store().save(&entries).expect("保存に失敗");

    // サマリーで集計されているか確認
    let summary = tracker.get_today_summary().expect("サマリーの取得に失敗");

    assert_eq!(summary.len(), 1);
    assert_eq!(summary.get("プログラミング").unwrap().as_secs(), 7200); // 2時間
}

#[test]
fn test_tracker_start_with_note() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("tracker_note.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    // メモ付きでタスクを開始
    tracker.start_task("ドキュメント作成".to_string(), Some("設計書更新".to_string()))
        .expect("タスクの開始に失敗");

    let entries = tracker.store().load().expect("読み込みに失敗");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].task, "ドキュメント作成");
    assert_eq!(entries[0].note, Some("設計書更新".to_string()));
}
