//! Arithmetic of formal power series.
//!
//! # Note on complexity
//! *We only consider cases where the precision is a power of 2.
//! If the precision is not a power of 2, the complexity is twice as bad.*
//!
//! We may assume the complexity $\mathcal{M}(d)$ of multiplication of two polynomials of degree $d$ and
//! the complexity $\mathcal{F}(d)$ of FFT/IFFT of length $d$ satisfies the following property:
//!
//! $$
//! \begin{aligned}
//! \mathcal{M}(d) &= \Theta(d\log d) , \mathcal{F}(d) = \Theta(d\log d), \\\\
//! \mathcal{M}(d) &= 3 \mathcal{F}(2d) + O(d) \ \left( = 6 \mathcal{F}(d) + O(d) \right).
//! \end{aligned}
//! $$
//!
//! If each step in Newton's method is performed in $a\mathcal{M}(d)$ time (where $d$ is the resulting precision of each iteration),
//! then the total complexity is $2a\mathcal{F}(d) + O(d)$.
//!
//! We omit $O(d)$ terms when it is not important.
//!
//! # Table of contents
//! $O(d)$ is omitted here.
//!
//! | Name | Expression | Complexity |
//! | ---- | ---------- | ---------- |
//! | [`fps_inv`] | $f^{-1} \mod x^d$ | $2\mathcal{M}(d)$ |
//! | [`fps_sqrt`] | $f^{1/2} \mod x^d$ | $6\mathcal{M}(d)$ |
use fp2::fft;
use fp2::fps_mul;
use fp2::ifft;
use fp2::Fp;
use fp2::PrimitiveRoot;
use std::iter::repeat;

/// Inverse FPS of `f`.
///
/// # Requirements
/// $f_0 \ne 0$
///
/// # Returns
/// $f^{-1} \mod x^d$
///
/// # Complexity
/// $2\mathcal{M}(d) + O(d)$.
///
/// It is because $3\mathcal{F}(2d)$ in each iteration.
///
/// # Examples
/// ```
/// use fp2::fp;
/// use fps::fps_inv;
/// let g = fps_inv::<998244353>(&[fp!(1), fp!(2)], 4);
/// assert_eq!(g, vec![fp!(1), fp!(-2), fp!(4), fp!(-8)]);
/// ```
pub fn fps_inv<const P: u64>(f: &[Fp<P>], precision: usize) -> Vec<Fp<P>>
where
    (): PrimitiveRoot<P>,
{
    assert!(
        !f.is_empty() && f[0] != Fp::new(0),
        "The constant term must be nonzero."
    );
    let mut g = vec![f[0].inv()];
    while g.len() < precision {
        g = {
            let precision = g.len() * 2;
            let fft_size = precision * 2;
            let mut f = f
                .iter()
                .copied()
                .take(precision)
                .chain(repeat(Fp::new(0)))
                .take(fft_size)
                .collect::<Vec<_>>();
            g.resize(fft_size, Fp::new(0));
            fft(&mut f);
            fft(&mut g);
            let mut result = f
                .iter()
                .zip(&g)
                .map(|(&f, &g)| g * (-f * g + 2))
                .collect::<Vec<_>>();
            ifft(&mut result);
            result.truncate(precision);
            result
        };
    }
    g.truncate(precision);
    g
}
/// Square root FPS of `f`.
///
/// # Requirements
/// $f_0 = 1$
///
/// # Returns
/// $f^{1/2} \mod x^d$
///
/// # Complexity
/// $6\mathcal{M}(d) + O(d)$
///
/// It is because it takes
/// - $\mathcal{M}(d): multiplication of polynomials of degree $d$,
/// - $2\mathcal{M}(d): inverse of polynomials of degree $d$,
///
/// and the sum of the above is $3\mathcal{M}(d)$.
///
/// # Implementation
/// In each iteration, it performs
/// - an inverse of precision $d$; it takes $2\mathcal{M}(d) + O(d)$ time,
/// - a multiplication of two FPSs of precision $d$; it takes $\mathcal{M}(d) + O(d)$ time,
///
/// The sum of the above is $3\mathcal{M}(d) + O(d)$, so it takes $6\mathcal{M}(d) + O(d)$ time in
/// total.
///
/// # Examples
/// ```
/// use fp2::fp;
/// use fps::fps_sqrt;
/// let g = fps_sqrt::<998244353>(&[fp!(1), fp!(2)], 4);
/// assert_eq!(g, vec![fp!(1), fp!(1), -fp!(2).inv(), fp!(2).inv()]);
/// ```
pub fn fps_sqrt<const P: u64>(f: &[Fp<P>], precision: usize) -> Vec<Fp<P>>
where
    (): PrimitiveRoot<P>,
{
    assert!(
        !f.is_empty() && f[0] == Fp::new(1),
        "The constant term must be 1."
    );
    let mut g = vec![f[0].inv()];
    let inv2 = Fp::new(2).inv();
    while g.len() < precision {
        g = {
            let precision = g.len() * 2;
            let fft_size = precision * 2;
            let f = f
                .iter()
                .copied()
                .take(precision)
                .chain(repeat(Fp::new(0)))
                .take(fft_size)
                .collect::<Vec<_>>();
            let mut g_inv = fps_inv(&g, precision);
            g_inv.resize(fft_size, Fp::new(0));
            let mut f_div_g = fps_mul(&f, &g_inv);
            f_div_g.truncate(precision);
            f_div_g
                .iter()
                .zip(g.iter().copied().chain(repeat(Fp::new(0))))
                .map(|(&f_div_g, g)| (f_div_g + g) * inv2)
                .collect()
        };
    }
    g.truncate(precision);
    g
}

#[cfg(test)]
mod tests {
    use super::*;
    use fp2::fps_mul;
    use rand::rngs::StdRng;
    use rand::Rng;
    use rand::SeedableRng;
    use std::iter;

    type Fp = fp2::Fp<998244353>;

    fn random_fps_one(rng: &mut StdRng, precision: usize) -> Vec<Fp> {
        iter::once(Fp::new(1))
            .chain(iter::repeat_with(|| Fp::new(rng.gen_range(0..100))))
            .take(precision)
            .collect()
    }

    #[test]
    fn test_fps_inv_random() {
        const PRECISION: usize = 40;
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..20 {
            let f = random_fps_one(&mut rng, PRECISION);
            let g = fps_inv(&f, PRECISION);
            assert!(g.len() <= PRECISION);
            let mut result = fps_mul(&f, &g);
            result.truncate(PRECISION);
            let mut expected = vec![Fp::new(0); PRECISION];
            expected[0] = Fp::new(1);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_fps_sqrt_random() {
        const PRECISION: usize = 40;
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..20 {
            let f = random_fps_one(&mut rng, PRECISION);
            let g = fps_sqrt(&f, PRECISION);
            assert!(g.len() <= PRECISION);
            let mut result = fps_mul(&g, &g);
            result.truncate(PRECISION);
            assert_eq!(result, f);
        }
    }
}
