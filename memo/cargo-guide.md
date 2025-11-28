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

### 5. インストール（cargo install 詳細解説）

#### 基本コマンド

```bash
cargo install --path .
```

#### コマンドの分解

```
cargo install --path .
│     │       │      │
│     │       │      └─ 「.」= 現在のディレクトリ
│     │       └──────── どこからインストールするか
│     └──────────────── 「インストール」サブコマンド
└────────────────────── Cargoコマンド
```

#### 内部で何が起きているか

```
1. cargo build --release を実行
2. target/release/time-checker が生成される
3. ~/.cargo/bin/time-checker にコピーされる
4. PATHに ~/.cargo/bin が含まれていれば、どこからでも実行可能に
```

#### 図解

```
【ビルド前】
time-checker/
└── src/main.rs

      ↓ cargo install --path .

【ビルド後】
time-checker/
└── target/release/time-checker  ← ここにビルドされる
                        │
                        │ コピー
                        ↓
~/.cargo/bin/time-checker  ← ここに配置（PATHに含まれている）

      ↓

どこからでも実行可能！
$ cd ~/Documents
$ time-checker start "作業"  ← どこからでもOK！
```

#### インストール前 vs 後

| 状態 | 実行方法 | 場所の制限 |
|------|----------|-----------|
| 未インストール | `./target/release/time-checker` | プロジェクトディレクトリ内のみ |
| インストール後 | `time-checker` | どこからでもOK |

#### インストール関連コマンド

```bash
# ローカルプロジェクトからインストール
cargo install --path .

# 強制再インストール（更新時）
cargo install --path . --force

# インストール先を確認
which time-checker
# → /Users/s_mtsbr/.cargo/bin/time-checker

# インストール済みパッケージ一覧
cargo install --list

# アンインストール
cargo uninstall time-checker
```

#### 様々なインストール方法

```bash
# 1. ローカルプロジェクトから（今回のケース）
cargo install --path .

# 2. crates.io から（公開パッケージ）
cargo install ripgrep          # 高速grep
cargo install bat              # catの高機能版
cargo install fd-find          # findの高速版
cargo install tokei            # コード行数カウント

# 3. GitHubから直接
cargo install --git https://github.com/sharkdp/bat

# 4. 特定のバージョンを指定
cargo install ripgrep --version 13.0.0

# 5. 特定のブランチから
cargo install --git https://github.com/user/repo --branch develop
```

#### PATHの確認

```bash
# ~/.cargo/bin がPATHに含まれているか確認
echo $PATH | tr ':' '\n' | grep cargo

# 含まれていない場合、以下を ~/.zshrc に追加
export PATH="$HOME/.cargo/bin:$PATH"

# 設定を反映
source ~/.zshrc
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

## クロスコンパイルと配布

### 結論から言うと

**MacでビルドしたバイナリはWindowsでは動きません。**

Rustはネイティブコードにコンパイルされるため、ビルドしたバイナリは**特定のOS + CPUアーキテクチャ**でのみ動作します。

### ビルドターゲットとは？

ビルド時に「どの環境向けか」を指定するのが**ターゲット**です。

```
ターゲット名の構造:
<arch>-<vendor>-<os>-<env>

例:
x86_64-apple-darwin        # macOS (Intel)
aarch64-apple-darwin       # macOS (Apple Silicon / M1, M2, M3)
x86_64-unknown-linux-gnu   # Linux (Intel/AMD)
x86_64-pc-windows-msvc     # Windows (Intel/AMD)
```

### 現在の環境を確認

```bash
# 自分の環境のターゲットを確認
rustc --print host

# time-checkerの例（Apple Silicon Mac）
# 出力: aarch64-apple-darwin
```

### クロスコンパイル（別OS向けにビルド）

#### 方法1: ターゲットを追加してビルド

```bash
# Windows向けターゲットを追加
rustup target add x86_64-pc-windows-gnu

# Windows向けにビルド
cargo build --release --target x86_64-pc-windows-gnu
```

**注意**: クロスコンパイルは設定が複雑で、リンカーの設定等が必要な場合があります。

#### 方法2: GitHub Actionsで各OS向けにビルド（推奨）

各OSのGitHub Runner上でビルドする方法が最も確実です。

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact: time-checker-linux-x86_64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact: time-checker-macos-arm64
          - os: macos-13
            target: x86_64-apple-darwin
            artifact: time-checker-macos-x86_64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact: time-checker-windows-x86_64.exe

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact }}
          path: target/${{ matrix.target }}/release/time-checker*
```

---

### 配布方法

#### 1. GitHub Releases（最も一般的）

**メリット:**
- 無料
- バージョン管理と連動
- ダウンロード数の確認可能

**手順:**
1. バージョンタグを作成: `git tag v0.1.0`
2. プッシュ: `git push origin v0.1.0`
3. GitHub上でReleaseを作成
4. 各OS向けのバイナリをアップロード

**ダウンロードURL例:**
```
https://github.com/sm11048/time-checker/releases/download/v0.1.0/time-checker-macos-arm64
https://github.com/sm11048/time-checker/releases/download/v0.1.0/time-checker-linux-x86_64
https://github.com/sm11048/time-checker/releases/download/v0.1.0/time-checker-windows-x86_64.exe
```

#### 2. crates.io（Rustコミュニティ向け）

```bash
# crates.ioにログイン
cargo login

# パッケージを公開
cargo publish
```

**メリット:**
- `cargo install time-checker`で誰でもインストール可能
- Rustユーザーには最も馴染みのある方法

**注意:**
- 一度公開すると削除できない
- パッケージ名は早い者勝ち

#### 3. Homebrew（macOS向け）

Formula（レシピ）を作成してHomebrewで配布：

```bash
# ユーザーがインストールする時
brew install sm11048/tap/time-checker
```

---

### 配布時のチェックリスト

#### バイナリ配布の場合

- [ ] 各OS向けにビルド（macOS, Linux, Windows）
- [ ] 各アーキテクチャ向けにビルド（x86_64, arm64）
- [ ] READMEにインストール方法を記載
- [ ] ライセンスファイルを含める
- [ ] バージョン番号を更新（Cargo.toml）
- [ ] CHANGELOGを更新

#### 配布ファイル名の推奨形式

```
<name>-<version>-<os>-<arch>.<ext>

例:
time-checker-0.1.0-macos-arm64.tar.gz
time-checker-0.1.0-linux-x86_64.tar.gz
time-checker-0.1.0-windows-x86_64.zip
```

---

### 実践: time-checkerを配布する場合

#### ステップ1: バージョンを確認

```bash
# Cargo.tomlのバージョンを確認・更新
grep version Cargo.toml
# version = "0.1.0"
```

#### ステップ2: タグを作成

```bash
git tag v0.1.0
git push origin v0.1.0
```

#### ステップ3: GitHub Releasesで公開

1. https://github.com/sm11048/time-checker/releases/new
2. タグ `v0.1.0` を選択
3. リリースノートを記載
4. バイナリファイルをアップロード

---

### よくある質問

#### Q: なぜMacのバイナリがWindowsで動かないの？

**A:** OSごとに実行ファイルの形式が違うためです。

| OS | 実行ファイル形式 | 拡張子 |
|----|------------------|--------|
| Windows | PE (Portable Executable) | `.exe` |
| macOS | Mach-O | なし |
| Linux | ELF | なし |

また、OSの機能（システムコール）の呼び出し方も異なります。

#### Q: ソースコードで配布すればいいのでは？

**A:** それも選択肢です。ただし：

- ユーザーがRustをインストールする必要がある
- ビルドに時間がかかる（依存クレートのダウンロード＆コンパイル）
- ビルドエラーが起きる可能性

技術者向けならソースコード配布でOK、一般ユーザー向けならバイナリ配布が親切です。

#### Q: Apple Silicon (M1/M2/M3) と Intel Mac は別？

**A:** はい、別のターゲットです。

```
aarch64-apple-darwin  # Apple Silicon (M1, M2, M3)
x86_64-apple-darwin   # Intel Mac
```

ただし、macOSにはRosetta 2があるため、Intel Mac向けバイナリはApple Siliconでも動作します（若干遅くなる）。

#### Q: 静的リンクと動的リンクの違いは？

**A:**
- **静的リンク**: 依存ライブラリをバイナリに含める → 単体で動作、ファイルサイズ大
- **動的リンク**: 実行時にライブラリを参照 → ファイルサイズ小、環境依存

Rustはデフォルトで静的リンク（musl libc使用時）が可能で、配布しやすいバイナリを作れます。

---

## 便利なCargoプラグインとツール

### 開発効率を上げるプラグイン

#### cargo-watch（ファイル変更を監視して自動実行）

```bash
# インストール
cargo install cargo-watch

# ファイル変更時に自動テスト
cargo watch -x test

# ファイル変更時に自動ビルド＆実行
cargo watch -x run

# 複数コマンドを連続実行
cargo watch -x check -x test -x run
```

**使いどころ**: TDD開発時にファイルを保存するたびに自動でテストが走る

#### cargo-edit（依存関係を簡単に追加・削除）

```bash
# インストール
cargo install cargo-edit

# 依存関係を追加（Cargo.tomlを自動編集）
cargo add serde
cargo add tokio --features full

# 開発用依存関係を追加
cargo add --dev tempfile

# 依存関係を削除
cargo rm serde

# 依存関係をアップグレード
cargo upgrade
```

**使いどころ**: Cargo.tomlを手動で編集する代わりにコマンドで管理

#### cargo-outdated（古い依存関係をチェック）

```bash
# インストール
cargo install cargo-outdated

# 古い依存関係を表示
cargo outdated
```

**出力例:**
```
Name             Project  Compat  Latest  Kind
----             -------  ------  ------  ----
chrono           0.4.31   0.4.38  0.4.38  Normal
serde            1.0.193  1.0.210 1.0.210 Normal
```

#### cargo-expand（マクロ展開を確認）

```bash
# インストール
cargo install cargo-expand

# マクロ展開後のコードを表示
cargo expand
```

**使いどころ**: `#[derive(Debug)]`などのマクロが何を生成しているか確認

---

### おすすめのRust製CLIツール

`cargo install`でインストールできる便利ツール：

| ツール | 説明 | インストール |
|--------|------|-------------|
| **ripgrep (rg)** | 超高速grep | `cargo install ripgrep` |
| **fd** | 高速find | `cargo install fd-find` |
| **bat** | catの高機能版（シンタックスハイライト） | `cargo install bat` |
| **exa** | lsの高機能版 | `cargo install exa` |
| **tokei** | コード行数カウント | `cargo install tokei` |
| **hyperfine** | ベンチマークツール | `cargo install hyperfine` |
| **just** | Makefileの代替 | `cargo install just` |
| **starship** | カスタムプロンプト | `cargo install starship` |

#### 使用例

```bash
# ripgrep: 高速に文字列検索
rg "fn main" --type rust

# fd: ファイル名で検索
fd "\.rs$"

# bat: シンタックスハイライト付きでファイル表示
bat src/main.rs

# tokei: プロジェクトのコード行数
tokei .

# hyperfine: コマンドのベンチマーク
hyperfine './target/release/time-checker status'
```

---

### 開発時のおすすめワークフロー

```bash
# ターミナル1: ファイル変更を監視して自動テスト
cargo watch -x test

# ターミナル2: コードを編集
vim src/main.rs

# 保存するたびにターミナル1でテストが自動実行される
```

#### VS Code + rust-analyzer

VS Codeを使う場合は `rust-analyzer` 拡張機能が必須：
- 自動補完
- エラー表示
- 型ヒント
- 定義ジャンプ

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
