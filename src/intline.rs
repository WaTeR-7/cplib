// スニペット原本なのでクレート内から呼ばれず、メソッドに dead_code が出る。
// この属性はファイル（= pub mod intline）に掛かるだけで、#[snippet] が抽出する
// mod my_template_intline の中には入らないため、貼り付け先には複製されない。
#![allow(dead_code)]

use cargo_snippet::snippet;

/// 整数係数の直線 `ax + by = c`。**コンストラクタで必ず正規化する**ので、同じ直線は
/// `==` で等しくなり、`Hash` / `Ord` も一致する。`HashSet<IntLine>` に入れれば異なる直線を
/// 数えられ、`HashMap<IntLine, _>` なら同一直線上の点をまとめられる。
///
/// 正規形は「gcd(|a|, |b|, |c|) = 1、かつ a > 0（a = 0 なら b > 0）」。
///
/// 一致判定が gcd の割り算と `==` だけで済み、掛け算が要らないのが要点。係数の比で
/// 一致を調べると `a₁c₂ = a₂c₁` のように分母を払う掛け算が要り、垂直二等分線どうしだと
/// 10²⁷ 程度になって i128 が必要になる。
///
/// ただし (a, b, c) を正規化しても、平行な直線どうしの (a, b) は揃わない
/// （`x + y = 1` は (1, 1, 1)、`2x + 2y = 1` は (2, 2, 1) のまま）。平行判定は
/// (a, b) だけを gcd で割った `normal` で行う。
///
/// 座標の絶対値は 10⁹ 程度を想定する（`through` は 1.5×10⁹、`bisector` は 2.1×10⁹ まで
/// i64 で溢れない）。
///
/// ```ignore
/// // ABC467 D: 2本の垂直二等分線が共有点を持つか
/// let l1 = IntLine::bisector((px, py), (qx, qy));
/// let l2 = IntLine::bisector((rx, ry), (sx, sy));
/// rout.yesno(l1.common_points(&l2) != CommonPoints::Zero).nl();
/// ```
#[snippet("intline")]
#[snippet(prefix = "use my_template_intline::*;")]
mod my_template_intline {
    /// 直線 `ax + by = c`。正規形を保つためフィールドは非公開で、係数は `coef` で読む。
    ///
    /// 名前が `Line` でないのは、解答テンプレートの入力マーカー `Line`（`LINE`）と
    /// glob import どうしでぶつかり、使った時点で E0659（ambiguous）になるため。
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct IntLine {
        a: i64,
        b: i64,
        c: i64,
    }

    /// 2直線の共有点（= 2元連立一次方程式の解）の個数。
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CommonPoints {
        /// 平行で一致しない。
        Zero,
        /// 1点で交わる。
        One,
        /// 一致する。
        Infinite,
    }

    impl IntLine {
        /// `ax + by = c` を正規化して作る。`(a, b) = (0, 0)` は直線ではないので panic する。
        pub fn new(a: i64, b: i64, c: i64) -> Self {
            assert!(a != 0 || b != 0, "(a, b) = (0, 0) は直線ではない");
            let g = gcd(gcd(a.abs(), b.abs()), c.abs());
            let s = if a > 0 || (a == 0 && b > 0) { 1 } else { -1 };
            Self {
                a: s * a / g,
                b: s * b / g,
                c: s * c / g,
            }
        }

        /// 2点 `p`, `q` を通る直線。`p == q` だと直線が決まらないので panic する。
        pub fn through(p: (i64, i64), q: (i64, i64)) -> Self {
            assert!(p != q, "2点が一致していると直線が決まらない");
            let (a, b) = (q.1 - p.1, p.0 - q.0);
            Self::new(a, b, a * p.0 + b * p.1)
        }

        /// 2点 `p`, `q` の垂直二等分線（`p`, `q` から等距離な点の集合）。`p == q` だと panic する。
        pub fn bisector(p: (i64, i64), q: (i64, i64)) -> Self {
            assert!(p != q, "2点が一致していると垂直二等分線が決まらない");
            // |x - p|² = |x - q|² を展開すると 2(q - p)·x = |q|² - |p|²
            Self::new(
                2 * (q.0 - p.0),
                2 * (q.1 - p.1),
                q.0 * q.0 + q.1 * q.1 - p.0 * p.0 - p.1 * p.1,
            )
        }

        /// 正規化済みの係数 `(a, b, c)`。
        pub fn coef(&self) -> (i64, i64, i64) {
            (self.a, self.b, self.c)
        }

        /// 法線ベクトル `(a, b)` を gcd で割った正規形。平行な直線どうしで等しくなるので、
        /// 平行な直線のグループ分けのキーに使える。
        pub fn normal(&self) -> (i64, i64) {
            let g = gcd(self.a.abs(), self.b.abs());
            (self.a / g, self.b / g)
        }

        /// 平行か。一致する直線も平行とみなす。
        pub fn is_parallel(&self, other: &Self) -> bool {
            self.normal() == other.normal()
        }

        /// 点 `p` が直線上にあるか。係数と座標の積は i128 で計算する。
        pub fn contains(&self, p: (i64, i64)) -> bool {
            self.a as i128 * p.0 as i128 + self.b as i128 * p.1 as i128 == self.c as i128
        }

        /// 2直線の共有点の個数。
        pub fn common_points(&self, other: &Self) -> CommonPoints {
            if !self.is_parallel(other) {
                CommonPoints::One
            } else if self == other {
                CommonPoints::Infinite
            } else {
                CommonPoints::Zero
            }
        }
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }
}

#[cfg(test)]
mod tests {
    use super::my_template_intline::*;
    use std::collections::{BTreeSet, HashSet};

    /// 決定的な xorshift。テストの再現性のため。
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        /// `lo..=hi` の一様乱数。
        fn range(&mut self, lo: i64, hi: i64) -> i64 {
            lo + (self.next() % (hi - lo + 1) as u64) as i64
        }
    }

    /// 正規化を使わず、係数の比だけで共有点の個数を決める素朴な判定。
    /// 係数行列が正則なら1個、そうでなければ拡大係数行列の 2×2 小行列式が
    /// すべて 0 のとき無数、それ以外は 0 個。
    fn brute_common((a1, b1, c1): (i64, i64, i64), (a2, b2, c2): (i64, i64, i64)) -> CommonPoints {
        let [a1, b1, c1, a2, b2, c2] = [a1, b1, c1, a2, b2, c2].map(i128::from);
        if a1 * b2 - a2 * b1 != 0 {
            CommonPoints::One
        } else if a1 * c2 - a2 * c1 == 0 && b1 * c2 - b2 * c1 == 0 {
            CommonPoints::Infinite
        } else {
            CommonPoints::Zero
        }
    }

    /// `(a, b) ≠ (0, 0)` な小さい係数 `(a, b, c)` を作る。
    fn random_coef(rng: &mut Rng) -> (i64, i64, i64) {
        loop {
            let (a, b) = (rng.range(-4, 4), rng.range(-4, 4));
            if (a, b) != (0, 0) {
                return (a, b, rng.range(-4, 4));
            }
        }
    }

    #[test]
    fn new_divides_by_gcd_and_fixes_sign() {
        assert_eq!(IntLine::new(2, 4, 6).coef(), (1, 2, 3));
        assert_eq!(IntLine::new(-2, -4, -6).coef(), (1, 2, 3));
        assert_eq!(IntLine::new(-3, 6, 9).coef(), (1, -2, -3));
        // a = 0 のときは b の符号で揃える
        assert_eq!(IntLine::new(0, -3, 6).coef(), (0, 1, -2));
        // c = 0 なら gcd(|a|, |b|) で割ることになる
        assert_eq!(IntLine::new(-4, 6, 0).coef(), (2, -3, 0));
        // 既に正規形なら変わらない
        assert_eq!(IntLine::new(2, 2, 1).coef(), (2, 2, 1));
    }

    #[test]
    fn same_line_compares_equal_regardless_of_scale() {
        let l = IntLine::new(3, -5, 7);
        for k in [-3, -2, -1, 2, 3, 1000] {
            assert_eq!(IntLine::new(3 * k, -5 * k, 7 * k), l);
        }
    }

    #[test]
    #[should_panic(expected = "直線ではない")]
    fn new_rejects_zero_normal() {
        let _ = IntLine::new(0, 0, 1);
    }

    #[test]
    fn through_examples() {
        // y = x
        assert_eq!(IntLine::through((0, 0), (1, 1)).coef(), (1, -1, 0));
        assert_eq!(
            IntLine::through((0, 0), (1, 1)),
            IntLine::through((-3, -3), (5, 5))
        );
        // 点の順序によらない
        assert_eq!(
            IntLine::through((1, 2), (4, -7)),
            IntLine::through((4, -7), (1, 2))
        );
        // 軸に平行な直線
        assert_eq!(IntLine::through((3, -1), (3, 5)).coef(), (1, 0, 3));
        assert_eq!(IntLine::through((-1, 4), (5, 4)).coef(), (0, 1, 4));
    }

    #[test]
    fn through_matches_cross_product() {
        // 点 r が直線 pq 上にある ⟺ (q - p) × (r - p) = 0
        let mut rng = Rng(20260912);
        for _ in 0..2000 {
            let p = (rng.range(-5, 5), rng.range(-5, 5));
            let q = (rng.range(-5, 5), rng.range(-5, 5));
            if p == q {
                continue;
            }
            let l = IntLine::through(p, q);
            for x in -8..=8 {
                for y in -8..=8 {
                    let cross = (q.0 - p.0) * (y - p.1) - (q.1 - p.1) * (x - p.0);
                    assert_eq!(l.contains((x, y)), cross == 0, "{p:?} {q:?} {:?}", (x, y));
                }
            }
        }
    }

    #[test]
    #[should_panic(expected = "2点が一致")]
    fn through_rejects_same_points() {
        let _ = IntLine::through((1, 2), (1, 2));
    }

    #[test]
    fn bisector_is_the_set_of_equidistant_points() {
        // x = 1
        assert_eq!(IntLine::bisector((0, 0), (2, 0)).coef(), (1, 0, 1));
        let mut rng = Rng(12345);
        for _ in 0..2000 {
            let p = (rng.range(-5, 5), rng.range(-5, 5));
            let q = (rng.range(-5, 5), rng.range(-5, 5));
            if p == q {
                continue;
            }
            let l = IntLine::bisector(p, q);
            assert_eq!(l, IntLine::bisector(q, p));
            for x in -8..=8 {
                for y in -8..=8 {
                    let d = |(u, v): (i64, i64)| (x - u) * (x - u) + (y - v) * (y - v);
                    assert_eq!(l.contains((x, y)), d(p) == d(q), "{p:?} {q:?} {:?}", (x, y));
                }
            }
        }
    }

    #[test]
    #[should_panic(expected = "2点が一致")]
    fn bisector_rejects_same_points() {
        let _ = IntLine::bisector((1, 2), (1, 2));
    }

    #[test]
    fn parallel_lines_share_normal_but_not_coef() {
        // x + y = 1 と 2x + 2y = 1 は平行だが、(a, b, c) の正規形では (a, b) が揃わない
        let l1 = IntLine::new(1, 1, 1);
        let l2 = IntLine::new(2, 2, 1);
        assert_eq!(l1.coef(), (1, 1, 1));
        assert_eq!(l2.coef(), (2, 2, 1));
        assert_eq!(l1.normal(), l2.normal());
        assert!(l1.is_parallel(&l2));
        assert_ne!(l1, l2);
        assert_eq!(l1.common_points(&l2), CommonPoints::Zero);
    }

    #[test]
    fn common_points_three_cases() {
        let x_eq_1 = IntLine::new(1, 0, 1);
        assert_eq!(
            x_eq_1.common_points(&IntLine::new(0, 1, 5)),
            CommonPoints::One
        );
        assert_eq!(
            x_eq_1.common_points(&IntLine::new(-3, 0, -3)),
            CommonPoints::Infinite
        );
        assert_eq!(
            x_eq_1.common_points(&IntLine::new(1, 0, 2)),
            CommonPoints::Zero
        );
        // 一致する直線も平行とみなす
        assert!(x_eq_1.is_parallel(&x_eq_1));
    }

    #[test]
    fn common_points_matches_brute_force() {
        let mut rng = Rng(998244353);
        let mut seen = [false; 3];
        for _ in 0..20000 {
            let e1 = random_coef(&mut rng);
            // 一致・平行が十分出るよう、2本目には1本目の定数倍や (a, b) だけの定数倍も混ぜる
            let k = rng.range(1, 3) * if rng.next() % 2 == 0 { 1 } else { -1 };
            let e2 = match rng.next() % 3 {
                0 => random_coef(&mut rng),
                1 => (e1.0 * k, e1.1 * k, e1.2 * k),
                _ => (e1.0 * k, e1.1 * k, rng.range(-12, 12)),
            };
            let (l1, l2) = (
                IntLine::new(e1.0, e1.1, e1.2),
                IntLine::new(e2.0, e2.1, e2.2),
            );
            let want = brute_common(e1, e2);
            assert_eq!(l1.common_points(&l2), want, "{e1:?} {e2:?}");
            assert_eq!(l1 == l2, want == CommonPoints::Infinite, "{e1:?} {e2:?}");
            assert_eq!(
                l1.is_parallel(&l2),
                want != CommonPoints::One,
                "{e1:?} {e2:?}"
            );
            seen[want as usize] = true;
        }
        assert!(seen.iter().all(|&s| s), "3種類とも出ること");
    }

    #[test]
    fn distinct_lines_through_grid_points() {
        // 3×3 の格子点を2点以上通る直線は、3点を通る8本（縦3・横3・斜め2）と
        // 2点だけを通る12本の計20本
        let pts: Vec<(i64, i64)> = (0..3).flat_map(|x| (0..3).map(move |y| (x, y))).collect();
        let mut hs = HashSet::new();
        let mut bs = BTreeSet::new();
        for i in 0..pts.len() {
            for j in i + 1..pts.len() {
                let l = IntLine::through(pts[i], pts[j]);
                hs.insert(l);
                bs.insert(l);
            }
        }
        assert_eq!(hs.len(), 20);
        assert_eq!(bs.len(), 20);
        // 向きは縦・横・傾き ±1・±2・±1/2 の8種類
        let normals: HashSet<_> = hs.iter().map(|l| l.normal()).collect();
        assert_eq!(normals.len(), 8);
    }

    #[test]
    fn abc467_d() {
        // P, Q の垂直二等分線と R, S の垂直二等分線が共有点を持つか
        let solve = |[px, py, qx, qy, rx, ry, sx, sy]: [i64; 8]| {
            let l1 = IntLine::bisector((px, py), (qx, qy));
            let l2 = IntLine::bisector((rx, ry), (sx, sy));
            l1.common_points(&l2) != CommonPoints::Zero
        };
        // サンプル
        assert!(solve([2, 0, 1, 1, -1, 0, 1, 2]));
        assert!(solve([1, 0, -1, 0, 0, 1, 0, -1]));
        assert!(!solve([4, 0, 3, 1, 2, 0, 1, 1]));
        // 平行で一致するが、どちらも原点を通らないケース
        assert!(solve([0, 0, 2, 0, 0, 1, 2, 1]));
    }

    #[test]
    fn extreme_coordinates_do_not_overflow() {
        // cargo test はオーバーフロー検査が有効なので、溢れれば panic する
        const M: i64 = 1_000_000_000;
        // 法線が (4×10⁹, ±(4×10⁹ - 2)) で gcd が 1 のまま残る2本。
        // 外積 a₁b₂ - a₂b₁ で平行判定すると i64 を超える
        let l1 = IntLine::bisector((-M, -M), (M, M - 1));
        let l2 = IntLine::bisector((-M, M), (M, -M + 1));
        assert_eq!(l1.coef(), (4 * M, 4 * M - 2, -2 * M + 1));
        assert_eq!(l1.common_points(&l2), CommonPoints::One);
        // l1 と平行で、中点が (0, -1/2) と (0, 1/2) にずれた垂直二等分線
        let l3 = IntLine::bisector((-M, -M + 1), (M, M));
        assert_eq!(l1.common_points(&l3), CommonPoints::Zero);
        // どちらも x + y = 0
        let l4 = IntLine::bisector((-M, -M), (M, M));
        let l5 = IntLine::bisector((-M + 1, -M + 1), (M - 1, M - 1));
        assert_eq!(l4.common_points(&l5), CommonPoints::Infinite);
        // 2点を通る直線は端点を含む
        let (p, q) = ((-M, M), (M, -M + 1));
        let l = IntLine::through(p, q);
        assert!(l.contains(p) && l.contains(q));
        assert_eq!(
            IntLine::through((-M, -M), (M, M)),
            IntLine::through((0, 0), (1, 1))
        );
    }
}
