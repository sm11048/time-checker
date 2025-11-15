use chrono::{Local, TimeZone};
use tempfile::tempdir;
use time_checker::data::{DataStore, TimeEntry};
use time_checker::tracker::Tracker;

/// エンドツーエンドのワークフローテスト
#[test]
fn test_complete_workflow() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("workflow.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    // 1. 最初のタスクを開始
    tracker
        .start_task("プログラミング".to_string(), None)
        .expect("タスク開始に失敗");

    // 現在のタスクがあることを確認
    let current = tracker
        .store()
        .get_current_task()
        .expect("取得に失敗");
    assert!(current.is_some());
    assert_eq!(current.unwrap().task, "プログラミング");

    // 2. 別のタスクを開始（自動終了のテスト）
    std::thread::sleep(std::time::Duration::from_millis(200));
    tracker
        .start_task("会議".to_string(), Some("週次定例".to_string()))
        .expect("タスク開始に失敗");

    // 現在のタスクが変わっていることを確認
    let current = tracker
        .store()
        .get_current_task()
        .expect("取得に失敗");
    assert!(current.is_some());
    let current_task = current.unwrap();
    assert_eq!(current_task.task, "会議");
    assert_eq!(current_task.note, Some("週次定例".to_string()));

    // エントリが2つあることを確認
    let entries = tracker.store().load().expect("読み込みに失敗");
    assert_eq!(entries.len(), 2);

    // 最初のタスクが終了していることを確認
    assert!(entries[0].end.is_some());
    assert!(entries[1].end.is_none());

    // 3. タスクを停止
    std::thread::sleep(std::time::Duration::from_millis(200));
    tracker.stop_task().expect("タスク停止に失敗");

    // 全てのタスクが終了していることを確認
    let current = tracker
        .store()
        .get_current_task()
        .expect("取得に失敗");
    assert!(current.is_none());

    // 4. サマリーを取得
    let summary = tracker
        .get_today_summary()
        .expect("サマリー取得に失敗");

    // 2つのタスクが集計されていることを確認
    assert_eq!(summary.len(), 2);
    assert!(summary.contains_key("プログラミング"));
    assert!(summary.contains_key("会議"));

    // 時間が正しく計測されていることを確認（0より大きい）
    assert!(summary.get("プログラミング").unwrap().as_secs() > 0 || summary.get("プログラミング").unwrap().as_millis() > 0);
    assert!(summary.get("会議").unwrap().as_secs() > 0 || summary.get("会議").unwrap().as_millis() > 0);
}

/// 同じタスク名の集計テスト
#[test]
fn test_same_task_aggregation() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("aggregation.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    // 同じタスクを複数回実行
    tracker
        .start_task("コーディング".to_string(), None)
        .expect("失敗");
    std::thread::sleep(std::time::Duration::from_millis(100));

    tracker
        .start_task("休憩".to_string(), None)
        .expect("失敗");
    std::thread::sleep(std::time::Duration::from_millis(100));

    tracker
        .start_task("コーディング".to_string(), None)
        .expect("失敗");
    std::thread::sleep(std::time::Duration::from_millis(100));

    tracker.stop_task().expect("失敗");

    // サマリーを取得
    let summary = tracker
        .get_today_summary()
        .expect("サマリー取得に失敗");

    // 集計されて2つのタスクになっていることを確認
    assert_eq!(summary.len(), 2);

    // コーディングの時間が2回分合計されていることを確認
    let coding_duration = summary.get("コーディング").unwrap();
    let break_duration = summary.get("休憩").unwrap();

    // 時間が正しく記録されていることを確認
    assert!(coding_duration.as_secs() > 0 || coding_duration.as_millis() > 0);
    assert!(break_duration.as_secs() > 0 || break_duration.as_millis() > 0);
}

/// 空のデータからの開始テスト
#[test]
fn test_start_from_empty() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("empty.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    // 何もない状態からstatus相当の操作
    let current = tracker
        .store()
        .get_current_task()
        .expect("取得に失敗");
    assert!(current.is_none());

    let summary = tracker
        .get_today_summary()
        .expect("サマリー取得に失敗");
    assert!(summary.is_empty());

    // タスクを開始
    tracker
        .start_task("初回タスク".to_string(), None)
        .expect("失敗");

    let current = tracker
        .store()
        .get_current_task()
        .expect("取得に失敗");
    assert!(current.is_some());
}

/// 進行中タスクがない状態でstopを呼ぶテスト
#[test]
fn test_stop_without_active_task() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("no_active.json");
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    let result = tracker.stop_task();
    assert!(result.is_err());
}

/// データの永続化テスト
#[test]
fn test_data_persistence() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("persistence.json");

    // 最初のセッション
    {
        let store = DataStore::new(data_file.clone());
        let tracker = Tracker::new(store);

        tracker
            .start_task("タスクA".to_string(), None)
            .expect("失敗");
        tracker.stop_task().expect("失敗");
    }

    // 2回目のセッション（データが永続化されているはず）
    {
        let store = DataStore::new(data_file.clone());
        let tracker = Tracker::new(store);

        let summary = tracker
            .get_today_summary()
            .expect("サマリー取得に失敗");

        assert_eq!(summary.len(), 1);
        assert!(summary.contains_key("タスクA"));
    }
}

/// 今日のエントリフィルタリングテスト
#[test]
fn test_today_filtering() {
    let dir = tempdir().expect("一時ディレクトリの作成に失敗");
    let data_file = dir.path().join("filtering.json");
    let store = DataStore::new(data_file);

    // 手動で昨日と今日のエントリを作成
    let now = Local::now();
    let today_start = now.date_naive().and_hms_opt(10, 0, 0).unwrap();
    let today_end = now.date_naive().and_hms_opt(11, 0, 0).unwrap();

    let yesterday = now.date_naive().pred_opt().unwrap().and_hms_opt(10, 0, 0).unwrap();

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

    let tracker = Tracker::new(store);
    let summary = tracker
        .get_today_summary()
        .expect("サマリー取得に失敗");

    // 今日のタスクのみが含まれていることを確認
    assert_eq!(summary.len(), 1);
    assert!(summary.contains_key("今日のタスク"));
    assert!(!summary.contains_key("昨日のタスク"));
}
