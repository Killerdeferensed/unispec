use std::ops::RangeInclusive;

use crate::{
    data_processing::{fit_peaks, get_background_step_function},
    peak::Peak,
    spec::Spectrum,
};
#[derive(PartialEq, Debug)]
pub struct ROI {
    range: RangeInclusive<usize>,
    peaks: Vec<Peak>,
    backgound: Vec<f64>,
}

impl ROI {
    pub fn add_peak(&mut self, spec: &[u32], mu: f64) {
        self.peaks.push(Peak::standard(mu))
    }

    pub fn create(spect: &Spectrum, range: RangeInclusive<usize>) -> ROI {
        let slc: &[u32] = &spect.data[range.clone()];
        Self {
            range,
            peaks: Vec::new(),
            backgound: get_background_step_function(slc),
        }
    }
    pub fn create_with_peaks(
        spect: &Spectrum,
        range: RangeInclusive<usize>,
        arg_peaks: Vec<f64>,
    ) -> ROI {
        let slc: &[u32] = &spect.data[range.clone()];
        let mut peaks: Vec<Peak> = vec![];
        for tpeak in arg_peaks {
            peaks.push(Peak::standard(tpeak));
        }
        Self {
            range,
            peaks,
            backgound: get_background_step_function(slc),
        }
    }
    pub fn fit_all(&mut self, peak_data: Vec<f64>) {
        let n = self.peaks.len();
        let start = self.range.start().clone();
        let initial_mu = self
            .peaks
            .iter()
            .map(|peak| peak.get_mu() - start.clone() as f64);
        let new_peaks = fit_peaks(n, initial_mu, peak_data)
            .expect("Should have number of peaks exact number of peaks");
        self.peaks = (0..n)
            .into_iter()
            .map(|i| {
                Peak::new(
                    new_peaks[i].get_magnitude().clone(),
                    new_peaks[i].get_mu().clone() + start.clone() as f64,
                    new_peaks[i].get_sigma().clone(),
                )
            })
            .collect()
    }
    pub fn get_range(&self) -> &RangeInclusive<usize> {
        &self.range
    }
    pub fn get_bg(&self) -> &Vec<f64> {
        &self.backgound
    }
    pub fn get_peaks(&self) -> &Vec<Peak> {
        &self.peaks
    }
}
