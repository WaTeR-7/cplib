# cplib

競技プログラミング用の自作ライブラリ（Rust）。AtCoder の現行環境（rustc 1.70.0, edition 2021）を
想定しています。

モジュールは今のところ未実装（随時追加予定）。

## 使い方

### コンテスト用ワークスペースから（推奨: path 依存）

このリポジトリを兄弟ディレクトリとして clone し、コンテスト側の `Cargo.toml` に追記します。

```
your-workspace/
  cplib/       # このリポジトリ
  your-contest/
```

```toml
[dependencies]
cplib = { path = "../cplib" }
```

### git 依存として直接参照する場合

```toml
[dependencies]
cplib = { git = "https://github.com/WaTeR-7/cplib" }
```

### 提出時の 1 ファイル化

[cargo-equip](https://github.com/qryxip/cargo-equip) で `use cplib::...` を含むソースを
1 ファイルにバンドルできます（path 依存・git 依存のいずれでも動作します）。

```bash
cargo equip --exclude-atcoder-202301-crates --remove docs --minify libs --rustfmt --check --bin <bin_name>
```

## ライセンス

CC0-1.0 (パブリックドメイン相当)。自由に利用・改変してください。
