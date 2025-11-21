# Cargo完全ガイド - time-checkerプロジェクトで学ぶ

## Cargoとは？

**Cargo（カーゴ）** は、Rustの公式パッケージマネージャー兼ビルドツールです。

### Cargoの役割

1. **プロジェクト管理** - 新規プロジェクトの作成と構成
2. **依存関係管理** - 外部ライブラリ（クレート）の管理
3. **ビルド** - コードのコンパイル
4. **テスト実行** - テストの実行と管理
5. **パッケージ公開** - crates.ioへの公開

簡単に言うと：
- **npm** (Node.js)
- **pip** (Python)
- **Maven/Gradle** (Java)

のRust版です。

---

## このプロジェクトで使ったCargoコマンド

### 1. プロジェクトの初期化

```bash
cargo init --name time-checker
```

**何をする？**
- 新しいRustプロジェクトを作成
- `Cargo.toml`（設定ファイル）を生成
- `src/main.rs`を生成
- `.gitignore`を生成

**実際に生成されたファイル:**
```
time-checker/
├── Cargo.toml      # プロジェクト設定ファイル
├── .gitignore      # Git除外設定
└── src/
    └── main.rs     # エントリーポイント
```

---

### 2. ビルド（コンパイル）

#### デバッグビルド
```bash
cargo build
```

**何をする？**
- ソースコードをコンパイル
- `target/debug/`に実行ファイルを生成
- **最適化なし**（コンパイル高速、実行遅い）
- デバッグ情報付き

**生成されるファイル:**
```
target/debug/time-checker  # 実行ファイル
```

#### リリースビルド
```bash
cargo build --release
```

**何をする？**
- 本番用に最適化してコンパイル
- `target/release/`に実行ファイルを生成
- **最適化あり**（コンパイル遅い、実行速い）
- デバッグ情報なし
- ファイルサイズも小さい

**time-checkerの場合:**
```bash
# リリースビルド後の実行
./target/release/time-checker --help
./target/release/time-checker start "プログラミング"
```

**Cargo.tomlの最適化設定:**
```toml
[profile.release]
opt-level = "z"      # サイズ優先の最適化
lto = true           # Link Time Optimization（リンク時最適化）
codegen-units = 1    # 並列コンパイル無効（さらに最適化）
strip = true         # デバッグ情報を削除
```

---

### 3. テストの実行

#### 全テスト実行
```bash
cargo test
```

**何をする？**
- `src/`内のユニットテスト（`#[test]`付き関数）を実行
- `tests/`内の統合テストを実行
- ドキュメントテスト（コメント内のコード例）を実行

**time-checkerの実行結果:**
```
running 8 tests   # CLI tests
running 10 tests  # Data tests
running 7 tests   # Tracker tests
running 6 tests   # Integration tests

test result: ok. 31 passed; 0 failed
```

#### 特定のテストのみ実行
```bash
# 統合テストのみ
cargo test --test integration_test

# データレイヤーのテストのみ
cargo test --test data_test

# 名前でフィルタ
cargo test tracker
```

#### テスト出力を詳細表示
```bash
cargo test -- --nocapture    # printlnの出力を表示
cargo test -- --show-output  # 成功したテストの出力も表示
```

---

### 4. プログラムの実行

#### ビルドして実行（2ステップ）
```bash
cargo build
./target/debug/time-checker start "作業"
```

#### ビルド＆実行を一度に
```bash
cargo run -- start "作業"
#         ↑  ↑
#         |  プログラムへの引数
#         引数の区切り
```

**例:**
```bash
# time-checker start "プログラミング" と同じ
cargo run -- start "プログラミング"

# time-checker --help と同じ
cargo run -- --help
```

---

### 5. インストール

```bash
cargo install --path .
```

**何をする？**
- リリースビルドを実行
- 実行ファイルを`~/.cargo/bin/`にコピー
- `~/.cargo/bin/`がPATHに含まれていれば、どこからでも実行可能に

**インストール後:**
```bash
# どこからでも実行可能！
time-checker start "作業"
time-checker status
```

**アンインストール:**
```bash
cargo uninstall time-checker
```

---

## Cargo.toml - プロジェクト設定ファイル

### 基本構造

```toml
[package]
name = "time-checker"           # パッケージ名
version = "0.1.0"               # バージョン
edition = "2024"                # Rustのエディション
authors = ["Your Name <email>"] # 作者情報
license = "MIT OR Apache-2.0"   # ライセンス
description = "説明文"

[dependencies]
# 実行時に必要なライブラリ

[dev-dependencies]
# テスト時のみ必要なライブラリ

[profile.release]
# リリースビルドの最適化設定
```

### time-checkerの依存関係

#### 本番用の依存関係 `[dependencies]`

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
clap_complete = "4.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
dirs = "5.0"
```

**各ライブラリの役割:**
- `clap` - コマンドライン引数のパース
- `clap_complete` - タブ補完機能
- `serde` + `serde_json` - JSONシリアライゼーション
- `chrono` - 日時処理
- `anyhow` - エラー処理
- `dirs` - ホームディレクトリの取得

**featuresとは？**
```toml
clap = { version = "4.5", features = ["derive"] }
#                           ↑ derive機能を有効化
```
- ライブラリの追加機能を選択的に有効化
- 必要な機能だけ使うことでコンパイル時間短縮＆バイナリサイズ削減

#### テスト用の依存関係 `[dev-dependencies]`

```toml
[dev-dependencies]
tempfile = "3.8"
```

- テスト時のみ使用
- `tempfile` - 一時ファイル/ディレクトリの作成（テストで使用）
- 本番バイナリには含まれない

---

## よく使うCargoコマンド一覧

### プロジェクト管理

```bash
# 新規プロジェクト作成（ディレクトリも作成）
cargo new my-project

# 既存ディレクトリでプロジェクト初期化
cargo init

# ライブラリプロジェクトを作成
cargo new my-lib --lib
```

### ビルド関連

```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release

# ビルドキャッシュをクリア
cargo clean

# ドキュメント生成
cargo doc --open
```

### テスト関連

```bash
# 全テスト実行
cargo test

# 特定のテスト実行
cargo test test_name

# ベンチマーク実行
cargo bench
```

### 実行・インストール

```bash
# ビルド＆実行
cargo run

# 引数付きで実行
cargo run -- arg1 arg2

# ローカルインストール
cargo install --path .

# crates.ioからインストール
cargo install ripgrep
```

### 依存関係管理

```bash
# 依存関係の更新
cargo update

# 使われていない依存関係を確認
cargo tree

# 依存関係のバージョン確認
cargo outdated  # cargo-outdatedプラグインが必要
```

### コード品質

```bash
# コードフォーマット
cargo fmt

# Linter実行（警告検出）
cargo clippy

# 未使用の依存関係を削除
cargo fix
```

---

## ディレクトリ構造の理解

```
time-checker/
├── Cargo.toml          # プロジェクト設定
├── Cargo.lock          # 依存関係の正確なバージョン記録（自動生成）
├── src/                # ソースコード
│   ├── main.rs         # バイナリのエントリーポイント
│   ├── lib.rs          # ライブラリのルート
│   ├── cli.rs          # モジュール
│   ├── data.rs
│   └── ...
├── tests/              # 統合テスト
│   ├── integration_test.rs
│   └── ...
└── target/             # ビルド成果物（.gitignoreで除外）
    ├── debug/          # デバッグビルド
    │   └── time-checker
    └── release/        # リリースビルド
        └── time-checker
```

### Cargo.lock とは？

- 依存関係の**正確なバージョン**を記録
- チーム全員が同じバージョンを使うことを保証
- **バイナリプロジェクト**: Gitに含める（推奨）
- **ライブラリプロジェクト**: Gitに含めない（.gitignoreで除外）

time-checkerは実行ファイルなので、Cargo.lockは`.gitignore`で除外しています。

---

## 実践例: time-checkerでの使い方

### 開発フロー

```bash
# 1. コードを編集
vim src/main.rs

# 2. テストを書く
vim tests/integration_test.rs

# 3. テスト実行（TDD）
cargo test

# 4. コードフォーマット
cargo fmt

# 5. Linter実行
cargo clippy

# 6. リリースビルド
cargo build --release

# 7. 動作確認
./target/release/time-checker start "テスト"

# 8. インストール
cargo install --path .

# 9. 使ってみる
time-checker start "実作業"
time-checker status
time-checker stop
```

---

## トラブルシューティング

### コンパイルエラーが出た

```bash
# エラーメッセージをよく読む
cargo build

# より詳細なエラー表示
cargo build --verbose

# キャッシュをクリアして再ビルド
cargo clean
cargo build
```

### テストが失敗する

```bash
# 詳細な出力を表示
cargo test -- --nocapture

# 特定のテストのみ実行
cargo test test_name -- --nocapture
```

### 依存関係の問題

```bash
# Cargo.lockを削除して再生成
rm Cargo.lock
cargo build

# 依存関係を最新に更新
cargo update
```

---

## まとめ

### Cargoでできること

✅ プロジェクトの作成と管理
✅ 依存関係の自動解決とダウンロード
✅ ビルド（デバッグ・リリース）
✅ テストの実行
✅ ドキュメント生成
✅ パッケージの公開とインストール

### 最低限覚えるべきコマンド

```bash
cargo new <name>        # 新規プロジェクト
cargo build             # ビルド
cargo build --release   # リリースビルド
cargo test              # テスト実行
cargo run               # ビルド＆実行
cargo install --path .  # インストール
```

### time-checkerで実際に使ったコマンド

1. `cargo init --name time-checker` - プロジェクト初期化
2. `cargo build --release` - リリースビルド（8回以上）
3. `cargo test` - テスト実行（31テスト）
4. `./target/release/time-checker` - 実行ファイルの動作確認

---

## 参考リソース

- 公式ドキュメント: https://doc.rust-lang.org/cargo/
- Rust Book (日本語): https://doc.rust-jp.rs/book-ja/
- crates.io: https://crates.io/ (ライブラリ検索)
