use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

use crate::base::TwoFloat;

pub(crate) fn fast_two_sum(a: f64, b: f64) -> (f64, f64) {
    // Joldes et al. (2017) Algorithm 1
    let s = a + b;
    let z = s - a;
    (s, b - z)
}

pub(crate) fn two_sum(a: f64, b: f64) -> (f64, f64) {
    // Joldes et al. (2017) Algorithm 2
    let s = a + b;
    let aa = s - b;
    let bb = s - aa;
    let da = a - aa;
    let db = b - bb;
    (s, da + db)
}

pub(crate) fn two_diff(a: f64, b: f64) -> (f64, f64) {
    // Joldes et al. (2017) Algorithm 2 for negative rhs
    let s = a - b;
    let aa = s + b;
    let bb = s - aa;
    let da = a - aa;
    let db = b + bb;
    (s, da - db)
}

pub(crate) fn two_prod(a: f64, b: f64) -> (f64, f64) {
    // Joldes et al. (2017) Algorithm 3
    let p = a * b;
    (p, a.mul_add(b, -p))
}

impl TwoFloat {
    /// Creates a new `TwoFloat` by adding two `f64` values using Algorithm 2
    /// from Joldes et al. (2017).
    pub fn new_add(x: f64, y: f64) -> TwoFloat {
        let (a, b) = two_sum(x, y);
        TwoFloat { hi: a, lo: b }
    }

    /// Creates a new `TwoFloat` by subtracting two `f64` values using
    /// Algorithm 2 from Joldes et al. (2017) modified for negative right-hand
    /// side.
    pub fn new_sub(x: f64, y: f64) -> TwoFloat {
        let (a, b) = two_diff(x, y);
        TwoFloat { hi: a, lo: b }
    }

    /// Creates a new `TwoFloat` by multiplying two `f64` values using
    /// Algorithm 3 from Joldes et al. (2017).
    pub fn new_mul(x: f64, y: f64) -> TwoFloat {
        let (a, b) = two_prod(x, y);
        TwoFloat { hi: a, lo: b }
    }

    /// Creates a new `TwoFloat` by dividing two `f64` values using Algorithm
    /// 15 from Joldes et al. (2017) modified for the left-hand-side having a
    /// zero value in the low word.
    pub fn new_div(x: f64, y: f64) -> TwoFloat {
        let th = x / y;
        let (ph, pl) = two_prod(th, y);
        let dh = x - ph;
        let d = dh - pl;
        let tl = d / y;
        let (a, b) = fast_two_sum(th, tl);
        TwoFloat { hi: a, lo: b }
    }
}

impl Neg for TwoFloat {
    type Output = TwoFloat;

    /// Returns a new `TwoFloat` with the negated value of `self`.
    fn neg(self) -> Self::Output {
        TwoFloat { hi: -self.hi, lo: -self.lo }
    }
}

impl<'a> Neg for &'a TwoFloat {
    type Output = TwoFloat;

    /// Returns a new `TwoFloat` with the negated value of `self`.
    fn neg(self) -> Self::Output {
        TwoFloat { hi: -self.hi, lo: -self.lo }
    }
}

macro_rules! op_common_impl {
    ($op_assign:ident, $op_assign_fn:ident, $op:ident, $op_fn:ident, $lhs_i:ident, $rhs_i: ident, $rhs:ty, $code:block, $($meta:meta)*) => {
        impl $op_assign<$rhs> for TwoFloat {
            $(#[$meta])*
            fn $op_assign_fn(&mut self, $rhs_i: $rhs) {
                let $lhs_i = *self;
                let (a, b) = $code;
                self.hi = a;
                self.lo = b;
            }
        }

        impl $op<$rhs> for TwoFloat {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(mut self, $rhs_i: $rhs) -> Self::Output {
                let $lhs_i = self;
                let (a, b) = $code;
                self.hi = a;
                self.lo = b;
                self
            }
        }

        impl<'a> $op<$rhs> for &'a TwoFloat {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(self, $rhs_i: $rhs) -> Self::Output {
                let $lhs_i = self;
                let (a, b) = $code;
                TwoFloat { hi: a, lo: b }
            }
        }
    };
}

macro_rules! op_impl {
    ($op_assign:ident, $op_assign_fn:ident, $op:ident, $op_fn:ident, $(#[$meta:meta])* |$lhs_i:ident : &TwoFloat, $rhs_i:ident : &TwoFloat| $code:block) => {
        op_common_impl!($op_assign, $op_assign_fn, $op, $op_fn, $lhs_i, $rhs_i, TwoFloat, $code, $($meta)*);

        impl<'a> $op_assign<&'a TwoFloat> for TwoFloat {
            $(#[$meta])*
            fn $op_assign_fn(&mut self, $rhs_i: &'a TwoFloat) {
                let $lhs_i = *self;
                let (a, b) = $code;
                self.hi = a;
                self.lo = b;
            }
        }

        impl<'a> $op<&'a TwoFloat> for TwoFloat {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(mut self, $rhs_i: &'a TwoFloat) -> Self::Output {
                let $lhs_i = self;
                let (a, b) = $code;
                self.hi = a;
                self.lo = b;
                self
            }
        }

        impl<'a, 'b> $op<&'b TwoFloat> for &'a TwoFloat {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(self, $rhs_i: &'b TwoFloat) -> Self::Output {
                let $lhs_i = self;
                let (a, b) = $code;
                TwoFloat { hi: a, lo: b }
            }
        }
    };
    ($op_assign:ident, $op_assign_fn:ident, $op:ident, $op_fn:ident, $(#[$meta:meta])* |$lhs_i:ident : &TwoFloat, $rhs_i:ident : $rhs:ty| $code:block) => {
        op_common_impl!($op_assign, $op_assign_fn, $op, $op_fn, $lhs_i, $rhs_i, $rhs, $code, $($meta)*);

        impl $op<TwoFloat> for $rhs {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(self, mut $lhs_i: TwoFloat) -> Self::Output {
                $lhs_i.$op_assign_fn(self);
                $lhs_i
            }
        }

        impl<'a> $op<&'a TwoFloat> for $rhs {
            type Output = TwoFloat;

            $(#[$meta])*
            fn $op_fn(self, $lhs_i: &'a TwoFloat) -> Self::Output {
                let $rhs_i = self;
                let (a, b) = $code;
                TwoFloat { hi: a, lo: b }
            }
        }
    };
    ($op_assign:ident, $op_assign_fn:ident, $op:ident, $op_fn:ident,
        $(#[$fwd:meta])* |$lhs_i:ident : &TwoFloat, $rhs_i: ident : $rhs:ty| $code:block,
        $(#[$rev:meta])* |$lhs_rev_i:ident : $lhs_rev:ty, $rhs_rev_i:ident : &TwoFloat| $code_rev:block) => {
        op_common_impl!($op_assign, $op_assign_fn, $op, $op_fn, $lhs_i, $rhs_i, $rhs, $code, $($fwd)*);

        impl $op<TwoFloat> for $lhs_rev {
            type Output = TwoFloat;

            $(#[$rev])*
            fn $op_fn(self, mut $rhs_rev_i: TwoFloat) -> Self::Output {
                let $lhs_rev_i = self;
                let (a, b) = $code_rev;
                $rhs_i.hi = a;
                $rhs_i.lo = b;
                $rhs_i
            }
        }

        impl<'a> $op<&'a TwoFloat> for $lhs_rev {
            type Output = TwoFloat;

            $(#[$rev])*
            fn $op_fn(self, $rhs_rev_i: &'a TwoFloat) -> Self::Output {
                let $lhs_rev_i = self;
                let (a, b) = $code_rev;
                TwoFloat { hi: a, lo: b }
            }
        }
    };
}

op_impl!(AddAssign, add_assign, Add, add,
    /// Implements addition of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 4.
    |lhs: &TwoFloat, rhs: f64| {
        let (sh, sl) = two_sum(lhs.hi, rhs);
        let v = lhs.lo + sl;
        fast_two_sum(sh, v)
    });

op_impl!(SubAssign, sub_assign, Sub, sub,
    /// Implements subtraction of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 4 modified for negative right-hand side.
    |lhs: &TwoFloat, rhs: f64| {
        let (sh, sl) = two_diff(lhs.hi, rhs);
        let v = lhs.lo + sl;
        fast_two_sum(sh, v)
    },
    /// Implements subtraction of `f64` and `TwoFloat` using Joldes et al.
    /// (2017) Algorithm 4 modified for negative left-hand side.
    |lhs: f64, rhs: &TwoFloat| {
        let (sh, sl) = two_diff(lhs, rhs.hi);
        let v = sl - rhs.lo;
        fast_two_sum(sh, v)
    });

op_impl!(MulAssign, mul_assign, Mul, mul,
    /// Implements multiplication of `TwoFloat` and `f64` using Joldes et al.
    /// (2017) Algorithm 9.
    |lhs: &TwoFloat, rhs: f64| {
        let (ch, cl1) = two_prod(lhs.hi, rhs);
        let cl3 = lhs.lo.mul_add(rhs, cl1);
        fast_two_sum(ch, cl3)
    });

op_impl!(DivAssign, div_assign, Div, div,
    /// Implements division of `TwoFloat` and `f64` using Joldes et al. (2017)
    /// Algorithm 15
    |lhs: &TwoFloat, rhs: f64| {
        let th = lhs.hi / rhs;
        let (ph, pl) = two_prod(th, rhs);
        let dh = lhs.hi - ph;
        let dt = dh - pl;
        let d = dt + lhs.lo;
        let tl = d / rhs;
        fast_two_sum(th, tl)
    },
    /// Implements division of `f64` and `TwoFloat` using Joldes et al. (2017)
    /// Algorithm 18 modified for the left-hand side having a zero value in
    /// the low word.
    |lhs: f64, rhs: &TwoFloat| {
    let th = rhs.hi.recip();
    let rh = 1.0 - rhs.hi * th;
    let rl = -(rhs.lo * th);
    let (eh, el) = fast_two_sum(rh, rl);
    let e = TwoFloat { hi: eh, lo: el };
    let d = &e * th;
    let m = &d + th;
    let (ch, cl1) = two_prod(m.hi, lhs);
    let cl3 = m.lo.mul_add(lhs, cl1);
    fast_two_sum(ch, cl3)
});

op_impl!(RemAssign, rem_assign, Rem, rem,
    |lhs: &TwoFloat, rhs: f64| {
        let quotient = (lhs / rhs).trunc();
        (lhs - quotient * rhs).data()
    },
    |lhs: f64, rhs: &TwoFloat| {
        let quotient = (lhs / rhs).trunc();
        (lhs - quotient * rhs).data()
    }
);

op_impl!(AddAssign, add_assign, Add, add,
    /// Implements addition of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 6.
    |lhs: &TwoFloat, rhs: &TwoFloat| {
        let (sh, sl) = two_sum(lhs.hi, rhs.hi);
        let (th, tl) = two_sum(lhs.lo, rhs.lo);
        let c = sl + th;
        let (vh, vl) = fast_two_sum(sh, c);
        let w = tl + vl;
        fast_two_sum(vh, w)
    });

op_impl!(SubAssign, sub_assign, Sub, sub,
    /// Implements subtraction of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 6 modified for a negative right-hand side.
    |lhs: &TwoFloat, rhs: &TwoFloat| {
        let (sh, sl) = two_diff(lhs.hi, rhs.hi);
        let (th, tl) = two_diff(lhs.lo, rhs.lo);
        let c = sl + th;
        let (vh, vl) = fast_two_sum(sh, c);
        let w = tl + vl;
        fast_two_sum(vh, w)
    });

op_impl!(MulAssign, mul_assign, Mul, mul,
    /// Implements multiplication of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 12.
    |lhs: &TwoFloat, rhs: &TwoFloat| {
        let (ch, cl1) = two_prod(lhs.hi, rhs.hi);
        let tl0 = lhs.lo * rhs.lo;
        let tl1 = lhs.hi.mul_add(rhs.lo, tl0);
        let cl2 = lhs.lo.mul_add(rhs.hi, tl1);
        let cl3 = cl1 + cl2;
        fast_two_sum(ch, cl3)
    });

op_impl!(DivAssign, div_assign, Div, div,
    /// Implements division of two `TwoFloat` values using Joldes et al.
    /// (2017) Algorithm 18.
    |lhs: &TwoFloat, rhs: &TwoFloat| {
        let th = rhs.hi.recip();
        let rh = 1.0 - rhs.hi * th;
        let rl = -(rhs.lo * th);
        let (eh, el) = fast_two_sum(rh, rl);
        let e = TwoFloat { hi: eh, lo: el };
        let d = e * th;
        let m = d + th;
        let z = lhs * &m;
        (z.hi, z.lo)
    });

op_impl!(RemAssign, rem_assign, Rem, rem,
    |lhs: &TwoFloat, rhs: &TwoFloat| {
        let quotient = (lhs / rhs).trunc();
        (lhs - &quotient * rhs).data()
    });

#[cfg(test)]
mod tests {
    use super::*;

    use crate::base::*;
    use crate::test_util::*;

    use rand::Rng;

    randomized_test!(fast_two_sum_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a + b).is_finite() });
        let (hi, lo) = if a.abs() >= b.abs() { fast_two_sum(a, b) } else { fast_two_sum(b, a) };

        assert_eq_ulp!(hi, a + b, 1, "Incorrect result of fast_two_sum({}, {})", a, b);
        assert!(no_overlap(hi, lo), "Overlapping bits in two_sum({}, {})", a, b);
    });

    randomized_test!(two_sum_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a + b).is_finite() });
        let (hi, lo) = two_sum(a, b);

        assert_eq_ulp!(hi, a + b, 1, "Incorrect result of two_sum({}, {})", a, b);
        assert!(no_overlap(hi, lo), "Overlapping bits in two_sum({}, {})", a, b);
    });

    randomized_test!(two_diff_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a - b).is_finite() });
        let (hi, lo) = two_diff(a, b);

        assert_eq_ulp!(hi, a - b, 1, "Incorrect resut of two_diff({}, {})", a, b);
        assert!(no_overlap(hi, lo), "Overlapping bits in two_diff({}, {})", a, b);
    });

    randomized_test!(two_prod_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a * b).is_finite() });
        let (hi, lo) = two_prod(a, b);

        assert_eq_ulp!(hi, a * b, 1, "Incorrect result of two_prod({}, {})", a, b);
        assert!(no_overlap(hi, lo), "Overlapping bits in two_prod({}, {})", a, b);
    });

    randomized_test!(new_add_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a + b).is_finite() });
        let expected = two_sum(a, b);
        let actual = TwoFloat::new_add(a, b);
        assert_eq!(actual.hi, expected.0, "Incorrect result of new_add({}, {})", a, b);
        assert_eq!(actual.lo, expected.1, "Incorrect result of new_add({}, {})", a, b);
    });

    randomized_test!(new_sub_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a - b).is_finite() });
        let expected = two_diff(a, b);
        let actual = TwoFloat::new_sub(a, b);
        assert_eq!(actual.hi, expected.0, "Incorrect result of new_sub({}, {})", a, b);
        assert_eq!(actual.lo, expected.1, "Incorrect result of new_sub({}, {})", a, b);
    });

    randomized_test!(new_mul_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a * b).is_finite() });
        let expected = two_prod(a, b);
        let actual = TwoFloat::new_mul(a, b);
        assert_eq!(actual.hi, expected.0, "Incorrect result of new_mul({}, {})", a, b);
        assert_eq!(actual.lo, expected.1, "Incorrect result of new_mul({}, {})", a, b);
    });

    randomized_test!(new_div_test, |rng: F64Rand| {
        let (a, b) = get_valid_pair(rng, |a: f64, b: f64| { (a / b).is_finite() });
        let actual = TwoFloat::new_div(a, b);
        let ef = |a: f64, b: f64| -> u64 { let ab = a.to_bits(); let bb = b.to_bits(); if ab > bb { ab - bb } else { bb - ab }};
        assert_eq_ulp!(actual.hi, a / b, 10, "Incorrect result of new_div({}, {}) - {}", a, b, ef(actual.hi, a / b));
        assert!(no_overlap(actual.hi, actual.lo), "Overlapping bits in new_div({}, {})", a, b);
    });

    randomized_test!(neg_test, |rng: F64Rand| {
        let a = TwoFloat { hi: rng(), lo: rng() };
        let b = -a;
        assert_eq!(b.hi, -a.hi, "Negation does not negate high word");
        assert_eq!(b.lo, -a.lo, "Negation does not negate low word");

        let c = -b;
        assert_eq!(c, a, "Double negation does not result in original value");

        let b2 = -&a;
        assert_eq!(b, b2, "Mismatch between -TwoFloat and -&TwoFloat");
    });

    macro_rules! diff_test {
        (%, $expected:ident, $result:ident, $lhs:ident, $rhs:ident) => ({
            let true_difference: f64 = (($expected - $result) / $expected).into();
            let differences: [f64; 3] = [true_difference.abs(), (1.0 - true_difference).abs(), (1.0 + true_difference).abs()];
            let min_difference = *differences.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
            assert!(min_difference < 1e-10, "Out of range result of {:?} {} {:?}", $lhs, stringify!($op), $rhs);
        });
        ($op:tt, $expected:ident, $result:ident, $lhs:ident, $rhs:ident) => ({
            let difference = (($expected - $result) / $expected).abs();
            assert!(difference.abs() < 1e-10, "Out of range result of {:?} {} {:?}", $lhs, stringify!($op), $rhs);
        });
    }

    macro_rules! op_test_f64_common {
        ($op:tt, $op_assign:tt) => {
            randomized_test!(ref_overloads_left_test, |rng: F64Rand| {
                let c = rng();
                let (a, b) = get_valid_pair(rng, |x, y| { ((x + y) $op c).is_finite() && no_overlap(x, y) });
                let value = TwoFloat { hi: a, lo: b };
                let result1 = value $op c;
                if result1.is_valid() {
                    let result2 = &value $op c;
                    assert!(result2.is_valid(), "Result validity mismatch between TwoFloat {0} f64 and &TwoFloat {0} f64", stringify!($op));
                    assert_eq!(result1, result2, "Mismatch between TwoFloat {0} f64 and &TwoFloat {0} f64", stringify!($op));

                    let mut result3 = value;
                    result3 $op_assign c;
                    assert!(result3.is_valid(), "Result validity mismatch between TwoFloat {} f64 and TwoFloat {} f64", stringify!($op), stringify!($op_assign));
                    assert_eq!(result1, result3, "Mismatch between TwoFloat {} f64 and TwoFloat {} f64", stringify!($op), stringify!($op_assign));
                }
            });

            randomized_test!(ref_overloads_right_test, |rng: F64Rand| {
                let c = rng();
                let (a, b) = get_valid_pair(rng, |x, y| { (c $op (x + y)).is_finite() && no_overlap(x, y) });
                let value = TwoFloat { hi: a, lo: b };
                let result1 = c $op value;
                if result1.is_valid() {
                    let result2 = c $op &value;
                    assert!(result2.is_valid(), "Result validity mismatch between f64 {0} TwoFloat and &f64 {0} &TwoFloat", stringify!($op));
                    assert_eq!(result1, result2, "Mismatch between f64 {0} TwoFloat and &TwoFloat {0} &TwoFloat", stringify!($op));
                }
            });

            #[test]
            fn value_left_test() {
                let mut rng = rand::thread_rng();
                let high_range = rand::distributions::Uniform::new_inclusive(-1e50f64, 1e50f64);
                let low_range = rand::distributions::Uniform::new_inclusive(-1f64, 1f64);

                loop {
                    let a = rng.sample(high_range);
                    let (b, c) = loop {
                        let b = rng.sample(high_range);
                        let c = rng.sample(low_range);
                        if no_overlap(b, c) { break (b, c); }
                    };

                    let value = TwoFloat { hi: b, lo: c };
                    let expected = a $op b;
                    if (!expected.is_finite()) { continue; }
                    let result = a $op value;
                    if (!result.is_valid()) { continue; }

                    assert!(no_overlap(result.hi, result.lo), "Result of {} {} {:?} contained overlap", a, stringify!($op), value);

                    diff_test!($op, expected, result, a, value);
                    break;
                }
            }
        };
    }

    macro_rules! op_test_f64 {
        ($test_name:ident, $op:tt, $op_assign:tt, true) => {
            #[cfg(test)]
            mod $test_name {
                use super::*;

                op_test_f64_common!($op, $op_assign);

                #[test]
                fn reversible_test() {
                    let mut rng = rand::thread_rng();
                    let high_range = rand::distributions::Uniform::new_inclusive(-1e50f64, 1e50f64);
                    let low_range = rand::distributions::Uniform::new_inclusive(-1f64, 1f64);
                    loop {
                        let a = rng.sample(high_range);
                        let (b, c) = loop {
                            let b = rng.sample(high_range);
                            let c = rng.sample(low_range);
                            if no_overlap(b, c) { break (b, c); }
                        };

                        let value = TwoFloat { hi: b, lo: c };
                        let expected = a $op value;
                        if (!expected.is_valid()) { continue; }

                        let result = value $op a;
                        assert!(no_overlap(result.hi, result.lo), "Result of {:?} {} {} contained overlap", value, stringify!($op), a);
                        assert_eq!(result, expected, "Operation {:?} {} {} gave different result to reversed", value, stringify!($op), a);
                        break;
                    }
                }
            }
        };
        ($test_name:ident, $op:tt, $op_assign:tt, false) => {
            #[cfg(test)]
            mod $test_name {
                use super::*;

                op_test_f64_common!($op, $op_assign);

                #[test]
                fn value_right_test() {
                    let mut rng = rand::thread_rng();
                    let high_range = rand::distributions::Uniform::new_inclusive(-1e50f64, 1e50f64);
                    let low_range = rand::distributions::Uniform::new_inclusive(-1f64, 1f64);
                    loop {
                        let a = rng.sample(high_range);
                        let (b, c) = loop {
                            let b = rng.sample(high_range);
                            let c = rng.sample(low_range);
                            if no_overlap(b, c) { break (b, c); }
                        };

                        let value = TwoFloat { hi: b, lo: c };
                        let expected = b $op a;
                        if (!expected.is_finite()) { continue; }
                        let result = value $op a;
                        if (!result.is_valid()) { continue; }

                        assert!(no_overlap(result.hi, result.lo), "Result of {:?} {} {} contained overlap", value, stringify!($op), a);

                        diff_test!($op, expected, result, value, a);
                        break;
                    }
                }
            }
        };
    }

    op_test_f64!(add_f64_test, +, +=, true);
    op_test_f64!(sub_f64_test, -, -=, false);
    op_test_f64!(mul_f64_test, *, *=, true);
    op_test_f64!(div_f64_test, /, /=, false);
    op_test_f64!(rem_f64_test, %, %=, false);

    macro_rules! op_test {
        ($test_name:ident, $op:tt, $op_assign:tt) => {
            #[cfg(test)]
            mod $test_name {
                use super::*;

                randomized_test!(ref_overloads_test, |rng: F64Rand| {
                    let (a, b) = get_valid_pair(rng, |x, y| { no_overlap(x, y) });
                    let (c, d) = get_valid_pair(rng, |x, y| { ((x + y) $op a).is_finite() && no_overlap(x, y) });
                    let value1 = TwoFloat { hi: a, lo: b };
                    let value2 = TwoFloat { hi: c, lo: d };
                    let result1 = value1 $op value2;
                    if result1.is_valid() {
                        let result2 = &value1 $op value2;
                        assert!(result2.is_valid(), "Result validity mismatch between TwoFloat {0} TwoFloat and &TwoFloat {0} TwoFloat", stringify!($op));
                        assert_eq!(result1, result2, "Mismatch between TwoFloat {0} TwoFloat and &TwoFloat {0} TwoFloat", stringify!($op));

                        let result3 = value1 $op &value2;
                        assert!(result3.is_valid(), "Result validity mismatch between TwoFloat {0} TwoFloat and TwoFloat {0} &TwoFloat", stringify!($op));
                        assert_eq!(result1, result3, "Mismatch between TwoFloat {0} TwoFloat and TwoFloat {0} &TwoFloat", stringify!($op));

                        let result4 = &value1 $op &value2;
                        assert!(result4.is_valid(), "Result validity mismatch between TwoFloat {0} TwoFloat and &TwoFloat {0} &TwoFloat", stringify!($op));
                        assert_eq!(result1, result4, "Mismatch between TwoFloat {0} TwoFloat and &TwoFloat {0} &TwoFloat", stringify!($op));

                        let mut result5 = value1;
                        result5 $op_assign value2;
                        assert!(result5.is_valid(), "Result validity mismatch between TwoFloat {} TwoFloat and TwoFloat {} TwoFloat", stringify!($op), stringify!($op_assign));
                        assert_eq!(result1, result5, "Mismatch between TwoFloat {} TwoFloat and TwoFloat {} TwoFloat", stringify!($op), stringify!($op_assign));

                        let mut result6 = value1;
                        result6 $op_assign &value2;
                        assert!(result6.is_valid(), "Result validity mismatch between TwoFloat {} TwoFloat and TwoFloat {} &TwoFloat", stringify!($op), stringify!($op_assign));
                        assert_eq!(result1, result6, "Mismatch between TwoFloat {} TwoFloat and TwoFloat {} &TwoFloat", stringify!($op), stringify!($op_assign));
                    }
                });

                #[test]
                fn value_test() {
                    let mut rng = rand::thread_rng();
                    let high_range = rand::distributions::Uniform::new_inclusive(-1e50f64, 1e50f64);
                    let low_range = rand::distributions::Uniform::new_inclusive(-1f64, 1f64);

                    loop {
                        let (a, b) = loop {
                            let x = rng.sample(high_range);
                            let y = rng.sample(low_range);
                            if no_overlap(x, y) { break (x, y); }
                        };
                        let (c, d) = loop {
                            let x = rng.sample(high_range);
                            let y = rng.sample(low_range);
                            if no_overlap(x, y) { break (x, y); }
                        };

                        let value1 = TwoFloat { hi: a, lo: b };
                        let value2 = TwoFloat { hi: c, lo: d };

                        let expected = a $op c;
                        if (!expected.is_finite()) { continue; }
                        let result = value1 $op value2;
                        if (!result.is_valid()) { continue; }

                        assert!(no_overlap(result.hi, result.lo), "Result of {:?} {} {:?} contained overlap", value1, stringify!($op), value2);

                        diff_test!($op, expected, result, value1, value2);
                        break;
                    }
                }
            }
        };
    }

    op_test!(add_test, +, +=);
    op_test!(sub_test, -, -=);
    op_test!(mul_test, *, *=);
    op_test!(div_test, /, /=);
    op_test!(rem_test, %, %=);
}
