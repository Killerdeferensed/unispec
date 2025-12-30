use crate::roi::ROI;
use std::{num::ParseIntError, ops::RangeInclusive};
#[derive(PartialEq, Debug)]
///This thing makes you life extremely hard
///
/// **DEAL WITH IT!**
pub struct Spectrum {
    pub data: Vec<u32>,
}
#[derive(PartialEq, Eq, Debug)]
pub enum SpectrumError {
    NaN(ParseIntError, usize),
    ValueOverflow,
}

///Stores info on spectrum and its usage
///
///Provides GUI friendly interface
pub struct SpectrumContext {
    spect: Spectrum,
    rois: Vec<ROI>,
    active: usize,
}

impl SpectrumContext {
    pub fn new(spect: Spectrum) -> Self {
        Self {
            spect,
            rois: Vec::new(),
            active: usize::MAX,
        }
    }
    fn get_spectrum(&self) -> &Spectrum {
        &self.spect
    }

    pub fn get_roi(&self) -> &[ROI] {
        self.rois.as_slice()
    }

    fn get_active(&self) -> Option<&ROI> {
        self.rois.get(self.active)
    }

    fn select_active(&mut self, i: usize) {
        self.active = i;
    }

    pub fn add_roi(&mut self, range: RangeInclusive<usize>) {
        let new_roi: ROI = ROI::create(&self.spect, range);
        self.rois.push(new_roi);
    }
    pub fn add_roi_with_peaks(&mut self, range: RangeInclusive<usize>, arg_peaks: Vec<f64>) {
        let new_roi: ROI = ROI::create_with_peaks(&self.spect, range, arg_peaks);
        self.rois.push(new_roi);
    }
    pub fn remove_roi(&mut self, index:usize){
        self.rois.remove(index);
    }
    fn add_peak(&mut self, mu: f64) {
        self.rois[self.active].add_peak(&self.spect.data, mu);
    }
    pub fn get_spectrum_end(&self) -> usize{
        self.spect.data.len() - 1
    }
    pub fn get_data(&self) -> &[u32]{
        &self.spect.data
    }
    pub fn fit_everything(&mut self){
        let data = &self.spect.data;
        for roi in self.rois.iter_mut(){
            let raw_data:Vec<f64> = data[roi.get_range().clone()].to_owned().into_iter().map(f64::from).collect();
            let peak_data = (0..roi.get_bg().len()).into_iter().map(|i| {
                raw_data[i] - roi.get_bg().clone()[i] 
            }).collect();
            roi.fit_all(peak_data);
        }
    }
}

impl Spectrum {
    pub fn try_get_from<S: AsRef<str>, I: Iterator<Item = S>>(
        value: I,
    ) -> Result<Self, SpectrumError> {
        let data = value
            .enumerate()
            .skip_while(|s| s.1.as_ref().starts_with('#'))
            .filter(|s| !s.1.as_ref().trim().is_empty())
            .map(|s| {
                let mut iter = s.1.as_ref().split(' ').filter(|s| !s.is_empty());
                let first: u32 = iter
                    .next()
                    .expect("There is at leats one element in iter!")
                    .parse()
                    .map_err(|err| SpectrumError::NaN(err, s.0))?;
                if let Some(second) = iter.next() {
                    if iter.next().is_some() {
                        return Err(SpectrumError::ValueOverflow);
                    }
                    let second: u32 = second.parse().map_err(|err| SpectrumError::NaN(err, s.0))?;
                    Ok::<_, SpectrumError>(second)
                } else {
                    Ok(first)
                }
            })
            .collect::<Result<Vec<_>, SpectrumError>>()?;
        Ok(Spectrum {
            data: data,
            //rois: vec![]
        })
    }

    pub fn new(arg: Vec<u32>) -> Self {
        Spectrum {
            data: arg,
            //rois: vec![]
        }
    }
}
