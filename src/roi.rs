use std::ops::RangeInclusive;

use crate::{peak::Peak, spec::Spectrum};
#[derive(PartialEq,Debug)]
pub struct ROI<'s>{
    pub source: &'s Spectrum,
    pub range: RangeInclusive<u16>,
    pub peaks: Vec<Peak>,
    pub backgound: Vec<f64>,
}