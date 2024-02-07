use crate::roi::ROI;
use std::{num::ParseIntError, ops::RangeInclusive};
#[derive(PartialEq, Debug)]
pub struct Spectrum {
    pub data: Vec<u32>,
    //pub rois: Vec<ROI>,
}
#[derive(PartialEq, Eq, Debug)]
pub enum SpectrumError {
    NaN(ParseIntError, usize),
    ValueOverflow,
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

    pub fn get_peaks<'s>(&'s self, roi: RangeInclusive<usize>) -> ROI<'s> {
        let slc: &[u32] = &self.data[roi];
        let data: Vec<u32> = slc.to_vec();
        todo!()
    }
}
