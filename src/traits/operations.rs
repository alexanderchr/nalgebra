//! Low level operations on vectors and matrices.



/// Result of a partial ordering.
#[deriving(TotalEq, Eq, Encodable, Decodable, Clone, Show)]
pub enum PartialOrdering {
    /// Result of a strict comparison.
    PartialLess,
    /// Equality relationship.
    PartialEqual,
    /// Result of a strict comparison.
    PartialGreater,
    /// Result of a comparison between two objects that are not comparable.
    NotComparable
}

impl PartialOrdering {
    /// Returns `true` if `self` is equal to `Equal`.
    pub fn is_eq(&self) -> bool {
        *self == PartialEqual
    }

    /// Returns `true` if `self` is equal to `Less`.
    pub fn is_lt(&self) -> bool {
        *self == PartialLess
    }

    /// Returns `true` if `self` is equal to `Less` or `Equal`.
    pub fn is_le(&self) -> bool {
        *self == PartialLess || *self == PartialEqual
    }

    /// Returns `true` if `self` is equal to `Greater`.
    pub fn is_gt(&self) -> bool {
        *self == PartialGreater
    }

    /// Returns `true` if `self` is equal to `Greater` or `Equal`.
    pub fn is_ge(&self) -> bool {
        *self == PartialGreater || *self == PartialEqual
    }

    /// Returns `true` if `self` is equal to `NotComparable`.
    pub fn is_not_comparable(&self) -> bool {
        *self == NotComparable
    }

    /// Creates a `PartialOrdering` from an `Ordering`.
    pub fn from_ordering(ord: Ordering) -> PartialOrdering {
        match ord {
            Less    => PartialLess,
            Equal   => PartialEqual,
            Greater => PartialGreater
        }
    }

    /// Converts this `PartialOrdering` to an `Ordering`.
    ///
    /// Returns `None` if `self` is `NotComparable`.
    pub fn to_ordering(self) -> Option<Ordering> {
        match self {
            PartialLess    => Some(Less),
            PartialEqual   => Some(Equal),
            PartialGreater => Some(Greater),
            NotComparable  => None
        }
    }
}

/// Pointwise ordering operations.
pub trait PartialOrd {
    /// Returns the infimum of `a` and `b`.
    fn inf(a: &Self, b: &Self) -> Self;

    /// Returns the supremum of `a` and `b`.
    fn sup(a: &Self, b: &Self) -> Self;

    /// Compare `a` and `b` using a partial ordering relation.
    fn partial_cmp(a: &Self, b: &Self) -> PartialOrdering;

    /// Returns `true` iff `a` and `b` are comparable and `a <= b`.
    #[inline]
    fn partial_le(a: &Self, b: &Self) -> bool {
        PartialOrd::partial_cmp(a, b).is_le()
    }

    /// Returns `true` iff `a` and `b` are comparable and `a < b`.
    #[inline]
    fn partial_lt(a: &Self, b: &Self) -> bool {
        PartialOrd::partial_cmp(a, b).is_lt()
    }

    /// Returns `true` iff `a` and `b` are comparable and `a >= b`.
    #[inline]
    fn partial_ge(a: &Self, b: &Self) -> bool {
        PartialOrd::partial_cmp(a, b).is_ge()
    }

    /// Returns `true` iff `a` and `b` are comparable and `a > b`.
    #[inline]
    fn partial_gt(a: &Self, b: &Self) -> bool {
        PartialOrd::partial_cmp(a, b).is_gt()
    }

    /// Return the minimum of `a` and `b` if they are comparable.
    #[inline]
    fn partial_min<'a>(a: &'a Self, b: &'a Self) -> Option<&'a Self> {
        match PartialOrd::partial_cmp(a, b) {
            PartialLess | PartialEqual => Some(a),
            PartialGreater             => Some(b),
            NotComparable              => None
        }
    }

    /// Return the maximum of `a` and `b` if they are comparable.
    #[inline]
    fn partial_max<'a>(a: &'a Self, b: &'a Self) -> Option<&'a Self> {
        match PartialOrd::partial_cmp(a, b) {
            PartialGreater | PartialEqual => Some(a),
            PartialLess                   => Some(b),
            NotComparable                 => None
        }
    }

    /// Clamp `value` between `min` and `max`. Returns `None` if `value` is not comparable to
    /// `min` or `max`.
    #[inline]
    fn partial_clamp<'a>(value: &'a Self, min: &'a Self, max: &'a Self) -> Option<&'a Self> {
        let v_min = PartialOrd::partial_cmp(value, min);
        let v_max = PartialOrd::partial_cmp(value, max);

        if v_min.is_not_comparable() || v_max.is_not_comparable() {
            None
        }
        else {
            if v_min.is_lt() {
                Some(min)
            }
            else if v_max.is_gt() {
                Some(max)
            }
            else {
                Some(value)
            }
        }
    }
}

/// Trait for testing approximate equality
pub trait ApproxEq<Eps> {
    /// Default epsilon for approximation.
    fn approx_epsilon(unused_self: Option<Self>) -> Eps;

    /// Tests approximate equality using a custom epsilon.
    fn approx_eq_eps(a: &Self, other: &Self, epsilon: &Eps) -> bool;

    /// Tests approximate equality.
    #[inline]
    fn approx_eq(a: &Self, b: &Self) -> bool {
        ApproxEq::approx_eq_eps(a, b, &ApproxEq::approx_epsilon(None::<Self>))
    }
}

impl ApproxEq<f32> for f32 {
    #[inline]
    fn approx_epsilon(_: Option<f32>) -> f32 {
        1.0e-6
    }

    #[inline]
    fn approx_eq_eps(a: &f32, b: &f32, epsilon: &f32) -> bool {
        (*a - *b).abs() < *epsilon
    }
}

impl ApproxEq<f64> for f64 {
    #[inline]
    fn approx_epsilon(_: Option<f64>) -> f64 {
        1.0e-6
    }

    #[inline]
    fn approx_eq_eps(a: &f64, b: &f64, approx_epsilon: &f64) -> bool {
        (*a - *b).abs() < *approx_epsilon
    }
}

/// Trait of objects having an absolute value.
/// This is useful if the object does not have the same type as its absolute value.
pub trait Absolute<A> {
    /// Computes some absolute value of this object.
    /// Typically, this will make all component of a matrix or vector positive.
    fn abs(&Self) -> A;
}

/// Trait of objects having an inverse. Typically used to implement matrix inverse.
pub trait Inv {
    /// Returns the inverse of `self`.
    fn inv_cpy(m: &Self) -> Option<Self>;

    /// In-place version of `inverse`.
    fn inv(&mut self) -> bool;
}

/// Trait of objects which can be transposed.
pub trait Transpose {
    /// Computes the transpose of a matrix.
    fn transpose_cpy(m: &Self) -> Self;

    /// In-place version of `transposed`.
    fn transpose(&mut self);
}

/// Traits of objects having an outer product.
pub trait Outer<M> {
    /// Computes the outer product: `self * other`
    fn outer(a: &Self, b: &Self) -> M;
}

/// Trait for computing the covariance of a set of data.
pub trait Cov<M> {
    /// Computes the covariance of the obsevations stored by `self`:
    ///
    ///   * For matrices, observations are stored in its rows.
    ///   * For vectors, observations are stored in its components (thus are 1-dimensional).
    fn cov(&Self) -> M;

    /// Computes the covariance of the obsevations stored by `self`:
    ///
    ///   * For matrices, observations are stored in its rows.
    ///   * For vectors, observations are stored in its components (thus are 1-dimensional).
    fn cov_to(m: &Self, out: &mut M) {
        *out = Cov::cov(m)
    }
}

/// Trait for computing the covariance of a set of data.
pub trait Mean<N> {
    /// Computes the mean of the observations stored by `self`.
    /// 
    ///   * For matrices, observations are stored in its rows.
    ///   * For vectors, observations are stored in its components (thus are 1-dimensional).
    fn mean(&Self) -> N;
}


// /// Cholesky decomposition.
// pub trait Chol {
//     /// Performs the cholesky decomposition on `self`. The resulting upper-triangular matrix is
//     /// returned. Returns `None` if the matrix is not symetric positive-definite.
//     fn chol(self) -> Option<Self>;
// 
//     /// Performs the cholesky decomposition on `self`. The resulting upper-triangular matrix is
//     /// written to a given parameter. If the decomposition fails `false` is returned; the state of
//     /// the output parameter is undefined.
//     fn chol_to(&self, out: &mut Self) -> bool {
//         match self.chol() {
//             None => false,
//             Some(decomp) => {
//                 *out = decomp;
//                 true
//             }
//         }
//     }
// }

// XXX: those two traits should not exist since there is generalized operator overloading of Add
// and Sub.
// However, using the same trait multiple time as a trait bound (ex: impl<T: Add<N, V> + Add<V, V>)
// does not work properly, mainly because the way we are doing generalized operator overloading is
// verry hacky.
//
// Hopefully, this will be fixed on a future version of the language!
/// Trait of objects having a right multiplication with another element.
pub trait RMul<V> {
    /// Computes `self * v`
    fn rmul(&self, v: &V) -> V;
}

impl<M: Mul<T, T>, T> RMul<T> for M {
    fn rmul(&self, v: &T) -> T {
        *self * *v
    }
}

/// Trait of objects having a left multiplication with another element.
pub trait LMul<V> {
    /// Computes `v * self`
    fn lmul(&self, &V) -> V;
}

impl<T: Mul<M, T>, M> LMul<T> for M {
    fn lmul(&self, v: &T) -> T {
        *v * *self
    }
}

// XXX: those traits should not exist since there is generalized operator overloading of Add and
// Sub.
// However, using the same trait multiple time as a trait bound (ex: impl<T: Add<N, V> + Add<V, V>)
// does not work properly, mainly because the way we are doing generalized operator overloading is
// verry hacky.
//
// Hopefully, this will be fixed on a future version of the language!
/// Trait of objects having an addition with a scalar.
pub trait ScalarAdd<N> {
    /// Gets the result of `self + n`.
    fn add_s(&self, n: &N) -> Self;
}

impl<N, T: Add<N, T>> ScalarAdd<N> for T {
    /// Gets the result of `self + n`.
    fn add_s(&self, n: &N) -> T {
        *self + *n
    }
}

/**
 * Trait of objects having a subtraction with a scalar.
 */
pub trait ScalarSub<N> {
    /// Gets the result of `self - n`.
    fn sub_s(&self, n: &N) -> Self;
}

impl<N, T: Sub<N, T>> ScalarSub<N> for T {
    /// Gets the result of `self - n`.
    fn sub_s(&self, n: &N) -> T {
        *self - *n
    }
}
