use std::{cmp, f64::consts::PI, iter, vec};

use ark_bls12_381::Fq as F;
use ark_ff::{Field, Fp, FpConfig, PrimeField};
use ark_poly::{univariate::SparsePolynomial, Polynomial};
use nalgebra::{zero, DMatrix, DVector};
use varpro::model;

use crate::peak::Peak;

#[derive(Debug)]
pub enum DataError {
    FittingSizeError,
}

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
            println!("Finished with by hi_square, n = {}", n);
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
            let size = cmp::min(c[mu].len(), c[mu - 1].len());
            let all_eq = (0..size).all(|j| {
                println!("{j}");
                c[mu][j] - sigma[mu][j] < c[mu - 1][j] && c[mu - 1][j] < c[mu][j] + sigma[mu][j]
            });
            if all_eq {
                if sigma[mu][n] / c[mu][n] < 10.0 {
                    if ud {
                        println!("Finished with ud = true, n = {}", n);
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
    println!("FINISHED WITH MU OVERLOAD! N = {}", n);

    z[mu].clone()
}

pub fn get_background_step_function(data: &[u32]) -> Vec<f64> {
    if data.len() < 10 {
        return Vec::from_iter(iter::repeat(data[0].try_into().unwrap()).take(data.len()));
    }
    let l = data.len();
    let y = Vec::from_iter(data.into_iter().map(|x| f64::from(*x)));

    let b_m: f64 = (0..5).into_iter().map(|j| y[j]).sum::<f64>() / 5.0;
    let b_n: f64 = (l - 5..l).into_iter().map(|j| y[j]).sum::<f64>() / 5.0;

    fn p_j(j: usize, b_m: f64, b_n: f64, l: usize, y: Vec<f64>) -> f64 {
        y[j] - ((b_n - b_m) / l as f64 * j as f64 + b_m)
    }

    let div: f64 = (0..l)
        .into_iter()
        .map(|j| p_j(j, b_m, b_n, l, y.clone()))
        .sum();

    let mut result: Vec<f64> = vec![];

    for i in 0..l {
        let add: f64 = (0..i)
            .into_iter()
            .map(|j| p_j(j, b_m, b_n, l, y.clone()))
            .sum();
        let sub: f64 = (i..l)
            .into_iter()
            .map(|j| p_j(j, b_m, b_n, l, y.clone()))
            .sum();
        let f_i = 0.5 * ((b_m + b_n) + (b_n - b_m) * (add - sub) / div);
        result.push(f_i);
    }
    result
}

pub fn gauss(x: &DVector<f64>, mu: f64, sigma: f64) -> DVector<f64> {
    x.map(|x| (-0.5 * ((x - mu) / sigma).powi(2)).exp() / (sigma * (2.0 * PI).sqrt()))
}

pub fn gauss_dmu(x: &DVector<f64>, mu: f64, sigma: f64) -> DVector<f64> {
    x.map(|x| {
        ((x - mu) / (sigma.powi(3) * (2.0 * PI).sqrt())) * (-0.5 * ((x - mu) / sigma).powi(2)).exp()
    })
}

pub fn gauss_dsigma(x: &DVector<f64>, mu: f64, sigma: f64) -> DVector<f64> {
    x.map(|x| {
        (1.0 / (sigma.powi(2) * (2.0 * PI).sqrt()))
            * (((x - mu) / sigma).powi(2) - 1.0)
            * (-0.5 * ((x - mu) / sigma).powi(2)).exp()
    })
}

pub fn fit_peaks(
    n: usize,
    initial_mu: impl Iterator<Item = f64> + ExactSizeIterator,
    peak_data: Vec<f64>,
) -> Result<Vec<Peak>, DataError> {
    use varpro::prelude::*;
    use varpro::solvers::levmar::{LevMarProblemBuilder, LevMarSolver};

    if n != initial_mu.len() {
        return Err(DataError::FittingSizeError);
    }
    let x = DVector::from_vec(Vec::from_iter(
        (0..i32::try_from(peak_data.len()).unwrap())
            .into_iter()
            .map(|i| f64::try_from(i).unwrap()),
    ));
    let mut model_params: Vec<String> = vec![];
    for i in (0..n) {
        model_params.push(format!("mu{}", i + 1));
        model_params.push(format!("sigma{}", i + 1));
    }
    println!("{:?}", &model_params[..]);
    let mut model = SeparableModelBuilder::<f64>::new(&model_params)
        .function([&model_params[0], &model_params[1]], gauss)
        .partial_deriv(&model_params[0], gauss_dmu)
        .partial_deriv(&model_params[1], gauss_dsigma);
    if n > 1 {
        for i in 1..n.try_into().unwrap() {
            model = model
                .function([&model_params[2 * i], &model_params[2 * i + 1]], gauss)
                .partial_deriv(&model_params[2 * i], gauss_dmu)
                .partial_deriv(&model_params[2 * i + 1], gauss_dsigma);
        }
    }
    let mut param: Vec<f64> = vec![];
    for p in initial_mu {
        param.push(p);
        param.push(5.0);
    }
    let model = model
        .initial_parameters(param)
        .independent_variable(x)
        .build()
        .unwrap();
    let problem = LevMarProblemBuilder::new(model)
        .observations(DVector::from_vec(peak_data))
        .build()
        .unwrap();
    let fit_result = LevMarSolver::default().fit(problem);
    let mut res: Vec<Peak> = vec![];
    match fit_result {
        Ok(data) => {
            let magnitudes = data.linear_coefficients().unwrap();
            let peak_params = data.nonlinear_parameters();
            let peak_params = Vec::from_iter(peak_params.into_iter());
            let magnitudes = Vec::from_iter(magnitudes.into_iter());
            println!(
                "L_params {}; L_magns {}",
                peak_params.len(),
                magnitudes.len()
            );

            for i in 0..n.try_into().unwrap() {
                res.push(Peak::new(
                    *magnitudes[i],
                    *peak_params[2 * i],
                    *peak_params[2 * i + 1],
                ));
            }
        }
        Err(e) => {
            println!("Should be able to fit!\n Error: {:?}", e);
        }
    }
    Ok(res)
}
