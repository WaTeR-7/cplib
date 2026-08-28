mod my_template_modint {
    use std::iter::{Product, Sum};
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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
    /// マクロを使った i8, i16, i32, i64, isize の RemEuclidU32 の実装
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
    /// マクロを使った u8, u16, u32, u64 usize の RemEuclidU32 の実装
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
    /// マクロを使った i128, u128 の RemEuclidU32 の実装
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
    /// マクロを使った、値 / 参照 それぞれの四則演算子の実装
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
