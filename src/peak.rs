use std::f64::consts::{LN_2, PI};

use nalgebra::{DMatrix, MatrixN, RowDVector, SVector, Vector, VectorN};

#[derive(PartialEq, Debug)]
pub struct Peak {
    magnitude: f64,
    mu: f64,
    sigma: f64,
}

impl Peak {
    pub fn new(magnitude: f64, mu: f64, sigma: f64) -> Self {
        Peak {
            magnitude,
            mu,
            sigma,
        }
    }
    pub fn standard(mu: f64) -> Self {
        Self {
            magnitude: 1.0,
            mu,
            sigma: 1.0,
        }
    }
    pub fn gauss(&self, x: f64) -> f64 {
        self.magnitude
            * (-0.5 * ((x - self.mu) / self.sigma).powi(2)).exp()
            * (1.0 / (self.sigma * (2.0 * PI).sqrt()))
    }
    pub fn integral(&self) -> f64 {
        self.magnitude
    }
    pub fn get_mu(&self) -> &f64 {
        &self.mu
    }
    pub fn get_sigma(&self) -> &f64 {
        &self.sigma
    }
    pub fn get_magnitude(&self) -> &f64 {
        &self.magnitude
    }
    pub fn fhwm(&self) -> f64 {
        2.0 * self.sigma * (2.0 * LN_2).sqrt()
    }
}
pub struct Calibration {
    x_points: Vec<f64>,
    y_points: Vec<f64>,
    coeficients: Vec<f64>,
}

impl Calibration {
    pub fn new() -> Self {
        Self {
            x_points: Vec::new(),
            y_points: Vec::new(),
            coeficients: Vec::new(),
        }
    }

    pub fn set_x(&mut self, x_arg: Vec<f64>) {
        self.x_points = x_arg
    }

    pub fn set_y(&mut self, y_arg: Vec<f64>) {
        self.y_points = y_arg
    }
    ///Currendtly does not work
    pub fn _calibrate(&mut self, n: usize) {
        println!("NOT YET IMPLEMENTED CORRECTLY!");
        return;
        let mut data: Vec<f64> = vec![];
        for i in (0..n) {
            for item in self.x_points.clone() {
                data.push(item.powi(i.try_into().unwrap()));
            }
        }
        let mut xij = DMatrix::from_column_slice(self.x_points.len(), n, data.as_slice());
        println!("XIJ = {}", xij);
        let yi = DMatrix::from_column_slice(self.y_points.len(), 1, self.y_points.as_slice());
        print!("YI = {}", yi);
        let xji = xij.clone().transpose();
        println!("XJI = {}", xji);
        let xij_xji = xij * xji.clone();
        println!("XIJ * XJI = {}", xij_xji);
        let Some(inv_xij_xji) = xij_xji.try_inverse() else {
            println!("Singular calibration matrix!");
            return;
        };
        println!("(XIJ * XJI)^-1 = {}", inv_xij_xji);
        let trnsf = inv_xij_xji * xji;
        println!("(XIJ * XJI)^-1 * XJI = {}", trnsf);
        let betta = trnsf * yi;
        println!("B = {}", betta);
        self.coeficients = betta.as_slice().to_owned();
    }

    pub fn calibrate_linear(&mut self) {
        if !self.is_calibratable() {
            println!("No full (x,y) points range!");
            return;
        }
        let m = self.x_points.len();
        let summ_x: f64 = self.x_points.clone().into_iter().sum();
        let summ_y: f64 = self.y_points.clone().into_iter().sum();
        let summ_xy: f64 = (0..self.x_points.len())
            .into_iter()
            .map(|i| self.x_points.clone()[i] * self.y_points.clone()[i])
            .sum();
        let summ_x2: f64 = (0..self.x_points.len())
            .into_iter()
            .map(|i| self.x_points.clone()[i].powi(2))
            .sum();
        let a = (m as f64 * summ_xy - summ_x * summ_y) / (m as f64 * summ_x2 - summ_x.powi(2));
        let b = summ_y / m as f64 - (a / m as f64) * summ_x;
        self.coeficients = vec![b, a];
    }

    pub fn channel_to_energy(&self, position: f64) -> f64 {
        (0..self.coeficients.len())
            .into_iter()
            .map(|i| {
                self.coeficients.clone()[i]
                    * position.powi({
                        let Ok(pow) = i32::try_from(i) else {
                            println!("failed to convert!");
                            return 0.0;
                        };
                        pow
                    })
            })
            .sum()
    }

    pub fn add(&mut self, x: f64, y: f64) {
        self.x_points.push(x);
        self.y_points.push(y);
    }

    pub fn is_calibrated(&self) -> bool {
        !self.coeficients.is_empty()
    }
    pub fn is_calibratable(&self) -> bool {
        (self.x_points.len() == self.y_points.len()) && (self.x_points.len() != 0)
    }
    pub fn get_coeficients_linear(&self) -> &Vec<f64> {
        &self.coeficients
    }
}
