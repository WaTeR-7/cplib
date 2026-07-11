#![allow(unused_imports, dead_code, unused_variables, unused_mut)]

use proconio::input;
use std::cmp::{max, min};
use std::fmt::Display;
use std::io::{self, BufWriter, StdoutLock, Write};

fn main() {
    let mut rout = RustOut::new();

    todo! {}
}

/// stdout を `BufWriter` で 1 つに束ねた高速出力バッファ。
///
/// `put`/`sp`/`nl`/`put_iter`/`flush` をチェインして書ける。`println!` の
/// 「毎回ロック＋行バッファ flush」を避けるのが主目的で、これだけで大量出力の
/// TLE はほぼ解消する。drop 時に自動 flush される。
struct RustOut {
    writer: BufWriter<StdoutLock<'static>>,
}

impl RustOut {
    fn new() -> Self {
        Self {
            writer: BufWriter::new(io::stdout().lock()),
        }
    }

    /// 値を 1 つ書く（`Display` があれば何でも）。
    fn put<T: Display>(&mut self, x: T) -> &mut Self {
        let _ = write!(self.writer, "{}", x);
        self
    }

    /// 空白を書く。
    fn sp(&mut self) -> &mut Self {
        let _ = self.writer.write_all(b" ");
        self
    }

    /// 改行を書く。
    fn nl(&mut self) -> &mut Self {
        let _ = self.writer.write_all(b"\n");
        self
    }

    /// イテレータ（スライス等）を `sep` 区切りで書く。
    fn put_iter<T: Display, I: IntoIterator<Item = T>>(&mut self, iter: I, sep: &str) -> &mut Self {
        let mut first = true;
        for x in iter {
            if !first {
                let _ = self.writer.write_all(sep.as_bytes());
            }
            let _ = write!(self.writer, "{}", x);
            first = false;
        }
        self
    }

    /// 明示的に flush する（通常は drop 時に自動 flush されるので不要）。
    /// ロックは保持したままなので、flush 後もそのまま書き続けられる。
    fn flush(&mut self) -> &mut Self {
        let _ = self.writer.flush();
        self
    }
}
