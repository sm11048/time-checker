// エントリーポイント

use clap::Parser;
use std::process;
use time_checker::cli::{Cli, Commands};
use time_checker::data::DataStore;
use time_checker::tracker::Tracker;

fn main() {
    // データファイルのパスを決定
    let data_dir = dirs::home_dir()
        .expect("ホームディレクトリの取得に失敗")
        .join(".time-checker");

    let data_file = data_dir.join("data.json");

    // DataStoreとTrackerを初期化
    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    // CLIコマンドをパース
    let cli = Cli::parse();

    // コマンドを実行
    let result = match cli.command {
        Commands::Start { task, note } => handle_start(&tracker, task, note),
        Commands::Stop => handle_stop(&tracker),
        Commands::Status => handle_status(&tracker),
        Commands::Show { period } => handle_show(&tracker, period),
    };

    // エラーハンドリング
    if let Err(e) = result {
        eprintln!("エラー: {}", e);
        process::exit(1);
    }
}

/// startコマンドの処理
fn handle_start(tracker: &Tracker, task: String, note: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    tracker.start_task(task.clone(), note)?;
    println!("タスクを開始しました: {}", task);
    Ok(())
}

/// stopコマンドの処理
fn handle_stop(tracker: &Tracker) -> Result<(), Box<dyn std::error::Error>> {
    tracker.stop_task()?;
    println!("タスクを停止しました");
    println!();

    // 今日のサマリーを表示
    display_summary(tracker)?;
    Ok(())
}

/// statusコマンドの処理
fn handle_status(tracker: &Tracker) -> Result<(), Box<dyn std::error::Error>> {
    // 現在のタスクを表示
    if let Some(current) = tracker.store().get_current_task()? {
        let elapsed = chrono::Local::now().signed_duration_since(current.start);
        let hours = elapsed.num_hours();
        let minutes = elapsed.num_minutes() % 60;

        println!("進行中のタスク: {}", current.task);
        println!("開始時刻: {}", current.start.format("%H:%M"));
        println!("経過時間: {}時間{}分", hours, minutes);

        if let Some(ref note) = current.note {
            println!("メモ: {}", note);
        }
    } else {
        println!("進行中のタスクはありません");
    }

    println!();

    // 今日のサマリーを表示
    display_summary(tracker)?;
    Ok(())
}

/// showコマンドの処理
fn handle_show(tracker: &Tracker, period: String) -> Result<(), Box<dyn std::error::Error>> {
    match period.as_str() {
        "today" => {
            display_summary(tracker)?;
            Ok(())
        }
        _ => {
            eprintln!("未対応の期間指定です: {}", period);
            eprintln!("現在対応している期間: today");
            Err("無効な期間指定".into())
        }
    }
}

/// サマリーを表示する共通関数
fn display_summary(tracker: &Tracker) -> Result<(), Box<dyn std::error::Error>> {
    let summary = tracker.get_today_summary()?;

    if summary.is_empty() {
        println!("今日の作業記録はありません");
        return Ok(());
    }

    println!("=== 今日の作業時間 ===");

    let mut tasks: Vec<_> = summary.iter().collect();
    tasks.sort_by_key(|(name, _)| *name);

    let mut total_seconds = 0u64;

    for (task, duration) in tasks {
        let seconds = duration.as_secs();
        total_seconds += seconds;

        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;

        println!("{}: {}時間{}分", task, hours, minutes);
    }

    println!();

    let total_hours = total_seconds / 3600;
    let total_minutes = (total_seconds % 3600) / 60;
    println!("合計: {}時間{}分", total_hours, total_minutes);

    Ok(())
}
