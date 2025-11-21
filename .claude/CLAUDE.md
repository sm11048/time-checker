# time-checker プロジェクトメモ

## プロジェクト概要

Rust Edition 2024を使用したシンプルな作業時間記録CLIツール。

- **リポジトリ**: https://github.com/sm11048/time-checker
- **言語**: Rust (Edition 2024)
- **ライセンス**: MIT OR Apache-2.0
- **開発手法**: TDD (Test-Driven Development)

## ディレクトリ構成

- `/Users/s_mtsbr/tools/time-checker/memo` - プロジェクトのメモ用ディレクトリ
- `src/` - ソースコード
- `tests/` - テストコード (31テスト全て成功)
- `design.md` - 設計書

## 主要コマンド

```bash
# タスク開始
time-checker start <task> [--note <note>]

# タスク停止
time-checker stop

# 現在の状態確認
time-checker status

# サマリー表示
time-checker show
```

## 開発ルール

- TDDで進める（テストファースト）
- テストは全て成功させてからコミット
- コミットメッセージは日本語で簡潔に
- Edition 2024の機能を積極的に活用

## テスト実行

```bash
# 全テスト実行
cargo test

# 特定のテストのみ
cargo test --test integration_test
```

## ビルド

```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release
```

## データ保存場所

- `~/.time-checker/data.json` - ユーザーの作業時間データ

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
