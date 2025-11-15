// CLIコマンド定義

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "time-checker")]
#[command(about = "シンプルな作業時間記録ツール", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// タスクを開始（進行中のタスクがあれば自動終了）
    Start {
        /// タスク名
        task: String,

        /// 備考・メモ（オプション）
        #[arg(short, long)]
        note: Option<String>,
    },

    /// 現在のタスクを停止して今日のサマリーを表示
    Stop,

    /// 現在のタスクと今日のサマリーを表示
    Status,

    /// 指定期間のサマリーを表示（デフォルトは今日）
    Show {
        /// 期間（省略時は今日）
        #[arg(default_value = "today")]
        period: String,
    },
}
