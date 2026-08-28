//! 競技プログラミング用の自作スニペット集。
//!
//! 各モジュールは「単体で貼れる自己完結コード」として書き、`#[snippet(...)]` で
//! 抽出単位を宣言する。`cargo test` で原本を検証し、`cargo snippet` でエディタ用
//! スニペット（VSCode / Neosnippet / UltiSnips）を書き出してコピペ／展開に使う。

pub mod modint;
