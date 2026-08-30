#![allow(unused_imports, dead_code, unused_variables, unused_mut, unused_macros)]

mod my_template_consts {
    /// 32bit の「無限大」定数。`i32::MAX` だと加算でオーバーフローするため余裕を持たせている。
    pub const INF32: i32 = 1 << 30;
    /// 64bit の「無限大」定数。`i64::MAX` だと加算でオーバーフローするため余裕を持たせている。
    pub const INF64: i64 = 1 << 60;
    /// 法 998244353（NTT-friendly な素数）。
    pub const P998: u64 = 998_244_353;
    /// 法 1,000,000,007。
    pub const P100: u64 = 1_000_000_007;
    /// 円周率。
    pub const PI: f64 = std::f64::consts::PI;
    /// グリッドの4近傍（上・下・左・右）への移動量。`!0` は `usize` の `-1` 相当。
    ///
    /// `wrapping_add` で足すと、0 から `-1` した場合は巨大な値に回り込むので、
    /// `ni < h && nj < w` の1回の比較で上下左右の範囲外をまとめて弾ける。
    ///
    /// ```ignore
    /// for (di, dj) in DXY {
    ///     let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
    ///     if ni < h && nj < w { /* ... */ }
    /// }
    /// ```
    pub const DXY: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];
    /// グリッドの8近傍（斜めを含む）への移動量。使い方は `DXY` と同じ。
    pub const DXY8: [(usize, usize); 8] = [
        (!0, !0),
        (!0, 0),
        (!0, 1),
        (0, !0),
        (0, 1),
        (1, !0),
        (1, 0),
        (1, 1),
    ];
}

mod my_template_minmax {
    /// 2値の最小値。`PartialOrd` だけを要求するので `f64` にも使える
    /// （`std::cmp::min` は `Ord` が要るので `f64` には使えない）。
    ///
    /// 同値なら std と同じく前の値を返す。`NaN` が混ざると比較がすべて false になるため
    /// 前の値が返る（`f64::min` のように NaN を無視する挙動にはならない）。
    pub fn min2<T: PartialOrd>(a: T, b: T) -> T {
        if b < a { b } else { a }
    }

    /// 2値の最大値。同値なら std と同じく後ろの値を返す。詳細は `min2` と同じ。
    pub fn max2<T: PartialOrd>(a: T, b: T) -> T {
        if a > b { a } else { b }
    }

    /// 3値の最小値。詳細は `min2` と同じ。
    pub fn min3<T: PartialOrd>(a: T, b: T, c: T) -> T {
        min2(min2(a, b), c)
    }

    /// 3値の最大値。詳細は `min2` と同じ。
    pub fn max3<T: PartialOrd>(a: T, b: T, c: T) -> T {
        max2(max2(a, b), c)
    }

    /// 4値の最小値。詳細は `min2` と同じ。
    pub fn min4<T: PartialOrd>(a: T, b: T, c: T, d: T) -> T {
        min2(min2(a, b), min2(c, d))
    }

    /// 4値の最大値。詳細は `min2` と同じ。
    pub fn max4<T: PartialOrd>(a: T, b: T, c: T, d: T) -> T {
        max2(max2(a, b), max2(c, d))
    }

    /// `x.chmin(y)` / `x.chmax(y)` で使う chmin/chmax トレイト。
    pub trait ChminChmax: PartialOrd + Sized {
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
}

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
    use std::io::{self, BufRead, Read, StdinLock};

    /// `RustIn::read` などに「何をどう読むか」を伝えるマーカー。
    ///
    /// `FromStr` を持つ型はブランケット実装で自分自身がマーカーになるため、`rin.read(US)` の
    /// ように「読みたい型のダミー値」を渡せばその型が返る（`Output = Self`）。
    /// 一方、`US1`（1-indexed を読んで 0-indexed で返す）のように**読む型と返す型が違う**
    /// マーカーは、このファイル固有のゼロサイズ型として定義する。ローカル型なので
    /// 上記ブランケット実装とは衝突しない（`(A, B)` のような外部型ではコヒーレンス違反になる
    /// ため、複数トークンをまとめるマーカーは `Tup2` などのローカル型で包んでいる）。
    ///
    /// `&self` を取るので `vec_of(US, n)` のように**実行時パラメータを持つマーカー**が書け、
    /// `rin` を受け取るので**複数トークンを消費するマーカー**（グリッド・グラフ）も書ける。
    pub trait Readable {
        /// このマーカーを読んだときに返る型。
        type Output;
        /// `rin` からトークンを消費して値を作る。
        fn read_value(&self, rin: &mut RustIn) -> Self::Output;
    }

    /// `FromStr` な型はダミー値そのものがマーカーになる（`US` → `usize`、`STR` → `String`）。
    impl<T> Readable for T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        type Output = T;
        fn read_value(&self, rin: &mut RustIn) -> T {
            rin.next_token().parse().unwrap()
        }
    }

    /// `FromStr` 経由のマーカーに使うダミー定数。
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
    pub const F32: f32 = 0.0;
    pub const F64: f64 = 0.0;
    pub const STR: String = String::new();
    pub const CHAR: char = ' ';

    /// 返す型が入力と異なるマーカー（ゼロサイズ型＋定数）をまとめて定義するマクロ。
    macro_rules! def_markers {
        ($(
            $(#[$attr:meta])*
            $name:ident as $konst:ident -> $out:ty { |$rin:ident| $body:expr }
        )*) => {$(
            $(#[$attr])*
            pub struct $name;
            $(#[$attr])*
            pub const $konst: $name = $name;
            impl Readable for $name {
                type Output = $out;
                fn read_value(&self, $rin: &mut RustIn) -> $out {
                    $body
                }
            }
        )*};
    }

    def_markers! {
        /// 1-indexed の頂点番号などを読み、`-1` して 0-indexed の `usize` で返す。
        Usize1 as US1 -> usize { |rin| rin.next_token().parse::<usize>().unwrap() - 1 }
        /// `US1` の符号付き版。
        Isize1 as IS1 -> isize { |rin| rin.next_token().parse::<isize>().unwrap() - 1 }
        /// 1トークンを `Vec<u8>` として読む。`s[i][j] == b'#'` のような添字アクセスができ、
        /// グリッド問題では `String` より扱いやすい。
        Bytes as BYTES -> Vec<u8> { |rin| rin.next_token().as_bytes().to_vec() }
        /// 1トークンを `Vec<char>` として読む（非ASCIIや `char` 前提の処理向け）。
        Chars as CHARS -> Vec<char> { |rin| rin.next_token().chars().collect() }
        /// 数字列を各桁の数値 `Vec<u8>` として読む（`"314"` → `[3, 1, 4]`）。
        Digits as DIGITS -> Vec<u8> { |rin| rin.next_token().bytes().map(|b| b - b'0').collect() }
        /// `"1"` を `true`、それ以外を `false` として読む。
        Bool01 as B01 -> bool { |rin| rin.next_token() == "1" }
        /// 現在位置から行末までを1つの `String` として読む（空白を含む1行が欲しいとき）。
        Line as LINE -> String { |rin| rin.next_line().to_owned() }
    }

    /// `vec_of(marker, n)`: マーカーを `n` 回読んで `Vec` にするマーカー。
    /// `vec_of(vec_of(US, m), n)` とネストすれば空白区切りの数値行列も読める
    /// （区切り無しの文字グリッドは `grid(h)` / `BYTES` を使う）。
    pub struct VecOf<M>(M, usize);

    /// `VecOf` を作る（`rin.read(vec_of(US, n))` で `Vec<usize>` が読める）。
    pub fn vec_of<M>(marker: M, n: usize) -> VecOf<M> {
        VecOf(marker, n)
    }

    impl<M: Readable> Readable for VecOf<M> {
        type Output = Vec<M::Output>;
        fn read_value(&self, rin: &mut RustIn) -> Self::Output {
            let mut v = Vec::with_capacity(self.1);
            for _ in 0..self.1 {
                v.push(self.0.read_value(rin));
            }
            v
        }
    }

    /// 複数のマーカーを連続して読み、タプルとして返すマーカー（`tup2`〜`tup4`）を定義するマクロ。
    macro_rules! def_tuple_markers {
        ($( $name:ident, $func:ident, ( $( $t:ident $field:ident $idx:tt ),+ ) );* $(;)?) => {$(
            /// 複数のマーカーを連続して読み、タプルとして返すマーカー。
            pub struct $name < $( $t ),+ >( $( $t ),+ );

            /// 上記マーカーを作る。`read(vec_of(tup2(US1, US1), m))` で 0-indexed の辺リストが読める。
            pub fn $func < $( $t ),+ >( $( $field: $t ),+ ) -> $name < $( $t ),+ > {
                $name( $( $field ),+ )
            }

            impl< $( $t: Readable ),+ > Readable for $name < $( $t ),+ > {
                type Output = ( $( $t::Output ),+ ,);
                fn read_value(&self, rin: &mut RustIn) -> Self::Output {
                    // タプル式は左から右に評価されるので、読む順序は書いた順になる。
                    ( $( self.$idx.read_value(rin) ),+ ,)
                }
            }
        )*};
    }

    def_tuple_markers! {
        Tup2, tup2, (A a 0, B b 1);
        Tup3, tup3, (A a 0, B b 1, C c 2);
        Tup4, tup4, (A a 0, B b 1, C c 2, D d 3);
    }

    /// `grid(h)`: 区切り無しの文字列 `h` 行を `Vec<Vec<u8>>` として読むマーカー
    /// （`vec_of(BYTES, h)` と同じだが意図が明確）。
    pub struct Grid(usize);

    /// `Grid` を作る。
    pub fn grid(h: usize) -> Grid {
        Grid(h)
    }

    impl Readable for Grid {
        type Output = Vec<Vec<u8>>;
        fn read_value(&self, rin: &mut RustIn) -> Self::Output {
            let mut g = Vec::with_capacity(self.0);
            for _ in 0..self.0 {
                g.push(rin.next_token().as_bytes().to_vec());
            }
            g
        }
    }

    /// `grid_pad(h, w, pad)`: 四辺を `pad` で囲んだ `(h + 2) * (w + 2)` のグリッドを読むマーカー。
    /// 番兵があるので探索時の範囲外判定が不要になる（入力の `(i, j)` は `(i + 1, j + 1)` に対応）。
    pub struct GridPad(usize, usize, u8);

    /// `GridPad` を作る。
    pub fn grid_pad(h: usize, w: usize, pad: u8) -> GridPad {
        GridPad(h, w, pad)
    }

    impl Readable for GridPad {
        type Output = Vec<Vec<u8>>;
        fn read_value(&self, rin: &mut RustIn) -> Self::Output {
            let (h, w, pad) = (self.0, self.1, self.2);
            let mut g = vec![vec![pad; w + 2]; h + 2];
            for i in 0..h {
                let row = rin.next_token().as_bytes();
                g[i + 1][1..=w].copy_from_slice(&row[..w]);
            }
            g
        }
    }

    /// `graph(n, m)` / `digraph(n, m)` / `tree(n)`: **1-indexed** の辺を `m` 本読み、
    /// `n` 頂点の隣接リスト `Vec<Vec<usize>>`（0-indexed）にするマーカー。
    pub struct GraphOf {
        n: usize,
        m: usize,
        directed: bool,
    }

    /// 無向グラフ（辺 `m` 本）。
    pub fn graph(n: usize, m: usize) -> GraphOf {
        GraphOf { n, m, directed: false }
    }

    /// 有向グラフ（辺 `m` 本、`u -> v` のみ張る）。
    pub fn digraph(n: usize, m: usize) -> GraphOf {
        GraphOf { n, m, directed: true }
    }

    /// 木（辺 `n - 1` 本の無向グラフ）。
    pub fn tree(n: usize) -> GraphOf {
        GraphOf { n, m: n - 1, directed: false }
    }

    impl Readable for GraphOf {
        type Output = Vec<Vec<usize>>;
        fn read_value(&self, rin: &mut RustIn) -> Self::Output {
            let mut g = vec![Vec::new(); self.n];
            for _ in 0..self.m {
                let u = rin.next_token().parse::<usize>().unwrap() - 1;
                let v = rin.next_token().parse::<usize>().unwrap() - 1;
                g[u].push(v);
                if !self.directed {
                    g[v].push(u);
                }
            }
            g
        }
    }

    /// `wgraph(n, m, W)` / `wdigraph(n, m, W)` / `wtree(n, W)`: 重み付き版。
    /// **1-indexed** の `u v w` を `m` 行読み、`Vec<Vec<(usize, W::Output)>>` にする。
    pub struct WGraphOf<W> {
        n: usize,
        m: usize,
        directed: bool,
        weight: W,
    }

    /// 重み付き無向グラフ（辺 `m` 本）。
    pub fn wgraph<W>(n: usize, m: usize, weight: W) -> WGraphOf<W> {
        WGraphOf { n, m, directed: false, weight }
    }

    /// 重み付き有向グラフ（辺 `m` 本）。
    pub fn wdigraph<W>(n: usize, m: usize, weight: W) -> WGraphOf<W> {
        WGraphOf { n, m, directed: true, weight }
    }

    /// 重み付き木（辺 `n - 1` 本の無向グラフ）。
    pub fn wtree<W>(n: usize, weight: W) -> WGraphOf<W> {
        WGraphOf { n, m: n - 1, directed: false, weight }
    }

    impl<W: Readable> Readable for WGraphOf<W>
    where
        W::Output: Clone,
    {
        type Output = Vec<Vec<(usize, W::Output)>>;
        fn read_value(&self, rin: &mut RustIn) -> Self::Output {
            let mut g = vec![Vec::new(); self.n];
            for _ in 0..self.m {
                let u = rin.next_token().parse::<usize>().unwrap() - 1;
                let v = rin.next_token().parse::<usize>().unwrap() - 1;
                let w = self.weight.read_value(rin);
                g[u].push((v, w.clone()));
                if !self.directed {
                    g[v].push((u, w));
                }
            }
            g
        }
    }

    /// 標準入力を1つの `String` バッファとして保持し、空白区切りのバイト範囲でトークン化する構造体。
    ///
    /// トークンごとに `String` を確保せず `&str` をバッファから借用するだけなので、
    /// 大量入力でもトークン数に比例したヒープ確保が発生しない。
    pub struct RustIn {
        buffer: String,
        spans: Vec<(usize, usize)>,
        cursor: usize,
        /// `Some` のときだけ「トークンが尽きたら1行読み足す」インタラクティブモードになる。
        reader: Option<io::BufReader<StdinLock<'static>>>,
    }

    impl RustIn {
        /// 標準入力を一括で読み込んで `RustIn` を生成する（バッチ問題向け・高速）。
        pub fn new() -> Self {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            let mut new = Self {
                buffer,
                spans: Vec::new(),
                cursor: 0,
                reader: None,
            };
            new.tokenize_from(0);
            new
        }

        /// トークンが尽きるたびに1行だけ読み足す `RustIn` を生成する（インタラクティブ問題向け）。
        ///
        /// `read_to_string` による一括先読みはジャッジからの応答をブロックしてしまい使えないため、
        /// `read_line` で1行ずつオンデマンドに読む。出力側は `RustOut::flush` を都度呼ぶこと。
        pub fn new_interactive() -> Self {
            Self {
                buffer: String::new(),
                spans: Vec::new(),
                cursor: 0,
                reader: Some(io::BufReader::new(io::stdin().lock())),
            }
        }

        /// マーカー `marker` を1つだけ読み、チェインを経由せずそのまま値を返す。
        /// 単一の値・`Vec`・グリッド・グラフなど「1つ読んで終わり」のときはこちらが短い
        /// （`rin.read(US).get()` と同じ）。複数個をまとめて読むときは `read` でチェインする。
        pub fn get<M: Readable>(&mut self, marker: M) -> M::Output {
            marker.read_value(self)
        }

        /// マーカー `marker` を1つ読み、チェインを開始する。
        pub fn read<'a, M: Readable>(
            &'a mut self,
            marker: M,
        ) -> RustInChain<'a, TupleElement1<M::Output>> {
            let value = marker.read_value(self);
            RustInChain {
                rust_in: self,
                values: Tuple::from_single(value),
            }
        }

        /// 使わないトークンを `n` 個読み飛ばす。
        pub fn skip(&mut self, n: usize) -> &mut Self {
            for _ in 0..n {
                self.next_token();
            }
            self
        }

        /// 次のトークンを1つ取り出す（コピーせずバッファから借用する）。
        /// トークンが尽きていて `reader` があれば1行読み足してから取り出す。
        fn next_token(&mut self) -> &str {
            while self.cursor >= self.spans.len() {
                if !self.refill_line() {
                    panic!("入力が不足しています");
                }
            }
            let (start, end) = self.spans[self.cursor];
            self.cursor += 1;
            &self.buffer[start..end]
        }

        /// 次のトークンの先頭から行末までを取り出し、その行のトークンを読み飛ばす。
        fn next_line(&mut self) -> &str {
            while self.cursor >= self.spans.len() {
                if !self.refill_line() {
                    panic!("入力が不足しています");
                }
            }
            let start = self.spans[self.cursor].0;
            let end = match self.buffer[start..].find('\n') {
                Some(offset) => start + offset,
                None => self.buffer.len(),
            };
            while self.cursor < self.spans.len() && self.spans[self.cursor].0 < end {
                self.cursor += 1;
            }
            self.buffer[start..end].trim_end()
        }

        /// `reader` から1行読み、トークン化してバッファに追加する。読めれば `true`、EOFなら `false`。
        fn refill_line(&mut self) -> bool {
            let Some(reader) = self.reader.as_mut() else {
                return false;
            };
            let start = self.buffer.len();
            let n = reader.read_line(&mut self.buffer).unwrap();
            if n == 0 {
                return false;
            }
            self.tokenize_from(start);
            true
        }

        /// `self.buffer[start..]` を空白区切りでトークン化し、バイト範囲として `spans` に追加する。
        fn tokenize_from(&mut self, start: usize) {
            let base_ptr = self.buffer.as_ptr() as usize;
            let new_spans: Vec<(usize, usize)> = self.buffer[start..]
                .split_whitespace()
                .map(|s| {
                    let off = s.as_ptr() as usize - base_ptr;
                    (off, off + s.len())
                })
                .collect();
            self.spans.extend(new_spans);
        }
    }

    /// `RustIn::read` から始まる、読んだ値をタプルとして組み立てていくチェイン。
    pub struct RustInChain<'a, TE> {
        rust_in: &'a mut RustIn,
        values: Tuple<TE>,
    }

    impl<'a, TE> RustInChain<'a, TE> {
        /// マーカー `marker` を1つ読み、タプルに要素として追加する。
        pub fn read<M: Readable>(
            self,
            marker: M,
        ) -> RustInChain<'a, <TE as TupleAdd<M::Output>>::TupleAddOutput>
        where
            TE: TupleAdd<M::Output>,
        {
            let value = marker.read_value(self.rust_in);
            RustInChain {
                rust_in: self.rust_in,
                values: self.values.add(value),
            }
        }

        /// 使わないトークンを `n` 個読み飛ばす（タプルには何も追加しない）。
        pub fn skip(self, n: usize) -> Self {
            self.rust_in.skip(n);
            self
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

use my_template_consts::*;
use my_template_minmax::*;
use my_template_rust_in::*;
use my_template_rust_out::*;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 深い再帰でも落ちないよう、大きめのスタックを確保したスレッドで解く。
// 木やグラフの DFS は深さが N に比例しうるが、main スレッドの既定は 8MB しかない。
// スタックのページは使うまで確保されないので、深くならない問題でも実質ノーコスト。
fn main() {
    let handle = std::thread::Builder::new()
        // panic メッセージを素の main と同じ `thread 'main' panicked at ...` に保つ
        .name("main".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(solve)
        .unwrap();
    if handle.join().is_err() {
        // solve 内の panic は既に表示済み。二重に出さず終了コードだけ合わせる
        std::process::exit(101);
    }
}

fn solve() {
    let mut rin = RustIn::new();
    let mut rout = RustOut::new();
}
