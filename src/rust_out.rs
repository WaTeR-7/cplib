//! 高速出力ヘルパー `RustOut`。
//!
//! stdout を `BufWriter` で 1 つに束ね、`put`/`sp`/`nl` をチェインして書く。
//! `println!` の「毎回ロック＋行バッファ flush」を避けるのが主目的で、これだけで
//! 大量出力の TLE はほぼ解消する。**外部クレート非依存**なのでそのまま貼れる。
//!
//! ```ignore
//! let mut out = RustOut::new();
//! out.put(ans).nl();
//! out.put_iter(&v, " ").nl();   // Vec を空白区切りで
//! // drop 時に自動 flush（明示するなら out.flush()）
//! ```

use cargo_snippet::snippet;

use std::fmt::Display;
use std::io::{self, BufWriter, StdoutLock, Write};

#[snippet(name = "rust_out", prefix = "use std::fmt::Display;")]
#[snippet(prefix = "use std::io::{self, BufWriter, StdoutLock, Write};")]
/// stdout を束ねた高速出力バッファ。drop 時に自動 flush される。
pub struct RustOut {
    w: BufWriter<StdoutLock<'static>>,
}

#[snippet(name = "rust_out")]
impl RustOut {
    pub fn new() -> Self {
        Self {
            w: BufWriter::new(io::stdout().lock()),
        }
    }

    /// 値を 1 つ書く（`Display` があれば何でも）。
    pub fn put<T: Display>(&mut self, x: T) -> &mut Self {
        let _ = write!(self.w, "{}", x);
        self
    }

    /// 空白を書く。
    pub fn sp(&mut self) -> &mut Self {
        let _ = self.w.write_all(b" ");
        self
    }

    /// 改行を書く。
    pub fn nl(&mut self) -> &mut Self {
        let _ = self.w.write_all(b"\n");
        self
    }

    /// イテレータ（スライス等）を `sep` 区切りで書く。
    pub fn put_iter<T: Display, I: IntoIterator<Item = T>>(
        &mut self,
        iter: I,
        sep: &str,
    ) -> &mut Self {
        let mut first = true;
        for x in iter {
            if !first {
                let _ = self.w.write_all(sep.as_bytes());
            }
            let _ = write!(self.w, "{}", x);
            first = false;
        }
        self
    }

    /// 明示的に flush する（通常は drop 時に自動 flush されるので不要）。
    pub fn flush(&mut self) {
        let _ = self.w.flush();
    }
}

impl Default for RustOut {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_builds_and_runs() {
        // 出力先は stdout（テストハーネスが捕捉）。ここでは API が
        // コンパイル・実行できることの確認までを行う。
        let mut out = RustOut::new();
        out.put(1i64).sp().put("x").nl();
        out.put_iter(vec![1u32, 2, 3], " ").nl();
        out.flush();
    }
}
