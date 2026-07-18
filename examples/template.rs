#![allow(unused_imports, dead_code, unused_variables, unused_mut, unused_macros)]

use proconio::input;
use std::f64::consts::PI;

/// 32bit の「無限大」定数。`i32::MAX` だと加算でオーバーフローするため余裕を持たせている。
const INF32: i32 = 1 << 30;
/// 64bit の「無限大」定数。`i64::MAX` だと加算でオーバーフローするため余裕を持たせている。
const INF64: i64 = 1 << 60;
/// 法 998244353（NTT-friendly な素数）。
const P998: u64 = 998_244_353;
/// 法 1,000,000,007。
const P100: u64 = 1_000_000_007;

/// 3値以上の最小値を求める可変長マクロ（`PartialOrd` のみでよく `f64` にも使える）。
macro_rules! min {
    ($a:expr $(,)?) => {
        $a
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {{
        let a = $a;
        let b = min!($($rest),+);
        if a < b {
            a
        } else {
            b
        }
    }};
}

/// 3値以上の最大値を求める可変長マクロ（`PartialOrd` のみでよく `f64` にも使える）。
macro_rules! max {
    ($a:expr $(,)?) => {
        $a
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {{
        let a = $a;
        let b = max!($($rest),+);
        if a > b {
            a
        } else {
            b
        }
    }};
}

/// `x.chmin(y)` / `x.chmax(y)` で使う chmin/chmax トレイト。
trait ChminChmax: PartialOrd + Sized {
    /// `self` が `other` より大きければ `self = other` として更新する。更新有無を bool で返す。
    fn chmin(&mut self, other: Self) -> bool {
        if *self > other {
            *self = other;
            true
        } else {
            false
        }
    }

    /// `self` が `other` より小さければ `self = other` として更新する。更新有無を bool で返す。
    fn chmax(&mut self, other: Self) -> bool {
        if *self < other {
            *self = other;
            true
        } else {
            false
        }
    }
}
impl<T: PartialOrd> ChminChmax for T {}

mod my_template_tuple {
    /// 中身が空のタプル要素の定義
    pub struct TupleElement0;

    /// 中身が1～12個のタプル要素の定義
    macro_rules! struct_tuple_elements {
    (
        $( $identity:ident < $( $field:ident : $type:ident ),+ > ),*
    ) => {
        $(
            pub struct $identity < $( $type ),+ > {
                $( $field : $type ),+
            }
        )*
    };
}
    struct_tuple_elements! {
        TupleElement1<a: A>,
        TupleElement2<a: A, b: B>,
        TupleElement3<a: A, b: B, c: C>,
        TupleElement4<a: A, b: B, c: C, d: D>,
        TupleElement5<a: A, b: B, c: C, d: D, e: E>,
        TupleElement6<a: A, b: B, c: C, d: D, e: E, f: F>,
        TupleElement7<a: A, b: B, c: C, d: D, e: E, f: F, g: G>,
        TupleElement8<a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H>,
        TupleElement9<a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I>,
        TupleElement10<a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J>,
        TupleElement11<a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K>,
        TupleElement12<a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L>
    }

    /// タプル要素に要素を追加するトレイト
    pub trait TupleAdd<T> {
        type TupleAddOutput;
        fn add(self, x: T) -> Self::TupleAddOutput;
    }

    /// 中身が空のタプル要素に要素を追加するトレイトの実装
    impl<A> TupleAdd<A> for TupleElement0 {
        type TupleAddOutput = TupleElement1<A>;
        fn add(self, a: A) -> Self::TupleAddOutput {
            TupleElement1 { a }
        }
    }

    /// 中身が1～11個のタプル要素に要素を追加するトレイトの実装
    macro_rules! impl_tuple_add {
    (
        $( $identity:ident < $( $type:ident ),+ + $new_type:ident >( $( $field:ident ),+ + $new_field:ident ) -> $new_identity:ident ),*
    ) => {
        $(
            impl< $( $type ),+, $new_type > TupleAdd< $new_type > for $identity < $( $type ),+ > {
                type TupleAddOutput = $new_identity < $( $type ),+, $new_type >;
                fn add(self, $new_field : $new_type) -> Self::TupleAddOutput {
                    $new_identity {
                        $( $field: self.$field ),+,
                        $new_field,
                    }
                }
            }
        )*
    };
}
    impl_tuple_add! {
        TupleElement1<A + B>(a + b) -> TupleElement2,
        TupleElement2<A, B + C>(a, b + c) -> TupleElement3,
        TupleElement3<A, B, C + D>(a, b, c + d) -> TupleElement4,
        TupleElement4<A, B, C, D + E>(a, b, c, d + e) -> TupleElement5,
        TupleElement5<A, B, C, D, E + F>(a, b, c, d, e + f) -> TupleElement6,
        TupleElement6<A, B, C, D, E, F + G>(a, b, c, d, e, f + g) -> TupleElement7,
        TupleElement7<A, B, C, D, E, F, G + H>(a, b, c, d, e, f, g + h) -> TupleElement8,
        TupleElement8<A, B, C, D, E, F, G, H + I>(a, b, c, d, e, f, g, h + i) -> TupleElement9,
        TupleElement9<A, B, C, D, E, F, G, H, I + J>(a, b, c, d, e, f, g, h, i + j) -> TupleElement10,
        TupleElement10<A, B, C, D, E, F, G, H, I, J + K>(a, b, c, d, e, f, g, h, i, j + k) -> TupleElement11,
        TupleElement11<A, B, C, D, E, F, G, H, I, J, K + L>(a, b, c, d, e, f, g, h, i, j, k + l) -> TupleElement12
    }

    /// タプル要素からタプルを取得するトレイト
    pub trait TupleGet {
        type TupleGetOutput;
        fn get(self) -> Self::TupleGetOutput;
    }

    /// 中身が空のタプル要素からタプルを取得するトレイトの実装
    impl TupleGet for TupleElement0 {
        type TupleGetOutput = ();
        fn get(self) -> Self::TupleGetOutput {
            ()
        }
    }

    /// 中身が1個のタプル要素からタプルを取得するトレイトの実装（`(A,)` ではなく素の `A` を返す）
    impl<A> TupleGet for TupleElement1<A> {
        type TupleGetOutput = A;
        fn get(self) -> Self::TupleGetOutput {
            self.a
        }
    }

    /// 中身が2～12個のタプル要素からタプルを取得するトレイトの実装
    macro_rules! impl_tuple_get {
    (
        $( $identity:ident < $( $type:ident ),+ >( $( $field:ident ),+ ) ),*
    ) => {
        $(
            impl< $( $type ),+ > TupleGet for $identity < $( $type ),+ > {
                type TupleGetOutput = ( $( $type ),+, );
                fn get(self) -> Self::TupleGetOutput {
                    ( $( self.$field ),+, )
                }
            }
        )*
    };
}
    impl_tuple_get! {
        TupleElement2<A, B>(a, b),
        TupleElement3<A, B, C>(a, b, c),
        TupleElement4<A, B, C, D>(a, b, c, d),
        TupleElement5<A, B, C, D, E>(a, b, c, d, e),
        TupleElement6<A, B, C, D, E, F>(a, b, c, d, e, f),
        TupleElement7<A, B, C, D, E, F, G>(a, b, c, d, e, f, g),
        TupleElement8<A, B, C, D, E, F, G, H>(a, b, c, d, e, f, g, h),
        TupleElement9<A, B, C, D, E, F, G, H, I>(a, b, c, d, e, f, g, h, i),
        TupleElement10<A, B, C, D, E, F, G, H, I, J>(a, b, c, d, e, f, g, h, i, j),
        TupleElement11<A, B, C, D, E, F, G, H, I, J, K>(a, b, c, d, e, f, g, h, i, j, k),
        TupleElement12<A, B, C, D, E, F, G, H, I, J, K, L>(a, b, c, d, e, f, g, h, i, j, k, l)
    }

    /// タプルの定義
    pub struct Tuple<TE> {
        element: TE,
    }

    /// タプルの型エイリアス
    pub type Tuple0 = Tuple<TupleElement0>;
    pub type Tuple1<A> = Tuple<TupleElement1<A>>;
    pub type Tuple2<A, B> = Tuple<TupleElement2<A, B>>;
    pub type Tuple3<A, B, C> = Tuple<TupleElement3<A, B, C>>;
    pub type Tuple4<A, B, C, D> = Tuple<TupleElement4<A, B, C, D>>;
    pub type Tuple5<A, B, C, D, E> = Tuple<TupleElement5<A, B, C, D, E>>;
    pub type Tuple6<A, B, C, D, E, F> = Tuple<TupleElement6<A, B, C, D, E, F>>;
    pub type Tuple7<A, B, C, D, E, F, G> = Tuple<TupleElement7<A, B, C, D, E, F, G>>;
    pub type Tuple8<A, B, C, D, E, F, G, H> = Tuple<TupleElement8<A, B, C, D, E, F, G, H>>;
    pub type Tuple9<A, B, C, D, E, F, G, H, I> = Tuple<TupleElement9<A, B, C, D, E, F, G, H, I>>;
    pub type Tuple10<A, B, C, D, E, F, G, H, I, J> =
        Tuple<TupleElement10<A, B, C, D, E, F, G, H, I, J>>;
    pub type Tuple11<A, B, C, D, E, F, G, H, I, J, K> =
        Tuple<TupleElement11<A, B, C, D, E, F, G, H, I, J, K>>;
    pub type Tuple12<A, B, C, D, E, F, G, H, I, J, K, L> =
        Tuple<TupleElement12<A, B, C, D, E, F, G, H, I, J, K, L>>;

    impl Tuple<TupleElement0> {
        /// 空のタプルを生成するメソッド
        pub fn new() -> Tuple0 {
            Tuple {
                element: TupleElement0 {},
            }
        }

        /// 1つの値からタプルを生成するメソッド
        pub fn from_single<A>(a: A) -> Tuple1<A> {
            Tuple {
                element: TupleElement1 { a },
            }
        }

        /// 2つの値からタプルを生成するメソッド
        pub fn from_double<A, B>(a: A, b: B) -> Tuple2<A, B> {
            Tuple {
                element: TupleElement2 { a, b },
            }
        }

        /// 3つの値からタプルを生成するメソッド
        pub fn from_triple<A, B, C>(a: A, b: B, c: C) -> Tuple3<A, B, C> {
            Tuple {
                element: TupleElement3 { a, b, c },
            }
        }
    }

    impl<TE> Tuple<TE> {
        /// タプルに要素を追加するメソッド
        pub fn add<T>(self, x: T) -> Tuple<<TE as TupleAdd<T>>::TupleAddOutput>
        where
            TE: TupleAdd<T>,
        {
            Tuple {
                element: self.element.add(x),
            }
        }

        /// タプルから要素を取得するメソッド
        pub fn get(self) -> <TE as TupleGet>::TupleGetOutput
        where
            TE: TupleGet,
        {
            self.element.get()
        }
    }
}

mod my_template_rust_in {
    use super::my_template_tuple::{Tuple, TupleAdd, TupleElement1, TupleGet};
    use std::io::{self, Read};

    /// `RustIn::read<T>()` に型情報を与えるためのダミー定数
    pub const I8: i8 = 0;
    pub const I16: i16 = 0;
    pub const I32: i32 = 0;
    pub const I64: i64 = 0;
    pub const I128: i128 = 0;
    pub const IS: isize = 0;
    pub const U8: u8 = 0;
    pub const U16: u16 = 0;
    pub const U32: u32 = 0;
    pub const U64: u64 = 0;
    pub const U128: u128 = 0;
    pub const US: usize = 0;
    pub const STR: String = String::new();
    pub const CHAR: char = ' ';

    /// 標準入力を空白区切りでトークン化して保持し、順に読み出す構造体。
    pub struct RustIn {
        tokens: Vec<String>,
        cursor: usize,
    }

    impl RustIn {
        /// 標準入力を読み込んで `RustIn` を生成する。
        pub fn new() -> Self {
            let mut new = Self {
                tokens: Vec::new(),
                cursor: 0,
            };
            new.load();
            new
        }

        /// 標準入力から追加でトークンを読み込む（複数回の入力に対応するため）。
        pub fn load(&mut self) -> &mut Self {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            let tokens: Vec<String> = buffer.split_whitespace().map(|s| s.to_string()).collect();
            self.tokens.extend(tokens);
            self
        }

        /// `_type` の型で1個読み、チェインを開始する。
        pub fn read<'a, T>(&'a mut self, _type: T) -> RustInChain<'a, TupleElement1<T>>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let token = self.next_token();
            let value: T = token.parse().unwrap();
            RustInChain {
                rust_in: self,
                values: Tuple::from_single(value),
            }
        }

        /// `_type` の型で `n` 個読み、`Vec<T>` を1要素目に持つチェインを開始する。
        pub fn read_vec<'a, T>(
            &'a mut self,
            _type: T,
            n: usize,
        ) -> RustInChain<'a, TupleElement1<Vec<T>>>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let v = self.next_tokens(n);
            RustInChain {
                rust_in: self,
                values: Tuple::from_single(v),
            }
        }

        /// 次のトークンを1つ取り出す。
        fn next_token(&mut self) -> String {
            let token = self.tokens[self.cursor].clone();
            self.cursor += 1;
            token
        }

        /// 次のトークンを `n` 個取り出し、`T` にパースして `Vec<T>` にまとめる。
        fn next_tokens<T>(&mut self, n: usize) -> Vec<T>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            (0..n)
                .map(|_| {
                    let token = self.next_token();
                    token.parse().unwrap()
                })
                .collect()
        }
    }

    /// `RustIn::read`/`read_vec` から始まる、読んだ値をタプルとして組み立てていくチェイン。
    pub struct RustInChain<'a, TE> {
        rust_in: &'a mut RustIn,
        values: Tuple<TE>,
    }

    impl<'a, TE> RustInChain<'a, TE> {
        /// `_type` の型で1個読み、タプルに要素として追加する。
        pub fn read<T>(mut self, _type: T) -> RustInChain<'a, <TE as TupleAdd<T>>::TupleAddOutput>
        where
            TE: TupleAdd<T>,
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let token = self.rust_in.next_token();
            let value: T = token.parse().unwrap();
            RustInChain {
                rust_in: self.rust_in,
                values: self.values.add(value),
            }
        }

        /// `_type` の型で `n` 個読み、`Vec<T>` を1要素としてタプルに追加する。
        pub fn read_vec<T>(
            mut self,
            _type: T,
            n: usize,
        ) -> RustInChain<'a, <TE as TupleAdd<Vec<T>>>::TupleAddOutput>
        where
            TE: TupleAdd<Vec<T>>,
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
        {
            let v: Vec<T> = self.rust_in.next_tokens(n);
            RustInChain {
                rust_in: self.rust_in,
                values: self.values.add(v),
            }
        }

        /// チェインで読んだ値をタプル（または単一値）として取り出す。
        pub fn get(self) -> <TE as TupleGet>::TupleGetOutput
        where
            TE: TupleGet,
        {
            self.values.get()
        }
    }
}

mod my_template_rust_out {
    use std::fmt::Display;
    use std::io::{self, BufWriter, StdoutLock, Write};

    /// stdout を `BufWriter` で1つに束ねた高速出力バッファ。
    ///
    /// `put`/`sp`/`nl`/`put_iter`/`put_prec`/`yesno`/`yes`/`no`/`put_if`/`flush` をチェインして出力できる。
    /// `println!` の「毎回ロック＋行バッファ flush」を避け、大量出力の TLE を解消する。
    ///
    /// - 整数: `fmt` を経由しない手書き itoa（桁を直接バッファへ書く）。
    /// - 小数: 既定で `{:.12}` の固定小数点。桁数を変えたいときは `put_prec(x, n)`。
    /// - 文字列: `str`/`String` は UTF-8 バイト列をそのまま出力する。
    /// - 文字: `char` は UTF-8 へエンコードしてバイト列を出力する。
    /// - イテレータ: `put_iter` でスライス等を `sep` 区切りで出力できる。
    /// - drop 時に自動 flush される。
    pub struct RustOut {
        writer: BufWriter<StdoutLock<'static>>,
    }

    impl RustOut {
        /// stdout をロックし `BufWriter` で包んで生成する。
        pub fn new() -> Self {
            Self {
                writer: BufWriter::new(io::stdout().lock()),
            }
        }

        /// `RoWrite` を実装した値を1つ出力する。
        pub fn put<T: RoWrite>(&mut self, x: T) -> &mut Self {
            x.ro_write(&mut self.writer);
            self
        }

        /// 空白を出力する。
        pub fn sp(&mut self) -> &mut Self {
            let _ = self.writer.write_all(b" ");
            self
        }

        /// 改行を出力する。
        pub fn nl(&mut self) -> &mut Self {
            let _ = self.writer.write_all(b"\n");
            self
        }

        /// イテレータ（スライス等）を `sep` 区切りで出力する。
        pub fn put_iter<T: RoWrite, I: IntoIterator<Item = T>>(
            &mut self,
            iter: I,
            sep: &str,
        ) -> &mut Self {
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
        pub fn yesno(&mut self, flag: bool) -> &mut Self {
            let _ = self.writer.write_all(if flag { b"Yes" } else { b"No" });
            self
        }

        /// `Yes` を出力する。改行は付けない。
        pub fn yes(&mut self) -> &mut Self {
            let _ = self.writer.write_all(b"Yes");
            self
        }

        /// `No` を出力する。改行は付けない。
        pub fn no(&mut self) -> &mut Self {
            let _ = self.writer.write_all(b"No");
            self
        }

        /// `cond` が真なら `yes`、偽なら `no` を出力する。改行は付けない。
        pub fn put_if<T: RoWrite>(&mut self, cond: bool, yes: T, no: T) -> &mut Self {
            self.put(if cond { yes } else { no })
        }

        /// 小数点以下 `prec` 桁の固定小数点で出力する（`put` の既定は12）。
        pub fn put_prec<T: Display>(&mut self, x: T, prec: usize) -> &mut Self {
            let _ = write!(self.writer, "{:.*}", prec, x);
            self
        }

        /// 明示的に flush する（通常は drop 時に自動 flush されるので不要）。
        /// ロックは保持したままなので、flush 後もそのまま出力し続けられる。
        pub fn flush(&mut self) -> &mut Self {
            let _ = self.writer.flush();
            self
        }
    }

    /// `RustOut` に出力できる値。整数は手書き itoa、その他は `Display` 経由。
    pub trait RoWrite {
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
}

use my_template_rust_in::*;
use my_template_rust_out::*;

fn main() {
    let mut rin = RustIn::new();
    let mut rout = RustOut::new();
    let (n, m) = rin.read(US).read(I32).get();
    let x = rin.read(I32).get();
    rout.put(n).sp().put(m).sp().put(x).nl();
}
