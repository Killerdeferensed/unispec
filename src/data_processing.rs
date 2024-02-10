use std::{iter, vec};

use ark_bls12_381::Fq as F;
use ark_ff::{Field, Fp, FpConfig, PrimeField};
use ark_poly::{univariate::SparsePolynomial, Polynomial};
use nalgebra::{zero, DMatrix, DVector};

pub fn get_background(data: &[u32]) -> Vec<f64> {
    let mut n: usize = 1;
    let mut m: usize = 0;
    let mut mu: usize = 0;
    let mut f: usize = 0;

    let mut ud = false;

    let l = data.len();

    let mut e: Vec<f64> = vec![]; // HI SQUARE

    let y = Vec::from_iter(data.into_iter().map(|x| f64::from(*x))); // Data Vector
    let x = Vec::from_iter(
        //Indexes vector (may have no use)
        (0..u32::try_from(l).expect("There should be less than 4bil points, lol"))
            .into_iter()
            .map(f64::from),
    );

    let mut w: Vec<Vec<f64>> = vec![]; // Weights matrix
    let mut gamma: Vec<Vec<f64>> = vec![vec![]]; // Orthogonality matrix
    let mut a: Vec<Vec<f64>> = vec![vec![]]; // Polynomial parameter [a] matrix
    let mut b: Vec<Vec<f64>> = vec![vec![]]; // Polynomial parameter [b] matrix
    let mut c: Vec<Vec<f64>> = vec![vec![]]; // Coefficient matrix

    let mut z: Vec<Vec<f64>> = vec![]; //Resulting background
    let mut sigma: Vec<Vec<f64>> = vec![]; // See sigma.rs

    // p[mu][j+1][x[i]] = (x[i] - a[mu][j])*p[mu][j][x[i]] - b[mu][j]*p[mu][j-1][x[i]]
    // Beacuse indexes are always equdistant x[i] = i. It`s needed only to convert usize to u32
    let mut p: Vec<Vec<Vec<f64>>> = vec![vec![]]; // Polynomials matrix L*N*Mu (May need fixing)

    while mu < u8::MAX.into() {
        if mu == 0 {
            w.push(Vec::from_iter((0..l).into_iter().map(|x| 1.0 / y[x])));
        }
        for j in 0..=n {
            if j < 2 {
                p[mu].push(Vec::from_iter(iter::repeat(1.0).take(l)));
            }
            gamma[mu].push(
                (0..l)
                    .into_iter()
                    .map(|i| w[mu][i] * p[mu][j][i].powi(2))
                    .sum(),
            );
            a[mu].push(
                (0..l)
                    .into_iter()
                    .map(|i| w[mu][i] * x[i] * p[mu][j][i].powi(2))
                    .sum(),
            );
            if j == 0 {
                b[mu].push(0.0);
            } else {
                b[mu].push(
                    (0..l)
                        .into_iter()
                        .map(|i| w[mu][i] * x[i] * p[mu][j][i] * p[mu][j - 1][i] / gamma[mu][j - 1])
                        .sum(),
                );
            }
            c[mu].push(
                (0..l)
                    .into_iter()
                    .map(|i| w[mu][i] * y[i] * p[mu][j][i])
                    .sum(),
            );
            if j > 0 {
                let p_mu_j = Vec::from_iter((0..l).into_iter().map(|i| p.clone()[mu][j][i])); //Matrix p row with index j Because Rust
                let p_mu_j_1 = Vec::from_iter((0..l).into_iter().map(|i| p.clone()[mu][j - 1][i])); //Matrix p row with index j-1 Because Rust
                p[mu].push(Vec::from_iter(
                    (0..l)
                        .into_iter()
                        .map(|i| (x[i] - a[mu][j]) * p_mu_j[i] - b[mu][j] * p_mu_j_1[i]),
                ));
                drop(p_mu_j);
                drop(p_mu_j_1);
            }
        }
        z.push(Vec::from_iter((0..l).into_iter().map(|i| {
            (0..usize::try_from(n).expect("REALLY?"))
                .into_iter()
                .map(|j| c[mu][j] * p[mu][j][i])
                .sum()
        })));
        e.push(
            (0..l)
                .into_iter()
                .map(|i| w[mu][i] * (y[i] - z[mu][i]).powi(2))
                .sum(),
        );
        f = l - n - m;
        if e[mu] < (f + m as usize) as f64 + { (2 * f) as f64 }.powi(1 / 2) {
            // println!("n = {}", n);
            return z[mu].clone();
        };
        sigma.push(Vec::from_iter(
            (0..=n)
                .into_iter()
                .map(|j| (e[mu] / f as f64 / gamma[mu][j]).powf(0.5)),
        ));
        m = 0;
        w.push(Vec::from_iter((0..l).into_iter().map(|i| {
            if y[i] > z[mu][i] + 2.0 * z[mu][i].powf(0.5) {
                return 1.0 / (y[i] - z[mu][i]).powi(2);
            }
            m = m + 1;
            1.0 / z[mu][i]
        })));
        if mu > 0 {
            let all_eq = (0..c[mu - 1].len()).all(|j| {
                c[mu][j] - sigma[mu][j] < c[mu - 1][j] && c[mu - 1][j] < c[mu][j] + sigma[mu][j]
            });
            if all_eq {
                if sigma[mu][n] / c[mu][n] < 1.0 {
                    if ud {
                        // println!("n = {}",n);
                        return z[mu].clone();
                    }
                    n = n + 1;
                } else {
                    n = n - 1;
                    ud = true;
                }
            }
        }
        mu = mu + 1;
        p.push(vec![]);
        gamma.push(vec![]);
        a.push(vec![]);
        b.push(vec![]);
        c.push(vec![]);
    }
    // println!("FINISHED WITH MU OVERLOAD! N = {}", n);

    z[mu].clone()
}
