# time-checker 設計書

## 1. プロジェクト概要

### 1.1 目的
作業時間を簡単に記録・管理できるコマンドラインツール。
作業開始時と終了時にコマンドを実行するだけで、タスクごとの作業時間を自動的に記録し、日次サマリーを表示できる。

### 1.2 想定ユースケース
- 日報作成のための作業時間記録
- タスクごとの時間管理
- 複数のタスクを行き来する際の時間集計
- 記録したデータをSlackなどに報告

### 1.3 技術スタック
- **言語**: Rust (Edition 2024)
- **主要クレート**:
  - `clap 4.5` - コマンドライン引数解析（derive API使用）
  - `clap_complete 4.5` - タブ補完機能
  - `serde 1.0` - データのシリアライズ/デシリアライズ
  - `serde_json 1.0` - データファイルの保存（JSON形式）
  - `toml 0.8` - 設定ファイルの読み書き（TOML形式）
  - `chrono 0.4` - 日時処理
  - `anyhow 1.0` - エラーハンドリング
  - `dirs 5.0` - クロスプラットフォームのディレクトリパス取得

---

## 2. 機能仕様（MVP）

### 2.1 実装する機能

#### 2.1.1 タスク開始 (`start`)
```bash
time-checker start <タスク名>
```
- 新しいタスクを開始し、開始時刻を記録
- **すでに進行中のタスクがある場合は、自動的に終了させてから**新しいタスクを開始
- 前のタスクが終了した場合は、その作業時間も表示
- 出力: タスク開始の確認メッセージ（前のタスクがあれば終了メッセージも）

**例（初回）:**
```bash
$ time-checker start タスクの洗い出し
✓ タスク開始: タスクの洗い出し (09:00)
```

**例（タスク切り替え）:**
```bash
$ time-checker start 実装作業
✓ タスク終了: タスクの洗い出し (作業時間: 2.50時間)
✓ タスク開始: 実装作業 (11:30)
```

#### 2.1.2 1日の終了 (`stop`)
```bash
time-checker stop
```
- 現在進行中のタスクを終了し、終了時刻を記録
- 終了時に今日の作業サマリーを表示
- 進行中のタスクがない場合はエラーメッセージを表示
- 1日の作業終了時に使用

**例:**
```bash
$ time-checker stop
✓ タスク終了: 実装作業 (作業時間: 4.00時間)

【本日の作業時間】2025-11-04

タスクの洗い出し: 2.50時間
実装作業: 4.00時間

合計: 6.50時間
```

#### 2.1.3 現在の状態確認 (`status`)
```bash
time-checker status
```
- 現在進行中のタスクと経過時間を表示
- 今日の作業サマリーも表示
- 進行中のタスクがない場合は、今日の作業サマリーのみ表示

**例（進行中のタスクあり）:**
```bash
$ time-checker status
進行中: 実装作業 (開始: 11:30, 経過: 3.50時間)

今日の作業時間:
タスクの洗い出し: 2.50時間
実装作業: 3.50時間（進行中）
合計: 6.00時間
```

**例（進行中のタスクなし）:**
```bash
$ time-checker status
進行中のタスクはありません

今日の作業時間:
タスクの洗い出し: 2.50時間
実装作業: 4.00時間
合計: 6.50時間
```

#### 2.1.4 サマリー表示 (`show`)
```bash
time-checker show [period]
```
- 指定期間のタスクと作業時間を表示
- `period`はオプション引数（省略時は`today`）
- 同名のタスクは自動的に時間を集計
- 合計作業時間も表示
- 出力はpbcopyへのパイプを想定したシンプルなテキスト形式

**例（デフォルト - 今日）:**
```bash
$ time-checker show
【本日の作業時間】2025-11-04

タスクの洗い出し: 2.50時間
実装作業: 4.00時間
コードレビュー: 1.25時間

合計: 7.75時間
```

**例（明示的に今日）:**
```bash
$ time-checker show today
【本日の作業時間】2025-11-04
...
```

**将来の拡張（Phase 3）:**
```bash
$ time-checker show week       # 今週のサマリー
$ time-checker show month      # 今月のサマリー
$ time-checker show 2025-11-05 # 特定日のサマリー
```

#### 2.1.5 タブ補完
- 過去に記録したタスク名からの補完機能
- 各シェル（bash, zsh, fish, PowerShell）に対応
- タブキーで補完候補を表示

**セットアップ例 (zsh):**
```bash
# 補完スクリプトを生成
time-checker completion zsh > ~/.zsh/completions/_time-checker

# .zshrcに追加
fpath=(~/.zsh/completions $fpath)
autoload -Uz compinit && compinit
```

### 2.2 ワークフロー例

**典型的な1日の使い方:**

```bash
# 朝、最初のタスクを開始
$ time-checker start タスクの洗い出し
✓ タスク開始: タスクの洗い出し (09:00)

# タスク切り替え（前のタスクは自動終了）
$ time-checker start 実装作業
✓ タスク終了: タスクの洗い出し (作業時間: 2.50時間)
✓ タスク開始: 実装作業 (11:30)

# 現在の状態を確認
$ time-checker status
進行中: 実装作業 (開始: 11:30, 経過: 3.50時間)

今日の作業時間:
タスクの洗い出し: 2.50時間
実装作業: 3.50時間（進行中）
合計: 6.00時間

# 別のタスクに切り替え
$ time-checker start コードレビュー
✓ タスク終了: 実装作業 (作業時間: 4.00時間)
✓ タスク開始: コードレビュー (15:30)

# 1日の終わりに終了
$ time-checker stop
✓ タスク終了: コードレビュー (作業時間: 1.25時間)

【本日の作業時間】2025-11-04

タスクの洗い出し: 2.50時間
実装作業: 4.00時間
コードレビュー: 1.25時間

合計: 7.75時間

# 報告用にコピー
$ time-checker show | pbcopy
```

### 2.2 同名タスクの集計機能
- 同じ名前のタスクを複数回実行した場合、サマリー表示時に自動的に時間を合計
- 例: A → B → A と作業した場合、Aの作業時間は合計される

### 2.3 休憩・中断の扱い

#### 基本方針: 休憩もタスクとして記録

休憩や中断も通常のタスクとして扱います。これにより：
- ✅ シンプルな設計（追加のコマンド不要）
- ✅ 1日の時間の使い方を完全に記録
- ✅ 休憩時間も可視化できる

**使用例:**
```bash
$ time-checker start 実装作業
✓ タスク開始: 実装作業 (10:00)

$ time-checker start 昼休憩
✓ タスク終了: 実装作業 (作業時間: 2.00時間)
✓ タスク開始: 昼休憩 (12:00)

$ time-checker start 実装作業
✓ タスク終了: 昼休憩 (作業時間: 1.00時間)
✓ タスク開始: 実装作業 (13:00)

$ time-checker show today
【本日の作業時間】2025-11-05

実装作業: 6.00時間
昼休憩: 1.00時間

合計: 7.00時間
```

#### 除外機能（将来の拡張）

特定のタスク（休憩など）を集計から除外する機能を提供：

**設定ファイル（config.toml）:**
```toml
[filters]
# 集計から除外するタスク名のリスト
exclude_tasks = ["昼休憩", "コーヒーブレイク", "休憩"]
```

**出力例（除外機能あり）:**
```bash
$ time-checker show today
【本日の作業時間】2025-11-05

実装作業: 6.00時間
コードレビュー: 1.50時間
小計: 7.50時間

--- 除外されたタスク ---
昼休憩: 1.00時間
コーヒーブレイク: 0.25時間

合計（除外含む）: 8.75時間
```

**コマンドラインオプション:**
```bash
# 一時的に特定のタスクを除外
time-checker show today --exclude "昼休憩,打ち合わせ"

# 除外設定を無視してすべて表示
time-checker show today --no-exclude
```

**メリット:**
- 作業時間と休憩時間を分けて把握できる
- 何が除外されたか透明性がある
- 柔軟な集計が可能

---

## 3. データ設計

### 3.1 データ構造

#### TimeEntry構造体
```rust
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimeEntry {
    /// タスク名
    pub task: String,

    /// 開始時刻
    pub start: DateTime<Local>,

    /// 終了時刻（進行中の場合はNone）
    pub end: Option<DateTime<Local>>,

    /// 備考・メモ（オプション）
    /// JSONはコメント非対応のため、noteフィールドで代替
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
```

### 3.2 ファイル保存

#### 保存場所
- **デフォルト**: `~/.local/share/time-checker/` (Linux/macOS)
  - Linux: `$HOME/.local/share/time-checker/`
  - macOS: `$HOME/Library/Application Support/time-checker/`
  - Windows: `%APPDATA%\time-checker\`
- **環境変数での変更**: `TIME_CHECKER_DATA_DIR`で保存場所を変更可能

#### ファイル構成
```
~/.local/share/time-checker/
├── data.json          # すべてのタスク記録（JSON形式）
└── config.toml        # 設定ファイル（TOML形式、将来の拡張用）
```

#### データ形式の選定理由

**ハイブリッドアプローチ: JSON（データ） + TOML（設定）**

**data.json (JSON形式):**
- **パフォーマンス最優先**: 毎回のコマンド実行で読み書きが発生するため、最速のテキスト形式を採用
- **エコシステムとの互換性**: `jq`での検索、他ツールとの連携が容易
- **Rustでの最高のサポート**: `serde_json`は最も成熟したクレート
- **コメント問題の対処**: `note`フィールドでコメント代替

**config.toml (TOML形式):**
- **Rustエコシステムの標準**: Cargo.tomlと同じ形式で親しみやすい
- **コメントサポート**: 設定の説明を記載可能
- **人間による編集を想定**: 明確な構造で編集しやすい

#### data.jsonの形式
```json
[
  {
    "task": "タスクの洗い出し",
    "start": "2025-11-04T13:45:00+09:00",
    "end": "2025-11-04T14:00:00+09:00",
    "note": "初期設計フェーズ完了"
  },
  {
    "task": "実装作業",
    "start": "2025-11-04T14:15:00+09:00",
    "end": "2025-11-04T17:30:00+09:00"
  },
  {
    "task": "タスクの洗い出し",
    "start": "2025-11-04T17:45:00+09:00",
    "end": null
  }
]
```

### 3.3 データアクセス

#### DataStore構造体
```rust
pub struct DataStore {
    file_path: PathBuf,
}

impl DataStore {
    /// 新しいDataStoreインスタンスを作成
    pub fn new() -> Result<Self>;

    /// すべてのエントリを読み込む
    pub fn load(&self) -> Result<Vec<TimeEntry>>;

    /// エントリを保存
    pub fn save(&self, entries: &[TimeEntry]) -> Result<()>;

    /// 新しいエントリを追加
    pub fn add_entry(&self, entry: TimeEntry) -> Result<()>;

    /// 現在進行中のタスクを取得
    pub fn get_current_task(&self) -> Result<Option<TimeEntry>>;

    /// 指定日のエントリを取得
    pub fn get_entries_by_date(&self, date: NaiveDate) -> Result<Vec<TimeEntry>>;
}
```

---

## 4. プロジェクト構造

```
time-checker/
├── Cargo.toml              # プロジェクト設定
├── Cargo.lock
├── README.md               # 使い方ドキュメント
├── design.md               # この設計書
├── LICENSE                 # ライセンス（MIT or Apache-2.0）
├── .gitignore
├── src/
│   ├── main.rs            # エントリーポイント
│   ├── cli.rs             # CLIコマンド定義（clap）
│   ├── data.rs            # データ保存・読み込み（DataStore）
│   ├── tracker.rs         # コアロジック（Tracker）
│   ├── completion.rs      # タブ補完機能
│   └── error.rs           # カスタムエラー型
└── tests/
    ├── integration_test.rs
    └── fixtures/
```

### モジュール設計

#### main.rs
- CLIのエントリーポイント
- コマンドのルーティング

#### cli.rs
- `clap`を使用したコマンド定義
- サブコマンド（start, stop, status, show, completion）の構造体

#### data.rs
- `DataStore`構造体の実装
- JSONファイルの読み書き
- データディレクトリの初期化

#### tracker.rs
- `Tracker`構造体の実装
- タスクの開始・終了ロジック
- サマリー生成ロジック
- 時間計算

#### completion.rs
- タブ補完スクリプトの生成
- 過去のタスク名取得

#### error.rs
- カスタムエラー型の定義
- エラーメッセージの一元管理

---

## 5. コマンド仕様詳細

### 5.1 time-checker start <タスク名>

**引数:**
- `<タスク名>`: 必須。タスクの名前（文字列）

**動作:**
1. 現在進行中のタスクがあるかチェック
2. 進行中のタスクがあれば自動的に終了し、作業時間を表示
3. 新しいタスクを開始し、現在時刻を記録
4. データファイルに保存

**出力（初回起動時）:**
```
✓ タスク開始: <タスク名> (HH:MM)
```

**出力（タスク切り替え時）:**
```
✓ タスク終了: <前のタスク名> (作業時間: X.XX時間)
✓ タスク開始: <タスク名> (HH:MM)
```

**エラーケース:**
- データファイルの読み書きに失敗した場合

---

### 5.2 time-checker stop

**引数:** なし

**動作:**
1. 現在進行中のタスクを取得
2. 終了時刻を記録し、作業時間を計算
3. データファイルに保存
4. 今日の作業サマリーを表示

**出力:**
```
✓ タスク終了: <タスク名> (作業時間: X.XX時間)

【本日の作業時間】YYYY-MM-DD

タスクA: X.XX時間
タスクB: X.XX時間
タスクC: X.XX時間

合計: X.XX時間
```

**エラーケース:**
- 進行中のタスクがない場合: `エラー: 進行中のタスクがありません`
- データファイルの読み書きに失敗した場合

---

### 5.3 time-checker status

**引数:** なし

**動作:**
1. 現在進行中のタスクを取得
2. 進行中のタスクがあれば、経過時間を計算して表示
3. 今日の作業サマリーを表示（進行中のタスクも含む）

**出力（進行中のタスクあり）:**
```
進行中: <タスク名> (開始: HH:MM, 経過: X.XX時間)

今日の作業時間:
タスクA: X.XX時間
タスクB: X.XX時間（進行中）
合計: X.XX時間
```

**出力（進行中のタスクなし）:**
```
進行中のタスクはありません

今日の作業時間:
タスクA: X.XX時間
タスクB: X.XX時間
合計: X.XX時間
```

**エラーケース:**
- データファイルの読み込みに失敗した場合

---

### 5.4 time-checker show [period]

**引数:**
- `[period]`: オプション。表示する期間（デフォルト: `today`）

**MVP（Phase 1）でサポートする期間:**
- `today` または省略: 今日のサマリー

**Phase 3で追加予定の期間:**
- `yesterday`: 昨日
- `week`: 今週
- `last-week`: 先週
- `month`: 今月
- `last-month`: 先月
- `YYYY-MM-DD`: 特定日（例: `2025-11-05`）
- `YYYY-MM-DD..YYYY-MM-DD`: 期間範囲（例: `2025-11-01..2025-11-30`）

**動作:**
1. 指定期間のタスクをすべて取得
2. 同名タスクの時間を集計
3. 各タスクと合計時間を計算
4. フォーマットして表示

**出力（今日）:**
```
【本日の作業時間】YYYY-MM-DD

タスクA: X.XX時間
タスクB: X.XX時間
タスクC: X.XX時間

合計: X.XX時間
```

**出力（今週 - Phase 3）:**
```
【今週の作業時間】YYYY-MM-DD 〜 YYYY-MM-DD

タスクA: X.XX時間
タスクB: X.XX時間
タスクC: X.XX時間

合計: X.XX時間
```

**注意:**
- 進行中のタスクは含まれない（確定した作業時間のみ）
- `status`コマンドとの違い: 進行中のタスク情報なし、pbcopyへのパイプ想定

**エラーケース:**
- 指定期間のタスクがない場合: `指定期間の作業記録はありません`
- 無効な期間指定の場合: `無効な期間指定です: <period>`
- データファイルの読み込みに失敗した場合

**MVPでのエラー例:**
```bash
$ time-checker show week
エラー: 無効な期間指定です: week
現在サポートされている期間: today（または省略）

将来のバージョンで week, month などが使えるようになります。
```

---

### 5.5 time-checker completion <シェル>

**引数:**
- `<シェル>`: bash, zsh, fish, powershell のいずれか

**動作:**
1. 指定されたシェル用の補完スクリプトを生成
2. 標準出力に出力

**出力:**
- 補完スクリプト（シェル形式）

**使用例:**
```bash
time-checker completion zsh > ~/.zsh/completions/_time-checker
```

---

## 6. タブ補完の実装

### 6.1 実装方針
- `clap_complete`クレートを使用
- 過去のタスク名から補完候補を生成
- 各シェルに対応した補完スクリプトを生成

### 6.2 補完候補の取得
```rust
pub fn get_task_suggestions() -> Vec<String> {
    let data_store = DataStore::new().ok()?;
    let entries = data_store.load().ok()?;

    // 重複を除いて過去のタスク名を取得
    let mut tasks: HashSet<String> = entries
        .iter()
        .map(|e| e.task.clone())
        .collect();

    let mut task_list: Vec<String> = tasks.into_iter().collect();
    task_list.sort();
    task_list
}
```

### 6.3 セットアップ手順

#### Zsh
```bash
time-checker completion zsh > ~/.zsh/completions/_time-checker
# .zshrcに追加
fpath=(~/.zsh/completions $fpath)
autoload -Uz compinit && compinit
```

#### Bash
```bash
time-checker completion bash > ~/.bash_completions/time-checker
# .bashrcに追加
source ~/.bash_completions/time-checker
```

#### Fish
```bash
time-checker completion fish > ~/.config/fish/completions/time-checker.fish
```

---

## 7. Cargo.toml

```toml
[package]
name = "time-checker"
version = "0.1.0"
edition = "2024"
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"
description = "A simple CLI tool for tracking work time"
repository = "https://github.com/yourusername/time-checker"
readme = "README.md"
keywords = ["cli", "time-tracking", "productivity"]
categories = ["command-line-utilities"]

[[bin]]
name = "time-checker"
path = "src/main.rs"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
clap_complete = "4.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"          # データファイル用（JSON形式）
toml = "0.8"                # 設定ファイル用（TOML形式）
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
dirs = "5.0"

[profile.release]
opt-level = "z"      # サイズ最適化
lto = true           # Link Time Optimization
codegen-units = 1    # 最適化の向上
strip = true         # デバッグシンボルを削除
```

---

## 8. 配布方法

### 8.1 初期段階: cargo install
```bash
cargo install time-checker
```

**メリット:**
- 実装が簡単
- Rustユーザーには最も自然
- 自動的にバージョン管理

**必要な作業:**
- crates.ioへの公開
- Cargo.tomlのメタデータ設定

### 8.2 将来: バイナリ配布
- GitHub Releasesでプラットフォームごとのバイナリを配布
- GitHub Actionsで自動ビルド
- macOS (Intel/Apple Silicon)、Linux、Windows対応

---

## 9. エラーハンドリング

### 9.1 エラー種別
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimeCheckerError {
    #[error("データファイルの読み込みに失敗しました: {0}")]
    DataLoadError(#[from] std::io::Error),

    #[error("JSONのパースに失敗しました: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("進行中のタスクがありません")]
    NoActiveTask,

    #[error("データディレクトリの作成に失敗しました")]
    DirectoryCreationError,
}
```

### 9.2 ユーザーへのエラー表示
- わかりやすい日本語のエラーメッセージ
- 解決方法のヒントを含める
- 技術的な詳細はverboseモード時のみ表示

---

## 10. 今後の拡張機能（将来実装予定）

### 10.1 表示機能の拡張

#### 予約語の追加
```bash
time-checker show yesterday     # 昨日のサマリー
time-checker show week          # 今週のサマリー
time-checker show last-week     # 先週のサマリー
time-checker show month         # 今月のサマリー
time-checker show last-month    # 先月のサマリー
```

#### 日付指定
```bash
time-checker show 2025-11-05    # 特定日のサマリー
```

#### 期間範囲指定
```bash
# 位置引数方式（Git風）
time-checker show 2025-11-01..2025-11-30

# オプションフラグ方式（より明示的）
time-checker show --from 2025-11-01 --to 2025-11-30
time-checker show --since "1 week ago"
```

#### 実装の考慮事項
- 期間パース処理（`parse_period`関数）
- 予約語の管理（拡張可能な設計）
- 日付形式の検証（`YYYY-MM-DD`）
- 範囲計算ロジック（週、月の開始・終了日）

### 10.2 データ管理機能
- `time-checker list` - すべてのタスク一覧
- `time-checker edit` - タスクの編集
- `time-checker delete` - タスクの削除
- `time-checker export` - CSVやJSON形式でのエクスポート

### 10.3 レポート機能
- `time-checker report --format json` - JSON形式での出力
- `time-checker report --format csv` - CSV形式での出力
- `time-checker report --format markdown` - Markdown形式での出力

### 10.4 タスク除外機能

特定のタスクを集計から除外する機能：

#### 設定ファイルでの除外

**config.toml:**
```toml
[filters]
# 集計から除外するタスク名のリスト
exclude_tasks = ["昼休憩", "コーヒーブレイク", "休憩", "打ち合わせ"]
```

#### コマンドラインオプション

```bash
# 一時的に特定のタスクを除外
time-checker show today --exclude "昼休憩,打ち合わせ"

# 除外設定を無視してすべて表示
time-checker show today --no-exclude
```

#### 出力形式

除外されたタスクも表示して透明性を保つ：

```
【本日の作業時間】2025-11-05

実装作業: 6.00時間
コードレビュー: 1.50時間
小計: 7.50時間

--- 除外されたタスク ---
昼休憩: 1.00時間
コーヒーブレイク: 0.25時間

合計（除外含む）: 8.75時間
```

#### 実装の考慮事項

- 除外タスクのパターンマッチング（完全一致、前方一致、正規表現など）
- 除外設定の優先順位（コマンドライン > 設定ファイル）
- `status`コマンドでも除外機能を適用

### 10.5 設定機能
- `time-checker config set <key> <value>` - 設定の変更
- デフォルトのフォーマット設定
- タイムゾーン設定
- 除外タスクの管理

### 10.6 その他
- タスクへのタグ付け機能
- プロジェクトごとの分類
- グラフ表示（ターミナルグラフ）
- Git連携（自動でブランチ名をタスク名として使用）

---

## 11. 実装の優先順位

### Phase 1: MVP（最小機能製品）
1. データ構造の定義（TimeEntry構造体）
2. DataStoreの実装（JSON読み書き）
3. `start`コマンド（自動終了機能付き）
4. `stop`コマンド（サマリー表示付き）
5. `status`コマンド（進行中タスクと今日のサマリー）
6. `show`コマンド（period引数: デフォルト`today`、pbcopy想定）
7. 同名タスクの集計機能
8. タブ補完機能
9. 基本的なエラーハンドリング

### Phase 2: 改善
1. テストの追加（ユニットテスト、統合テスト）
2. ドキュメントの充実（README、使い方ガイド）
3. CI/CDの設定（GitHub Actions）
4. バイナリ配布の準備

### Phase 3: 拡張機能
1. `show`コマンドの拡張（week, month, yesterday, 日付指定、範囲指定）
2. タスク除外機能（設定ファイル、コマンドラインオプション）
3. エクスポート機能（CSV、JSON、Markdown）
4. 設定機能（config set/get）
5. データ管理機能（edit, delete）

---

## 12. 開発環境

### 12.1 必要なツール
- Rust 1.85.0以上（Edition 2024をサポート）
- Cargo

### 12.2 開発コマンド
```bash
# ビルド
cargo build

# 実行
cargo run -- start "テストタスク"

# テスト
cargo test

# リリースビルド
cargo build --release

# フォーマット
cargo fmt

# Lint
cargo clippy
```

---

## 13. まとめ

time-checkerは、作業時間を簡単に記録できるシンプルなコマンドラインツールです。
MVPでは以下の機能を実装します：

- タスクの開始・終了記録
- 今日の作業サマリー表示（同名タスクの集計付き）
- 過去のタスク名からのタブ補完

Rustで実装することで、配布や環境移行が容易なシングルバイナリを提供できます。
将来的には、週次・月次のレポート機能や、エクスポート機能などの拡張も予定しています。

---

## 14. 技術選定の詳細

### 14.1 Rust Edition 2024を選択した理由

#### Edition 2024の概要
- **リリース日**: 2025年2月20日（Rust 1.85.0と共に）
- **名称**: 2024年に計画されたため「Edition 2024」
- **サイクル**: 3年ごとのリリース（2015 → 2018 → 2021 → 2024）

#### Edition 2024の主な改善点

**1. 一時変数のライフタイム管理の改善**
```rust
// Edition 2021: if文全体でロックが保持され、デッドロックの原因に
if lock.read().unwrap().is_some() {
    // ...
} else {
    lock.write().unwrap(); // デッドロック！
}

// Edition 2024: 一時スコープでロックが解放され、安全に動作
```

この改善は、time-checkerのようなファイルI/Oが頻繁に発生するCLIツールで特に有益です。

**2. Unsafe Rustの改善**
- `env::set_var`や`env::get_var`がunsafeとしてマーク
- マルチスレッド環境での安全性向上

**3. 新しい予約キーワード `gen`**
- 将来のジェネレータ/コルーチン機能に備えた予約語

**4. `impl Trait`でのライフタイム捕捉の改善**
- より直感的な型推論

#### 新規プロジェクトでEdition 2024を採用すべき理由

1. **最新の言語機能**: 今後3年間は最新のEdition
2. **安全性の向上**: デッドロック回避、unsafe関数の明示化
3. **将来への投資**: 新機能へのアクセスが容易
4. **移行の容易さ**: `cargo fix --edition`で自動移行可能
5. **Edition間の互換性**: 異なるEditionのクレートを混在可能

### 14.2 データ形式の詳細比較

#### 実測ベンチマーク結果

信頼性の高いベンチマークプロジェクト（Canop/bench-config-deserializers）から得られた実測データ：

| 形式 | デシリアライズ時間 | スループット | JSON比 |
|------|-------------------|-------------|---------|
| **serde_json** | 40.8ms | 506 MB/s | 基準（1.0x） |
| **basic-toml** | 361.7ms | 39 MB/s | **8.9倍遅い** |
| **toml** | 466.5ms | 31 MB/s | **11.4倍遅い** |

**重要**: TOMLはJSONと比較して約**9-11倍遅い**ことが実証されています。

#### time-checkerでの実際の影響

想定データ量と処理時間の予測：

| エントリ数 | 期間 | JSON<br>サイズ | TOML<br>サイズ | JSON<br>時間 | TOML<br>時間 | 差分 | 体感 |
|-----------|------|--------------|--------------|------------|------------|------|------|
| 10 | 1日 | 1.5-2 KB | 2-3 KB | 0.04ms | 0.36ms | 0.32ms | ✅ 知覚不可 |
| 100 | 2週間 | 15-20 KB | 20-30 KB | 0.4ms | 3.6ms | 3.2ms | ✅ 知覚不可 |
| 300 | 1ヶ月 | 45-60 KB | 60-90 KB | 1.2ms | 10.8ms | 9.6ms | ✅ 知覚困難 |
| 500 | 1.5ヶ月 | 75-100 KB | 100-150 KB | 2.0ms | 18.0ms | 16.0ms | ✅ 知覚困難 |
| **1000** | **3ヶ月** | **150-200 KB** | **200-300 KB** | **4.0ms** | **36.0ms** | **32.0ms** | ⚠️ **わずかに感じる** |
| **3600** | **1年** | **540-720 KB** | **720-1080 KB** | **14.4ms** | **129.6ms** | **115.2ms** | ❌ **明確に遅い** |
| **7200** | **2年** | **1.08-1.44 MB** | **1.44-2.16 MB** | **28.8ms** | **259.2ms** | **230.4ms** | ❌ **非常に遅い** |

**注**: `start`コマンドは読み込み+書き込みを行うため、実際の処理時間は上記の2倍になります。

#### 人間の知覚閾値

UX研究（Jakob Nielsen等）による標準的な閾値：

| 閾値 | 体感 | 説明 |
|------|------|------|
| **< 13ms** | 検出不可能 | 人間の視覚的知覚限界 |
| **< 50ms** | 即座の反応 | 完璧な応答性、特別なフィードバック不要 |
| **< 100ms** | 瞬時の反応 | 非常に速い |
| **100-400ms** | わずかな遅延 | 許容範囲だが感知可能 |
| **> 400ms** | 明確な遅延 | ユーザーが「効かなかったかも」と感じる |

#### なぜハイブリッドアプローチを選択したか

**データファイル（data.json）にJSONを選択:**

1. **長期使用でのパフォーマンス**
   - 1000エントリ以上で体感可能な遅延（32ms以上の差）
   - 3600エントリ（1年分）で115ms以上の差 → 明確に遅いと感じる
   - `start`/`end`コマンドは頻繁に実行されるため、遅延が蓄積

2. **エコシステムとの親和性**
   - `jq`での柔軟な検索・フィルタ
   ```bash
   # 特定タスクの検索
   cat data.json | jq '.[] | select(.task == "実装作業")'

   # 今週の合計時間を計算
   cat data.json | jq '[.[] | select(.start > "2025-11-01")] | length'
   ```
   - 他のツール（SlackのAPI等）との連携が容易
   - 標準的なデータ交換形式

3. **Rustでの最高のサポート**
   - `serde_json`: 月3000万以上のダウンロード
   - 非常に高速で安定している
   - エラーメッセージも優れている

4. **業界標準**
   - TimeWarrior、TaskWarriorなど主要な時間管理ツールがJSON採用
   - データファイルは機械的な読み書きが主目的

**設定ファイル（config.toml）にTOMLを選択:**

1. **Rustエコシステムの標準**
   - Cargo.tomlと同じ形式
   - Rustプログラマにとって自然

2. **コメントのサポート**
   - 設定の説明を記載可能
   ```toml
   # time-checker 設定ファイル

   [display]
   # 時間の表示形式（decimal: 小数、hm: 時間:分）
   time_format = "decimal"
   ```

3. **人間による編集を想定**
   - 明確な構造で編集しやすい
   - インデントに依存しないため、フォーマットエラーが少ない

4. **読み込み頻度が低い**
   - 起動時に1回読み込むだけ
   - パフォーマンスへの影響は軽微

**コメント問題への対処:**

JSONはコメントをサポートしないため、`note`フィールドを追加：
```json
{
  "task": "実装作業",
  "start": "2025-11-04T14:15:00+09:00",
  "end": "2025-11-04T17:30:00+09:00",
  "note": "ファイルI/O処理を実装。パフォーマンステストも完了。"
}
```

**メリット:**
- プログラムから直接アクセス可能
- 検索・フィルタリングが容易
- タイムラインと紐づいた記録

#### 代替案: すべてTOML

**「すべてTOMLにすべきか？」の検討**

**短期的使用（< 500エントリ）の場合:**
- ✅ 体感可能な遅延はほぼない（差は16ms以下）
- ✅ コメントが書ける
- ✅ 一貫したデータ形式
- ✅ 手動編集がしやすい

**長期的使用（> 1000エントリ）の場合:**
- ❌ 明確な遅延が発生（差は32ms以上）
- ❌ 1年後には100ms以上の遅延
- ❌ `jq`などのツールが使えない
- ❌ 他ツールとの連携が困難

**推奨: ハイブリッドアプローチを維持**

理由:
1. **将来性**: データが増えるほど差が拡大
2. **エコシステム**: 標準的なツールとの連携
3. **業界標準**: 時間管理ツールの一般的なアプローチ
4. **適材適所**: 各形式を最適なユースケースに適用

**すべてTOMLの例（参考）:**
```toml
# data.toml の例
[[entries]]
task = "設計書作成"
start = 2025-11-05T09:00:00+09:00
end = 2025-11-05T12:00:00+09:00
# 初期設計フェーズ完了

[[entries]]
task = "実装作業"
start = 2025-11-05T13:00:00+09:00
# 進行中
```

短期的なプロトタイプや個人用途で、データ量が少ない（< 500エントリ）ことが確実な場合は、すべてTOMLでも問題ありません。しかし、長期的な使用を想定する場合は、ハイブリッドアプローチが最適です。

### 14.3 最終的な技術選定のまとめ

| 項目 | 選定内容 | 理由 |
|------|----------|------|
| **Rust Edition** | 2024 | 最新の言語機能、安全性の向上、長期的なメンテナンス性 |
| **データファイル** | JSON | パフォーマンス、エコシステム、Rustサポート |
| **設定ファイル** | TOML | Rustの標準、コメント対応、人間が編集しやすい |
| **コメント対処** | noteフィールド | JSONの制約を補う |

この組み合わせにより、パフォーマンスと使いやすさを両立し、Rustエコシステムのベストプラクティスに従った設計になります。
