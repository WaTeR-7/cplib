use cargo_snippet::snippet;

/// コンパイル時定数を法とする剰余体。
#[snippet("modint")]
mod my_template_modint {
    use std::iter::{Product, Sum};
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub struct ModInt<const MOD: u32> {
        val: u32,
    }

    impl<const MOD: u32> ModInt<MOD> {
        /// 整数型から ModInt を生成する
        #[inline]
        pub fn new<T: RemEuclidU32>(v: T) -> Self {
            Self {
                val: v.rem_euclid_u32(MOD),
            }
        }

        /// 値が 0 の ModInt を生成する
        #[inline]
        pub fn zero() -> Self {
            Self::new(0)
        }

        /// 値が 1 の ModInt を生成する
        #[inline]
        pub fn one() -> Self {
            Self::new(1)
        }

        /// ModInt から値を取り出す
        #[inline]
        pub fn value(&self) -> u32 {
            self.val
        }

        /// 累乗を計算する
        #[inline]
        pub fn pow(self, mut x: u64) -> Self {
            let mut res = Self::one();
            let mut pow = self;
            while x > 0 {
                if x & 1 == 1 {
                    res *= pow;
                }
                pow *= pow;
                x >>= 1;
            }
            res
        }

        /// 逆元を取る
        /// 逆元が存在しない場合は panic する
        #[inline]
        pub fn inv(self) -> Self {
            let (g, x, _) = ext_gcd(self.val as i64, MOD as i64);
            assert_eq!(g, 1, "{} is not invertible mod {}", self.val, MOD);
            Self::new(x)
        }
    }

    /// a * x + b * y = g を満たす (g, x, y) を返す拡張ユークリッド
    fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (g, x, y) = ext_gcd(b, a % b);
            (g, y, x - (a / b) * y)
        }
    }

    /// rem_euclid を u32 で返せるようにするトレイト
    pub trait RemEuclidU32 {
        fn rem_euclid_u32(self, mdls: u32) -> u32;
    }

    /// 符号付き整数の RemEuclidU32 の実装のマクロ
    macro_rules! impl_rem_euclid_signed {
        ($($t:ty),*) => {$(
            impl RemEuclidU32 for $t {
                #[inline]
                fn rem_euclid_u32(self, mdls: u32) -> u32 {
                    (self as i64).rem_euclid(mdls as i64) as u32
                }
            }
        )*};
    }
    // マクロを使った i8, i16, i32, i64, isize の RemEuclidU32 の実装
    impl_rem_euclid_signed!(i8, i16, i32, i64, isize);

    /// 符号なし整数の RemEuclidU32 の実装のマクロ
    macro_rules! impl_rem_euclid_unsigned {
        ($($t:ty),*) => {$(
            impl RemEuclidU32 for $t {
                #[inline]
                fn rem_euclid_u32(self, mdls: u32) -> u32 {
                    (self as u64).rem_euclid(mdls as u64) as u32
                }
            }
        )*};
    }
    // マクロを使った u8, u16, u32, u64, usize の RemEuclidU32 の実装
    impl_rem_euclid_unsigned!(u8, u16, u32, u64, usize);

    /// 128 bit 整数の RemEuclidU32 の実装のマクロ
    macro_rules! impl_rem_euclid_128 {
        ($($t:ty),*) => {$(
            impl RemEuclidU32 for $t {
                #[inline]
                fn rem_euclid_u32(self, mdls: u32) -> u32 {
                    self.rem_euclid(mdls as $t) as u32
                }
            }
        )*};
    }
    // マクロを使った i128, u128 の RemEuclidU32 の実装
    impl_rem_euclid_128!(i128, u128);

    /// ModInt += ModInt の実装
    impl<const MOD: u32> AddAssign for ModInt<MOD> {
        #[inline]
        fn add_assign(&mut self, rhs: Self) {
            self.val += rhs.val;
            if self.val >= MOD {
                self.val -= MOD;
            }
        }
    }

    /// ModInt -= ModInt の実装
    impl<const MOD: u32> SubAssign for ModInt<MOD> {
        #[inline]
        fn sub_assign(&mut self, rhs: Self) {
            if self.val < rhs.val {
                self.val += MOD;
            }
            self.val -= rhs.val;
        }
    }

    /// ModInt *= ModInt の実装
    impl<const MOD: u32> MulAssign for ModInt<MOD> {
        #[inline]
        fn mul_assign(&mut self, rhs: Self) {
            self.val = (self.val as u64 * rhs.val as u64 % MOD as u64) as u32;
        }
    }

    /// ModInt /= ModInt の実装
    impl<const MOD: u32> DivAssign for ModInt<MOD> {
        #[inline]
        fn div_assign(&mut self, rhs: Self) {
            *self *= rhs.inv();
        }
    }

    /// - ModInt の実装
    impl<const MOD: u32> Neg for ModInt<MOD> {
        type Output = Self;
        #[inline]
        fn neg(self) -> Self {
            Self::new(if self.val == 0 { 0 } else { MOD - self.val })
        }
    }

    /// 値 / 参照 それぞれの四則演算子を *_assign から実装するマクロ
    macro_rules! impl_binop {
        ($($tr:ident, $tr_f:ident, $tr_asn:ident, $tr_asn_f:ident;)*) => {$(
            /// ModInt o ModInt の実装
            impl<const MOD: u32> $tr for ModInt<MOD> {
                type Output = Self;
                #[inline]
                fn $tr_f(mut self, rhs: Self) -> Self {
                    $tr_asn::$tr_asn_f(&mut self, rhs);
                    self
                }
            }

            /// ModInt o &ModInt の実装
            impl<const MOD: u32> $tr<&ModInt<MOD>> for ModInt<MOD> {
                type Output = Self;
                #[inline]
                fn $tr_f(self, rhs: &Self) -> Self {
                    $tr::$tr_f(self, *rhs)
                }
            }

            /// &ModInt o ModInt の実装
            impl<const MOD: u32> $tr<ModInt<MOD>> for &ModInt<MOD> {
                type Output = ModInt<MOD>;
                #[inline]
                fn $tr_f(self, rhs: ModInt<MOD>) -> ModInt<MOD> {
                    $tr::$tr_f(*self, rhs)
                }
            }

            /// &ModInt o &ModInt の実装
            impl<const MOD: u32> $tr for &ModInt<MOD> {
                type Output = ModInt<MOD>;
                #[inline]
                fn $tr_f(self, rhs: Self) -> ModInt<MOD> {
                    $tr::$tr_f(*self, *rhs)
                }
            }

            /// ModInt o= &ModInt の実装
            impl<const MOD: u32> $tr_asn<&ModInt<MOD>> for ModInt<MOD> {
                #[inline]
                fn $tr_asn_f(&mut self, rhs: &Self) {
                    $tr_asn::$tr_asn_f(self, *rhs);
                }
            }
        )*};
    }
    // マクロを使った、値 / 参照 それぞれの四則演算子の実装
    impl_binop! {
        Add, add, AddAssign, add_assign;
        Sub, sub, SubAssign, sub_assign;
        Mul, mul, MulAssign, mul_assign;
        Div, div, DivAssign, div_assign;
    }

    /// Vec<ModInt>.into_iter().sum() の実装
    impl<const MOD: u32> Sum for ModInt<MOD> {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), Add::add)
        }
    }

    /// Vec<ModInt>.iter().sum() の実装
    impl<'a, const MOD: u32> Sum<&'a ModInt<MOD>> for ModInt<MOD> {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), Add::add)
        }
    }

    /// Vec<ModInt>.into_iter().product() の実装
    impl<const MOD: u32> Product for ModInt<MOD> {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::one(), Mul::mul)
        }
    }

    /// Vec<ModInt>.iter().product() の実装
    impl<'a, const MOD: u32> Product<&'a ModInt<MOD>> for ModInt<MOD> {
        fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(Self::one(), Mul::mul)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::my_template_modint::ModInt;
    use std::collections::{BTreeSet, HashSet};

    /// 法 998244353（NTT-friendly な素数）。競プロで最も使う設定。
    const P: u32 = 998_244_353;
    type Mint = ModInt<998_244_353>;

    #[test]
    fn new_reduces_by_mod() {
        assert_eq!(Mint::new(0).value(), 0);
        assert_eq!(Mint::new(1).value(), 1);
        assert_eq!(Mint::new(P).value(), 0);
        assert_eq!(Mint::new(P + 1).value(), 1);
        assert_eq!(Mint::new(2 * P as u64).value(), 0);
        assert_eq!(Mint::new(P as u64 * 3 + 5).value(), 5);
    }

    #[test]
    fn new_accepts_negative() {
        assert_eq!(Mint::new(-1i32).value(), P - 1);
        assert_eq!(Mint::new(-(P as i64)).value(), 0);
        assert_eq!(Mint::new(-(P as i64) - 1).value(), P - 1);
        assert_eq!(Mint::new(-1i8).value(), P - 1);
        assert_eq!(Mint::new(-1isize).value(), P - 1);
    }

    #[test]
    fn new_accepts_every_int_type() {
        // 符号なし
        assert_eq!(Mint::new(7u8).value(), 7);
        assert_eq!(Mint::new(7u16).value(), 7);
        assert_eq!(Mint::new(7u32).value(), 7);
        assert_eq!(Mint::new(7usize).value(), 7);
        assert_eq!(Mint::new(u64::MAX).value(), 932_051_909);
        assert_eq!(Mint::new(u128::MAX).value(), 299_560_063);
        // 符号付き。境界値も rem_euclid なので必ず非負に落ちる
        assert_eq!(Mint::new(7i8).value(), 7);
        assert_eq!(Mint::new(7i16).value(), 7);
        assert_eq!(Mint::new(i32::MIN).value(), 847_249_411);
        assert_eq!(Mint::new(i64::MIN).value(), 532_218_398);
        assert_eq!(Mint::new(i128::MIN).value(), 848_464_321);
    }

    #[test]
    fn zero_one_and_default() {
        assert_eq!(Mint::zero().value(), 0);
        assert_eq!(Mint::one().value(), 1);
        assert_eq!(Mint::default().value(), 0);
    }

    #[test]
    fn arithmetic() {
        let (a, b) = (Mint::new(10), Mint::new(3));
        assert_eq!((a + b).value(), 13);
        assert_eq!((a - b).value(), 7);
        assert_eq!((b - a).value(), P - 7);
        assert_eq!((a * b).value(), 30);
        assert_eq!((a / b * b).value(), 10);
        // 繰り上がり・繰り下がりが MOD をまたぐケース
        assert_eq!((Mint::new(P - 1) + Mint::one()).value(), 0);
        assert_eq!((Mint::zero() - Mint::one()).value(), P - 1);
        // 乗算は u64 に広げてから剰余を取るので (P-1)^2 でも溢れない
        assert_eq!((Mint::new(P - 1) * Mint::new(P - 1)).value(), 1);
    }

    #[test]
    fn assign_ops() {
        let mut a = Mint::new(10);
        a += Mint::new(3);
        assert_eq!(a.value(), 13);
        a -= Mint::new(20);
        assert_eq!(a.value(), P - 7);
        a *= Mint::new(0);
        assert_eq!(a.value(), 0);
        let mut b = Mint::new(12);
        b /= Mint::new(4);
        assert_eq!(b.value(), 3);
    }

    #[test]
    fn value_and_reference_operands_agree() {
        let (a, b) = (Mint::new(10), Mint::new(3));
        // 値 o 値 / 値 o 参照 / 参照 o 値 / 参照 o 参照 の4通りが一致すること
        for [x, y, z, w] in [
            [a + b, a + &b, &a + b, &a + &b],
            [a - b, a - &b, &a - b, &a - &b],
            [a * b, a * &b, &a * b, &a * &b],
            [a / b, a / &b, &a / b, &a / &b],
        ] {
            assert_eq!(x.value(), y.value());
            assert_eq!(x.value(), z.value());
            assert_eq!(x.value(), w.value());
        }
        // o= も右辺に参照を取れること
        let mut c = a;
        c += &b;
        assert_eq!(c.value(), (a + b).value());
        let mut c = a;
        c -= &b;
        assert_eq!(c.value(), (a - b).value());
        let mut c = a;
        c *= &b;
        assert_eq!(c.value(), (a * b).value());
        let mut c = a;
        c /= &b;
        assert_eq!(c.value(), (a / b).value());
    }

    #[test]
    fn negation() {
        assert_eq!((-Mint::zero()).value(), 0);
        assert_eq!((-Mint::one()).value(), P - 1);
        let a = Mint::new(12345);
        assert_eq!((a + -a).value(), 0);
        assert_eq!((-(-a)).value(), a.value());
    }

    #[test]
    fn pow_basics() {
        assert_eq!(Mint::new(2).pow(0).value(), 1);
        assert_eq!(Mint::new(2).pow(1).value(), 2);
        assert_eq!(Mint::new(2).pow(10).value(), 1024);
        assert_eq!(Mint::zero().pow(0).value(), 1);
        assert_eq!(Mint::zero().pow(5).value(), 0);
        // 指数が u64 の広い範囲でも有限回で止まること（x >>= 1 の回帰テスト）
        assert_eq!(Mint::new(3).pow(12_345_678_901_234).value(), 220_965_188);
    }

    #[test]
    fn pow_matches_fermat() {
        // P は素数なので a^(P-1) = 1
        for a in [2u32, 3, 5, 1234, P - 1] {
            assert_eq!(Mint::new(a).pow((P - 1) as u64).value(), 1);
        }
    }

    #[test]
    fn inv_roundtrip() {
        for a in [1u32, 2, 3, 1234, P - 1] {
            let a = Mint::new(a);
            assert_eq!((a * a.inv()).value(), 1);
            // P が素数なら逆元はフェルマーの小定理でも求まる
            assert_eq!(a.inv().value(), a.pow((P - 2) as u64).value());
        }
        assert_eq!(Mint::new(2).inv().value(), 499_122_177);
    }

    #[test]
    #[should_panic(expected = "not invertible")]
    fn inv_of_zero_panics() {
        let _ = Mint::zero().inv();
    }

    #[test]
    #[should_panic(expected = "not invertible")]
    fn inv_panics_when_not_coprime() {
        // 法が合成数のとき、法と互いに素でない値には逆元がない
        let _ = ModInt::<10>::new(2).inv();
    }

    #[test]
    fn inv_works_for_composite_mod_when_coprime() {
        let a = ModInt::<10>::new(3);
        assert_eq!((a * a.inv()).value(), 1);
    }

    #[test]
    fn sum_and_product() {
        let v = vec![Mint::new(3), Mint::new(4), Mint::new(5)];
        assert_eq!(v.iter().sum::<Mint>().value(), 12);
        assert_eq!(v.iter().product::<Mint>().value(), 60);
        assert_eq!(v.clone().into_iter().sum::<Mint>().value(), 12);
        assert_eq!(v.into_iter().product::<Mint>().value(), 60);
        // 空列では単位元が返ること
        let e: Vec<Mint> = Vec::new();
        assert_eq!(e.iter().sum::<Mint>().value(), 0);
        assert_eq!(e.iter().product::<Mint>().value(), 1);
        // 総和・総積も MOD で畳まれること
        let big = vec![Mint::new(P - 1); 3];
        assert_eq!(big.iter().sum::<Mint>().value(), P - 3);
        assert_eq!(big.iter().product::<Mint>().value(), P - 1);
    }

    #[test]
    fn derives_eq_ord_hash() {
        assert!(Mint::new(P + 5) == Mint::new(5));
        assert!(Mint::new(1) < Mint::new(2));
        let set: HashSet<Mint> = [Mint::new(5), Mint::new(P + 5)].into_iter().collect();
        assert_eq!(set.len(), 1);
        let set: BTreeSet<Mint> = [Mint::new(5), Mint::new(P + 5)].into_iter().collect();
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn mod_close_to_2_pow_31_does_not_overflow() {
        // 加算は u32 のまま行うので、2*(MOD-1) が u32 に収まる MOD <= 2^31 が上限。
        // 使える最大クラスの法として 2^31-1（メルセンヌ素数）で確認する。
        const Q: u32 = 2_147_483_647;
        type Big = ModInt<2_147_483_647>;
        assert_eq!((Big::new(Q - 1) + Big::new(Q - 1)).value(), Q - 2);
        assert_eq!((Big::zero() - Big::new(Q - 1)).value(), 1);
        assert_eq!((Big::new(Q - 1) * Big::new(Q - 1)).value(), 1);
        let a = Big::new(123_456_789);
        assert_eq!((a * a.inv()).value(), 1);
    }
}
