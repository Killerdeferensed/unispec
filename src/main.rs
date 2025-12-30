use std::{
    cell::RefCell,
    cmp::min,
    fs,
    io::{stdin, stdout, Stdout, Write},
    ops::RangeInclusive,
    path::Path,
    rc::Rc,
    usize,
};

use chrono::{DateTime, Datelike, Local};
use file_loader::load_from_file;
use nalgebra::iter;
use peak::Calibration;
use plotly::{Plot, Scatter, Trace};
use rfd::FileDialog;
use rust_xlsxwriter::{workbook, Workbook};
use spec::{Spectrum, SpectrumContext};
mod data_processing;
pub mod file_loader;
mod peak;
mod roi;
mod spec;
mod tests;
/// Shell commands description
/// `help\list\commands`
/// > Do I really need to xplain this?
///
/// `spec <filename/everything/open>`
/// > Reads file and tries to add it to a spectrum list.
/// > Loaded file is searched in input folder
/// > open - chose spectrums via file explorer(May not be implemented!)
///
/// `roi <spectrum number/all> <from> <to> <[peaks positions in spectrum]>`
/// > Creates range of interest in specified spectrum
///
/// `tree <spectrum number[optional]>`
/// > Shows loaded spectrums and their ranges of interests
///
/// `plt <spectrum number/all> <roi number/all>`
/// > Plotting command
/// > All ROI can be plotted only for a single spectrum due to excessive information given
/// > Same goes for plotting one specified ROI for a multiple spectums 
///
/// `rm <spectrum number> <roi number>`
/// > Removes loaded spectrums or their ROIs. Provide no ROI number to remove whole spectrum.
/// > Provide no numbers at all to clear everything
///
/// `execute`
/// > Bring imperial inqusition to your place.
/// > **Confess your sins and beg for mercy!**
/// 
/// `calib <add/lin> <[Channel number] [Respective energy]>`
/// > Calibration command
/// > `add` - add a point for E=E(C) linear approximation
/// > `lin` - calibrate E=E(C) linearry
///
/// `export <filemode>`
/// > export fitting result into designated file
/// > currently implemented for -> excel
///
/// `quit/q`
/// > The end of all your problems
///
/// `sudo makemeasandwich`
/// > does not yet implemented
fn main() {
    if !Path::new("./input").is_dir() {
        match fs::create_dir("./input") {
            Ok(_) => (),
            Err(err) => {
                println!("Couldn`t find&create directory for input files (command: spec).\n{err}");
            }
        }
    }
    if !Path::new("./output").is_dir() {
        match fs::create_dir("./output") {
            Ok(_) => (),
            Err(err) => {
                println!("Couldn`t find&create directory for output files (commands: execute/export).\n{err}");
            }
        }
    }
    let mut calib = Calibration::new();
    // let mut specs: Vec<SpectrumContext> = Vec::new();
    // let mut specs = Rc::new(RefCell::new(specs));
    // let specs = Rc::clone(&specs);
    let mut specs: Vec<SpectrumContext> = Vec::new();
    loop {
        print!("unispec>> ");
        match stdout().flush() {
            Ok(_) => (),
            Err(e) => {
                println!(":/\nYou must be the unluckiest person living or the luckiest son of a gun in entire Milky Way galaxy\n{e}");
            }
        };

        let mut com_line = String::new();
        match stdin().read_line(&mut com_line) {
            Ok(_n) => (),
            Err(e) => {
                println!("You, BOZO!, somehow managed to screw up shell command line: {e}");
            }
        }
        let com_line = com_line.replace(&['\r', '\n'][..], "");
        let com_line: Vec<&str> = com_line.split(' ').collect();

        match com_line[0] {
            "spec" => {
                if com_line.len() == 1 {
                    println!("spec <filename> -- reads file and tries to add it to a spectrum list.\nLoaded file is searched in input folder.");
                    continue;
                }
                match com_line[1] {
                    "everything" => {
                        let Ok(inp_dir) = fs::read_dir("./input") else {
                            println!("Unable to open input folder\nMake sure it is present in installation directory");
                            continue;
                        };
                        for item in inp_dir {
                            match item {
                                Ok(content) => {
                                    if let Some(path) = content.file_name().to_str() {
                                        let data = load_from_file(format!("./input/{}", path));
                                        match data {
                                            Ok(d) => {
                                                let Ok(spec) = Spectrum::try_get_from(d.lines()) else {
                                                    println!("Can`t create spectrum!");
                                                    continue;
                                                };
                                                specs.push(SpectrumContext::new(spec));
                                            }
                                            Err(e) => {
                                                println!("Can`t open file {e}");
                                                continue;
                                            }
                                        }
                                        println!("Successfully loaded {path}")
                                    } else {
                                        println!("Filename is empty somehow");
                                        continue;
                                    }
                                }
                                Err(e) => {
                                    println!("Couldn`t extract file!\n{e}");
                                    continue;
                                }
                            }
                        }
                        continue;
                    }
                    smthng => {
                        let content = match load_from_file(format!("./input/{}", smthng)) {
                            Ok(content) => content,
                            Err(error) => {
                                println!("Can`t open file: {error}");
                                continue;
                            }
                        };
                        let spect = match Spectrum::try_get_from(content.lines()) {
                            Ok(content) => content,
                            Err(err) => {
                                println!("Can`t create new spectrum: {err:?}");
                                continue;
                            }
                        };
                        specs.push(SpectrumContext::new(spect));
                        continue;
                    }
                }
            }
            "roi" => {
                if com_line.len() == 1 {
                    println!("roi <spectrum number/all> <from> <to> <[peaks positions in spectrum]>\nCreates range of interest in specified spectrum");
                    continue;
                }
                if com_line.len() < 4 {
                    println!("Please specify ranges to create new ROI\nroi {} <from> <to> <[peaks positions in spectrum]>", com_line[1]);
                    continue;
                }
                if com_line.len() < 5 {
                    println!("At least one peak must be given to create new ROI\nroi {} {} {} <[peaks positions in spectrum]>", com_line[1],com_line[2],com_line[3]);
                    continue;
                }
                match com_line[1] {
                    "all" => {
                        let Ok(from) = com_line[2].parse::<usize>() else {
                            println!("Please give a valid number to start new ROI");
                            continue;
                        };
                        let Ok(mut to) = com_line[3].parse::<usize>() else {
                            println!("Please give a valid number to end new ROI");
                            continue;
                        };
                        let arg_peaks: Vec<f64> = (4..com_line.len()).into_iter().map(|s| {
                            let Ok(content) = com_line[s].parse::<f64>() else {
                                println!("Please give a valid number for peak position in spectrum: {}",com_line[s]);
                                return 0.0;
                            };
                            content
                        }).collect();

                        if !is_peaks_in_roi(&arg_peaks, from, to) {
                            println!("Peaks out of ROI range!");
                            continue;
                        }
                        if from > to {
                            println!("End of ROI must be further than its start!");
                            continue;
                        }
                        for item in specs.iter_mut() {
                            if to > item.get_spectrum_end() {
                                to = item.get_spectrum_end();
                            }
                            item.add_roi_with_peaks(
                                RangeInclusive::new(from, to),
                                arg_peaks.clone(),
                            );
                            // item.add_roi(RangeInclusive::new(from, to));
                        }
                        continue;
                    }
                    something => {
                        let Ok(index) = something.parse::<usize>() else {
                            println!("Please provide correct number of spectrum to create new ROI\n{something} is not a valid spectrum number");
                            continue;
                        };
                        if index < 1 || index > specs.len() {
                            println!("Can`t find a spectrum with number {index}");
                            continue;
                        }
                        let Ok(from) = com_line[2].parse::<usize>() else {
                            println!("Please give a valid number to start new ROI");
                            continue;
                        };
                        let Ok(mut to) = com_line[3].parse::<usize>() else {
                            println!("Please give a valid number to end new ROI");
                            continue;
                        };
                        let arg_peaks: Vec<f64> = (4..com_line.len()).into_iter().map(|s| {
                            let Ok(content) = com_line[s].parse::<f64>() else {
                                println!("Please give a valid number for peak position in spectrum: {}\nConseder removing this ROI!",com_line[s]);
                                return 0.0;
                            };
                            content
                        }).collect();

                        if !is_peaks_in_roi(&arg_peaks, from, to) {
                            println!("Peaks out of ROI range!");
                            continue;
                        }
                        if from > to {
                            println!("End of ROI must be further than its start!");
                            continue;
                        }
                        if to > specs[index - 1].get_spectrum_end() {
                            to = specs[index - 1].get_spectrum_end();
                        }
                        specs[index - 1]
                            .add_roi_with_peaks(RangeInclusive::new(from, to), arg_peaks);
                        continue;
                    }
                }
            }
            "tree" => {
                if specs.len() == 0 {
                    println!("No spetrums are currently loaded!");
                    continue;
                }
                if com_line.len() > 1 {
                    let Ok(index) = com_line[1].parse::<usize>() else {
                        let mut iter_spec: u32 = 1;
                        println!("Currently loaded spectrums");
                        for item in &specs {
                            println!("╠Sepctrum No {}", iter_spec);
                            let mut iter_roi: u32 = 1;
                            for r in item.get_roi() {
                                println!(
                                    "║╠ROI {}: {} - {}:",
                                    iter_roi,
                                    r.get_range().start(),
                                    r.get_range().end()
                                );
                                for p in r.get_peaks() {
                                    println!("║║╠Peak: mu = {}, sigma = {}", p.get_mu(), p.get_sigma());
                                }
                                iter_roi = iter_roi + 1;
                            }
                            iter_spec = iter_spec + 1;
                        }
                        continue;
                    };
                    if index < 1 || index > specs.len() {
                        println!("Could not find spectrum with number {index} to show its tree");
                        continue;
                    }
                    println!("Sepctrum No {index}");
                    let mut iter_roi: u32 = 1;
                    for r in specs[index-1].get_roi() {
                        println!(
                            "╠ROI {}: {} - {}:",
                            iter_roi,
                            r.get_range().start(),
                            r.get_range().end()
                        );
                        for p in r.get_peaks() {
                            println!("║╠Peak: mu = {}, sigma = {}", p.get_mu(), p.get_sigma());
                        }
                        iter_roi = iter_roi + 1;
                    }
                } else {
                    let mut iter_spec: u32 = 1;
                    println!("Currently loaded spectrums");
                    for item in &specs {
                        println!("╠Sepctrum No {}", iter_spec);
                        let mut iter_roi: u32 = 1;
                        for r in item.get_roi() {
                            println!(
                                "║╠ROI {}: {} - {}:",
                                iter_roi,
                                r.get_range().start(),
                                r.get_range().end()
                            );
                            for p in r.get_peaks() {
                                println!("║║╠Peak: mu = {}, sigma = {}", p.get_mu(), p.get_sigma());
                            }
                            iter_roi = iter_roi + 1;
                        }
                        iter_spec = iter_spec + 1;
                    }
                    continue;
                }
            }
            "plt" => {
                if com_line.len() == 1 {
                    println!("plt <spectrum number/all> <roi number/all>\nPlotting command\nWill graphicaly show current state of your work");
                    continue;
                }
                if com_line.len() == 2 {
                    match com_line[1] {
                        "all" => {
                            if specs.is_empty() {
                                println!("Nothing to plot!");
                                continue;
                            }
                            let mut plot = Plot::new();
                            let traces = Vec::from_iter((0..specs.len()).into_iter().map(|s| {
                                Scatter::new(
                                    (0..specs[s].get_spectrum_end() + 1).collect(),
                                    specs[s].get_data().to_owned(),
                                ).text(format!("Spectrum {s}")).name(format!("Spectrum {s}")) as Box<dyn Trace>
                            }));
                            plot.add_traces(traces);
                            // let trace = Scatter::new(vec![1,4,5,1], vec![10,15,16,8]);
                            // plot.add_trace(trace);
                            plot.show();
                            continue;
                        }
                        smthn => {
                            let Ok(index) = smthn.parse::<usize>() else {
                                println!("Provide valid spectrum number");
                                continue;
                            };
                            let mut plot = Plot::new();
                            let trace = Scatter::new(
                                (0..specs[index-1].get_spectrum_end() + 1).collect(),
                                specs[index-1].get_data().to_owned(),
                            );
                            plot.add_trace(trace);
                            plot.show();
                            continue;
                        }
                    }
                }
                if com_line.len() > 2{
                    match com_line[1] {
                        "all" => {
                            println!("Due to excessive visual informatioin ROIS can`t be plotted for ALL spectrums");
                            continue;
                        }
                        arg => {
                            let Ok(index_spec) = arg.parse::<usize>() else {
                                println!("Provide a valid spectrum number");
                                continue;
                            };
                            if index_spec < 1 || index_spec > specs.len(){
                                println!("No spectrums found");
                                continue;
                            }
                            match com_line[2] {
                                "all" => {
                                    let mut plot = Plot::new();
                                    for roi in specs[index_spec-1].get_roi(){
                                        let data = specs[index_spec-1].get_data()[roi.get_range().clone()].to_owned().into_iter().map(f64::from).collect();
                                        let bg = roi.get_bg().clone();
                                        let peak_data: Vec<f64> = Vec::from_iter(roi.get_range().clone().into_iter().map(|s| {
                                            (0..roi.get_peaks().len()).into_iter().map(|j| roi.get_peaks()[j].gauss(s as f64)).sum()
                                        }));
                                        let range:Vec<usize> = roi.get_range().clone().into_iter().collect();
                                        let traces = vec![
                                            Scatter::new(range.clone(), data).name("Spectrum").text("Spectrum") as Box<dyn Trace>,
                                            Scatter::new(range.clone(), bg).name("Background").text("Background") as Box<dyn Trace>,
                                            Scatter::new(range.clone(), peak_data).name("Peaks").text("Peaks") as Box<dyn Trace>
                                        ];
                                        plot.add_traces(traces);
                                    }
                                    plot.show();
                                    continue;
                                }
                                smthng => {
                                    let Ok(index_roi) = smthng.parse::<usize>() else {
                                        println!("Provide a valid TOI number");
                                        continue;
                                    };
                                    let mut plot = Plot::new();
                                    let roi = &specs[index_spec-1].get_roi()[index_roi - 1];
                                    let data:Vec<f64> = specs[index_spec-1].get_data()[roi.get_range().clone()].to_owned().into_iter().map(f64::from).collect();
                                    let bg = roi.get_bg().clone();
                                    let peak_data: Vec<f64> = Vec::from_iter(roi.get_range().clone().into_iter().map(|s| {
                                        (0..roi.get_peaks().len()).into_iter().map(|j| roi.get_peaks()[j].gauss(s as f64)).sum()
                                    }));
                                    let range:Vec<usize> = roi.get_range().clone().into_iter().collect();
                                    let traces = vec![
                                        Scatter::new(range.clone(), data).name("Spectrum").text("Spectrum") as Box<dyn Trace>,
                                        Scatter::new(range.clone(), bg).name("Background").text("Background") as Box<dyn Trace>,
                                        Scatter::new(range.clone(), peak_data).name("Peaks").text("Peaks") as Box<dyn Trace>
                                    ];
                                    plot.add_traces(traces);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            "rm" => {
                if com_line.len() == 1 {
                    println!(
                        "WARNING! You are about to clear every loaded spectrum\n Proceed? (Y):"
                    );
                    let mut ystr = String::new();
                    match stdin().read_line(&mut ystr) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("{e}");
                            continue;
                        }
                    }
                    let ystr = ystr.replace(&['\r', '\n'][..], "");
                    let ystr: Vec<&str> = ystr.split(" ").collect();
                    if ystr[0] == "Y" {
                        specs.clear();
                        println!("Clear!");
                        continue;
                    } else {
                        println!("Aborting!");
                        continue;
                    }
                }
                if com_line.len() == 2 {
                    let Ok(index) = com_line[1].parse::<usize>() else {
                        println!("Please provide a valid number of spectrum to remove");
                        continue;
                    };
                    if index < 1 || index > specs.len() {
                        println!("This spectrum does not exists");
                        continue;
                    }
                    specs.remove(index - 1);
                    println!("Succesfully removed!");
                    continue;
                }
                if com_line.len() > 2 {
                    let Ok(spec_index) = com_line[1].parse::<usize>() else {
                        println!("Please provide a valid number of spectrum to romove ROI from");
                        continue;
                    };
                    let Ok(roi_index) = com_line[2].parse::<usize>() else {
                        println!("Please provide a valid number of ROI to rmove from spectrum number {spec_index}");
                        continue;
                    };
                    if spec_index < 1 || spec_index > specs.len() {
                        println!("This spectrum does not exist");
                        continue;
                    }
                    if roi_index < 1 || roi_index > specs[spec_index-1].get_roi().len() {
                        println!("This ROI does not exist");
                        continue;
                    }
                    specs[spec_index-1].remove_roi(roi_index-1);
                    continue;
                }
            }
            "execute" => {
                for item in specs.iter_mut(){
                    item.fit_everything();
                }
                continue;
            }
            "calib" => {
                if com_line.len() == 1{
                    println!("calib <add/lin> <[Channel number] [Respective energy (keV)]>");
                    println!("Calibration command");
                    println!("`add` - add a point for E=E(C) linear approximation");
                    println!("`lin` - calibrate E=E(C) linearry");
                    continue;
                }
                match com_line[1] {
                    "add" => {
                        if com_line.len() < 4{
                            println!("Provide a valid number of arguments");
                            continue;
                        }
                        let Ok(x) = com_line[2].parse::<f64>() else {
                            println!("Provide a valid calibration possition");
                            continue;
                        };
                        let Ok(y) = com_line[3].parse::<f64>() else {
                            println!("Provide a valid energy (keV)");
                            continue;
                        };
                        calib.add(x, y);
                        continue;
                    }
                    "lin" => {
                        calib.calibrate_linear();
                        let coefs = calib.get_coeficients_linear();
                        println!("Calibrated:\n E(C) = {} * C + {}", coefs[1], coefs[0]);
                        continue;
                    }
                    _ => {
                        println!("Unknown argument!");
                        continue;
                    }
                }
            }
            "export" => {
                if com_line.len() < 2 {
                    println!("export <filemode>");
                    println!("export data to file");
                    println!("currently avaliadble modes:\nexcel");
                    continue;
                }
                match com_line[1] {
                    "excel" => {
                        if !calib.is_calibrated(){
                            println!("Please, calibrate spectrum!");
                            continue;
                        }
                        let mut wb = Workbook::new();
                        let mut spec_iter = 1_u32;
                        for item  in &specs{
                            let sheet = wb.add_worksheet();
                            let _ = sheet.set_name(format!("Spectrum No {spec_iter}"));
                            let _ = sheet.write(0, 0, "Channel");
                            let _ = sheet.write(0, 1, "Energy [keV]");
                            let _ = sheet.write(0, 2, "Magnitude");
                            let _ = sheet.write(0, 3, "Sigma");
                            let _ = sheet.write(0, 4, "FWHM");
                            let _ = sheet.write(0, 5, "Iso");
                            let _ = sheet.write(0, 6, "Comment");
                            let mut row = 1_u32;
                            for roi in item.get_roi(){
                                for peak in roi.get_peaks(){
                                    let _ = sheet.write(row, 0, peak.get_mu().floor());
                                    let _ = sheet.write(row, 1, calib.channel_to_energy(peak.get_mu().clone()));
                                    let _ = sheet.write(row, 2, peak.integral());
                                    let _ = sheet.write(row, 3, peak.get_sigma().clone());
                                    let _ = sheet.write(row, 4, peak.fhwm());
                                    row = row + 1;
                                }
                            }
                            spec_iter = spec_iter + 1;
                        }
                        let local: DateTime<Local> = Local::now();
                        match wb.save(format!("./output/session_{}_{}_{}.xlsx", local.day(), local.month(), local.year())) {
                            Ok(_) => (),
                            Err(e) => {
                                println!("Can`t export to file {e}");
                            }
                        }
                    }
                    _ => {
                        println!("This mode is may be not implemented yet");
                        continue;
                    }
                }
            }
            "help" | "commands" | "list" =>{
                println!("TODO: help text");
                continue;
            }
            "quit" | "q" => {
                println!("Quiting...");
                break;
            }
            comm => {
                println!("Unknow command: {comm}");
                continue;
            }
        }
    }
}

fn is_peaks_in_roi(arg_peaks: &[f64], from: usize, to: usize) -> bool {
    let mut min = arg_peaks[0];
    let mut max = arg_peaks[0];
    for &p in &arg_peaks[1..] {
        if p > max {
            max = p;
        }
        if p < min {
            min = p;
        }
    }
    !((min < from as f64) || (max > to as f64))
}
