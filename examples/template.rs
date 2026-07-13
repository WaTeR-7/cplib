#![allow(unused_imports, dead_code, unused_variables, unused_mut)]

use proconio::input;
use std::cmp::{max, min};
use std::fmt::Display;
use std::io::{self, BufWriter, StdoutLock, Write};

fn main() {
    let mut rout = RustOut::new();
}

/// stdout を `BufWriter` で1つに束ねた高速出力バッファ。
///
/// `put`/`sp`/`nl`/`put_iter`/`put_prec`/`yesno`/`put_if`/`flush` をチェインして出力できる。
/// `println!` の「毎回ロック＋行バッファ flush」を避け、大量出力の TLE を解消する。
///
/// - 整数: `fmt` を経由しない手書き itoa（桁を直接バッファへ書く）。
/// - 小数: 既定で `{:.12}` の固定小数点。桁数を変えたいときは `put_prec(x, n)`。
/// - 文字列: `str`/`String` は UTF-8 バイト列をそのまま出力する。
/// - 文字: `char` は UTF-8 へエンコードしてバイト列を出力する。
/// - イテレータ: `put_iter` でスライス等を `sep` 区切りで出力できる。
/// - drop 時に自動 flush される。
struct RustOut {
    writer: BufWriter<StdoutLock<'static>>,
}

impl RustOut {
    /// stdout をロックし `BufWriter` で包んで生成する。
    fn new() -> Self {
        Self {
            writer: BufWriter::new(io::stdout().lock()),
        }
    }

    /// `RoWrite` を実装した値を1つ出力する。
    fn put<T: RoWrite>(&mut self, x: T) -> &mut Self {
        x.ro_write(&mut self.writer);
        self
    }

    /// 空白を出力する。
    fn sp(&mut self) -> &mut Self {
        let _ = self.writer.write_all(b" ");
        self
    }

    /// 改行を出力する。
    fn nl(&mut self) -> &mut Self {
        let _ = self.writer.write_all(b"\n");
        self
    }

    /// イテレータ（スライス等）を `sep` 区切りで出力する。
    fn put_iter<T: RoWrite, I: IntoIterator<Item = T>>(&mut self, iter: I, sep: &str) -> &mut Self {
        let mut first = true;
        for x in iter {
            if !first {
                let _ = self.writer.write_all(sep.as_bytes());
            }
            x.ro_write(&mut self.writer);
            first = false;
        }
        self
    }

    /// bool を `Yes`/`No` で出力する。改行は付けない。
    fn yesno(&mut self, flag: bool) -> &mut Self {
        let _ = self.writer.write_all(if flag { b"Yes" } else { b"No" });
        self
    }

    /// `cond` が真なら `yes`、偽なら `no` を出力する。改行は付けない。
    fn put_if<T: RoWrite>(&mut self, cond: bool, yes: T, no: T) -> &mut Self {
        self.put(if cond { yes } else { no })
    }

    /// 小数点以下 `prec` 桁の固定小数点で出力する（`put` の既定は12）。
    fn put_prec<T: Display>(&mut self, x: T, prec: usize) -> &mut Self {
        let _ = write!(self.writer, "{:.*}", prec, x);
        self
    }

    /// 明示的に flush する（通常は drop 時に自動 flush されるので不要）。
    /// ロックは保持したままなので、flush 後もそのまま出力し続けられる。
    fn flush(&mut self) -> &mut Self {
        let _ = self.writer.flush();
        self
    }
}

/// `RustOut` に出力できる値。整数は手書き itoa、その他は `Display` 経由。
trait RoWrite {
    fn ro_write<W: Write>(&self, w: &mut W);
}

/// 参照は中身へ転送（`put_iter(&vec, ..)` が `&i64` を渡すため）。
impl<T: RoWrite + ?Sized> RoWrite for &T {
    fn ro_write<W: Write>(&self, w: &mut W) {
        (**self).ro_write(w);
    }
}

/// 符号なし整数：末尾から桁をバッファに書いて一括 `write_all`。
macro_rules! impl_ro_uint {
    ($wide:ty, $cap:expr, $($t:ty),*) => {$(
        impl RoWrite for $t {
            fn ro_write<W: Write>(&self, w: &mut W) {
                let mut buf = [0u8; $cap];
                let mut i = buf.len();
                let mut x = *self as $wide;
                loop {
                    i -= 1;
                    buf[i] = b'0' + (x % 10) as u8;
                    x /= 10;
                    if x == 0 {
                        break;
                    }
                }
                let _ = w.write_all(&buf[i..]);
            }
        }
    )*};
}

/// 符号あり整数：`unsigned_abs()` で絶対値化（`MIN` も安全）してから桁を書く。
macro_rules! impl_ro_iint {
    ($wide:ty, $cap:expr, $($t:ty),*) => {$(
        impl RoWrite for $t {
            fn ro_write<W: Write>(&self, w: &mut W) {
                let mut buf = [0u8; $cap];
                let mut i = buf.len();
                let neg = *self < 0;
                let mut x = self.unsigned_abs() as $wide;
                loop {
                    i -= 1;
                    buf[i] = b'0' + (x % 10) as u8;
                    x /= 10;
                    if x == 0 {
                        break;
                    }
                }
                if neg {
                    i -= 1;
                    buf[i] = b'-';
                }
                let _ = w.write_all(&buf[i..]);
            }
        }
    )*};
}

// u64 幅（最大20桁）に収まる型。
impl_ro_uint!(u64, 20, u8, u16, u32, u64, usize);
impl_ro_iint!(u64, 20, i8, i16, i32, i64, isize);
// 128bit（最大39桁 + 符号）。
impl_ro_uint!(u128, 40, u128);
impl_ro_iint!(u128, 40, i128);

/// 文字列はバイト列を直接出力する。
impl RoWrite for str {
    fn ro_write<W: Write>(&self, w: &mut W) {
        let _ = w.write_all(self.as_bytes());
    }
}
impl RoWrite for String {
    fn ro_write<W: Write>(&self, w: &mut W) {
        let _ = w.write_all(self.as_bytes());
    }
}
impl RoWrite for char {
    fn ro_write<W: Write>(&self, w: &mut W) {
        let mut b = [0u8; 4];
        let _ = w.write_all(self.encode_utf8(&mut b).as_bytes());
    }
}

/// 浮動小数点は既定で `{:.12}` の固定小数点。桁数を変えたいときは `put_prec`。
macro_rules! impl_ro_float {
    ($($t:ty),*) => {$(
        impl RoWrite for $t {
            fn ro_write<W: Write>(&self, w: &mut W) {
                let _ = write!(w, "{:.12}", self);
            }
        }
    )*};
}
impl_ro_float!(f32, f64);
