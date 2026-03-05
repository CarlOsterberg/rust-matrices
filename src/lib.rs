#![no_std]

use core::fmt;
use core::ops::*;

pub trait Numeric:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + AddAssign
    + Copy
    + Default
    + PartialEq
    + fmt::Display
{
}

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + AddAssign
        + Copy
        + Default
        + PartialEq
        + fmt::Display,
> Numeric for T
{
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix<T: Numeric, const M: usize, const N: usize> {
    array: [[T; N]; M],
}

impl<T: Numeric, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new() -> Matrix<T, M, N> {
        Matrix {
            array: [[T::default(); N]; M],
        }
    }

    pub fn from_array(array: [[T; N]; M]) -> Matrix<T, M, N> {
        Matrix { array }
    }

    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut array = [[T::default(); M]; N];
        for m in 0..M {
            for n in 0..N {
                array[n][m] = self.array[m][n];
            }
        }
        Matrix { array }
    }
}

impl<T: Numeric, const M: usize, const N: usize> PartialEq for Matrix<T, M, N> {
    fn eq(&self, rhs: &Matrix<T, M, N>) -> bool {
        self.array == rhs.array
    }

    fn ne(&self, rhs: &Matrix<T, M, N>) -> bool {
        self.array != rhs.array
    }
}

impl<T: Numeric, const M: usize, const N: usize> Add for Matrix<T, M, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut array = [[T::default(); N]; M];
        for m in 0..M {
            for n in 0..N {
                array[m][n] = self.array[m][n] + rhs.array[m][n];
            }
        }
        Matrix { array }
    }
}

impl<T: Numeric, const M: usize, const N: usize> Sub for Matrix<T, M, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut array = [[T::default(); N]; M];
        for m in 0..M {
            for n in 0..N {
                array[m][n] = self.array[m][n] - rhs.array[m][n];
            }
        }
        Matrix { array }
    }
}

impl<T: Numeric, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        let mut array = [[T::default(); N]; M];
        for m in 0..M {
            for n in 0..N {
                array[m][n] = self.array[m][n] * scalar;
            }
        }
        Matrix { array }
    }
}

impl<T: Numeric, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>>
    for Matrix<T, M, N>
{
    type Output = Matrix<T, M, P>;

    fn mul(self, rhs: Matrix<T, N, P>) -> Matrix<T, M, P> {
        let mut array = [[T::default(); P]; M];
        for m in 0..M {
            for p in 0..P {
                for n in 0..N {
                    array[m][p] += self.array[m][n] * rhs.array[n][p];
                }
            }
        }
        Matrix { array }
    }
}

// Orphan rule means we have to explicitly create LHS scalar implementation
// for each type we want to support.
macro_rules! impl_scalar_lhs {
    ($($t:ty),*) => {
        $(
            impl<const M: usize, const N: usize> Mul<Matrix<$t, M, N>> for $t {
                type Output = Matrix<$t, M, N>;

                fn mul(self, rhs: Matrix<$t, M, N>) -> Matrix<$t, M, N> {
                    rhs * self
                }
            }
        )*
    };
}

impl_scalar_lhs!(i8, i16, i32, i64, f32, f64, u8, u16, u32, u64);

impl<T: Numeric, const M: usize, const N: usize> fmt::Display for Matrix<T, M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix {}x{}\n", M, N)?;
        for m in 0..M {
            if m > 0 {
                write!(f, "\n")?;
            }
            for n in 0..N {
                if m == 0 && n == 0 {
                    write!(f, "⌈\t")?;
                } else if m == M - 1 && n == 0 {
                    write!(f, "⌊\t")?;
                } else if n == 0 {
                    write!(f, "|\t")?;
                }
                write!(f, "{}\t", self.array[m][n])?;
                if m == 0 && n == N - 1 {
                    write!(f, "⌉")?;
                } else if m == M - 1 && n == N - 1 {
                    write!(f, "⌋")?;
                } else if n == N - 1 {
                    write!(f, "|")?;
                }
            }
        }
        write!(f, "")
    }
}
