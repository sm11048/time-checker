# time-checker

シンプルな作業時間記録CLIツール

## 概要

time-checkerは、コマンドラインから簡単に作業時間を記録・管理できるツールです。タスクの開始・停止を記録し、日次のサマリーを表示できます。Rustで実装されており、軽量で高速に動作します。

## 特徴

- ✨ シンプルなCLIインターフェース
- 🚀 タスク開始時に前のタスクを自動終了
- 📊 同名タスクの自動集計
- 💾 JSON形式でデータ保存（`~/.time-checker/data.json`）
- ⚡ ミリ秒単位の正確な時間計測
- 📝 タスクへのメモ機能
- 🔒 ローカルデータ保存（プライバシー重視）

## インストール

### Cargoからインストール

```bash
# リポジトリをクローン
git clone https://github.com/YOUR_USERNAME/time-checker.git
cd time-checker

# インストール
cargo install --path .
```

### ソースからビルド

```bash
# ビルド
cargo build --release

# 実行ファイルは ./target/release/time-checker に生成されます
```

## 使い方

### 基本的なワークフロー

```bash
# タスクを開始
time-checker start "プログラミング"

# 現在の状態を確認
time-checker status

# 別のタスクを開始（前のタスクは自動終了）
time-checker start "会議"

# メモ付きでタスクを開始
time-checker start "ドキュメント作成" --note "設計書更新"

# タスクを停止して今日のサマリーを表示
time-checker stop

# 今日のサマリーを表示
time-checker show
```

### コマンド一覧

#### `start <task> [--note <note>]`
新しいタスクを開始します。進行中のタスクがある場合は自動的に終了します。

```bash
time-checker start "プログラミング"
time-checker start "会議" --note "週次定例"
time-checker start "会議" -n "週次定例"  # 短縮形
```

#### `stop`
現在のタスクを停止し、今日のサマリーを表示します。

```bash
time-checker stop
```

#### `status`
現在進行中のタスクと今日のサマリーを表示します。

```bash
time-checker status
```

出力例：
```
進行中のタスク: プログラミング
開始時刻: 09:30
経過時間: 1時間30分

=== 今日の作業時間 ===
プログラミング: 3時間15分
会議: 1時間30分

合計: 4時間45分
```

#### `show [period]`
指定期間のサマリーを表示します（現在は`today`のみ対応）。

```bash
time-checker show        # 今日のサマリー
time-checker show today  # 明示的に今日を指定
```

### ヘルプの表示

```bash
time-checker --help
time-checker start --help
```

## 開発

### 必要な環境

- Rust 1.91.1以上（Edition 2024を使用）
- Cargo

### 依存クレート

- clap 4.5 - CLIパース
- clap_complete 4.5 - タブ補完
- serde 1.0 / serde_json 1.0 - データシリアライゼーション
- chrono 0.4 - 日時処理
- anyhow 1.0 - エラー処理
- dirs 5.0 - ディレクトリパス取得

### テストの実行

```bash
# 全テストを実行
cargo test

# 特定のテストのみ実行
cargo test --test integration_test
cargo test --test data_test
```

テストカバレッジ：
- **CLIテスト**: 8テスト
- **Dataテスト**: 10テスト
- **Trackerテスト**: 7テスト
- **統合テスト**: 6テスト

### ビルド

```bash
# デバッグビルド
cargo build

# リリースビルド（最適化）
cargo build --release
```

### プロジェクト構成

```
time-checker/
├── Cargo.toml          # プロジェクト設定
├── design.md           # 設計書
├── src/
│   ├── main.rs         # エントリーポイント
│   ├── lib.rs          # ライブラリルート
│   ├── cli.rs          # CLIコマンド定義
│   ├── data.rs         # データ構造とDataStore
│   ├── tracker.rs      # ビジネスロジック
│   ├── error.rs        # エラー型
│   └── completion.rs   # タブ補完（今後実装）
└── tests/              # テストファイル群
```

## データ形式

作業データは `~/.time-checker/data.json` にJSON形式で保存されます。

```json
[
  {
    "task": "プログラミング",
    "start": "2025-11-15T09:00:00+09:00",
    "end": "2025-11-15T10:30:00+09:00",
    "note": "Rust実装"
  }
]
```

## ロードマップ

### Phase 1 (MVP) - 完了 ✅
- タスクの開始・停止
- 今日のサマリー表示
- 同名タスクの集計

### Phase 3 - 今後の予定
- 週/月単位のサマリー
- 除外タスク設定（休憩など）
- タブ補完機能
- CSV/Markdownエクスポート

## ライセンス

このプロジェクトは以下のライセンスのいずれかで利用できます：

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

お好きな方を選択してください。

## 貢献

Issue報告やPull Requestを歓迎します！

## 作成者

個人プロジェクトとして開発しています。
