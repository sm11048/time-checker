# time-checker ソースコード解説（Rust初心者向け）

このドキュメントでは、time-checkerプロジェクトのソースコードをRust初心者の方が順番に読んで理解できるよう解説します。

---

## 目次

1. [プロジェクト構成の概要](#1-プロジェクト構成の概要)
2. [読む順番のおすすめ](#2-読む順番のおすすめ)
3. [Cargo.toml - プロジェクト設定](#3-cargotoml---プロジェクト設定)
4. [src/lib.rs - モジュール公開](#4-srclibrs---モジュール公開)
5. [src/error.rs - エラー型定義](#5-srcerrorrs---エラー型定義)
6. [src/cli.rs - コマンドライン定義](#6-srcclirs---コマンドライン定義)
7. [src/data.rs - データ構造と永続化](#7-srcdatars---データ構造と永続化)
8. [src/tracker.rs - ビジネスロジック](#8-srctrackerrs---ビジネスロジック)
9. [src/main.rs - エントリーポイント](#9-srcmainrs---エントリーポイント)
10. [よく使われるRustの文法まとめ](#10-よく使われるrustの文法まとめ)

---

## 1. プロジェクト構成の概要

```
time-checker/
├── Cargo.toml          # プロジェクト設定・依存関係
├── src/
│   ├── main.rs         # エントリーポイント（プログラムの開始地点）
│   ├── lib.rs          # ライブラリのルート（モジュール公開）
│   ├── cli.rs          # CLIコマンドの定義
│   ├── data.rs         # データ構造とファイル操作
│   ├── tracker.rs      # ビジネスロジック
│   ├── error.rs        # エラー型の定義
│   └── completion.rs   # タブ補完（未実装）
└── tests/              # 統合テスト
```

### ファイル間の依存関係

```
main.rs
  └─→ cli.rs      (コマンドライン引数の定義)
  └─→ data.rs     (データ保存)
  └─→ tracker.rs  (ビジネスロジック)
        └─→ data.rs
        └─→ error.rs
```

---

## 2. 読む順番のおすすめ

Rust初心者の方には、以下の順番で読むことをおすすめします：

1. **error.rs** - 最もシンプル。Rustの`enum`と`trait`実装の基礎が学べる
2. **cli.rs** - clapライブラリを使った構造体定義。derive属性の使い方
3. **data.rs** - ファイル操作、シリアライズ、`Option`の使い方
4. **tracker.rs** - ビジネスロジック、`HashMap`の操作
5. **lib.rs** - モジュールシステムの理解
6. **main.rs** - 全体の流れ、エラーハンドリング

---

## 3. Cargo.toml - プロジェクト設定

```toml
[package]
name = "time-checker"
version = "0.1.0"
edition = "2024"            # Rustのエディション（言語仕様のバージョン）
```

### 依存クレート（ライブラリ）

| クレート | 用途 |
|---------|------|
| `clap` | コマンドライン引数のパース |
| `serde` | データのシリアライズ/デシリアライズ |
| `serde_json` | JSON形式の変換 |
| `chrono` | 日時の操作 |
| `dirs` | ホームディレクトリの取得 |

**ポイント**: `features = ["derive"]` は、そのクレートの追加機能を有効にする設定です。

---

## 4. src/lib.rs - モジュール公開

```rust
// time-checker library
// ライブラリとしてモジュールを公開（テスト用）

pub mod cli;
pub mod data;
pub mod tracker;
pub mod completion;
pub mod error;
```

### 解説

- `pub mod xxx;` で「このモジュールを外部から使えるようにする」宣言
- `pub` = public（公開）の意味
- `mod` = module（モジュール）の略
- 各 `xxx.rs` ファイルが1つのモジュールになる

### なぜlib.rsが必要？

Rustでは、ライブラリ（lib.rs）とバイナリ（main.rs）を分けることで：
- テストからライブラリの機能をインポートできる
- コードの再利用性が高まる

---

## 5. src/error.rs - エラー型定義

```rust
// エラー型の定義
use std::fmt;

#[derive(Debug)]
pub enum TimeCheckerError {
    NoActiveTask,
    DataLoadError(String),
    DataSaveError(String),
    InvalidPeriod(String),
}
```

### 解説

#### `use std::fmt;`
- 標準ライブラリの `fmt`（フォーマット）モジュールを使う宣言
- 後で `Display` トレイトを実装するために必要

#### `#[derive(Debug)]`
- **derive属性**: コンパイラに「この型にDebugトレイトを自動実装して」と指示
- `Debug` トレイトがあると `{:?}` でデバッグ出力できる

#### `pub enum TimeCheckerError`
- `pub` = 公開（他のファイルから使える）
- `enum` = 列挙型（複数の種類のどれか1つを表す型）
- このアプリで起こりうるエラーを4種類定義

#### バリアントの種類
```rust
NoActiveTask,              // データを持たないバリアント
DataLoadError(String),     // String型のデータを持つバリアント
```

### Displayトレイトの実装

```rust
impl fmt::Display for TimeCheckerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeCheckerError::NoActiveTask => write!(f, "進行中のタスクがありません"),
            TimeCheckerError::DataLoadError(msg) => write!(f, "データの読み込みに失敗しました: {}", msg),
            // ...
        }
    }
}
```

#### `impl Trait for Type`
- 「Type に Trait を実装する」という意味
- `Display` トレイトを実装すると `{}` で出力できる

#### `match self`
- `self` = このエラー自体
- `match` = パターンマッチ（switch文の強力版）
- 全てのバリアントを処理しないとコンパイルエラー（安全！）

#### `write!(f, "...")`
- フォーマッタに文字列を書き込むマクロ
- `{}` の部分に変数が埋め込まれる

### std::error::Error の実装

```rust
impl std::error::Error for TimeCheckerError {}
```

- 空の実装でOK（デフォルト実装を使う）
- これを実装すると `Box<dyn std::error::Error>` として扱える
- Rustの標準的なエラー型として使えるようになる

---

## 6. src/cli.rs - コマンドライン定義

```rust
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "time-checker")]
#[command(about = "シンプルな作業時間記録ツール", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
```

### 解説

#### clapクレート
- コマンドライン引数を簡単にパースできるライブラリ
- derive機能で構造体からCLIを自動生成

#### `#[derive(Parser, Debug)]`
- `Parser` = clapがCLI定義として使う
- 複数のトレイトを同時にderiveできる

#### `#[command(...)]` 属性
- clapへの設定を指定する属性
- `name` = コマンド名
- `about` = ヘルプに表示される説明
- `version` = Cargo.tomlのバージョンを表示

#### 構造体フィールド
```rust
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
```
- `Commands` 型のフィールドを1つ持つ
- `#[command(subcommand)]` = サブコマンドとして扱う

### サブコマンドの定義

```rust
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
    Stop,
    Status,
    Show {
        #[arg(default_value = "today")]
        period: String,
    },
}
```

#### `/// コメント`
- ドキュメントコメント（3つのスラッシュ）
- clapはこれをヘルプメッセージとして使う

#### 名前付きフィールドを持つバリアント
```rust
Start {
    task: String,
    note: Option<String>,
}
```
- `Start` バリアントは2つのフィールドを持つ
- `Option<String>` = 値があるかもしれないし、ないかもしれない

#### `#[arg(...)]` 属性
- `short` = 短いオプション（`-n`）
- `long` = 長いオプション（`--note`）
- `default_value` = 省略時のデフォルト値

---

## 7. src/data.rs - データ構造と永続化

### TimeEntry構造体

```rust
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TimeEntry {
    pub task: String,
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
```

#### derive属性
- `Serialize` = この構造体をJSONなどに変換できる
- `Deserialize` = JSONなどからこの構造体を復元できる
- `Clone` = `.clone()` でコピーできる
- `PartialEq` = `==` で比較できる

#### `DateTime<Local>`
- chronoクレートの日時型
- `Local` = ローカルタイムゾーン

#### `#[serde(skip_serializing_if = "Option::is_none")]`
- JSONに書き出す時、`None` なら省略する
- 結果のJSONがすっきりする

### DataStore構造体

```rust
pub struct DataStore {
    data_file: PathBuf,
}
```

- `PathBuf` = ファイルパスを表す型（所有権あり版）
- `Path` との違い: `PathBuf` は `String`、`Path` は `&str` のような関係

### メソッドの実装

```rust
impl DataStore {
    pub fn new(data_file: PathBuf) -> Self {
        Self { data_file }
    }
```

#### `impl Type` ブロック
- 構造体にメソッドを追加する

#### `Self`
- 「この構造体自身の型」を指す
- `DataStore` と書いても同じだが、`Self` が慣用的

#### コンストラクタパターン
- Rustには `new` キーワードがない
- 代わりに `new()` という関数を作る慣習

### save メソッド

```rust
pub fn save(&self, entries: &[TimeEntry]) -> Result<(), TimeCheckerError> {
    // 親ディレクトリが存在しない場合は作成
    if let Some(parent) = self.data_file.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            TimeCheckerError::DataSaveError(format!("ディレクトリの作成に失敗: {}", e))
        })?;
    }
    // ...
}
```

#### `&self`
- このメソッドはDataStoreの参照を受け取る（所有権を奪わない）
- `self` だと所有権を奪って、メソッド後に使えなくなる

#### `&[TimeEntry]`
- `TimeEntry` のスライス（配列への参照）
- `Vec<TimeEntry>` も `&[]` として渡せる

#### `Result<(), TimeCheckerError>`
- `Ok(())` = 成功（返す値なし）
- `Err(TimeCheckerError)` = 失敗

#### `if let Some(x) = ...`
- `Option` から値を取り出すパターンマッチ
- `Some` の場合だけ中のコードが実行される

#### `.map_err(|e| {...})?`
- `.map_err()` = エラーを別の型に変換
- `|e|` = クロージャ（無名関数）の引数
- `?` = エラーなら早期リターン、成功なら値を取り出す

### load メソッド

```rust
pub fn load(&self) -> Result<Vec<TimeEntry>, TimeCheckerError> {
    if !self.data_file.exists() {
        return Ok(Vec::new());
    }
    // ...
}
```

#### `Vec::new()`
- 空のベクタ（可変長配列）を作成
- `vec![]` マクロでも同じ

### get_current_task メソッド

```rust
pub fn get_current_task(&self) -> Result<Option<TimeEntry>, TimeCheckerError> {
    let entries = self.load()?;
    Ok(entries.iter().rev().find(|e| e.end.is_none()).cloned())
}
```

#### メソッドチェーン
```rust
entries.iter()      // イテレータに変換
    .rev()          // 逆順にする
    .find(|e| ...)  // 条件に合う最初の要素を探す
    .cloned()       // 参照からCloneして所有権のある値に
```

#### `|e| e.end.is_none()`
- クロージャ（無名関数）
- `|引数| 式` の形式
- `is_none()` = `Option` が `None` かどうか

### get_today_entries メソッド

```rust
pub fn get_today_entries(&self) -> Result<Vec<TimeEntry>, TimeCheckerError> {
    let entries = self.load()?;
    let today = Local::now().date_naive();

    Ok(entries
        .into_iter()
        .filter(|e| e.start.date_naive() == today)
        .collect())
}
```

#### `into_iter()` vs `iter()`
- `iter()` = 要素への参照のイテレータ
- `into_iter()` = 要素の所有権を移動するイテレータ

#### `.filter(|e| ...)`
- 条件を満たす要素だけを残す

#### `.collect()`
- イテレータを `Vec` などのコレクションに変換
- 戻り値の型から自動的に何に変換するか決まる

---

## 8. src/tracker.rs - ビジネスロジック

### Tracker構造体

```rust
pub struct Tracker {
    store: DataStore,
}

impl Tracker {
    pub fn new(store: DataStore) -> Self {
        Self { store }
    }

    pub fn store(&self) -> &DataStore {
        &self.store
    }
```

#### 所有権のパターン
- `Tracker` が `DataStore` を所有（フィールドとして保持）
- `store()` メソッドは参照を返す（所有権は渡さない）

### start_task メソッド

```rust
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
```

#### `let mut entries`
- `mut` = 可変（変更可能）
- Rustではデフォルトで不変

#### `.iter_mut()`
- 要素を変更できるイテレータ
- `iter()` は読み取り専用

#### 構造体の初期化（省略記法）
```rust
TimeEntry {
    task,       // task: task と同じ（変数名とフィールド名が同じ場合）
    start: now,
    end: None,
    note,       // note: note と同じ
}
```

### get_today_summary メソッド

```rust
pub fn get_today_summary(&self) -> Result<HashMap<String, Duration>, TimeCheckerError> {
    let entries = self.store.get_today_entries()?;
    let mut summary: HashMap<String, Duration> = HashMap::new();

    for entry in entries {
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
```

#### `HashMap<String, Duration>`
- キーが `String`、値が `Duration` のハッシュマップ
- Pythonの辞書、JavaScriptのオブジェクトに相当

#### `.unwrap_or_else(Local::now)`
- `Some(x)` なら `x` を返す
- `None` なら `Local::now()` を実行してその結果を返す

#### `.entry().and_modify().or_insert()`
```rust
summary
    .entry(entry.task.clone())  // キーのエントリを取得
    .and_modify(|d| *d += std_duration)  // 既存なら値を変更
    .or_insert(std_duration);   // なければ挿入
```
- HashMapに値を追加・更新する慣用的なパターン

#### `*d += std_duration`
- `*` = 参照を外す（デリファレンス）
- `d` は `&mut Duration` なので、値を変更するには `*` が必要

#### `as u64`
- 型キャスト
- `i64` から `u64` への変換

---

## 9. src/main.rs - エントリーポイント

### main関数

```rust
use clap::Parser;
use std::process;
use time_checker::cli::{Cli, Commands};
use time_checker::data::DataStore;
use time_checker::tracker::Tracker;

fn main() {
    let data_dir = dirs::home_dir()
        .expect("ホームディレクトリの取得に失敗")
        .join(".time-checker");

    let data_file = data_dir.join("data.json");

    let store = DataStore::new(data_file);
    let tracker = Tracker::new(store);

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Start { task, note } => handle_start(&tracker, task, note),
        Commands::Stop => handle_stop(&tracker),
        Commands::Status => handle_status(&tracker),
        Commands::Show { period } => handle_show(&tracker, period),
    };

    if let Err(e) = result {
        eprintln!("エラー: {}", e);
        process::exit(1);
    }
}
```

#### `use time_checker::...`
- `time_checker` はこのプロジェクト自体（lib.rs）
- `::` でモジュール階層をたどる

#### `.expect("メッセージ")`
- `Option` や `Result` が失敗なら、メッセージと共にパニック（強制終了）
- `unwrap()` と似ているが、エラーメッセージを指定できる

#### `.join("...")`
- パスを連結する
- `/home/user` + `.time-checker` → `/home/user/.time-checker`

#### `Cli::parse()`
- コマンドライン引数をパースして `Cli` 構造体を作成
- clapが提供する関数

#### パターンマッチで分解
```rust
Commands::Start { task, note } => handle_start(&tracker, task, note),
```
- `Start` バリアントの場合、`task` と `note` を取り出す
- 取り出した値を `handle_start` に渡す

#### `eprintln!`
- 標準エラー出力に出力するマクロ
- `println!` は標準出力

### handle_status 関数

```rust
fn handle_status(tracker: &Tracker) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(current) = tracker.store().get_current_task()? {
        let elapsed = chrono::Local::now().signed_duration_since(current.start);
        let hours = elapsed.num_hours();
        let minutes = elapsed.num_minutes() % 60;

        println!("進行中のタスク: {}", current.task);
        // ...

        if let Some(ref note) = current.note {
            println!("メモ: {}", note);
        }
    } else {
        println!("進行中のタスクはありません");
    }
    // ...
}
```

#### `Box<dyn std::error::Error>`
- どんなエラー型でも入れられる「箱」
- `dyn` = 動的ディスパッチ（実行時に型が決まる）

#### `if let Some(ref note)`
- `ref` = 値を借用する（所有権を奪わない）
- `current` の所有権を保ったまま `note` を参照

### display_summary 関数

```rust
fn display_summary(tracker: &Tracker) -> Result<(), Box<dyn std::error::Error>> {
    let summary = tracker.get_today_summary()?;

    let mut tasks: Vec<_> = summary.iter().collect();
    tasks.sort_by_key(|(name, _)| *name);

    let mut total_seconds = 0u64;

    for (task, duration) in tasks {
        let seconds = duration.as_secs();
        total_seconds += seconds;
        // ...
    }
    // ...
}
```

#### `Vec<_>`
- `_` = コンパイラに型を推論させる
- `summary.iter().collect()` の結果の型

#### `sort_by_key(|(name, _)| *name)`
- タプルの最初の要素（名前）でソート
- `_` = 使わない値を無視

#### `0u64`
- 型サフィックス付きのリテラル
- `0` を `u64` 型として宣言

---

## 10. よく使われるRustの文法まとめ

### 所有権と借用

| 記法 | 意味 |
|------|------|
| `x` | 所有権を持つ |
| `&x` | 不変の借用（参照） |
| `&mut x` | 可変の借用 |
| `*x` | 参照を外す（デリファレンス） |

### Option と Result

| 型 | 意味 |
|------|------|
| `Option<T>` | 値があるかもしれない（`Some(T)` or `None`） |
| `Result<T, E>` | 成功か失敗（`Ok(T)` or `Err(E)`） |
| `?` 演算子 | エラーなら早期リターン |

### よく使うメソッド

```rust
// Option
.unwrap()           // Some(x) → x、None → パニック
.expect("msg")      // Some(x) → x、None → メッセージ付きパニック
.unwrap_or(default) // Some(x) → x、None → default
.is_none()          // None かどうか
.is_some()          // Some かどうか

// Result
.unwrap()           // Ok(x) → x、Err → パニック
.expect("msg")      // Ok(x) → x、Err → メッセージ付きパニック
.map_err(|e| ...)   // エラーを変換

// イテレータ
.iter()             // 参照のイテレータ
.into_iter()        // 所有権を移すイテレータ
.iter_mut()         // 可変参照のイテレータ
.filter(|x| ...)    // 条件でフィルタ
.map(|x| ...)       // 変換
.find(|x| ...)      // 条件に合う最初の要素
.collect()          // コレクションに変換
```

### 属性（Attributes）

```rust
#[derive(...)]      // トレイトの自動実装
#[cfg(test)]        // テスト時のみコンパイル
#[allow(...)]       // 警告を抑制
```

---

## 次のステップ

1. `cargo build` でコンパイルしてみる
2. `cargo test` でテストを実行してみる
3. `cargo doc --open` でドキュメントを生成してみる
4. コードを少し変更して動作を確認してみる

---

## 参考リンク

- [The Rust Programming Language（日本語）](https://doc.rust-jp.rs/book-ja/)
- [Rust by Example（日本語）](https://doc.rust-jp.rs/rust-by-example-ja/)
- [clap ドキュメント](https://docs.rs/clap/)
- [serde ドキュメント](https://serde.rs/)
