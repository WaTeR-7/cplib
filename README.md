# cplib

競技プログラミング用の自作ライブラリ（Rust）。AtCoder の現行環境（2025-10 / rustc 1.89.0, edition 2024）を
想定しています。

[cargo-snippet](https://github.com/hatoo/cargo-snippet) ベースのスニペット集として運用します（下記「スニペット運用」）。現在のモジュール: `modint`（剰余体）、`dfs`（再帰 DFS の雛形。貼ってから問題に合わせて書き換える）。

## 解答テンプレート

`examples/template.rs` は `cargo compete new` が生成する各解答ファイル（`a.rs` など）の雛形の「正」です。
ここに好みの `use`・入出力ヘルパー・マクロなどを育てていきます。

example として置いているため、AtCoder と同じ環境でコンパイルチェックできます:

```bash
cargo build --examples   # rust-toolchain.toml により rustc 1.89.0 で検証
```

`[dev-dependencies]` に AtCoder 2025-10 環境の crate セットを持たせているので、テンプレートが
proconio などを使ってもチェックが通ります（dev-dependencies は cplib を使う側には伝播しません）。

コンテスト用ワークスペース側では、この内容を `compete.toml` の `[template] src` にインラインで
ミラーして使います（cargo-compete は現行スキーマでは外部ファイル参照に非対応のため）。ミラーは
ワークスペース側の `sync-template.py` で自動化しています。

## スニペット運用

**背景**: AtCoder は 1 ファイル提出。以前は `use cplib::...` を cargo-equip で 1 ファイル化していましたが、cargo-equip は rustc 1.89 で動作しません（メンテ停止・proc-macro ABI 非互換・edition 2024 非対応、いずれも確認済み）。そこで **cplib を「テスト済みのスニペット原本」とし、[cargo-snippet](https://github.com/hatoo/cargo-snippet) で抽出してエディタ展開／コピペする**方式にしています。テキストコピーなので rustc のバージョンに依存せず壊れず、貼った先で問題ごとに自由に改造できるのが利点です。

### モジュールを書く

`src/<name>.rs` に**単体で貼れる自己完結モジュール**を書き、抽出単位を `#[snippet(...)]` で宣言します。**外部クレート非依存**にすること（提出コードに埋め込めるように）。必要な `use` は `prefix` で宣言すると抽出物の先頭に付きます。

```rust
use cargo_snippet::snippet;

#[snippet(name = "foo", prefix = "use std::collections::HashMap;")]
pub fn foo() { /* ... */ }
```

`lib.rs` に `pub mod <name>;` を足せば `cargo test` で原本を検証できます（`#[snippet]` は no-op でコンパイル/実行に無影響）。

### スニペットを生成する

```bash
cargo install cargo-snippet --features binaries   # 初回のみ
rustup component add rustfmt                       # 出力整形に必要
./gen-snippets.sh ~/.config/nvim/snippets/rust.json   # src/ → VSCode 形式スニペット
```

出力先はエディタ側の設定で決まるので、スクリプトは既定値を持たず引数で受け取ります。

### エディタで使う

出力は VSCode 形式のみです（LuaSnip の `from_vscode`、blink.cmp、VSCode 本体が読める形式）。

LazyVim なら `~/.config/nvim/snippets/<filetype>.json` がそのまま読まれるので、Rust 用は
`~/.config/nvim/snippets/rust.json` を出力先に指定します。挿入モードでスニペット名
（例 `modint`）を打つと候補に出ます。

エディタが別のパスを見ているなら、そのパスを引数に渡すだけで済みます。

### 提出

解答は cplib に依存しない自己完結ファイルなので、`cargo compete submit`（`[submit] kind = "file"`）でそのまま提出できます。cargo-equip は不要です。

## ライセンス

CC0-1.0 (パブリックドメイン相当)。自由に利用・改変してください。
