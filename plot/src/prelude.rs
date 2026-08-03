#![allow(unused)]
use epy::prelude::*;
use serde::Deserialize;
use std::{collections::HashSet, fmt, path::Path};

#[derive(Clone, Deserialize)]
pub struct Record {
    pub bench: Bench,
    pub dataset: Dataset,
    pub threads: usize,
    pub size: usize,
    runtime: f64,
    rapl: f64,
    #[serde(default)]
    rapl2: f64,
    #[serde(default)]
    uncore: f64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Device {
    CN125,
    CN132,
    Laptop,
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Bench {
    PBBS,
    VQhull,
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Dataset {
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "disk")]
    Disk,
    #[serde(rename = "kuzmin")]
    Kuzmin,
}

impl Record {
    pub fn runtime(&self) -> f64 {
        self.runtime
    }

    pub fn rapl(&self) -> f64 {
        self.rapl + self.rapl2
    }

    pub fn uncore(&self) -> f64 {
        self.uncore
    }

    pub fn bytes_moved(&self) -> f64 {
        use Dataset::*;
        match self.size {
            100_000 => match self.dataset {
                Circle => 74e6,
                Disk => 7.2e6,
                Kuzmin => 8.8e6,
            },
            1_000_000 => match self.dataset {
                Circle => 88e7,
                Disk => 7.2e7,
                Kuzmin => 5.6e7,
            },
            10_000_000 => match self.dataset {
                Circle => 86e8,
                Disk => 7.2e8,
                Kuzmin => 5.6e8,
            },
            100_000_000 => match self.dataset {
                Circle => 73e9,
                Disk => 7.2e9,
                Kuzmin => 5.6e9,
            },
            1_000_000_000 => match self.dataset {
                Circle => 69e10,
                Disk => 7.2e10,
                Kuzmin => 2.4e10,
            },
            _ => panic!("Unexpected size: {}", self.size),
        }
    }

    /// Energy efficiency (GB/J)
    pub fn energy_efficiency(&self) -> f64 {
        if self.rapl() == 0.0 {
            return f64::NAN;
        }
        (self.bytes_moved() / self.rapl()) / 1e9
    }

    /// Bandwidth (GB/s)
    pub fn bandwidth(&self) -> f64 {
        debug_assert!(self.runtime > 0.0);
        (self.bytes_moved() / self.runtime) / 1e9
    }

    /// Bandwidth utilization (%)
    pub fn bandwidth_utilization(&self, device: Device) -> f64 {
        (self.bandwidth() / device.stream_bandwidth()) * 100.
    }
}

impl Device {
    pub fn physical_cores(&self) -> usize {
        use Device::*;
        match self {
            CN125 => 8,
            CN132 => 32,
            Laptop => 6,
        }
    }

    pub fn logical_cores(&self) -> usize {
        use Device::*;
        self.physical_cores() * match self {
            CN125 => 2,
            CN132 => 1,
            Laptop => 2,
        }
    }

    pub fn theoretical_bandwidth(&self) -> f64 {
        use Device::*;
        match self {
            CN125 => 51.2,
            CN132 => 204.8,
            Laptop => 76.8,
        }
    }

    pub fn stream_bandwidth(&self) -> f64 {
        use Device::*;
        match self {
            CN125 => 41.0,
            CN132 => 140.0,
            Laptop => 52.0,
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Device::*;
        match self {
            CN125 => write!(f, "cn125"),
            CN132 => write!(f, "cn132"),
            Laptop => write!(f, "laptop"),
        }
    }
}

pub fn read_csv(filename: &str) -> DataFrame<Record> {
    let path = format!("../results/{}.csv", filename);
    let path = Path::new(&path);
    DataFrame::from_csv(path).unwrap()
}

pub fn unique_sizes(df: &DataFrame<Record>) -> Vec<usize> {
    let mut t = df.rows()
        .iter()
        .map(|r| r.size)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    t.sort_unstable();
    t
}

pub fn unique_threads(df: &DataFrame<Record>) -> Vec<usize> {
    let mut t = df.rows()
        .iter()
        .map(|r| r.threads)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    t.sort_unstable();
    t
}

/// Copy VQhull styles to corresponding PBBS styles
///
/// Note that error bands are included in the axes, so every
/// data point is a tuple of error band and line plot.
pub fn copy_linestyle(ax: &mut Axis) {
    let mut styles = ax.data.iter_mut()
        .filter_map(|e| match e {
            AxisElement::AddPlot { style, .. } => Some(style),
            _ => None,
        })
        .collect::<Vec<_>>();
    // VQhull + PBBS, 3 datasets, line + error band
    debug_assert_eq!(styles.len(), 2 * 3 * 2);
    // Circle
    styles[6].fill = styles[0].fill.clone();
    styles[7].color = styles[1].color.clone();
    styles[7].mark = styles[1].mark.clone();
    styles[7].dashed = true;
    // Disk
    styles[8].fill = styles[2].fill.clone();
    styles[9].color = styles[3].color.clone();
    styles[9].mark = styles[3].mark.clone();
    styles[9].dashed = true;
    // Kuzmin
    styles[10].fill = styles[4].fill.clone();
    styles[11].color = styles[5].color.clone();
    styles[11].mark = styles[5].mark.clone();
    styles[11].dashed = true;
}

pub fn adjust_legend(ax: &mut Axis) {
    let mut i = 0;
    let labels = ["Circle", "Disk", "Kuzmin"];
    ax.data.retain_mut(|e| {
        if let AxisElement::LegendEntry(label) = e {
            if i < labels.len() {
                *label = labels[i].to_string();
                i += 1;
                true
            } else {
                false
            }
        } else {
            true
        }
    });
}

pub fn remove_legend(ax: &mut Axis) {
    ax.data.retain(|e| !matches!(e, AxisElement::LegendEntry(_)));
}
